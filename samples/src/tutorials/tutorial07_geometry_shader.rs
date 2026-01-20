use std::path::Path;

use diligent::{
    geometry_primitives::GeometryPrimitiveVertexFlags,
    graphics_utilities::{create_uniform_buffer, linear_to_srgba},
    *,
};

use diligent_samples::{
    sample_base::{
        sample::{get_adjusted_projection_matrix, get_surface_pretransform_matrix, SampleBase},
        sample_app::{self},
    },
    textured_cube::TexturedCube,
};

#[repr(C)]
struct Constants {
    world_view_proj: [f32; 4 * 4],
    viewport_size: [f32; 4],
    line_width: f32,
}

struct GeometryShader {
    device: Boxed<RenderDevice>,
    immediate_context: Boxed<ImmediateDeviceContext>,

    textured_cube: TexturedCube,

    convert_ps_output_to_gamma: bool,

    pipeline_state: Boxed<GraphicsPipelineState>,
    srb: Boxed<ShaderResourceBinding>,

    _texture_srv: Boxed<TextureView>,

    rotation_matrix: glam::Mat4,

    line_width: f32,

    vertex_shader_constants: Boxed<Buffer>,
}

impl SampleBase for GeometryShader {
    fn get_render_device(&self) -> &RenderDevice {
        &self.device
    }
    fn get_immediate_context(&self) -> &ImmediateDeviceContext {
        &self.immediate_context
    }

    fn modify_engine_init_info(
        engine_ci: &mut diligent_samples::sample_base::sample::EngineCreateInfo,
    ) {
        engine_ci
            .features
            .set_geometry_shaders(DeviceFeatureState::Enabled);
    }

    fn new(
        engine_factory: &EngineFactory,
        device: Boxed<RenderDevice>,
        immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        _deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        swap_chain_descs: &[&SwapChainDesc],
    ) -> Self {
        // We are only using one swap chain
        let swap_chain_desc = swap_chain_descs[0];

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = matches!(
            swap_chain_desc.color_buffer_format(),
            TextureFormat::RGBA8_UNORM | TextureFormat::BGRA8_UNORM,
        );

        // Create a shader source stream factory to load shaders from files.
        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        let shader_ci = ShaderCreateInfo::builder()
            .entry_point("main")
            // Tell the system that the shader source code is in HLSL.
            // For OpenGL, the engine will convert this into GLSL under the hood.
            .source_language(ShaderLanguage::HLSL)
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
            .shader_source_input_stream_factory(&shader_source_factory);

        // Create a vertex shader
        let vertex_shader = {
            let shader_ci = shader_ci
                .clone()
                .name("Cube VS")
                .source(ShaderSource::FilePath(Path::new(
                    "assets/cube_geometry.vsh",
                )))
                .shader_type(ShaderType::Vertex)
                .build();

            device.create_shader(&shader_ci).unwrap()
        };

        // Create a geometry shader
        let geometry_shader = {
            let shader_ci = shader_ci
                .clone()
                .name("Cube GS")
                .source(ShaderSource::FilePath(Path::new(
                    "assets/cube_geometry.gsh",
                )))
                .shader_type(ShaderType::Geometry)
                .build();

            device.create_shader(&shader_ci).unwrap()
        };

        // Create a pixel shader
        let pixel_shader = {
            let shader_ci = shader_ci
                .clone()
                .name("Cube PS")
                .source(ShaderSource::FilePath(Path::new(
                    "assets/cube_geometry.psh",
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
            // Cull back faces
            .cull_mode(CullMode::Back)
            .build();

        let depth_desc = DepthStencilStateDesc::builder()
            // Enable depth testing
            .depth_enable(true)
            .build();

        let mut rtv_formats = std::array::from_fn(|_| None);
        rtv_formats[0] = Some(swap_chain_desc.color_buffer_format());

        let pipeline_output = GraphicsPipelineRenderTargets::builder()
            // This tutorial will render to a single render target
            .num_render_targets(1)
            // Set render target format which is the format of the swap chain's color buffer
            .rtv_formats(rtv_formats)
            // Set depth buffer format which is the format of the swap chain's back buffer
            .dsv_format(swap_chain_desc.depth_buffer_format())
            .build();

        // Pipeline state object encompasses configuration of all GPU stages
        let pso_create_info = PipelineStateCreateInfo::builder()
            // Define variable type that will be used by default
            .default_variable_type(ShaderResourceVariableType::Static)
            // Shader variables should typically be mutable, which means they are expected
            // to change on a per-instance basis
            .shader_resource_variables(&[ShaderResourceVariableDesc::builder()
                .name(c"g_Texture")
                .variable_type(ShaderResourceVariableType::Mutable)
                .shader_stages(ShaderTypes::Pixel)
                .build()])
            .immutable_samplers(&[ImmutableSamplerDesc::builder()
                .shader_stages(ShaderTypes::Pixel)
                .sampler_or_texture_name(c"g_Texture")
                .sampler_desc(&sampler_desc)
                .build()])
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
                    .input_layouts(&input_layouts![
                        // Attribute 0 - vertex position
                        LayoutElement::builder().slot(0).f32_3(),
                        // Attribute 1 - texture coordinates
                        LayoutElement::builder().slot(0).f32_2(),
                    ])
                    .build(),
            )
            .vertex_shader(&vertex_shader)
            .geometry_shader(&geometry_shader)
            .pixel_shader(&pixel_shader)
            .build();

        let pso = device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        // Create dynamic uniform buffer that will store shader constants
        let shader_constants = create_uniform_buffer(
            &device,
            std::mem::size_of::<Constants>() as u64,
            c"Shader constants CB",
            Usage::Dynamic,
            BindFlags::UniformBuffer,
            CpuAccessFlags::Write,
        )
        .unwrap();

        // Since we did not explicitly specify the type for 'VSConstants', 'GSConstants',
        // and 'PSConstants' variables, default type (SHADER_RESOURCE_VARIABLE_TYPE_STATIC) will be used.
        // Static variables never change and are bound directly to the pipeline state object.
        pso.get_static_variable_by_name(ShaderType::Vertex, "VSConstants")
            .unwrap()
            .set(&shader_constants, SetShaderResourceFlags::None);
        pso.get_static_variable_by_name(ShaderType::Geometry, "GSConstants")
            .unwrap()
            .set(&shader_constants, SetShaderResourceFlags::None);
        pso.get_static_variable_by_name(ShaderType::Pixel, "PSConstants")
            .unwrap()
            .set(&shader_constants, SetShaderResourceFlags::None);

        // Since we are using mutable variable, we must create a shader resource binding object
        // http://diligentgraphics.com/2016/03/23/resource-binding-model-in-diligent-engine-2-0/
        let srb = pso.create_shader_resource_binding(true).unwrap();

        let texture_srv = {
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
            Boxed::<TextureView>::from_ref(
                texture
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
            )
        };

        srb.get_variable_by_name("g_Texture", ShaderTypes::Pixel)
            .unwrap()
            .set(&texture_srv, SetShaderResourceFlags::None);

        let textured_cube = TexturedCube::new(
            &device,
            GeometryPrimitiveVertexFlags::PosTex,
            BindFlags::VertexBuffer,
            None,
            BindFlags::IndexBuffer,
            None,
        )
        .unwrap();

        GeometryShader {
            device,
            convert_ps_output_to_gamma,
            pipeline_state: pso,
            immediate_context: immediate_contexts.into_iter().nth(0).unwrap(),
            srb,
            vertex_shader_constants: shader_constants,
            rotation_matrix: glam::Mat4::IDENTITY,
            line_width: 3.0,
            _texture_srv: texture_srv,
            textured_cube,
        }
    }

    fn update_ui(&mut self, ui: &mut imgui::Ui) {
        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .begin()
        {
            let _ = ui.slider("Line Width", 1.0, 10.0, &mut self.line_width);
        }
    }

    fn update(&mut self, current_time: f64, _elapsed_time: f64) {
        // Apply rotation
        self.rotation_matrix = glam::Mat4::from_rotation_x(-std::f32::consts::PI * 0.1)
            * glam::Mat4::from_rotation_y(current_time as f32);
    }

    fn render(&self, swap_chain: &SwapChain) {
        let immediate_context = self.get_immediate_context();

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
            let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 5.0));

            proj * srf_pre_transform * view
        };

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

            immediate_context.clear_render_target::<f32>(
                rtv,
                &clear_color,
                ResourceStateTransitionMode::Transition,
            );
        }

        immediate_context.clear_depth(dsv, 1.0, ResourceStateTransitionMode::Transition);

        {
            // Map the buffer and write current world-view-projection matrix
            let mut constants = immediate_context
                .map_buffer_write(&self.vertex_shader_constants, MapFlags::Discard);

            constants[0] = Constants {
                world_view_proj: (view_proj_matrix * self.rotation_matrix).to_cols_array(),
                viewport_size: [
                    swap_chain_desc.width() as f32,
                    swap_chain_desc.height() as f32,
                    1.0 / swap_chain_desc.width() as f32,
                    1.0 / swap_chain_desc.height() as f32,
                ],
                line_width: self.line_width,
            };
        }

        {
            // Bind vertex and index buffers
            immediate_context.set_vertex_buffers(
                &[(self.textured_cube.get_vertex_buffer(), 0)],
                ResourceStateTransitionMode::Transition,
                SetVertexBufferFlags::Reset,
            );
            immediate_context.set_index_buffer(
                self.textured_cube.get_index_buffer(),
                0,
                ResourceStateTransitionMode::Transition,
            );
        }

        // Set the pipeline state
        let graphics = immediate_context.set_graphics_pipeline_state(&self.pipeline_state);

        // Commit shader resources. RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode
        // makes sure that resources are transitioned to required states.
        immediate_context
            .commit_shader_resources(&self.srb, ResourceStateTransitionMode::Transition);

        let draw_attribs = DrawIndexedAttribs::builder()
            .num_indices(36)
            .index_type(ValueType::Uint32)
            // Verify the state of vertex and index buffers
            .flags(DrawFlags::VerifyAll)
            .build();

        graphics.draw_indexed(&draw_attribs);
    }

    fn get_name() -> &'static str {
        "Tutorial07: GeometryShader"
    }
}

fn main() {
    sample_app::main::<GeometryShader>().unwrap()
}
