use std::path::Path;

use diligent::{
    buffer::Buffer,
    device_context::{
        DeferredDeviceContext, DrawAttribs, DrawFlags, ImmediateDeviceContext,
        ResourceStateTransitionMode,
    },
    engine_factory::EngineFactory,
    graphics_types::{
        BindFlags, CpuAccessFlags, DeviceFeatureState, FilterType, MapFlags, PrimitiveTopology,
        SetShaderResourceFlags, ShaderType, ShaderTypes, TextureAddressMode, TextureFormat, Usage,
    },
    graphics_utilities::{create_uniform_buffer, linear_to_srgba},
    pipeline_resource_signature::ImmutableSamplerDesc,
    pipeline_state::{
        CullMode, DepthStencilStateDesc, GraphicsPipelineDesc, GraphicsPipelineRenderTargets,
        GraphicsPipelineStateCreateInfo, PipelineState, RasterizerStateDesc,
    },
    render_device::RenderDevice,
    sampler::SamplerDesc,
    shader::{ShaderCompileFlags, ShaderCreateInfo, ShaderLanguage, ShaderSource},
    shader_resource_binding::ShaderResourceBinding,
    shader_resource_variable::{ShaderResourceVariableDesc, ShaderResourceVariableType},
    swap_chain::SwapChain,
    texture::{TextureDesc, TextureDimension, TextureSubResource},
    texture_view::{TextureView, TextureViewType},
};
use diligent_samples::sample_base::{
    sample::{get_adjusted_projection_matrix, get_surface_pretransform_matrix, SampleBase},
    sample_app::SampleApp,
};
use diligent_tools::native_app;

struct Tessellation {
    immediate_context: ImmediateDeviceContext,

    main_pipeline: (PipelineState, ShaderResourceBinding),
    wireframe_pipeline: Option<(PipelineState, ShaderResourceBinding)>,

    animate: bool,
    wireframe: bool,
    tess_density: f32,
    distance: f32,

    _height_map_srv: TextureView,
    _color_map_srv: TextureView,

    rotation_angle: f32,

    convert_ps_output_to_gamma: bool,

    height_map_width: u32,
    height_map_height: u32,
    block_size: u32,

    shader_constants: Buffer,

    adaptive_tessellation: bool,
}

#[repr(C)]
struct GlobalConstants {
    num_horz_blocks: u32, // Number of blocks along the horizontal edge
    num_vert_blocks: u32, // Number of blocks along the horizontal edge
    f_num_horz_blocks: f32,
    f_num_vert_blocks: f32,

    block_size: f32,
    length_scale: f32,
    height_scale: f32,
    line_width: f32,

    tess_density: f32,
    adaptive_tessellation: i32,
    dummy2: [f32; 2],

    world_view: [f32; 4 * 4],
    world_view_proj: [f32; 4 * 4],
    viewport_size: [f32; 4],
}

impl SampleBase for Tessellation {
    fn new(
        engine_factory: &EngineFactory,
        device: &RenderDevice,
        immediate_contexts: Vec<ImmediateDeviceContext>,
        _deferred_contexts: Vec<DeferredDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
        let wireframe_supported = if let DeviceFeatureState::Disabled =
            device.get_device_info().features().geometry_shaders
        {
            false
        } else {
            true
        };

        let swap_chain_desc = swap_chain.get_desc();

        // Cull back faces. For some reason, in OpenGL the order is reversed
        let cull_mode = if {
            #[cfg(feature = "opengl")]
            {
                device.get_device_info().device_type() == RenderDeviceType::OpengGL
            }

            #[cfg(not(feature = "opengl"))]
            {
                false
            }
        } {
            CullMode::Front
        } else {
            CullMode::Back
        };

        let mut rtv_formats = std::array::from_fn(|_| None);
        rtv_formats[0] = Some(swap_chain_desc.color_buffer_format);

        // Pipeline state object encompasses configuration of all GPU stages
        let pso_create_info = GraphicsPipelineStateCreateInfo::new(
            "Terrain PSO",
            GraphicsPipelineDesc::builder()
                .rasterizer_desc(RasterizerStateDesc::builder().cull_mode(cull_mode).build())
                .depth_stencil_desc(DepthStencilStateDesc::builder().depth_enable(true).build())
                .output(
                    GraphicsPipelineRenderTargets::builder()
                        .num_render_targets(1)
                        .rtv_formats(rtv_formats)
                        .dsv_format(swap_chain_desc.depth_buffer_format)
                        .build(),
                )
                .primitive_topology(PrimitiveTopology::ControlPointPatchList1)
                .build(),
        );

        let shader_constants = create_uniform_buffer(
            device,
            std::mem::size_of::<GlobalConstants>() as u64,
            "Global shader constants CB",
            Usage::Dynamic,
            BindFlags::UniformBuffer,
            CpuAccessFlags::Write,
        )
        .unwrap();

        let block_size = 32;

        let convert_ps_output_to_gamma = match swap_chain_desc.color_buffer_format {
            TextureFormat::RGBA8_UNORM | TextureFormat::BGRA8_UNORM => true,
            _ => false,
        };

        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        let shader_ci = ShaderCreateInfo::builder()
            .source_language(ShaderLanguage::HLSL)
            .use_combined_texture_samplers(true)
            .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
            .macros(vec![
                ("BLOCK_SIZE", format!("{block_size}").as_str()),
                (
                    "CONVERT_PS_OUTPUT_TO_GAMMA",
                    if convert_ps_output_to_gamma { "1" } else { "0" },
                ),
            ])
            .shader_source_input_stream_factory(&shader_source_factory);

        let vertex_shader = device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Terrain VS")
                    .entry_point("TerrainVS")
                    .source(ShaderSource::FilePath(Path::new("assets/terrain.vsh")))
                    .shader_type(ShaderType::Vertex)
                    .build(),
            )
            .unwrap();

        let hull_shader = device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Terrain HS")
                    .entry_point("TerrainHS")
                    .source(ShaderSource::FilePath(Path::new("assets/terrain.hsh")))
                    .shader_type(ShaderType::Hull)
                    .build(),
            )
            .unwrap();

        let domain_shader = device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Terrain DS")
                    .entry_point("TerrainDS")
                    .source(ShaderSource::FilePath(Path::new("assets/terrain.dsh")))
                    .shader_type(ShaderType::Domain)
                    .build(),
            )
            .unwrap();

        let pixel_shader = device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Terrain PS")
                    .entry_point("TerrainPS")
                    .source(ShaderSource::FilePath(Path::new("assets/terrain.psh")))
                    .shader_type(ShaderType::Pixel)
                    .build(),
            )
            .unwrap();

        let pso_create_info = pso_create_info
            .vertex_shader(&vertex_shader)
            .hull_shader(&hull_shader)
            .domain_shader(&domain_shader)
            .pixel_shader(&pixel_shader);

        let pso_create_info = pso_create_info
            .default_variable_type(ShaderResourceVariableType::Static)
            .set_shader_resource_variables([
                ShaderResourceVariableDesc::builder()
                    .name("g_HeightMap")
                    .variable_type(ShaderResourceVariableType::Mutable)
                    .shader_stages(ShaderTypes::Hull | ShaderTypes::Domain)
                    .build(),
                ShaderResourceVariableDesc::builder()
                    .name("g_Texture")
                    .variable_type(ShaderResourceVariableType::Mutable)
                    .shader_stages(ShaderTypes::Pixel)
                    .build(),
            ]);

        let linear_clamp_sampler = SamplerDesc::builder()
            .name("Linear Sampler")
            .mag_filter(FilterType::Linear)
            .min_filter(FilterType::Linear)
            .mip_filter(FilterType::Linear)
            .address_u(TextureAddressMode::Clamp)
            .address_v(TextureAddressMode::Clamp)
            .address_w(TextureAddressMode::Clamp)
            .build();

        let samplers = [
            ImmutableSamplerDesc::new(
                ShaderTypes::Hull | ShaderTypes::Domain,
                "g_HeightMap",
                &linear_clamp_sampler,
            ),
            ImmutableSamplerDesc::new(ShaderTypes::Pixel, "g_Texture", &linear_clamp_sampler),
        ];

        let pso_create_info = pso_create_info.set_immutable_samplers(samplers);

        let main_pipeline = device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        let wireframe_pipeline = if wireframe_supported {
            let wire_pixel_shader = device
                .create_shader(
                    &shader_ci
                        .clone()
                        .name("Wireframe Terrain PS")
                        .entry_point("WireTerrainPS")
                        .source(ShaderSource::FilePath(Path::new("assets/terrain_wire.psh")))
                        .shader_type(ShaderType::Pixel)
                        .build(),
                )
                .unwrap();

            let wire_geometry_shader = device
                .create_shader(
                    &shader_ci
                        .clone()
                        .name("Terrain GS")
                        .entry_point("TerrainGS")
                        .source(ShaderSource::FilePath(Path::new("assets/terrain.gsh")))
                        .shader_type(ShaderType::Geometry)
                        .build(),
                )
                .unwrap();

            let pso_create_info = pso_create_info
                .pixel_shader(&wire_pixel_shader)
                .geometry_shader(&wire_geometry_shader);

            let wire_pipeline = device
                .create_graphics_pipeline_state(&pso_create_info)
                .unwrap();

            wire_pipeline
                .get_static_variable_by_name(ShaderType::Geometry, "GSConstants")
                .unwrap()
                .set(&shader_constants, SetShaderResourceFlags::None);
            wire_pipeline
                .get_static_variable_by_name(ShaderType::Pixel, "PSConstants")
                .unwrap()
                .set(&shader_constants, SetShaderResourceFlags::None);

            Some(wire_pipeline)
        } else {
            None
        };

        let (height_map_srv, width, height) = {
            let image = image::ImageReader::open("assets/ps_height_1k.png")
                .unwrap()
                .decode()
                .unwrap();

            let texture_desc = TextureDesc::builder()
                .name("Terrain height map")
                .dimension(TextureDimension::Texture2D)
                .width(image.width())
                .height(image.height())
                .format(TextureFormat::R16_UNORM)
                .bind_flags(BindFlags::ShaderResource)
                .usage(Usage::Immutable)
                .build();

            let subresource = TextureSubResource::builder()
                .from_host(
                    image.as_bytes(),
                    image.width() as u64 * std::mem::size_of::<u16>() as u64,
                )
                .build();

            let texture = device
                .create_texture(&texture_desc, &[&subresource], None)
                .unwrap();

            // Get shader resource view from the texture
            (
                texture
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
                image.width(),
                image.height(),
            )
        };

        let color_map_srv = {
            let image = image::ImageReader::open("assets/ps_texture_2k.png")
                .unwrap()
                .decode()
                .unwrap();

            let texture_desc = TextureDesc::builder()
                .name("Terrain color map")
                .dimension(TextureDimension::Texture2D)
                .width(image.width())
                .height(image.height())
                .format(TextureFormat::RGBA8_UNORM_SRGB)
                .bind_flags(BindFlags::ShaderResource)
                .usage(Usage::Immutable)
                .build();

            let subresource = TextureSubResource::builder()
                .from_host(
                    image.as_bytes(),
                    image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
                )
                .build();

            let texture = device
                .create_texture(&texture_desc, &[&subresource], None)
                .unwrap();

            // Get shader resource view from the texture
            texture
                .get_default_view(TextureViewType::ShaderResource)
                .unwrap()
        };

        fn apply_to_pipelines(
            f: impl Fn(&PipelineState),
            main_pipeline: &PipelineState,
            wireframe_pipeline: &Option<PipelineState>,
        ) {
            f(&main_pipeline);
            if let Some(wireframe_pipeline) = &wireframe_pipeline {
                f(wireframe_pipeline)
            }
        }

        #[rustfmt::skip]
        apply_to_pipelines(
            |pso| {
                pso.get_static_variable_by_name(ShaderType::Vertex, "VSConstants").unwrap().set(&shader_constants, SetShaderResourceFlags::None);
                pso.get_static_variable_by_name(ShaderType::Hull, "HSConstants").unwrap().set(&shader_constants, SetShaderResourceFlags::None);
                pso.get_static_variable_by_name(ShaderType::Domain, "DSConstants").unwrap().set(&shader_constants, SetShaderResourceFlags::None);
            },
            &main_pipeline,
            &wireframe_pipeline,
        );

        let main_srb = main_pipeline.create_shader_resource_binding(true).unwrap();
        let main_pipeline = (main_pipeline, main_srb);

        let wireframe_pipeline = wireframe_pipeline.and_then(|pso| {
            let srb = pso.create_shader_resource_binding(true).unwrap();
            Some((pso, srb))
        });

        fn apply_to_srb(
            f: impl Fn(&(PipelineState, ShaderResourceBinding)),
            main_pipeline: &(PipelineState, ShaderResourceBinding),
            wireframe_pipeline: &Option<(PipelineState, ShaderResourceBinding)>,
        ) {
            f(&main_pipeline);
            if let Some(wireframe_pipeline) = &wireframe_pipeline {
                f(wireframe_pipeline)
            }
        }

        #[rustfmt::skip]
        apply_to_srb(
            |(_, srb)| {
                srb.get_variable_by_name("g_Texture", ShaderTypes::Pixel).unwrap().set(&color_map_srv, SetShaderResourceFlags::None);
                srb.get_variable_by_name("g_HeightMap", ShaderTypes::Domain).unwrap().set(&height_map_srv, SetShaderResourceFlags::None);
                srb.get_variable_by_name("g_HeightMap", ShaderTypes::Hull).unwrap().set(&height_map_srv, SetShaderResourceFlags::None);
            },
            &main_pipeline,
            &wireframe_pipeline,
        );

        Tessellation {
            animate: true,
            wireframe: false,
            tess_density: 32.0,
            distance: 10.0,
            immediate_context: immediate_contexts.into_iter().nth(0).unwrap(),
            main_pipeline,
            wireframe_pipeline,

            _color_map_srv: color_map_srv,
            _height_map_srv: height_map_srv,

            convert_ps_output_to_gamma,

            rotation_angle: 0.0,

            block_size: 32,

            height_map_width: width,
            height_map_height: height,

            shader_constants,

            adaptive_tessellation: true,
        }
    }

    fn get_immediate_context(&self) -> &ImmediateDeviceContext {
        &self.immediate_context
    }

    fn modify_engine_init_info(
        engine_ci: &mut diligent_samples::sample_base::sample::EngineCreateInfo,
    ) {
        engine_ci.features.tessellation = DeviceFeatureState::Enabled;
        engine_ci.features.geometry_shaders = DeviceFeatureState::Optional;
    }

    fn update_ui(&mut self, ui: &mut imgui::Ui) {
        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .begin()
        {
            let _ = ui.checkbox("Animate", &mut self.animate);
            let _ = ui.checkbox("Adaptive tessellation", &mut self.adaptive_tessellation);
            if self.wireframe_pipeline.is_some() {
                let _ = ui.checkbox("Wireframe", &mut self.wireframe);
            }
            let _ = ui.slider("Tess density", 1.0, 32.0, &mut self.tess_density);
            let _ = ui.slider("Distance", 1.0, 20.0, &mut self.distance);
        }
    }

    fn update(&mut self, _current_time: f64, elapsed_time: f64) {
        // Set world view matrix
        if self.animate {
            self.rotation_angle += elapsed_time as f32 * 0.2;

            if self.rotation_angle > std::f32::consts::PI * 2.0 {
                self.rotation_angle -= std::f32::consts::PI * 2.0;
            }
        }
    }

    fn render(&self, swap_chain: &SwapChain) {
        let immediate_context = self.get_immediate_context();

        let swap_chain_desc = swap_chain.get_desc();

        let proj_matrix = {
            // Get pretransform matrix that rotates the scene according the surface orientation
            let srf_pre_transform = get_surface_pretransform_matrix(
                swap_chain_desc.pre_transform,
                &glam::Vec3::new(0.0, 0.0, 1.0),
            );

            // Get projection matrix adjusted to the current screen orientation
            let proj = get_adjusted_projection_matrix(
                &swap_chain_desc,
                std::f32::consts::PI / 4.0,
                0.1,
                1000.0,
            );
            proj * srf_pre_transform
        };

        let model_view_matrix = {
            let model_matrix = glam::Mat4::from_rotation_x(-std::f32::consts::PI * 0.1)
                * glam::Mat4::from_rotation_y(self.rotation_angle);

            // Camera is at (0, 0, -self.distance) looking along Z axis
            let view_matrix = glam::Mat4::from_translation(glam::Vec3 {
                x: 0.0,
                y: 0.0,
                z: self.distance,
            });

            view_matrix * model_matrix
        };

        let mut rtv = swap_chain.get_current_back_buffer_rtv();
        let mut dsv = swap_chain.get_depth_buffer_dsv();

        // Clear the back buffer
        {
            let clear_color = {
                let clear_color = [0.350, 0.350, 0.350, 1.0];

                if self.convert_ps_output_to_gamma {
                    // If manual gamma correction is required, we need to clear the render target with sRGB color
                    linear_to_srgba(clear_color)
                } else {
                    clear_color
                }
            };

            immediate_context.clear_render_target::<f32>(
                &mut rtv,
                &clear_color,
                ResourceStateTransitionMode::Transition,
            );
        }

        immediate_context.clear_depth(&mut dsv, 1.0, ResourceStateTransitionMode::Transition);

        let num_horz_blocks = self.height_map_width / self.block_size;
        let num_vert_blocks = self.height_map_height / self.block_size;

        {
            // Map the buffer and write rendering data
            let mut constants =
                immediate_context.map_buffer_write(&self.shader_constants, MapFlags::Discard);

            let buffer_write = unsafe { constants.as_mut() };

            *buffer_write = GlobalConstants {
                block_size: self.block_size as f32,
                num_horz_blocks,
                num_vert_blocks,
                f_num_horz_blocks: num_horz_blocks as f32,
                f_num_vert_blocks: num_vert_blocks as f32,

                length_scale: 10.0,
                height_scale: 10.0 / 25.0,

                world_view: model_view_matrix.to_cols_array(),
                world_view_proj: (proj_matrix * model_view_matrix).to_cols_array(),

                tess_density: self.tess_density,
                adaptive_tessellation: if self.adaptive_tessellation { 1 } else { 0 },

                dummy2: [0.0, 0.0],

                viewport_size: [
                    swap_chain_desc.width as f32,
                    swap_chain_desc.height as f32,
                    1.0 / swap_chain_desc.width as f32,
                    1.0 / swap_chain_desc.height as f32,
                ],

                line_width: 3.0,
            };
        }

        let (pso, srb) = if self.wireframe {
            self.wireframe_pipeline.as_ref().unwrap()
        } else {
            &self.main_pipeline
        };

        // Set the pipeline state
        immediate_context.set_pipeline_state(pso);

        // Commit shader resources. RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode
        // makes sure that resources are transitioned to required states.
        immediate_context.commit_shader_resources(srb, ResourceStateTransitionMode::Transition);

        let draw_attribs = DrawAttribs::builder()
            .num_vertices(num_horz_blocks * num_vert_blocks)
            .flags(DrawFlags::VerifyAll)
            .build();

        immediate_context.draw(&draw_attribs);
    }

    fn get_name() -> &'static str {
        "Tutorial08: Tessellation"
    }
}

fn main() {
    native_app::main::<SampleApp<Tessellation>>().unwrap()
}
