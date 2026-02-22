use core::f32;
use std::{cell::RefCell, ffi::CString, ops::Deref, path::Path};

use diligent::{geometry_primitives::*, graphics_utilities::*, *};
use diligent_samples::sample_base::{
    first_person_camera::FirstPersonCamera,
    sample::SampleBase,
    sample_app::{self},
};
use diligent_tools::native_app;

use image::EncodableLayout;

#[allow(non_camel_case_types)]
type float2 = [f32; 2];
#[allow(non_camel_case_types)]
type float3 = [f32; 3];
#[allow(non_camel_case_types)]
type float4 = [f32; 4];
#[allow(non_camel_case_types)]
type float4x4 = [f32; 4 * 4];
#[allow(non_camel_case_types)]
type uint4 = [u32; 4];

const NUM_LIGHTS: usize = 2;
const MAX_DISPERS_SAMPLES: usize = 16;
const NUM_TEXTURES: usize = 4;
const NUM_CUBES: usize = 4;
const OPAQUE_GEOM_MASK: usize = 0x01;
const TRANSPARENT_GEOM_MASK: usize = 0x02;
const HIT_GROUP_STRIDE: u32 = 2;
const PRIMARY_RAY_INDEX: u32 = 0;
const SHADOW_RAY_INDEX: u32 = 1;

const COLOR_BUFFER_FORMAT: TextureFormat = TextureFormat::RGBA8_UNORM;

#[repr(C)]
struct Constants {
    // Camera world position
    camera_pos: float4,
    inv_view_proj: float4x4,

    // Near and far clip plane distances
    clip_planes: float2,
    padding0: float2,

    // The number of shadow PCF samples
    shadow_pcf: i32,
    // Maximum ray recursion depth
    max_recursion: i32,
    padding2: float2,

    // Reflection sphere properties
    sphere_reflection_color_mask: float3,
    sphere_reflection_blur: i32,

    // Refraction cube properties
    glass_reflection_color_mask: float3,
    glass_absorption: f32,
    glass_material_color: float4,
    glass_index_of_refraction: float2, // min and max IOR

    glass_enable_dispersion: bool, // We're assuming that the size of bool is 1 byte.
    padding3: [i8; 3],             // This is why we need this padding of 3 bytes.

    dispersion_sample_count: u32,                      // 1..16
    dispersion_samples: [float4; MAX_DISPERS_SAMPLES], // [rgb color] [IOR scale]
    disc_points: [float4; 8],                          // packed float2[16]

    // Light properties
    ambient_color: float4,
    light_pos: [float4; NUM_LIGHTS],
    light_color: [float4; NUM_LIGHTS],
}

#[repr(C)]
struct BoxAttribs {
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
    padding0: f32,
    padding1: f32,
}

struct RayTracing {
    camera: FirstPersonCamera,
    constants: Constants,
    max_recursion_depth: i32,

    image_blit_pso: Boxed<GraphicsPipelineState>,
    image_blit_srb: RefCell<Boxed<ShaderResourceBinding>>,

    ray_tracing_pso: Boxed<RayTracingPipelineState>,
    ray_tracing_srb: RefCell<Boxed<ShaderResourceBinding>>,

    constant_buffer: Boxed<Buffer>,
    _cube_attribs_buffer: Boxed<Buffer>,
    _box_attribs_cb: Boxed<Buffer>,

    cube_blas: Boxed<BottomLevelAS>,
    procedural_blas: Boxed<BottomLevelAS>,

    tlas: Boxed<TopLevelAS>,

    scratch_buffer: Boxed<Buffer>,
    instance_buffer: Boxed<Buffer>,

    sbt: RefCell<Boxed<ShaderBindingTable>>,

    animate: bool,
    enabled_cubes: [bool; 4],

    animation_time: f64,

    color_rt: Boxed<Texture>,

    dispersion_factor: f32,
}

fn create_graphics_pso(
    factory: &EngineFactory,
    device: &RenderDevice,
    swap_chain_desc: &SwapChainDesc,
) -> Boxed<GraphicsPipelineState> {
    // Create graphics pipeline to blit render target into swapchain image.

    let mut rtv_formats = std::array::from_fn(|_| None);
    rtv_formats[0] = swap_chain_desc.color_buffer_format();

    let graphics_pso_desc = GraphicsPipelineDesc::builder()
        .rasterizer_desc(
            RasterizerStateDesc::builder()
                .cull_mode(CullMode::None)
                .build(),
        )
        .depth_stencil_desc(DepthStencilStateDesc::builder().depth_enable(false).build())
        .output(
            GraphicsPipelineRenderTargets::builder()
                .num_render_targets(1)
                .rtv_formats(rtv_formats)
                .build(),
        )
        .primitive_topology(PrimitiveTopology::TriangleStrip)
        .build();

    let shader_source_factory = factory
        .create_default_shader_source_stream_factory(&[])
        .unwrap();

    let shader_ci = ShaderCreateInfo::builder()
        .source_language(ShaderLanguage::HLSL)
        .compiler(ShaderCompiler::DXC)
        .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
        .shader_source_input_stream_factory(&shader_source_factory);

    let vertex_shader = device
        .create_shader(
            &shader_ci
                .clone()
                .shader_type(ShaderType::Vertex)
                .name("Image blit VS")
                .source(ShaderSource::FilePath(Path::new("assets/ImageBlit.vsh")))
                .entry_point("main")
                .build(),
        )
        .unwrap();

    let pixel_shader = device
        .create_shader(
            &shader_ci
                .clone()
                .shader_type(ShaderType::Pixel)
                .name("Image blit PS")
                .source(ShaderSource::FilePath(Path::new("assets/ImageBlit.psh")))
                .entry_point("main")
                .build(),
        )
        .unwrap();

    let pso_ci = PipelineStateCreateInfo::builder()
        .default_variable_type(ShaderResourceVariableType::Dynamic)
        .name(c"Image blit PSO")
        .graphics()
        .graphics_pipeline_desc(graphics_pso_desc)
        .vertex_shader(&vertex_shader)
        .pixel_shader(&pixel_shader)
        .build();

    device.create_graphics_pipeline_state(&pso_ci).unwrap()
}

fn create_ray_tracing_pso(
    engine_factory: &EngineFactory,
    device: &RenderDevice,
) -> Boxed<RayTracingPipelineState> {
    // Create a shader source stream factory to load shaders from files.
    let shader_source_factory = engine_factory
        .create_default_shader_source_stream_factory(&[])
        .unwrap();

    let shader_ci = ShaderCreateInfo::builder()
        // We will not be using combined texture samplers as they
        // are only required for compatibility with OpenGL, and ray
        // tracing is not supported in OpenGL backend.
        .use_combined_texture_samplers(false)
        .macros(vec![("NUM_TEXTURES", format!("{NUM_TEXTURES}"))])
        // Only new DXC compiler can compile HLSL ray tracing shaders.
        .compiler(ShaderCompiler::DXC)
        // Use row-major matrices.
        .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
        // Shader model 6.3 is required for DXR 1.0, shader model 6.5 is required for DXR 1.1 and enables additional features.
        // Use 6.3 for compatibility with DXR 1.0 and VK_NV_ray_tracing.
        .language_version(Version { major: 6, minor: 3 })
        .source_language(ShaderLanguage::HLSL)
        .shader_source_input_stream_factory(&shader_source_factory);

    let ray_gen_shader = device
        .create_shader(
            &shader_ci
                .clone()
                .name("Ray tracing RG")
                .shader_type(ShaderType::RayGen)
                .source(ShaderSource::FilePath(Path::new("assets/RayTrace.rgen")))
                .entry_point("main")
                .build(),
        )
        .unwrap();

    let (primary_miss_shader, shadow_miss_shader) = (
        device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Primary ray miss shader")
                    .shader_type(ShaderType::RayMiss)
                    .source(ShaderSource::FilePath(Path::new(
                        "assets/PrimaryMiss.rmiss",
                    )))
                    .entry_point("main")
                    .build(),
            )
            .unwrap(),
        device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Shadow ray miss shader")
                    .shader_type(ShaderType::RayMiss)
                    .source(ShaderSource::FilePath(Path::new("assets/ShadowMiss.rmiss")))
                    .entry_point("main")
                    .build(),
            )
            .unwrap(),
    );

    let (
        cube_primary_hit_shader,
        ground_hit_shader,
        glass_primary_hit_shader,
        sphere_primary_hit_shader,
    ) = (
        device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Cube primary ray closest hit shader")
                    .shader_type(ShaderType::RayClosestHit)
                    .source(ShaderSource::FilePath(Path::new(
                        "assets/CubePrimaryHit.rchit",
                    )))
                    .entry_point("main")
                    .build(),
            )
            .unwrap(),
        device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Ground primary ray closest hit shader")
                    .shader_type(ShaderType::RayClosestHit)
                    .source(ShaderSource::FilePath(Path::new("assets/Ground.rchit")))
                    .entry_point("main")
                    .build(),
            )
            .unwrap(),
        device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Glass primary ray closest hit shader")
                    .shader_type(ShaderType::RayClosestHit)
                    .source(ShaderSource::FilePath(Path::new(
                        "assets/GlassPrimaryHit.rchit",
                    )))
                    .entry_point("main")
                    .build(),
            )
            .unwrap(),
        device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Sphere primary ray closest hit shader")
                    .shader_type(ShaderType::RayClosestHit)
                    .source(ShaderSource::FilePath(Path::new(
                        "assets/SpherePrimaryHit.rchit",
                    )))
                    .entry_point("main")
                    .build(),
            )
            .unwrap(),
    );

    let sphere_intersection_shader = device
        .create_shader(
            &shader_ci
                .clone()
                .name("Sphere intersection shader")
                .shader_type(ShaderType::RayIntersection)
                .source(ShaderSource::FilePath(Path::new(
                    "assets/SphereIntersection.rint",
                )))
                .entry_point("main")
                .build(),
        )
        .unwrap();

    let max_recursion_depth = device
        .get_adapter_info()
        .ray_tracing()
        .max_recursion_depth()
        .min(8);

    let sampler_desc = SamplerDesc::builder()
        .name(c"Linear Wrap Sampler")
        .min_filter(FilterType::Linear)
        .mag_filter(FilterType::Linear)
        .mip_filter(FilterType::Linear)
        .address_u(TextureAddressMode::Wrap)
        .address_v(TextureAddressMode::Wrap)
        .address_w(TextureAddressMode::Wrap)
        .build();

    let immutable_samplers = [ImmutableSamplerDesc::builder()
        .shader_stages(ShaderTypes::RayClosestHit)
        .sampler_or_texture_name(c"g_SamLinearWrap")
        .sampler_desc(&sampler_desc)
        .build()];

    let shader_resource_variables = [
        ShaderResourceVariableDesc::builder()
            .name(c"g_ConstantsCB")
            .shader_stages(ShaderTypes::RayGen | ShaderTypes::RayMiss | ShaderTypes::RayClosestHit)
            .variable_type(ShaderResourceVariableType::Static)
            .build(),
        ShaderResourceVariableDesc::builder()
            .name(c"g_ColorBuffer")
            .shader_stages(ShaderTypes::RayGen)
            .variable_type(ShaderResourceVariableType::Dynamic)
            .build(),
    ];

    let general_shaders = [
        // Ray generation shader is an entry point for a ray tracing pipeline.
        RayTracingGeneralShaderGroup::builder()
            .name(c"Main")
            .shader(&ray_gen_shader)
            .build(),
        // Primary ray miss shader.
        RayTracingGeneralShaderGroup::builder()
            .name(c"PrimaryMiss")
            .shader(&primary_miss_shader)
            .build(),
        // Shadow ray miss shader.
        RayTracingGeneralShaderGroup::builder()
            .name(c"ShadowMiss")
            .shader(&shadow_miss_shader)
            .build(),
    ];

    let triangle_hit_shaders = [
        // Primary ray hit group for the textured cube.
        RayTracingTriangleHitShaderGroup::builder()
            .name(c"CubePrimaryHit")
            .closest(&cube_primary_hit_shader)
            .build(),
        // Primary ray hit group for the ground.
        RayTracingTriangleHitShaderGroup::builder()
            .name(c"GroundHit")
            .closest(&ground_hit_shader)
            .build(),
        // Primary ray hit group for the glass cube.
        RayTracingTriangleHitShaderGroup::builder()
            .name(c"GlassPrimaryHit")
            .closest(&glass_primary_hit_shader)
            .build(),
    ];

    let procedural_hit_shaders = [
        // Intersection and closest hit shaders for the procedural sphere.
        RayTracingProceduralHitShaderGroup::builder()
            .name(c"SpherePrimaryHit")
            .intersection(&sphere_intersection_shader)
            .closest(&sphere_primary_hit_shader)
            .build(),
        // Only intersection shader is needed for shadows.
        RayTracingProceduralHitShaderGroup::builder()
            .name(c"SphereShadowHit")
            .intersection(&sphere_intersection_shader)
            .build(),
    ];

    let pso_create_info = PipelineStateCreateInfo::builder()
        .shader_resource_variables(&shader_resource_variables)
        .default_variable_type(ShaderResourceVariableType::Mutable)
        .immutable_samplers(&immutable_samplers)
        .name(c"Ray tracing PSO")
        .raytracing()
        .general_shaders(&general_shaders)
        .triangle_hit_shaders(&triangle_hit_shaders)
        .procedural_hit_shaders(&procedural_hit_shaders)
        // Specify the maximum ray recursion depth.
        // WARNING: the driver does not track the recursion depth and it is the
        //          application's responsibility to not exceed the specified limit.
        //          The value is used to reserve the necessary stack size and
        //          exceeding it will likely result in driver crash.
        .max_recursion_depth(max_recursion_depth as u8)
        // Per-shader data is not used.
        .shader_record_size(0)
        // TODO
        // DirectX 12 only: set attribute and payload size. Values should be as small as possible to minimize the memory usage.
        //PSOCreateInfo.MaxAttributeSize = std::max<Uint32>(sizeof(/*BuiltInTriangleIntersectionAttributes*/ float2), sizeof(HLSL::ProceduralGeomIntersectionAttribs));
        //PSOCreateInfo.MaxPayloadSize   = std::max<Uint32>(sizeof(HLSL::PrimaryRayPayload), sizeof(HLSL::ShadowRayPayload));
        .build();

    device
        .create_ray_tracing_pipeline_state(&pso_create_info)
        .unwrap()
}

fn load_textures(device: &RenderDevice) -> ([Boxed<Texture>; NUM_TEXTURES], Boxed<Texture>) {
    let textures = std::array::from_fn(|i| i).map(|tex_id| {
        let image_name = format!("DGLogo{tex_id}");
        let image = image::ImageReader::open(format!("assets/{image_name}.png"))
            .unwrap()
            .decode()
            .unwrap();

        let image_name = CString::new(image_name).unwrap();

        let texture_desc = TextureDesc::builder()
            .name(&image_name)
            .dimension(TextureDimension::Texture2D)
            .width(image.width())
            .height(image.height())
            .format(TextureFormat::RGBA8_UNORM_SRGB)
            .bind_flags(BindFlags::ShaderResource)
            .usage(Usage::Immutable)
            .build();

        let texture_data = TextureSubResource::builder()
            .from_host(
                image.as_bytes(),
                image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
            )
            .build();

        device
            .create_texture(&texture_desc, &[texture_data], None)
            .unwrap()
    });

    let ground = {
        let image = image::ImageReader::open("assets/Ground.jpg")
            .unwrap()
            .decode()
            .unwrap();

        let image = image.to_rgba8();

        let texture_desc = TextureDesc::builder()
            .name(c"Ground")
            .dimension(TextureDimension::Texture2D)
            .width(image.width())
            .height(image.height())
            .format(TextureFormat::RGBA8_UNORM)
            .bind_flags(BindFlags::ShaderResource)
            .usage(Usage::Immutable)
            .build();

        let texture_data = [TextureSubResource::builder()
            .from_host(
                image.as_bytes(),
                image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
            )
            .build()];

        device
            .create_texture(&texture_desc, &texture_data, None)
            .unwrap()
    };

    (textures, ground)
}

fn create_tlas(device: &RenderDevice) -> (Boxed<TopLevelAS>, Boxed<Buffer>, Boxed<Buffer>) {
    const NUM_INSTANCES: usize = NUM_CUBES + 3;

    let tlas = {
        let desc = TopLevelASDesc::builder()
            .name(c"TLAS")
            .max_instance_count(NUM_INSTANCES)
            .flags(RayTracingBuildAsFlags::AllowUpdate | RayTracingBuildAsFlags::PreferFastTrace)
            .build();

        device.create_tlas(&desc).unwrap()
    };

    let scratch_buffer = {
        let sbs = tlas.get_scratch_buffer_sizes();
        let desc = BufferDesc::builder()
            .name(c"TLAS Scratch Buffer")
            .size(u64::max(sbs.build, sbs.update))
            .usage(Usage::Default)
            .bind_flags(BindFlags::RayTracing)
            .build();

        device.create_buffer(&desc).unwrap()
    };

    let instance_buffer = {
        let desc = BufferDesc::builder()
            .size((TLAS_INSTANCE_DATA_SIZE * NUM_INSTANCES as u32) as u64)
            .name(c"TLAS Instance Buffer")
            .usage(Usage::Default)
            .bind_flags(BindFlags::RayTracing)
            .build();

        device.create_buffer(&desc).unwrap()
    };

    (tlas, scratch_buffer, instance_buffer)
}

fn create_and_build_cube_blas(
    device: &RenderDevice,
    immediate_context: &ImmediateDeviceContext,
) -> (Boxed<BottomLevelAS>, Boxed<Buffer>) {
    let (cube_verts, cube_indices, cube_geo_info) = create_geometry_primitive(
        &GeometryPrimitiveAttributes::builder()
            .geometry_type(GeometryPrimitive::Cube { size: 2.0 })
            .vertex_flags(GeometryPrimitiveVertexFlags::All)
            .build(),
    )
    .unwrap();

    assert_eq!(cube_geo_info.vertex_size, std::mem::size_of::<CubeVertex>());

    #[repr(C)]
    struct CubeVertex {
        pos: float3,
        normal: float3,
        uv: float2,
    }

    let verts = cube_verts.get_data_slice::<CubeVertex>(cube_geo_info.num_vertices, 0);
    let indices = cube_indices.get_data_slice::<u32>(cube_geo_info.num_indices, 0);

    // Create a buffer with cube attributes.
    // These attributes will be used in the hit shader to calculate UVs and normal for intersection point.

    let cube_attribs_buffer = {
        #[repr(C)]
        struct CubeAttribs {
            uvs: [float4; 24],
            normals: [float4; 24],
            primitives: [uint4; 12],
        }

        let mut attribs = CubeAttribs {
            uvs: std::array::from_fn(|_| [0.0, 0.0, 0.0, 0.0]),
            normals: std::array::from_fn(|_| [0.0, 0.0, 0.0, 0.0]),
            primitives: std::array::from_fn(|_| [0, 0, 0, 0]),
        };

        for (i, vert) in verts.iter().enumerate() {
            attribs.uvs[i] = [vert.uv[0], vert.uv[1], 0.0, 0.0];
            attribs.normals[i] = [vert.normal[0], vert.normal[1], vert.normal[2], 0.0];
        }

        for (i, ind) in indices.chunks_exact(3).enumerate() {
            attribs.primitives[i] = [ind[0], ind[1], ind[2], 0];
        }

        device
            .create_buffer_with_data(
                &BufferDesc::builder()
                    .name(c"Cube Attribs")
                    .usage(Usage::Immutable)
                    .bind_flags(BindFlags::UniformBuffer)
                    .size(std::mem::size_of_val(&attribs) as u64)
                    .build(),
                &attribs,
                None,
            )
            .unwrap()
    };

    let (mut cube_vertex_buffer, mut cube_index_buffer, _) = create_geometry_primitive_buffers(
        device,
        &GeometryPrimitiveAttributes::builder()
            .geometry_type(GeometryPrimitive::Cube { size: 2.0 })
            .vertex_flags(GeometryPrimitiveVertexFlags::Position)
            .build(),
        &GeometryPrimitiveBuffersCreateInfo::builder()
            .vertex_buffer_bind_flags(BindFlags::RayTracing)
            .index_buffer_bind_flags(BindFlags::RayTracing)
            .build(),
    )
    .unwrap();

    // Create & build bottom level acceleration structure

    let geometry_name = c"Cube";
    let max_vertex_count = cube_geo_info.num_vertices;
    let vertex_value_type = ValueType::Float32;
    let vertex_component_count = 3;
    let max_primitive_count = cube_geo_info.num_indices / 3;
    let index_type = ValueType::Uint32;

    // Create BLAS
    let triangles = BLASTriangleDesc::builder()
        .geometry_name(geometry_name)
        .max_vertex_count(max_vertex_count)
        .vertex_value_type(vertex_value_type)
        .vertex_component_count(vertex_component_count)
        .max_primitive_count(max_primitive_count)
        .index_type(index_type)
        .build();

    let mut blas = device
        .create_blas(
            &BottomLevelASDesc::builder()
                .name(c"Cube BLAS")
                .flags(RayTracingBuildAsFlags::PreferFastTrace)
                .triangles(&[triangles])
                .build(),
        )
        .unwrap();

    // Create scratch buffer
    let mut scratch_buffer = device
        .create_buffer(
            &BufferDesc::builder()
                .name(c"BLAS Scratch Buffer")
                .usage(Usage::Default)
                .bind_flags(BindFlags::RayTracing)
                .size(blas.get_scratch_buffer_sizes().build)
                .build(),
        )
        .unwrap();

    // Build BLAS
    let triangle_data = [BLASBuildTriangleData::builder()
        .geometry_name(geometry_name)
        .vertex_buffer(cube_vertex_buffer.transition_state())
        .vertex_stride(std::mem::size_of::<float3>() as u32)
        .vertex_count(cube_geo_info.num_vertices)
        .vertex_value_type(vertex_value_type)
        .vertex_component_count(vertex_component_count)
        .index_buffer((cube_index_buffer.transition_state(), 0, Some(index_type)))
        .primitive_count(max_primitive_count)
        .flags(RaytracingGeometryFlags::Opaque)
        .build()];

    // Build BLAS
    {
        let attribs = BuildBLASAttribs::builder()
            .blas(blas.transition_state())
            .triangle_data(&triangle_data)
            // Scratch buffer will be used to store temporary data during BLAS build.
            // Previous content in the scratch buffer will be discarded.
            // Allow engine to change resource states.
            .scratch_buffer(scratch_buffer.transition_state())
            .build();

        immediate_context.build_blas(&attribs);
    }

    (blas, cube_attribs_buffer)
}

fn create_and_build_procedural_blas(
    device: &RenderDevice,
    immediate_context: &ImmediateDeviceContext,
) -> (Boxed<BottomLevelAS>, Boxed<Buffer>) {
    //static_assert(sizeof(HLSL::BoxAttribs) % 16 == 0, "BoxAttribs must be aligned by 16 bytes");

    const BOXES: [BoxAttribs; 1] = [BoxAttribs {
        min_x: -2.5,
        min_y: -2.5,
        min_z: -2.5,
        max_x: 2.5,
        max_y: 2.5,
        max_z: 2.5,
        padding0: 0.0,
        padding1: 0.0,
    }];

    // Create box buffer
    let mut box_attribs_cb = {
        let buffer_desc = BufferDesc::builder()
            .name(c"AABB Buffer")
            .usage(Usage::Immutable)
            .bind_flags(BindFlags::RayTracing | BindFlags::ShaderResource)
            .size(std::mem::size_of_val(&BOXES) as u64)
            .element_byte_stride(std::mem::size_of_val(&BOXES[0]) as u32)
            .mode(BufferMode::Structured)
            .build();

        device
            .create_buffer_with_data(&buffer_desc, &BOXES, None)
            .unwrap()
    };

    // Create & build bottom level acceleration structure

    // Create BLAS
    let mut procedural_blas = {
        let box_info = [BLASBoundingBoxDesc::builder()
            .geometry_name(c"Box")
            .max_box_count(1)
            .build()];

        let as_desc = BottomLevelASDesc::builder()
            .name(c"Procedural BLAS")
            .boxes(&box_info)
            .flags(RayTracingBuildAsFlags::PreferFastTrace)
            .build();

        device.create_blas(&as_desc).unwrap()
    };

    // Create scratch buffer
    let mut scratch_buffer = {
        let buff_desc = BufferDesc::builder()
            .name(c"BLAS Scratch Buffer")
            .usage(Usage::Default)
            .bind_flags(BindFlags::RayTracing)
            .size(procedural_blas.get_scratch_buffer_sizes().build)
            .build();

        device.create_buffer(&buff_desc).unwrap()
    };

    // Build BLAS
    {
        let box_data = [BLASBuildBoundingBoxData::builder()
            .geometry_name(c"Box")
            .box_count(1)
            .box_buffer(box_attribs_cb.transition_state())
            .box_stride(std::mem::size_of_val(&BOXES[0]) as u32)
            .build()];

        let attribs = BuildBLASAttribs::builder()
            .blas(procedural_blas.transition_state())
            .box_data(&box_data)
            // Scratch buffer will be used to store temporary data during BLAS build.
            // Previous content in the scratch buffer will be discarded.
            // Allow engine to change resource states.
            .scratch_buffer(scratch_buffer.transition_state())
            .build();

        immediate_context.build_blas(&attribs);
    }

    (procedural_blas, box_attribs_cb)
}

fn create_sbt(
    device: &RenderDevice,
    raytracing_pso: &RayTracingPipelineState,
) -> Boxed<ShaderBindingTable> {
    device
        .create_sbt(
            &ShaderBindingTableDesc::builder()
                .name(c"SBT")
                .raytracing_pso(raytracing_pso)
                .build(),
        )
        .unwrap()
}

impl RayTracing {
    fn update_tlas(&mut self, first_build: bool, context: &DeviceContext) {
        let animate_opaque_cube = |index: usize| -> glam::Mat4 {
            struct CubeInstanceData {
                base_pos: float3,
                time_offset: f32,
            }
            const CUBE_INST_DATA: [CubeInstanceData; NUM_CUBES] = [
                CubeInstanceData {
                    base_pos: [1.0, 1.0, 1.0],
                    time_offset: 0.00,
                },
                CubeInstanceData {
                    base_pos: [2.0, 0.0, -1.0],
                    time_offset: 0.53,
                },
                CubeInstanceData {
                    base_pos: [-1.0, 1.0, 2.0],
                    time_offset: 1.27,
                },
                CubeInstanceData {
                    base_pos: [-2.0, 0.0, -1.0],
                    time_offset: 4.16,
                },
            ];
            let t = f32::sin(self.animation_time as f32 * f32::consts::PI * 0.5)
                + CUBE_INST_DATA[index].time_offset;
            let pos = glam::Vec3::from(CUBE_INST_DATA[index].base_pos) * 2.0
                + glam::Vec3::from([f32::sin(t * 1.13), f32::sin(t * 0.77), f32::sin(t * 2.15)])
                    * 0.5;
            let angle = 0.1
                * f32::consts::PI
                * (self.animation_time as f32 + CUBE_INST_DATA[index].time_offset * 2.0);

            glam::Mat4::from_translation(pos) * glam::Mat4::from_rotation_y(angle)
        };

        let cube_instance = TLASBuildInstanceData::builder().blas(
            // Allow engine to change resource states.
            self.cube_blas.transition_state(),
        );

        let instances = [
            cube_instance
                .clone()
                .instance_name(c"Cube Instance 1")
                .custom_id(0)
                .mask(if self.enabled_cubes[0] {
                    OPAQUE_GEOM_MASK as _
                } else {
                    0
                })
                .transform(
                    animate_opaque_cube(0)
                        .transpose()
                        .to_cols_array()
                        .first_chunk::<12>()
                        .unwrap(),
                )
                .build(),
            cube_instance
                .clone()
                .instance_name(c"Cube Instance 2")
                .custom_id(1)
                .mask(if self.enabled_cubes[1] {
                    OPAQUE_GEOM_MASK as _
                } else {
                    0
                })
                .transform(
                    animate_opaque_cube(1)
                        .transpose()
                        .to_cols_array()
                        .first_chunk::<12>()
                        .unwrap(),
                )
                .build(),
            cube_instance
                .clone()
                .instance_name(c"Cube Instance 3")
                .custom_id(2)
                .mask(if self.enabled_cubes[2] {
                    OPAQUE_GEOM_MASK as _
                } else {
                    0
                })
                .transform(
                    animate_opaque_cube(2)
                        .transpose()
                        .to_cols_array()
                        .first_chunk::<12>()
                        .unwrap(),
                )
                .build(),
            cube_instance
                .clone()
                .instance_name(c"Cube Instance 4")
                .custom_id(3)
                .mask(if self.enabled_cubes[3] {
                    OPAQUE_GEOM_MASK as _
                } else {
                    0
                })
                .transform(
                    animate_opaque_cube(3)
                        .transpose()
                        .to_cols_array()
                        .first_chunk::<12>()
                        .unwrap(),
                )
                .build(),
            cube_instance
                .clone()
                .instance_name(c"Ground Instance")
                .mask(OPAQUE_GEOM_MASK as _)
                .transform(
                    (glam::Mat4::from_translation(glam::vec3(0.0, -6.0, 0.0))
                        * glam::Mat4::from_scale(glam::vec3(100.0, 0.1, 100.0)))
                    .transpose()
                    .to_cols_array()
                    .first_chunk::<12>()
                    .unwrap(),
                )
                .build(),
            TLASBuildInstanceData::builder()
                .instance_name(c"Sphere Instance")
                .custom_id(0)
                .blas(self.procedural_blas.transition_state())
                .mask(OPAQUE_GEOM_MASK as _)
                .transform(
                    glam::Mat4::from_translation(glam::vec3(-3.0, -3.0, -5.0))
                        .transpose()
                        .to_cols_array()
                        .first_chunk::<12>()
                        .unwrap(),
                )
                .build(),
            cube_instance
                .instance_name(c"Glass Instance")
                .mask(TRANSPARENT_GEOM_MASK as _)
                .transform(
                    (glam::Mat4::from_translation(glam::vec3(3.0, -4.0, -5.0))
                        * glam::Mat4::from_scale(glam::vec3(1.5, 1.5, 1.5))
                        * glam::Mat4::from_rotation_y(
                            self.animation_time as f32 * f32::consts::PI * 0.25,
                        ))
                    .transpose()
                    .to_cols_array()
                    .first_chunk::<12>()
                    .unwrap(),
                )
                .build(),
        ];

        // Build or update TLAS
        let attribs = BuildTLASAttribs::builder()
            .tlas(self.tlas.transition_state())
            .update(!first_build)
            // Scratch buffer will be used to store temporary data during TLAS build or update.
            // Previous content in the scratch buffer will be discarded.
            .scratch_buffer(self.scratch_buffer.transition_state())
            // Instance buffer will store instance data during TLAS build or update.
            // Previous content in the instance buffer will be discarded.
            .instance_buffer(self.instance_buffer.transition_state())
            // Instances will be converted to the format that is required by the graphics driver and copied to the instance buffer.
            .instances(instances.as_slice())
            // Bind hit shaders per instance, it allows you to change the number of geometries in BLAS without invalidating the shader binding table.
            .binding_mode(HitGroupBindingMode::PerInstance)
            .hit_group_stride(HIT_GROUP_STRIDE)
            .build();

        context.build_tlas(&attribs);
    }
}

impl SampleBase for RayTracing {
    fn new(
        engine_factory: &EngineFactory,
        device: &RenderDevice,
        main_context: &ImmediateDeviceContext,
        _immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        _deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        swap_chain_descs: &[&SwapChainDesc],
    ) -> Self {
        // We are only using one swap chain
        let swap_chain_desc = swap_chain_descs[0];

        if !device
            .get_adapter_info()
            .ray_tracing()
            .cap_flags()
            .contains(RaytracingCapFlags::StandaloneShaders)
        {
            panic!("Ray tracing shaders are not supported by device")
        }

        // Create a buffer with shared constants.
        let buffer_desc = BufferDesc::builder()
            .name(c"Constant buffer")
            .size(std::mem::size_of::<Constants>() as u64)
            .usage(Usage::Default)
            .bind_flags(BindFlags::UniformBuffer)
            .build();

        let constant_buffer = device.create_buffer(&buffer_desc).unwrap();

        let image_blit_pso = create_graphics_pso(engine_factory, device, swap_chain_desc);
        let image_blit_srb = image_blit_pso.create_shader_resource_binding(true).unwrap();

        let ray_tracing_pso = create_ray_tracing_pso(engine_factory, device);

        ray_tracing_pso
            .get_static_variable_by_name(ShaderType::RayGen, "g_ConstantsCB")
            .unwrap()
            .set(&constant_buffer, SetShaderResourceFlags::None);
        ray_tracing_pso
            .get_static_variable_by_name(ShaderType::RayMiss, "g_ConstantsCB")
            .unwrap()
            .set(&constant_buffer, SetShaderResourceFlags::None);
        ray_tracing_pso
            .get_static_variable_by_name(ShaderType::RayClosestHit, "g_ConstantsCB")
            .unwrap()
            .set(&constant_buffer, SetShaderResourceFlags::None);

        let ray_tracing_srb = ray_tracing_pso
            .create_shader_resource_binding(true)
            .unwrap();

        {
            let (logos, ground) = load_textures(device);

            // Get shader resource view from the texture array
            let logo_srvs = logos.each_ref().map(|texture| {
                texture
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap()
            });

            let ground_srv = ground
                .get_default_view(TextureViewType::ShaderResource)
                .unwrap();

            ray_tracing_srb
                .get_variable_by_name("g_CubeTextures", ShaderTypes::RayClosestHit)
                .unwrap()
                .set_array(
                    logo_srvs.map(|srv| srv.deref()).as_slice(),
                    SetShaderResourceFlags::None,
                );

            ray_tracing_srb
                .get_variable_by_name("g_GroundTexture", ShaderTypes::RayClosestHit)
                .unwrap()
                .set(ground_srv, SetShaderResourceFlags::None);
        }

        let (cube_blas, cube_attribs_buffer) = create_and_build_cube_blas(device, main_context);

        ray_tracing_srb
            .get_variable_by_name("g_CubeAttribsCB", ShaderTypes::RayClosestHit)
            .unwrap()
            .set(&cube_attribs_buffer, SetShaderResourceFlags::None);

        let (procedural_blas, box_attribs_cb) =
            create_and_build_procedural_blas(device, main_context);

        ray_tracing_srb
            .get_variable_by_name("g_BoxAttribs", ShaderTypes::RayIntersection)
            .unwrap()
            .set(
                box_attribs_cb
                    .get_default_view(BufferViewType::ShaderResource)
                    .unwrap(),
                SetShaderResourceFlags::None,
            );

        let (tlas, scratch_buffer, instance_buffer) = create_tlas(device);
        ray_tracing_srb
            .get_variable_by_name("g_TLAS", ShaderTypes::RayGen)
            .unwrap()
            .set(&tlas, SetShaderResourceFlags::None);
        ray_tracing_srb
            .get_variable_by_name("g_TLAS", ShaderTypes::RayClosestHit)
            .unwrap()
            .set(&tlas, SetShaderResourceFlags::None);

        let sbt = create_sbt(device, &ray_tracing_pso);
        {
            sbt.bind_ray_gen_shader("Main");
            sbt.bind_miss_shader("PrimaryMiss", PRIMARY_RAY_INDEX);
            sbt.bind_miss_shader("ShadowMiss", SHADOW_RAY_INDEX);
        }

        let max_recursion_depth = 8;

        let near = 0.1;
        let far = 100.0;
        let aspect_ratio = swap_chain_desc.width() as f32 / swap_chain_desc.height() as f32;

        let mut camera = FirstPersonCamera::new(
            &glam::vec3(1.0, 0.0, 0.0),
            &glam::vec3(0.0, 1.0, 0.0),
            true,
            near,
            far,
            aspect_ratio,
            f32::consts::PI / 4.0,
            swap_chain_desc.pre_transform(),
        );

        let initial_position = glam::vec3(7.0, -0.5, -16.5);

        camera.set_pos(&initial_position);
        camera.set_rotation(0.48, -0.145);
        camera.set_rotation_speed(0.005);
        camera.set_speed_up_scales(5.0, 10.0);

        let inv_view_proj = camera.projection_matrix().inverse();

        let texture_desc = TextureDesc::builder()
            .name(c"Color buffer")
            .dimension(TextureDimension::Texture2D)
            .width(swap_chain_desc.width())
            .height(swap_chain_desc.height())
            .bind_flags(BindFlags::UnorderedAccess | BindFlags::ShaderResource)
            .format(COLOR_BUFFER_FORMAT)
            .build();

        let color_rt = device.create_texture(&texture_desc, &[], None).unwrap();

        let mut sample = Self {
            animate: true,
            camera,
            enabled_cubes: [true, true, true, true],
            max_recursion_depth,
            constants: Constants {
                clip_planes: [0.1, 100.0],
                shadow_pcf: 1,
                max_recursion: i32::min(6, max_recursion_depth),

                // Sphere constants.
                sphere_reflection_color_mask: [0.81, 1.0, 0.45],
                sphere_reflection_blur: 1,

                // Glass cube constants.
                glass_reflection_color_mask: [0.22, 0.83, 0.93],
                glass_absorption: 0.5,
                glass_material_color: [0.33, 0.93, 0.29, 1.0],
                glass_index_of_refraction: [1.5, 1.02],
                glass_enable_dispersion: false,
                padding3: [0, 0, 0],

                // Wavelength to RGB and index of refraction interpolation factor.
                dispersion_samples: [
                    [0.140000, 0.000000, 0.266667, 0.53],
                    [0.130031, 0.037556, 0.612267, 0.25],
                    [0.100123, 0.213556, 0.785067, 0.16],
                    [0.050277, 0.533556, 0.785067, 0.00],
                    [0.000000, 0.843297, 0.619682, 0.13],
                    [0.000000, 0.927410, 0.431834, 0.38],
                    [0.000000, 0.972325, 0.270893, 0.27],
                    [0.000000, 0.978042, 0.136858, 0.19],
                    [0.324000, 0.944560, 0.029730, 0.47],
                    [0.777600, 0.871879, 0.000000, 0.64],
                    [0.972000, 0.762222, 0.000000, 0.77],
                    [0.971835, 0.482222, 0.000000, 0.62],
                    [0.886744, 0.202222, 0.000000, 0.73],
                    [0.715967, 0.000000, 0.000000, 0.68],
                    [0.459920, 0.000000, 0.000000, 0.91],
                    [0.218000, 0.000000, 0.000000, 0.99],
                ],
                dispersion_sample_count: 4,

                ambient_color: [0.015, 0.015, 0.015, 0.0],
                light_pos: [[8.00, 8.0, 0.00, 0.0], [0.00, 4.0, -5.00, 0.0]],
                light_color: [[1.00, 0.8, 0.80, 0.0], [0.85, 1.0, 0.85, 0.0]],

                // Random points on disc.
                disc_points: [
                    [0.0, 0.0, 0.9, -0.9],
                    [-0.8, 1.0, -1.1, -0.8],
                    [1.5, 1.2, -2.1, 0.7],
                    [0.1, -2.2, -0.2, 2.4],
                    [2.4, -0.3, -3.0, 2.8],
                    [2.0, -2.6, 0.7, 3.5],
                    [-3.2, -1.6, 3.4, 2.2],
                    [-1.8, -3.2, -1.1, 3.6],
                ],

                camera_pos: [
                    initial_position.x,
                    initial_position.y,
                    initial_position.z,
                    1.0,
                ],
                inv_view_proj: inv_view_proj.to_cols_array(),
                padding0: [0.0, 0.0],
                padding2: [0.0, 0.0],
            },

            animation_time: 0.0,

            _cube_attribs_buffer: cube_attribs_buffer,
            _box_attribs_cb: box_attribs_cb,
            constant_buffer,

            image_blit_pso,
            image_blit_srb: RefCell::new(image_blit_srb),
            ray_tracing_pso,
            ray_tracing_srb: RefCell::new(ray_tracing_srb),
            sbt: RefCell::new(sbt),
            tlas,
            scratch_buffer,
            instance_buffer,
            cube_blas,
            procedural_blas,

            color_rt,

            dispersion_factor: 0.1,
        };

        sample.update_tlas(true, main_context);

        // Hit groups for primary ray
        {
            let sbt = sample.sbt.borrow();
            sbt.bind_hit_group_for_instance(
                &sample.tlas,
                "Cube Instance 1",
                PRIMARY_RAY_INDEX,
                Some("CubePrimaryHit"),
            );
            sbt.bind_hit_group_for_instance(
                &sample.tlas,
                "Cube Instance 2",
                PRIMARY_RAY_INDEX,
                Some("CubePrimaryHit"),
            );
            sbt.bind_hit_group_for_instance(
                &sample.tlas,
                "Cube Instance 3",
                PRIMARY_RAY_INDEX,
                Some("CubePrimaryHit"),
            );
            sbt.bind_hit_group_for_instance(
                &sample.tlas,
                "Cube Instance 4",
                PRIMARY_RAY_INDEX,
                Some("CubePrimaryHit"),
            );
            sbt.bind_hit_group_for_instance(
                &sample.tlas,
                "Ground Instance",
                PRIMARY_RAY_INDEX,
                Some("GroundHit"),
            );
            sbt.bind_hit_group_for_instance(
                &sample.tlas,
                "Glass Instance",
                PRIMARY_RAY_INDEX,
                Some("GlassPrimaryHit"),
            );
            sbt.bind_hit_group_for_instance(
                &sample.tlas,
                "Sphere Instance",
                PRIMARY_RAY_INDEX,
                Some("SpherePrimaryHit"),
            );
        }

        // Hit groups for shadow ray.
        {
            {
                let sbt = sample.sbt.borrow();
                // None means no shaders are bound and hit shader invocation will be skipped.
                sbt.bind_hit_group_for_tlas(&sample.tlas, SHADOW_RAY_INDEX, None::<&str>);

                // We must specify the intersection shader for procedural geometry.
                sbt.bind_hit_group_for_instance(
                    &sample.tlas,
                    "Sphere Instance",
                    SHADOW_RAY_INDEX,
                    Some("SphereShadowHit"),
                );
            }

            // Update SBT with the shader groups we bound
            main_context.update_sbt(&mut sample.sbt.borrow_mut());
        }

        sample
    }

    fn modify_engine_init_info(
        engine_ci: &mut diligent_samples::sample_base::sample::EngineCreateInfo,
    ) {
        engine_ci
            .features
            .set_ray_tracing(DeviceFeatureState::Enabled);
    }

    fn get_name() -> &'static str {
        "Tutorial 21 : Ray Tracing"
    }

    fn render(
        &self,
        main_context: Boxed<ImmediateDeviceContext>,
        mut swap_chain: Boxed<SwapChain>,
    ) -> (Boxed<ImmediateDeviceContext>, Boxed<SwapChain>) {
        // Trace rays
        let main_context = {
            self.ray_tracing_srb
                .borrow()
                .get_variable_by_name("g_ColorBuffer", ShaderTypes::RayGen)
                .unwrap()
                .set(
                    self.color_rt
                        .get_default_view(TextureViewType::UnorderedAccess)
                        .unwrap(),
                    SetShaderResourceFlags::None,
                );

            let ray_tracing = main_context.set_ray_tracing_pipeline_state(&self.ray_tracing_pso);

            ray_tracing
                .commit_shader_resources(self.ray_tracing_srb.borrow_mut().transition_state());

            let swap_chain_desc = swap_chain.desc();

            let mut sbt = self.sbt.borrow_mut();
            let attribs = TraceRaysAttribs::builder()
                .sbt(&mut sbt)
                .dimension_x(swap_chain_desc.width())
                .dimension_y(swap_chain_desc.height())
                .build();

            ray_tracing.trace_rays(&attribs);

            ray_tracing.finish()
        };

        // Blit to swapchain image
        {
            self.image_blit_srb
                .borrow()
                .get_variable_by_name("g_Texture", ShaderTypes::Pixel)
                .unwrap()
                .set(
                    self.color_rt
                        .get_default_view(TextureViewType::ShaderResource)
                        .unwrap(),
                    SetShaderResourceFlags::None,
                );

            let rtv = swap_chain.get_current_back_buffer_rtv_mut().unwrap();
            main_context.set_render_targets(&[rtv.transition_state()], None);

            let blit = main_context.set_graphics_pipeline_state(&self.image_blit_pso);
            blit.commit_shader_resources(self.image_blit_srb.borrow_mut().transition_state());

            blit.draw(
                &DrawAttribs::builder()
                    .num_vertices(3)
                    .flags(DrawFlags::VerifyAll)
                    .build(),
            );

            (blit.finish(), swap_chain)
        }
    }

    fn update(
        &mut self,
        main_context: &ImmediateDeviceContext,
        _current_time: f64,
        elapsed_time: f64,
    ) {
        const MAX_ANIMATION_TIME_DELTA: f64 = 1.0 / 60.0;

        self.animation_time += f64::min(MAX_ANIMATION_TIME_DELTA, elapsed_time);

        self.camera.update(elapsed_time);

        // Do not allow going underground
        let mut pos = self.camera.position();
        if pos.y < -5.7 {
            pos.y = -5.7;
            self.camera.set_pos(&pos);
        }

        // Update constants
        {
            let camera_world_pos = self.camera.world_matrix().col(3);
            let camera_view_proj = *self.camera.projection_matrix() * *self.camera.view_matrix();

            self.constants.camera_pos = camera_world_pos.to_array();
            self.constants.inv_view_proj = camera_view_proj.inverse().to_cols_array();

            main_context.update_buffer(
                self.constant_buffer.transition_state(),
                0,
                std::mem::size_of::<Constants>() as u64,
                &self.constants,
            );
        }

        self.update_tlas(false, main_context);
    }

    fn update_ui(
        &mut self,
        _device: &RenderDevice,
        _main_context: &ImmediateDeviceContext,
        ui: &mut imgui::Ui,
    ) {
        const MAX_INDEX_OF_REFRACTION: f32 = 2.0;
        const MAX_DISPERSION: f32 = 0.5;

        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .begin()
        {
            let _ = ui.checkbox("Animate", &mut self.animate);

            ui.text("Use WASD to move camera");

            ui.slider("Shadow blur", 0, 16, &mut self.constants.shadow_pcf);
            ui.slider(
                "Max recursion",
                0,
                self.max_recursion_depth,
                &mut self.constants.max_recursion,
            );

            {
                let num_cubes = self.enabled_cubes.len();
                for (i, enable_cube) in self.enabled_cubes.iter_mut().enumerate() {
                    ui.checkbox(format!("Cube {i}"), enable_cube);
                    if i + 1 < num_cubes {
                        ui.same_line();
                    }
                }
            }

            ui.separator();

            ui.text("Glass cube");
            ui.checkbox("Dispersion", &mut self.constants.glass_enable_dispersion);

            ui.slider(
                "Index of refraction",
                1.0,
                MAX_INDEX_OF_REFRACTION,
                &mut self.constants.glass_index_of_refraction[0],
            );

            if self.constants.glass_enable_dispersion {
                ui.slider(
                    "Dispersion factor",
                    0.0,
                    MAX_DISPERSION,
                    &mut self.dispersion_factor,
                );

                // TODO
                //int rsamples = PlatformMisc::GetLSB(m_Constants.DispersionSampleCount);
                //ImGui::SliderInt("Dispersion samples", &rsamples, 1, PlatformMisc::GetLSB(Uint32{MAX_DISPERS_SAMPLES}), std::to_string(1 << rsamples).c_str());
                //m_Constants.DispersionSampleCount = 1u << rsamples;
            }

            ui.color_edit3(
                "Reflection color",
                &mut self.constants.glass_reflection_color_mask,
            );

            ui.color_edit3(
                "Material color",
                self.constants
                    .glass_material_color
                    .first_chunk_mut::<3>()
                    .unwrap(),
            );

            ui.slider("Absorption", 0.0, 2.0, &mut self.constants.glass_absorption);

            ui.separator();

            ui.text("Sphere");

            ui.slider(
                "Reflection blur",
                1,
                16,
                &mut self.constants.sphere_reflection_blur,
            );

            ui.color_edit3(
                "Color mask",
                &mut self.constants.sphere_reflection_color_mask,
            );
        }
    }

    fn window_resize(&mut self, device: &RenderDevice, new_swap_chain: &SwapChainDesc) {
        let aspect_ratio = new_swap_chain.width() as f32 / new_swap_chain.height() as f32;
        self.camera.set_projection_attribs(
            self.constants.clip_planes[0],
            self.constants.clip_planes[1],
            aspect_ratio,
            f32::consts::PI / 4.0,
            new_swap_chain.pre_transform(),
        );

        let texture_desc = TextureDesc::builder()
            .name(c"Color buffer")
            .dimension(TextureDimension::Texture2D)
            .width(new_swap_chain.width())
            .height(new_swap_chain.height())
            .bind_flags(BindFlags::UnorderedAccess | BindFlags::ShaderResource)
            .format(COLOR_BUFFER_FORMAT)
            .build();

        self.color_rt = device.create_texture(&texture_desc, &[], None).unwrap();
    }

    fn handle_event(&mut self, event: native_app::events::Event) {
        self.camera.apply_event(&event);
    }
}

fn main() {
    sample_app::main::<RayTracing>().unwrap()
}
