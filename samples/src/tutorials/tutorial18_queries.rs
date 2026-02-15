use std::cell::RefCell;

use diligent::{geometry_primitives::*, graphics_utilities::*, *};
use diligent_samples::{
    sample_base::{
        sample::{SampleBase, get_adjusted_projection_matrix, get_surface_pretransform_matrix},
        sample_app::{self},
    },
    textured_cube::{self, TexturedCube},
};

struct Queries {
    textured_cube: TexturedCube,
    rotation_matrix: glam::Mat4,

    cube_pso: Boxed<GraphicsPipelineState>,
    cube_srb: Boxed<ShaderResourceBinding>,
    cube_vs_constants: Boxed<Buffer>,
    _cube_texture_srv: Boxed<TextureView>,

    query_pipeline_stats: Option<Boxed<Query<QueryDataPipelineStatistics>>>,
    query_occlusion: Option<Boxed<Query<QueryDataOcclusion>>>,
    query_duration: Option<Boxed<Query<QueryDataDuration>>>,
    query_duration_from_timestamps: Option<DurationQueryHelper>,

    pipeline_statistics: RefCell<Option<QueryDataPipelineStatistics>>,
    occlusion: RefCell<Option<QueryDataOcclusion>>,
    duration: RefCell<Option<QueryDataDuration>>,
    duration_from_timestamps: RefCell<Option<f64>>,

    convert_ps_output_to_gamma: bool,
}

impl SampleBase for Queries {
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

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = matches!(
            swap_chain_desc.color_buffer_format(),
            Some(TextureFormat::RGBA8_UNORM) | Some(TextureFormat::BGRA8_UNORM)
        );

        // Create a shader source stream factory to load shaders from files.
        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        // Create dynamic uniform buffer that will store our transformation matrix
        // Dynamic buffers can be frequently updated by the CPU
        let cube_vs_constants = create_uniform_buffer(
            device,
            std::mem::size_of::<glam::Mat4>() as u64,
            c"VS constants CB",
            Usage::Dynamic,
            BindFlags::UniformBuffer,
            CpuAccessFlags::Write,
        )
        .unwrap();

        let cube_pso_ci = textured_cube::CreatePSOInfo::new(
            device,
            swap_chain_desc.color_buffer_format(),
            swap_chain_desc.depth_buffer_format(),
            &shader_source_factory,
            "assets/cube_texture.vsh",
            "assets/cube_texture.psh",
            GeometryPrimitiveVertexFlags::PosTex,
            [],
            1,
        );

        let cube_pso =
            TexturedCube::create_pipeline_state(cube_pso_ci, convert_ps_output_to_gamma).unwrap();

        // Since we did not explicitly specify the type for 'Constants' variable, default
        // type (SHADER_RESOURCE_VARIABLE_TYPE_STATIC) will be used. Static variables
        // never change and are bound directly to the pipeline state object.
        cube_pso
            .get_static_variable_by_name(ShaderType::Vertex, "Constants")
            .unwrap()
            .set(&cube_vs_constants, SetShaderResourceFlags::None);

        // Since we are using mutable variable, we must create a shader resource binding object
        // http://diligentgraphics.com/2016/03/23/resource-binding-model-in-diligent-engine-2-0/
        let cube_srb = cube_pso.create_shader_resource_binding(true).unwrap();

        let cube_texture_srv = {
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

            let texture = device
                .create_texture(&texture_desc, &subresource, None)
                .unwrap();

            // Get shader resource view from the texture
            Boxed::<TextureView>::from_ref(
                texture
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
            )
        };

        cube_srb
            .get_variable_by_name("g_Texture", ShaderTypes::Pixel)
            .unwrap()
            .set(&cube_texture_srv, SetShaderResourceFlags::None);

        let device_info = device.get_device_info();
        let device_features = device_info.features();

        let textured_cube = TexturedCube::new(
            device,
            GeometryPrimitiveVertexFlags::PosTex,
            BindFlags::VertexBuffer,
            None,
            BindFlags::IndexBuffer,
            None,
        )
        .unwrap();

        let query_pipeline_stats = if !matches!(
            device_features.pipeline_statistics_queries(),
            DeviceFeatureState::Disabled
        ) {
            device
                .create_query_pipeline_statistics(Some(c"Pipeline statistics query"))
                .ok()
        } else {
            None
        };
        let query_occlusion = if !matches!(
            device_features.occlusion_queries(),
            DeviceFeatureState::Disabled
        ) {
            device.create_query_occlusion(Some(c"Occlusion query")).ok()
        } else {
            None
        };

        let query_duration = if !matches!(
            device_features.duration_queries(),
            DeviceFeatureState::Disabled
        ) {
            device.create_query_duration(Some(c"Duration query")).ok()
        } else {
            None
        };

        let query_duration_from_timestamps = if !matches!(
            device_features.timestamp_queries(),
            DeviceFeatureState::Disabled
        ) {
            DurationQueryHelper::new(device).ok()
        } else {
            None
        };

        let sample = Queries {
            textured_cube,

            query_pipeline_stats,
            query_occlusion,
            query_duration,
            query_duration_from_timestamps,

            pipeline_statistics: RefCell::new(None),
            occlusion: RefCell::new(None),
            duration: RefCell::new(None),
            duration_from_timestamps: RefCell::new(None),

            convert_ps_output_to_gamma,
            cube_vs_constants,
            cube_srb,
            cube_pso,
            _cube_texture_srv: cube_texture_srv,
            rotation_matrix: glam::Mat4::IDENTITY,
        };

        main_context.flush();

        sample
    }

    fn modify_engine_init_info(
        engine_ci: &mut diligent_samples::sample_base::sample::EngineCreateInfo,
    ) {
        let features = &mut engine_ci.features;

        features.set_occlusion_queries(DeviceFeatureState::Optional);
        features.set_binary_occlusion_queries(DeviceFeatureState::Optional);
        features.set_timestamp_queries(DeviceFeatureState::Optional);
        features.set_pipeline_statistics_queries(DeviceFeatureState::Optional);
        features.set_duration_queries(DeviceFeatureState::Optional);
    }

    fn render(
        &self,
        main_context: Boxed<ImmediateDeviceContext>,
        swap_chain: Boxed<SwapChain>,
    ) -> (Boxed<ImmediateDeviceContext>, Boxed<SwapChain>) {
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
            let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 5.0));

            proj * srf_pre_transform * view
        };

        {
            // Map the buffer and write current world-view-projection matrix
            let mut constant_buffer_data =
                main_context.map_buffer_write(&self.cube_vs_constants, MapFlags::Discard);

            constant_buffer_data[0] = view_proj_matrix * self.rotation_matrix;
        }

        let rtv = swap_chain.get_current_back_buffer_rtv().unwrap();
        let dsv = swap_chain.get_depth_buffer_dsv().unwrap();

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

            main_context.clear_render_target::<f32>(
                rtv,
                &clear_color,
                ResourceStateTransitionMode::Transition,
            );
        }

        main_context.clear_depth(dsv, 1.0, ResourceStateTransitionMode::Transition);

        {
            // Bind vertex and index buffers
            main_context.set_vertex_buffers(
                &[(self.textured_cube.get_vertex_buffer(), 0)],
                ResourceStateTransitionMode::Transition,
                SetVertexBufferFlags::Reset,
            );
            main_context.set_index_buffer(
                self.textured_cube.get_index_buffer(),
                0,
                ResourceStateTransitionMode::Transition,
            );
        }

        // Set the cube's pipeline state
        let graphics = main_context.set_graphics_pipeline_state(&self.cube_pso);

        // Commit the cube shader's resources
        graphics.commit_shader_resources(&self.cube_srb, ResourceStateTransitionMode::Transition);

        {
            let pipeline_token = self
                .query_pipeline_stats
                .as_ref()
                .map(|query| graphics.begin_query(query));

            let occlusion_token = self
                .query_occlusion
                .as_ref()
                .map(|query| graphics.begin_query(query));

            let duration_token = self
                .query_duration
                .as_ref()
                .map(|query| graphics.begin_query(query));

            let ts_token = self
                .query_duration_from_timestamps
                .as_ref()
                .map(|ts| graphics.query_timestamp(ts));

            graphics.draw_indexed(
                &DrawIndexedAttribs::builder()
                    .num_indices(36)
                    .index_type(ValueType::Uint32)
                    // Verify the state of vertex and index buffers
                    .flags(DrawFlags::VerifyAll)
                    .build(),
            );

            if let Some(token) = pipeline_token {
                *self.pipeline_statistics.borrow_mut() = token.data(true);
            };

            if let Some(token) = occlusion_token {
                *self.occlusion.borrow_mut() = token.data(true);
            };

            if let Some(token) = duration_token {
                *self.duration.borrow_mut() = token.data(true);
            };

            if let Some(token) = ts_token {
                *self.duration_from_timestamps.borrow_mut() = token.duration();
            };
        }

        (graphics.finish(), swap_chain)
    }

    fn update(
        &mut self,
        _main_context: &ImmediateDeviceContext,
        current_time: f64,
        _elapsed_time: f64,
    ) {
        // Apply rotation
        self.rotation_matrix = glam::Mat4::from_rotation_x(-std::f32::consts::PI * 0.1)
            * glam::Mat4::from_rotation_y(current_time as f32);
    }

    fn update_ui(
        &mut self,
        _device: &RenderDevice,
        _main_context: &ImmediateDeviceContext,
        ui: &mut imgui::Ui,
    ) {
        if let Some(_window_token) = ui
            .window("Query data")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .begin()
            && (self.query_pipeline_stats.is_some()
                || self.query_occlusion.is_some()
                || self.query_duration.is_some()
                || self.query_duration_from_timestamps.is_some())
        {
            let mut params = String::new();
            let mut values = String::new();
            if let Some(ref pipeline_statistics) = *self.pipeline_statistics.borrow() {
                params += r"Input vertices
Input primitives
VS Invocations
Clipping Invocations
Rasterized Primitives
PS Invocations
";

                values += format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n",
                    pipeline_statistics.input_vertices,
                    pipeline_statistics.input_primitives,
                    pipeline_statistics.vs_invocations,
                    pipeline_statistics.clipping_invocations,
                    pipeline_statistics.clipping_primitives,
                    pipeline_statistics.ps_invocations,
                )
                .as_str();
            }

            if let Some(ref occlusion) = *self.occlusion.borrow() {
                params += "Samples rendered\n";

                values += occlusion.num_samples.to_string().as_str();
                values += "\n";
            }

            if let Some(ref duration_data) = *self.duration.borrow() {
                if duration_data.frequency > 0 {
                    params += "Duration (mus)\n";
                    values += format!(
                        "{}\n",
                        duration_data.duration as f32 / duration_data.frequency as f32 * 1000000.0
                    )
                    .as_str();
                } else {
                    params += "Duration unavailable\n";
                    values += "\n";
                }
            }

            if let Some(duration_from_timestamps) = *self.duration_from_timestamps.borrow() {
                params += "Duration from TS (mus)\n";
                values += format!("{}", duration_from_timestamps * 1000000.0).as_str();
            }

            ui.text_disabled(params);
            ui.same_line();
            ui.text_disabled(values);
        }
    }

    fn get_name() -> &'static str {
        "Tutorial18: Queries"
    }
}

fn main() {
    sample_app::main::<Queries>().unwrap()
}
