use std::{cell::RefCell, collections::HashMap, path::Path};

use diligent::{
    geometry_primitives::GeometryPrimitiveVertexFlags, graphics_utilities::create_uniform_buffer, *,
};
use diligent_samples::{
    GetDeviceType,
    sample_base::{
        sample::{SampleBase, get_adjusted_projection_matrix, get_surface_pretransform_matrix},
        sample_app,
    },
    textured_cube::TexturedCube,
};
use rand::distr::uniform::{UniformFloat, UniformSampler};

#[allow(non_camel_case_types)]
type float3 = [f32; 3];
#[allow(non_camel_case_types)]
type float4x4 = [f32; 4 * 4];
#[allow(non_camel_case_types)]
type float4 = [f32; 4];

#[repr(C)]
#[derive(Clone)]
struct LightAttribs {
    location: glam::Vec3,
    size: f32,
    color: float3,
}

struct GBuffer {
    color_buffer: Boxed<Texture>,
    depth_z_buffer: Boxed<Texture>,
    depth_buffer: Boxed<Texture>,
    opengl_offsreen_color_buffer: RefCell<Option<Boxed<Texture>>>,
}

const GRID_DIM: i32 = 7;

// Use 16-bit format to make sure it works on mobile devices
const DEPTH_BUFFER_FORMAT: TextureFormat = TextureFormat::D16_UNORM;

type LightMoveDir = glam::Vec3;

struct RenderPasses {
    // Cube resources
    cube_pso: Boxed<GraphicsPipelineState>,
    cube_srb: Boxed<ShaderResourceBinding>,
    textured_cube: TexturedCube,
    shader_constants_cb: Boxed<Buffer>,
    _cube_texture_srv: Boxed<TextureView>,

    lights_buffer: Boxed<Buffer>,

    light_volume_pso: Boxed<GraphicsPipelineState>,
    light_volume_srb: Boxed<ShaderResourceBinding>,
    ambient_light_pso: Boxed<GraphicsPipelineState>,
    ambient_light_srb: Boxed<ShaderResourceBinding>,

    g_buffer: GBuffer,

    render_pass: Boxed<RenderPass>,

    lights_count: i32,
    show_light_volumes: bool,
    animate_lights: bool,

    lights: Vec<LightAttribs>,
    light_move_dirs: Vec<glam::Vec3>,

    is_device_gl: bool,

    framebuffer_cache: RefCell<HashMap<*const TextureView, Boxed<Framebuffer>>>,

    // HACK
    device: *const RenderDevice,
}

#[repr(C)]
struct Constants {
    view_proj: float4x4,
    view_proj_inv: float4x4,
    viewport_size: float4,

    show_light_volumes: i32,
    padding0: i32,
    padding1: i32,
    padding2: i32,
}

fn init_lights(lights_count: u32) -> Vec<(LightAttribs, LightMoveDir)> {
    // Randomly distribute lights within the volume
    let mut rng = rand::rng();

    let rnd = UniformFloat::<f32>::new(0.0, 1.0).unwrap();

    let mut lights = Vec::with_capacity(lights_count as usize);

    for _ in 0..lights_count {
        let location = glam::Vec3 {
            x: rnd.sample(&mut rng),
            y: rnd.sample(&mut rng),
            z: rnd.sample(&mut rng),
        }
        .map(|v| (v - 0.5) * 2.0 * GRID_DIM as f32);
        lights.push((
            LightAttribs {
                color: [
                    rnd.sample(&mut rng),
                    rnd.sample(&mut rng),
                    rnd.sample(&mut rng),
                ],
                location,
                size: 0.25 + rnd.sample(&mut rng) * 0.25,
            },
            glam::Vec3 {
                x: rnd.sample(&mut rng),
                y: rnd.sample(&mut rng),
                z: rnd.sample(&mut rng),
            }
            .map(|v| v - 0.5),
        ));
    }

    lights
}

fn create_cube_pso(
    device: &RenderDevice,
    shader_source_factory: &ShaderSourceInputStreamFactory,
    render_pass: &RenderPass,
) -> Boxed<GraphicsPipelineState> {
    let shader_ci = ShaderCreateInfo::builder()
        .entry_point("main")
        // Tell the system that the shader source code is in HLSL.
        // For OpenGL, the engine will convert this into GLSL under the hood.
        .source_language(ShaderLanguage::HLSL)
        // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
        .use_combined_texture_samplers(true)
        // Pack matrices in row-major order
        .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
        .shader_source_input_stream_factory(shader_source_factory);

    // Create a vertex shader
    let vertex_shader = {
        let shader_ci = shader_ci
            .clone()
            .name("Cube VS")
            .source(ShaderSource::FilePath(Path::new(
                "assets/render_passes_cube.vsh",
            )))
            .shader_type(ShaderType::Vertex)
            .build();

        device.create_shader(&shader_ci).unwrap()
    };

    // Create a pixel shader
    let pixel_shader = {
        let shader_ci = shader_ci
            .clone()
            .name("Cube PS")
            .source(ShaderSource::FilePath(Path::new(
                "assets/render_passes_cube.psh",
            )))
            .shader_type(ShaderType::Pixel)
            .build();

        device.create_shader(&shader_ci).unwrap()
    };

    // Define immutable sampler for g_Texture. Immutable samplers should be used whenever possible
    let sampler_desc = SamplerDesc::builder()
        .name(c"Cube texture sampler")
        .min_filter(FilterType::Linear)
        .mag_filter(FilterType::Linear)
        .mip_filter(FilterType::Linear)
        .address_u(TextureAddressMode::Clamp)
        .address_v(TextureAddressMode::Clamp)
        .address_w(TextureAddressMode::Clamp)
        .build();

    let rasterizer_desc = RasterizerStateDesc::builder()
        .cull_mode(CullMode::Back)
        .build();

    let depth_desc = DepthStencilStateDesc::builder().depth_enable(true).build();

    // This PSO will be used within the first subpass
    // When pRenderPass is not null, all RTVFormats and DSVFormat must be TEX_FORMAT_UNKNOWN,
    // while NumRenderTargets must be 0
    let pipeline_output = GraphicsPipelineRenderPass::new(render_pass, 0);

    let shader_resource_variables = [ShaderResourceVariableDesc::builder()
        .name(c"g_Texture")
        .variable_type(ShaderResourceVariableType::Mutable)
        .shader_stages(ShaderTypes::Pixel)
        .build()];

    let immutable_samplers = [ImmutableSamplerDesc::builder()
        .shader_stages(ShaderTypes::Pixel)
        .sampler_or_texture_name(c"g_Texture")
        .sampler_desc(&sampler_desc)
        .build()];

    let input_layouts = input_layouts![
        // Attribute 0 - vertex position
        LayoutElement::builder().slot(0).f32_3(),
        // Attribute 1 - texture coordinates
        LayoutElement::builder().slot(0).f32_2(),
    ];

    // Pipeline state object encompasses configuration of all GPU stages
    let pso_create_info = PipelineStateCreateInfo::builder()
        // Define variable type that will be used by default
        .default_variable_type(ShaderResourceVariableType::Static)
        // Shader variables should typically be mutable, which means they are expected
        // to change on a per-instance basis
        .shader_resource_variables(&shader_resource_variables)
        .immutable_samplers(&immutable_samplers)
        // Pipeline state name is used by the engine to report issues.
        .name(c"Cube PSO")
        .graphics()
        .graphics_pipeline_desc(
            GraphicsPipelineDesc::builder()
                .rasterizer_desc(rasterizer_desc)
                .depth_stencil_desc(depth_desc)
                .output(pipeline_output)
                // Primitive topology defines what kind of primitives will be rendered by this pipeline state
                .primitive_topology(PrimitiveTopology::TriangleList)
                // Define vertex shader input layout
                .input_layouts(&input_layouts)
                .build(),
        )
        .vertex_shader(&vertex_shader)
        .pixel_shader(&pixel_shader)
        .build();

    device
        .create_graphics_pipeline_state(&pso_create_info)
        .unwrap()
}

fn create_light_volume_pso(
    device: &RenderDevice,
    shader_source_factory: &ShaderSourceInputStreamFactory,
    render_pass: &RenderPass,
    convert_ps_output_to_gamma: bool,
) -> Boxed<GraphicsPipelineState> {
    let shader_ci = ShaderCreateInfo::builder()
        .entry_point("main")
        // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
        .use_combined_texture_samplers(true)
        // Pack matrices in row-major order
        .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
        // Presentation engine always expects input in gamma space. Normally, pixel shader output is
        // converted from linear to gamma space by the GPU. However, some platforms (e.g. Android in GLES mode,
        // or Emscripten in WebGL mode) do not support gamma-correction. In this case the application
        // has to do the conversion manually.
        .macros(vec![(
            "CONVERT_PS_OUTPUT_TO_GAMMA",
            if convert_ps_output_to_gamma { "1" } else { "0" },
        )])
        .shader_source_input_stream_factory(shader_source_factory);

    // Create a vertex shader
    let vertex_shader = {
        let shader_ci = shader_ci
            .clone()
            .source_language(ShaderLanguage::HLSL)
            .name("Light volume VS")
            .source(ShaderSource::FilePath(Path::new("assets/light_volume.vsh")))
            .shader_type(ShaderType::Vertex)
            .build();

        device.create_shader(&shader_ci).unwrap()
    };

    // Create a pixel shader
    let pixel_shader = {
        let device_type = device.get_device_info().device_type();

        let use_glsl = device_type.is_vulkan() || device_type.is_metal();

        let shader_ci = shader_ci
            .clone()
            .source_language(if use_glsl {
                ShaderLanguage::GLSL
            } else {
                ShaderLanguage::HLSL
            })
            .name("Light volume PS")
            .source(ShaderSource::FilePath(Path::new(if use_glsl {
                "assets/light_volume_glsl.psh"
            } else {
                "assets/light_volume_hlsl.psh"
            })))
            .maybe_glsl_extensions(if use_glsl {
                Some("#extension GL_ARB_shading_language_include : enable\n")
            } else {
                None
            })
            .shader_type(ShaderType::Pixel)
            .build();

        device.create_shader(&shader_ci).unwrap()
    };

    let rasterizer_desc = RasterizerStateDesc::builder()
        // Cull back faces
        .cull_mode(CullMode::Back)
        .build();

    let depth_desc = DepthStencilStateDesc::builder()
        .depth_enable(true)
        .depth_write_enable(false) // Do not write depth
        .build();

    // This PSO will be used within the second subpass
    let pipeline_output = GraphicsPipelineRenderPass::new(render_pass, 1);

    let shader_resource_variables = [
        ShaderResourceVariableDesc::builder()
            .name(c"g_SubpassInputColor")
            .variable_type(ShaderResourceVariableType::Mutable)
            .shader_stages(ShaderTypes::Pixel)
            .build(),
        ShaderResourceVariableDesc::builder()
            .name(c"g_SubpassInputDepthZ")
            .variable_type(ShaderResourceVariableType::Mutable)
            .shader_stages(ShaderTypes::Pixel)
            .build(),
    ];

    #[rustfmt::skip]
    let input_layouts = input_layouts![
        // Attribute 0 - vertex position
        LayoutElement::builder().slot(0).f32_3(),
        // Attribute 1 - texture coordinates (we don't use them)
        LayoutElement::builder().slot(0).f32_2(),
        // Attribute 2 - light position
        LayoutElement::builder().slot(1).f32_4().frequency(InputElementFrequency::PerInstance),
        // Attribute 3 - light color
        LayoutElement::builder().slot(1).f32_3().frequency(InputElementFrequency::PerInstance),
    ];

    let mut blend_targets = std::array::from_fn(|_| RenderTargetBlendDesc::default());
    blend_targets[0] = RenderTargetBlendDesc::builder()
        .blend_enable(true)
        .blend_op(BlendOperation::Add)
        .src_blend(BlendFactor::One)
        .dest_blend(BlendFactor::One)
        .src_blend_alpha(BlendFactor::Zero)
        .dest_blend_alpha(BlendFactor::One)
        .build();

    // We will use alpha-blending to accumulate influence of all lights
    let blend_desc = BlendStateDesc::builder()
        .render_targets(blend_targets)
        .build();

    let pso_create_info = PipelineStateCreateInfo::builder()
        // Define variable type that will be used by default
        .default_variable_type(ShaderResourceVariableType::Static)
        .shader_resource_variables(&shader_resource_variables)
        //.immutable_samplers(&immutable_samplers)
        .name(c"Deferred lighting PSO")
        .graphics()
        .graphics_pipeline_desc(
            GraphicsPipelineDesc::builder()
                .rasterizer_desc(rasterizer_desc)
                .depth_stencil_desc(depth_desc)
                .output(pipeline_output)
                .blend_desc(blend_desc)
                .primitive_topology(PrimitiveTopology::TriangleList)
                .input_layouts(&input_layouts)
                .build(),
        )
        .vertex_shader(&vertex_shader)
        .pixel_shader(&pixel_shader)
        .build();

    device
        .create_graphics_pipeline_state(&pso_create_info)
        .unwrap()
}

fn create_ambient_light_pso(
    device: &RenderDevice,
    shader_source_factory: &ShaderSourceInputStreamFactory,
    render_pass: &RenderPass,
    convert_ps_output_to_gamma: bool,
) -> Boxed<GraphicsPipelineState> {
    let shader_ci = ShaderCreateInfo::builder()
        .entry_point("main")
        .use_combined_texture_samplers(true)
        .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
        // If the device does not support gamma correction, we will have to do it in the shader.
        // Notice that blending in gamma space is not mathematically correct, but we have no choice.
        .macros(vec![(
            "CONVERT_PS_OUTPUT_TO_GAMMA",
            if convert_ps_output_to_gamma { "1" } else { "0" },
        )])
        .shader_source_input_stream_factory(shader_source_factory);

    // Create a vertex shader
    let vertex_shader = {
        let shader_ci = shader_ci
            .clone()
            .source_language(ShaderLanguage::HLSL)
            .name("Ambient light VS")
            .source(ShaderSource::FilePath(Path::new(
                "assets/ambient_light.vsh",
            )))
            .shader_type(ShaderType::Vertex)
            .build();

        device.create_shader(&shader_ci).unwrap()
    };

    // Create a pixel shader
    let pixel_shader = {
        let device_type = device.get_device_info().device_type();

        let use_glsl = device_type.is_vulkan() || device_type.is_metal();

        let shader_ci = shader_ci
            .clone()
            .source_language(if use_glsl {
                ShaderLanguage::GLSL
            } else {
                ShaderLanguage::HLSL
            })
            .name("Ambient light PS")
            .source(ShaderSource::FilePath(Path::new(if use_glsl {
                "assets/ambient_light_glsl.psh"
            } else {
                "assets/ambient_light_hlsl.psh"
            })))
            .shader_type(ShaderType::Pixel)
            .build();

        device.create_shader(&shader_ci).unwrap()
    };

    // This PSO will be used within the second subpass
    let pipeline_output = GraphicsPipelineRenderPass::new(render_pass, 1);

    let shader_resource_variables = [
        ShaderResourceVariableDesc::builder()
            .name(c"g_SubpassInputColor")
            .variable_type(ShaderResourceVariableType::Mutable)
            .shader_stages(ShaderTypes::Pixel)
            .build(),
        ShaderResourceVariableDesc::builder()
            .name(c"g_SubpassInputDepthZ")
            .variable_type(ShaderResourceVariableType::Mutable)
            .shader_stages(ShaderTypes::Pixel)
            .build(),
    ];

    let pso_create_info = PipelineStateCreateInfo::builder()
        // Define variable type that will be used by default
        .default_variable_type(ShaderResourceVariableType::Static)
        .shader_resource_variables(&shader_resource_variables)
        .name(c"Ambient light PSO")
        .graphics()
        .graphics_pipeline_desc(
            GraphicsPipelineDesc::builder()
                .rasterizer_desc(
                    RasterizerStateDesc::builder()
                        .cull_mode(CullMode::None)
                        .build(),
                )
                .depth_stencil_desc(
                    DepthStencilStateDesc::builder()
                        .depth_enable(false) // Disable depth
                        .build(),
                )
                .output(pipeline_output)
                .primitive_topology(PrimitiveTopology::TriangleStrip)
                .build(),
        )
        .vertex_shader(&vertex_shader)
        .pixel_shader(&pixel_shader)
        .build();

    device
        .create_graphics_pipeline_state(&pso_create_info)
        .unwrap()
}

fn create_lights_buffer(lights_count: u32, device: &RenderDevice) -> Boxed<Buffer> {
    device
        .create_buffer(
            &BufferDesc::builder()
                .name(c"Lights instances buffer")
                .usage(Usage::Dynamic)
                .bind_flags(BindFlags::VertexBuffer)
                .cpu_access_flags(CpuAccessFlags::Write)
                .size(std::mem::size_of::<LightAttribs>() as u64 * lights_count as u64)
                .build(),
        )
        .unwrap()
}

fn create_render_pass(device: &RenderDevice, swap_chain_desc: &SwapChainDesc) -> Boxed<RenderPass> {
    // Prepare render pass attachment descriptions

    let fmt = [
        TextureFormat::R32_FLOAT,
        TextureFormat::R16_UNORM,
        TextureFormat::R16_FLOAT,
    ]
    .into_iter()
    .find(|&fmt| {
        device
            .get_texture_format_info_ext(fmt)
            .bind_flags()
            .contains(BindFlags::RenderTarget)
    }).unwrap_or_else(||{
        println!("This device does not support rendering to any of R32_FLOAT, R16_UNORM or R16_FLOAT formats. Using R8 as fallback.");
        TextureFormat::R8_UNORM});

    // Attachment 0 - Color buffer
    let attachments = [
        RenderPassAttachmentDesc::builder()
            .format(TextureFormat::RGBA8_UNORM)
            .initial_state(ResourceState::RenderTarget)
            .final_state(ResourceState::InputAttachment)
            .load_op(AttachmentLoadOperation::Clear)
            .store_op(AttachmentStoreOperation::Discard)
            .build(),
        // Attachment 1 - Depth Z
        RenderPassAttachmentDesc::builder()
            .format(fmt)
            .initial_state(ResourceState::RenderTarget)
            .final_state(ResourceState::InputAttachment)
            .load_op(AttachmentLoadOperation::Clear)
            .store_op(AttachmentStoreOperation::Discard)
            .build(),
        // Attachment 2 - Depth buffer
        RenderPassAttachmentDesc::builder()
            .format(DEPTH_BUFFER_FORMAT)
            .initial_state(ResourceState::DepthWrite)
            .final_state(ResourceState::DepthWrite)
            .load_op(AttachmentLoadOperation::Clear)
            .store_op(AttachmentStoreOperation::Discard)
            .build(),
        // Attachment 3 - Final color buffer
        RenderPassAttachmentDesc::builder()
            .maybe_format(swap_chain_desc.color_buffer_format())
            .initial_state(ResourceState::RenderTarget)
            .final_state(ResourceState::RenderTarget)
            .load_op(AttachmentLoadOperation::Clear)
            .store_op(AttachmentStoreOperation::Store)
            .build(),
    ];

    // Subpass 0 attachments - 2 render targets and depth buffer
    let rt_attachment_refs0 = [
        AttachmentReference::builder()
            .index(0)
            .state(ResourceState::RenderTarget)
            .build(),
        AttachmentReference::builder()
            .index(1)
            .state(ResourceState::RenderTarget)
            .build(),
    ];

    let depth_attachment_ref0 = AttachmentReference::builder()
        .index(2)
        .state(ResourceState::DepthWrite)
        .build();

    // Subpass 1 attachments - 1 render target, depth buffer, 2 input attachments
    let rt_attachment_refs1 = [AttachmentReference::builder()
        .index(3)
        .state(ResourceState::RenderTarget)
        .build()];

    let depth_attachment_ref1 = AttachmentReference::builder()
        .index(2)
        .state(ResourceState::DepthWrite)
        .build();

    let input_attachment_refs1 = [
        AttachmentReference::builder()
            .index(0)
            .state(ResourceState::InputAttachment)
            .build(),
        AttachmentReference::builder()
            .index(1)
            .state(ResourceState::InputAttachment)
            .build(),
    ];

    // Subpass 0 - Render G-buffer
    // Subpass 1 - Lighting
    let subpasses = [
        SubpassDesc::builder()
            .render_target_attachments(&rt_attachment_refs0)
            .depth_stencil_attachment(&depth_attachment_ref0)
            .build(),
        SubpassDesc::builder()
            .render_target_attachments(&rt_attachment_refs1)
            .depth_stencil_attachment(&depth_attachment_ref1)
            .input_attachments(&input_attachment_refs1)
            .build(),
    ];

    // We need to define dependency between subpasses 0 and 1 to ensure that
    // all writes are complete before we use the attachments for input in subpass 1.
    let subpass_dependencies = [SubpassDependencyDesc::builder()
        .src_subpass_index(0)
        .dst_subpass_index(1)
        .src_stage_mask(PipelineStageFlags::RenderTarget)
        .dst_stage_mask(PipelineStageFlags::PixelShader)
        .src_access_mask(AccessFlags::RenderTargetWrite)
        .dst_access_mask(AccessFlags::ShaderRead)
        .build()];

    device
        .create_render_pass(
            &RenderPassDesc::builder()
                .name(c"Deferred shading render pass desc")
                .attachments(&attachments)
                .subpasses(&subpasses)
                .dependencies(&subpass_dependencies)
                .build(),
        )
        .unwrap()
}

fn create_gbuffer(
    device: &RenderDevice,
    swap_chain_desc: &SwapChainDesc,
    render_pass: &RenderPass,
) -> GBuffer {
    let render_pass_desc = render_pass.desc();
    #[cfg(target_os = "macos")]
    let memoryless_tex_bind_flags = BindFlags::None;

    #[cfg(not(target_os = "macos"))]
    let memoryless_tex_bind_flags = device
        .get_adapter_info()
        .memory()
        .memoryless_texture_bind_flags();

    let maybe_memoryless_flag = if memoryless_tex_bind_flags.bits()
        == (BindFlags::RenderTarget | BindFlags::InputAttachement).bits()
    {
        MiscTextureFlags::Memoryless
    } else {
        MiscTextureFlags::None
    };

    // Create window-size offscreen render target
    let texture_desc = TextureDesc::builder()
        .dimension(TextureDimension::Texture2D)
        .width(swap_chain_desc.width())
        .height(swap_chain_desc.height())
        .mip_levels(1);

    let color_buffer = device
        .create_texture(
            &texture_desc
                .clone()
                .format(render_pass_desc.attachments()[0].format().unwrap())
                .name(c"Color G-buffer")
                .bind_flags(BindFlags::RenderTarget | BindFlags::InputAttachement)
                .misc_flags(maybe_memoryless_flag)
                .clear_color([0.0, 0.0, 0.0, 1.0])
                .build(),
            &[],
            None,
        )
        .unwrap();

    // OpenGL does not allow combining swap chain render target with any
    // other render target, so we have to create an auxiliary texture.
    let opengl_offsreen_color_buffer = if device.get_device_info().device_type().is_gl() {
        Some(
            device
                .create_texture(
                    &texture_desc
                        .clone()
                        .name(c"OpenGL Offscreen Render Target")
                        .bind_flags(BindFlags::RenderTarget | BindFlags::InputAttachement)
                        .format(swap_chain_desc.color_buffer_format().unwrap())
                        .build(),
                    &[],
                    None,
                )
                .unwrap(),
        )
    } else {
        None
    };

    let depth_z_buffer = device
        .create_texture(
            &texture_desc
                .clone()
                .name(c"Depth Z G-buffer")
                .bind_flags(BindFlags::RenderTarget | BindFlags::InputAttachement)
                .format(render_pass_desc.attachments()[1].format().unwrap())
                .misc_flags(maybe_memoryless_flag)
                .clear_color([1.0, 1.0, 1.0, 1.0])
                .build(),
            &[],
            None,
        )
        .unwrap();

    let depth_buffer = device
        .create_texture(
            &texture_desc
                .clone()
                .name(c"Depth buffer")
                .format(render_pass_desc.attachments()[2].format().unwrap())
                .bind_flags(BindFlags::DepthStencil)
                .misc_flags(maybe_memoryless_flag)
                .clear_depth(1.0)
                .clear_stencil(0)
                .build(),
            &[],
            None,
        )
        .unwrap();

    GBuffer {
        color_buffer,
        depth_z_buffer,
        depth_buffer,
        opengl_offsreen_color_buffer: RefCell::new(opengl_offsreen_color_buffer),
    }
}

fn create_framebuffer(
    device: &RenderDevice,
    render_pass: &RenderPass,
    gbuffer: &GBuffer,
    current_rtv: &TextureView,
) -> Boxed<Framebuffer> {
    let attachments = vec![
        gbuffer
            .color_buffer
            .get_default_view(TextureViewType::RenderTarget)
            .unwrap(),
        gbuffer
            .depth_z_buffer
            .get_default_view(TextureViewType::RenderTarget)
            .unwrap(),
        gbuffer
            .depth_buffer
            .get_default_view(TextureViewType::DepthStencil)
            .unwrap(),
        current_rtv,
    ];

    device
        .create_framebuffer(
            &FramebufferDesc::builder()
                .name(c"G-buffer framebuffer")
                .render_pass(render_pass)
                .attachments(&attachments)
                .build(),
        )
        .unwrap()
}

impl RenderPasses {
    fn update_lights(&mut self, elapsed_time: f32) {
        let volume_min = glam::Vec3::new(-GRID_DIM as f32, -GRID_DIM as f32, -GRID_DIM as f32);
        let volume_max = glam::Vec3::new(GRID_DIM as f32, GRID_DIM as f32, GRID_DIM as f32);

        for (light, dir) in self.lights.iter_mut().zip(self.light_move_dirs.iter_mut()) {
            light.location += *dir * elapsed_time;

            let clamp_coord = |coord: &mut f32, dir: &mut f32, min: f32, max: f32| {
                if *coord < min {
                    *coord += (min - *coord) * 2.0;
                    *dir *= -1.0;
                } else {
                    if *coord > max {
                        *coord -= (*coord - max) * 2.0;
                        *dir *= -1.0;
                    }
                }
            };

            clamp_coord(
                &mut light.location.x,
                &mut dir.x,
                volume_min.x,
                volume_max.x,
            );
            clamp_coord(
                &mut light.location.y,
                &mut dir.y,
                volume_min.y,
                volume_max.y,
            );
            clamp_coord(
                &mut light.location.z,
                &mut dir.z,
                volume_min.z,
                volume_max.z,
            );
        }
    }
}

impl SampleBase for RenderPasses {
    fn get_name() -> &'static str {
        "Tutorial19: RenderPass"
    }

    fn make_swap_chains_create_info(
        settings: &diligent_samples::sample_base::sample_app_settings::SampleAppSettings,
    ) -> Vec<SwapChainCreateInfo> {
        vec![
            SwapChainCreateInfo::builder()
                .width(settings.width)
                .height(settings.height)
                // We do not need the depth buffer from the swap chain in this sample
                .depth_buffer_format(None)
                .build(),
        ]
    }

    fn new(
        engine_factory: &EngineFactory,
        device: &RenderDevice,
        main_context: &ImmediateDeviceContext,
        _other_immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        _deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        windows: &[&SwapChainDesc],
    ) -> Self {
        let swap_chain_desc = windows[0];

        let mut shader_constants_cb = create_uniform_buffer(
            device,
            std::mem::size_of::<Constants>() as u64,
            c"Shader constants CB",
            Usage::Dynamic,
            BindFlags::UniformBuffer,
            CpuAccessFlags::Write,
        )
        .unwrap();

        let mut textured_cube = TexturedCube::new(
            device,
            GeometryPrimitiveVertexFlags::PosTex,
            BindFlags::VertexBuffer,
            None,
            BindFlags::IndexBuffer,
            None,
        )
        .unwrap();

        // Create a shader source stream factory to load shaders from files.
        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        let render_pass = create_render_pass(device, swap_chain_desc);

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = matches!(
            swap_chain_desc.color_buffer_format(),
            Some(TextureFormat::RGBA8_UNORM) | Some(TextureFormat::BGRA8_UNORM),
        );

        let cube_pso = create_cube_pso(device, &shader_source_factory, &render_pass);

        cube_pso
            .get_static_variable_by_name(ShaderType::Vertex, "ShaderConstants")
            .unwrap()
            .set(&shader_constants_cb, SetShaderResourceFlags::None);

        let light_volume_pso = create_light_volume_pso(
            device,
            &shader_source_factory,
            &render_pass,
            convert_ps_output_to_gamma,
        );

        light_volume_pso
            .get_static_variable_by_name(ShaderType::Vertex, "ShaderConstants")
            .unwrap()
            .set(&shader_constants_cb, SetShaderResourceFlags::None);
        light_volume_pso
            .get_static_variable_by_name(ShaderType::Pixel, "ShaderConstants")
            .unwrap()
            .set(&shader_constants_cb, SetShaderResourceFlags::None);

        let ambient_light_pso = create_ambient_light_pso(
            device,
            &shader_source_factory,
            &render_pass,
            convert_ps_output_to_gamma,
        );

        let lights_count = 10000;

        let mut lights_buffer = create_lights_buffer(lights_count, device);

        let image = image::ImageReader::open("assets/DGLogo.png")
            .unwrap()
            .decode()
            .unwrap();

        let texture_desc = TextureDesc::builder()
            .name(c"DGLogo")
            .dimension(TextureDimension::Texture2D)
            .width(image.width())
            .height(image.height())
            .format(TextureFormat::RGBA8_UNORM_SRGB)
            .bind_flags(BindFlags::ShaderResource)
            .usage(Usage::Immutable)
            .build();

        let subresource = [TextureSubResource::builder()
            .from_host(
                image.as_bytes(),
                image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
            )
            .build()];

        let mut cube_texture = device
            .create_texture(&texture_desc, &subresource, None)
            .unwrap();

        {
            let (vertex_buffer, index_buffer) = textured_cube.vertex_and_index_buffer_mut();
            // Transition all resources to required states as no transitions are allowed within the render pass.
            let barriers = [
                StateTransitionDesc::builder()
                    .resource(&mut shader_constants_cb)
                    .new_state(ResourceState::ConstantBuffer)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(vertex_buffer)
                    .new_state(ResourceState::VertexBuffer)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(index_buffer)
                    .new_state(ResourceState::IndexBuffer)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(&mut lights_buffer)
                    .new_state(ResourceState::VertexBuffer)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(&mut cube_texture)
                    .new_state(ResourceState::ShaderResource)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
            ];

            main_context.transition_resource_states(&barriers);
        }

        // Get shader resource view from the texture
        let cube_texture_srv = Boxed::<TextureView>::from_ref(
            cube_texture
                .get_default_view(TextureViewType::ShaderResource)
                .unwrap(),
        );

        let g_buffer = create_gbuffer(device, swap_chain_desc, &render_pass);

        // Create SRBs that reference the framebuffer textures

        let light_volume_srb = light_volume_pso
            .create_shader_resource_binding(true)
            .unwrap();
        if let Some(input_color) =
            light_volume_srb.get_variable_by_name("g_SubpassInputColor", ShaderTypes::Pixel)
        {
            input_color.set(
                g_buffer
                    .color_buffer
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
                SetShaderResourceFlags::None,
            );
        }
        if let Some(depth_z_buffer) =
            light_volume_srb.get_variable_by_name("g_SubpassInputDepthZ", ShaderTypes::Pixel)
        {
            depth_z_buffer.set(
                g_buffer
                    .depth_z_buffer
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
                SetShaderResourceFlags::None,
            );
        }

        let ambient_light_srb = ambient_light_pso
            .create_shader_resource_binding(true)
            .unwrap();
        if let Some(input_color) =
            ambient_light_srb.get_variable_by_name("g_SubpassInputColor", ShaderTypes::Pixel)
        {
            input_color.set(
                g_buffer
                    .color_buffer
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
                SetShaderResourceFlags::None,
            );
        }
        if let Some(depth_z_buffer) =
            ambient_light_srb.get_variable_by_name("g_SubpassInputDepthZ", ShaderTypes::Pixel)
        {
            depth_z_buffer.set(
                g_buffer
                    .depth_z_buffer
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
                SetShaderResourceFlags::None,
            );
        }

        let cube_srb = cube_pso.create_shader_resource_binding(true).unwrap();

        cube_srb
            .get_variable_by_name("g_Texture", ShaderTypes::Pixel)
            .unwrap()
            .set(&cube_texture_srv, SetShaderResourceFlags::None);

        let light_vecs = init_lights(lights_count);

        let (lights, light_move_dirs) = light_vecs.into_iter().unzip();

        RenderPasses {
            cube_pso,
            cube_srb,
            textured_cube,
            shader_constants_cb,
            _cube_texture_srv: cube_texture_srv,
            lights_buffer,
            light_volume_pso,
            light_volume_srb,
            ambient_light_pso,
            ambient_light_srb,
            g_buffer,
            render_pass,
            lights_count: lights_count as i32,
            show_light_volumes: false,
            animate_lights: true,
            lights,
            light_move_dirs,
            framebuffer_cache: RefCell::new(HashMap::new()),
            device,
            is_device_gl: device.get_device_info().device_type().is_gl(),
        }
    }

    fn render(
        &self,
        main_context: Boxed<ImmediateDeviceContext>,
        swap_chain: &mut SwapChain,
    ) -> Boxed<ImmediateDeviceContext> {
        let swap_chain_desc = swap_chain.desc();

        let view_proj_matrix = {
            let swap_chain_desc = swap_chain.desc();

            // Get pretransform matrix that rotates the scene according the surface orientation
            let srf_pre_transform = get_surface_pretransform_matrix(
                swap_chain_desc.pre_transform(),
                &glam::Vec3::new(0.0, 0.0, 1.0),
            );

            // Get projection matrix adjusted to the current screen orientation
            let proj = get_adjusted_projection_matrix(
                swap_chain_desc,
                std::f32::consts::PI / 4.0,
                0.1,
                100.0,
            );

            // Set cube view matrix
            let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 25.0));

            proj * srf_pre_transform * view
        };

        {
            let mut constants = main_context
                .map_buffer_write::<Constants>(&self.shader_constants_cb, MapFlags::Discard)
                .unwrap();

            constants[0].view_proj = *view_proj_matrix.as_ref();
            constants[0].view_proj_inv = *view_proj_matrix.inverse().as_ref();
            constants[0].viewport_size = [
                swap_chain_desc.width() as f32,
                swap_chain_desc.height() as f32,
                1.0 / swap_chain_desc.width() as f32,
                1.0 / swap_chain_desc.height() as f32,
            ];

            constants[0].show_light_volumes = if self.show_light_volumes { 1 } else { 0 }
        }

        let clear_values = [
            // Color
            OptimizedClearValue::builder()
                .color([0.0, 0.0, 0.0, 0.0])
                .build(),
            //Depth Z
            OptimizedClearValue::builder()
                .color([1.0, 1.0, 1.0, 1.0])
                .build(),
            //Depth Z
            OptimizedClearValue::builder()
                .depth_stencil(DepthStencilClearValue::depth(1.0))
                .build(),
            OptimizedClearValue::builder()
                .color([0.0625, 0.0625, 0.0625, 1.0])
                .build(),
        ];

        {
            let render_pass = {
                let opengl_offsreen_color_buffer =
                    self.g_buffer.opengl_offsreen_color_buffer.borrow();

                let current_back_buffer = if self.is_device_gl {
                    opengl_offsreen_color_buffer
                        .as_ref()
                        .and_then(|tex| tex.get_default_view(TextureViewType::RenderTarget))
                } else {
                    swap_chain.get_current_back_buffer_rtv()
                };

                let mut framebuffers = self.framebuffer_cache.borrow_mut();
                let framebuffer = framebuffers
                    .entry(current_back_buffer.map_or(std::ptr::null(), std::ptr::from_ref))
                    .or_insert(create_framebuffer(
                        unsafe { &*self.device },
                        &self.render_pass,
                        &self.g_buffer,
                        current_back_buffer.unwrap(),
                    ));

                main_context.begin_render_pass(
                    &BeginRenderPassAttribs::builder()
                        .render_pass(&self.render_pass)
                        .frame_buffer(framebuffer.transition_state())
                        .clear_values(&clear_values)
                        .build(),
                )
            };

            let mut render_pass = {
                // Bind vertex and index buffers
                // Note that RESOURCE_STATE_TRANSITION_MODE_TRANSITION are not allowed inside render pass!
                render_pass.set_vertex_buffers(
                    [(self.textured_cube.vertex_buffer().verify_state(), 0)],
                    SetVertexBufferFlags::Reset,
                );
                render_pass.set_index_buffer(self.textured_cube.index_buffer().verify_state(), 0);

                // Set the cube's pipeline state
                let graphics = render_pass.set_graphics_pipeline_state(&self.cube_pso);

                // Commit the cube shader's resources
                graphics.commit_shader_resources(self.cube_srb.verify_state());

                // Draw the grid
                graphics.draw_indexed(
                    &DrawIndexedAttribs::builder()
                        .index_type(ValueType::Uint32)
                        .num_indices(36)
                        .num_instances(GRID_DIM as u32 * GRID_DIM as u32)
                        .flags(DrawFlags::VerifyAll) // Verify the state of vertex and index buffers
                        .build(),
                );

                graphics.finish_pipeline()
            };

            render_pass.next_subpass();

            render_pass = {
                // Set the lighting PSO
                let graphics = render_pass.set_graphics_pipeline_state(&self.ambient_light_pso);

                // Commit shader resources
                graphics.commit_shader_resources(self.ambient_light_srb.verify_state());

                // Draw quad
                graphics.draw(
                    &DrawAttribs::builder()
                        .num_vertices(4)
                        .flags(DrawFlags::VerifyAll) // Verify the state of vertex and index buffers
                        .build(),
                );

                graphics.finish_pipeline()
            };

            render_pass = {
                {
                    // Map the cube's constant buffer and fill it in with its view-projection matrix
                    let mut lights_data = render_pass
                        .map_buffer_write(&self.lights_buffer, MapFlags::Discard)
                        .unwrap();

                    lights_data.clone_from_slice(self.lights.as_slice());
                }

                // Bind vertex and index buffers

                // Note that RESOURCE_STATE_TRANSITION_MODE_TRANSITION are not allowed inside render pass!
                render_pass.set_vertex_buffers(
                    [
                        (self.textured_cube.vertex_buffer().verify_state(), 0),
                        (self.lights_buffer.verify_state(), 0),
                    ],
                    SetVertexBufferFlags::None,
                );
                render_pass.set_index_buffer(self.textured_cube.index_buffer().verify_state(), 0);

                // Set the lighting PSO
                let graphics = render_pass.set_graphics_pipeline_state(&self.light_volume_pso);

                // Commit shader resources
                graphics.commit_shader_resources(self.light_volume_srb.verify_state());

                {
                    // Draw lights
                    // Verify the state of vertex and index buffers

                    graphics.draw_indexed(
                        &DrawIndexedAttribs::builder()
                            .index_type(ValueType::Uint32)
                            .num_indices(36)
                            .num_instances(self.lights_count as u32)
                            .flags(DrawFlags::VerifyAll)
                            .build(),
                    );
                }
                graphics.finish_pipeline()
            };

            let main_context = render_pass.end_render_pass();

            if let Some(ref mut opengl_offsreen_color_buffer) =
                *self.g_buffer.opengl_offsreen_color_buffer.borrow_mut()
            {
                // In OpenGL we now have to copy our off-screen buffer to the default framebuffer
                let offscreen_render_target = opengl_offsreen_color_buffer;
                let back_buffer = swap_chain
                    .get_current_back_buffer_rtv_mut()
                    .unwrap()
                    .get_texture_mut();

                main_context.copy_texture(
                    &CopyTextureAttribs::builder()
                        .src_texture(offscreen_render_target.transition_state())
                        .dst_texture(back_buffer.transition_state())
                        .build(),
                );
            }
            main_context
        }
    }

    fn update_ui(
        &mut self,
        device: &RenderDevice,
        _main_context: &ImmediateDeviceContext,
        ui: &mut imgui::Ui,
    ) {
        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .begin()
        {
            if ui.input_int("Lights count", &mut self.lights_count).build() {
                self.lights_count = self.lights_count.clamp(100, 50000);
                let light_vecs = init_lights(self.lights_count as u32);
                (self.lights, self.light_move_dirs) = light_vecs.into_iter().unzip();
                self.lights_buffer = create_lights_buffer(self.lights_count as u32, device);
            }
            ui.checkbox("Show light volumes", &mut self.show_light_volumes);
            ui.checkbox("Animate lights", &mut self.animate_lights);
        }
    }

    fn update(
        &mut self,
        _main_context: &ImmediateDeviceContext,
        _current_time: f64,
        elapsed_time: f64,
    ) {
        if self.animate_lights {
            self.update_lights(elapsed_time as f32);
        }
    }
}

fn main() {
    sample_app::main::<RenderPasses>().unwrap()
}
