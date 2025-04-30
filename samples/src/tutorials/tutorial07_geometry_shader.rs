use std::path::Path;

use diligent::{
    accessories::linear_to_srgba,
    buffer::{Buffer, BufferMode},
    device_context::{
        DeferredDeviceContext, DrawFlags, DrawIndexedAttribs, ImmediateDeviceContext,
        ResourceStateTransitionMode, SetVertexBufferFlags,
    },
    engine_factory::EngineFactory,
    geometry_primitives::GeometryPrimitiveVertexFlags,
    graphics_types::{
        BindFlags, CpuAccessFlags, DeviceFeatureState, FilterType, MapFlags, PrimitiveTopology,
        SetShaderResourceFlags, ShaderType, ShaderTypes, TextureAddressMode, TextureFormat, Usage,
        ValueType,
    },
    graphics_utilities::create_uniform_buffer,
    input_layout::LayoutElement,
    pipeline_resource_signature::ImmutableSamplerDesc,
    pipeline_state::{
        BlendStateDesc, CullMode, DepthStencilStateDesc, GraphicsPipelineDesc,
        GraphicsPipelineRenderTargets, GraphicsPipelineStateCreateInfo, PipelineState,
        RasterizerStateDesc,
    },
    render_device::RenderDevice,
    sampler::SamplerDesc,
    shader::{
        ShaderCompileFlags, ShaderCreateInfo, ShaderLanguage, ShaderSource,
        ShaderSourceInputStreamFactory,
    },
    shader_resource_binding::ShaderResourceBinding,
    shader_resource_variable::{ShaderResourceVariableDesc, ShaderResourceVariableType},
    swap_chain::SwapChain,
    texture::{TextureDesc, TextureDimension, TextureSubResource},
    texture_view::{TextureView, TextureViewType},
};

use diligent_tools::native_app;

use diligent_samples::{
    sample::{get_adjusted_projection_matrix, get_surface_pretransform_matrix, SampleBase},
    sample_app::SampleApp,
    textured_cube::TexturedCube,
};

#[repr(C)]
struct Constants {
    world_view_proj: [f32; 4 * 4],
    viewport_size: [f32; 4],
    line_width: f32,
}

struct GeometryShader {
    immediate_context: ImmediateDeviceContext,

    textured_cube: TexturedCube,

    convert_ps_output_to_gamma: bool,

    pipeline_state: PipelineState,
    srb: ShaderResourceBinding,

    _texture_srv: TextureView,

    rotation_matrix: glam::Mat4,

    line_width: f32,

    vertex_shader_constants: Buffer,
}

impl SampleBase for GeometryShader {
    fn get_immediate_context(&self) -> &ImmediateDeviceContext {
        &self.immediate_context
    }

    fn modify_engine_init_info(engine_ci: &mut diligent_samples::sample::EngineCreateInfo) {
        engine_ci.features.geometry_shaders = DeviceFeatureState::Enabled;
    }

    fn new(
        engine_factory: &EngineFactory,
        device: &RenderDevice,
        immediate_contexts: Vec<ImmediateDeviceContext>,
        _deferred_contexts: Vec<DeferredDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
        let swap_chain_desc = swap_chain.get_desc();

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = match swap_chain_desc.color_buffer_format {
            TextureFormat::RGBA8_UNORM | TextureFormat::BGRA8_UNORM => true,
            _ => false,
        };

        // Create a shader source stream factory to load shaders from files.
        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        fn common_shader_ci<'a>(
            name: &'a str,
            source: ShaderSource<'a>,
            shader_type: ShaderType,
            convert_ps_output_to_gamma: bool,
            shader_source_factory: &'a ShaderSourceInputStreamFactory,
        ) -> ShaderCreateInfo<'a> {
            ShaderCreateInfo::new(name, source, shader_type)
                .entry_point("main")
                // Tell the system that the shader source code is in HLSL.
                // For OpenGL, the engine will convert this into GLSL under the hood.
                .language(ShaderLanguage::HLSL)
                // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
                .use_combined_texture_samplers(true)
                // Pack matrices in row-major order
                .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
                // Presentation engine always expects input in gamma space. Normally, pixel shader output is
                // converted from linear to gamma space by the GPU. However, some platforms (e.g. Android in GLES mode,
                // or Emscripten in WebGL mode) do not support gamma-correction. In this case the application
                // has to do the conversion manually.
                .set_macros(vec![(
                    "CONVERT_PS_OUTPUT_TO_GAMMA",
                    if convert_ps_output_to_gamma { "1" } else { "0" },
                )])
                .shader_source_input_stream_factory(Some(&shader_source_factory))
        }

        // Create a vertex shader
        let vertex_shader = {
            let shader_ci = common_shader_ci(
                "Cube VS",
                ShaderSource::FilePath(Path::new("assets/cube_geometry.vsh")),
                ShaderType::Vertex,
                convert_ps_output_to_gamma,
                &shader_source_factory,
            );

            device.create_shader(&shader_ci).unwrap()
        };

        // Create a geometry shader
        let geometry_shader = {
            let shader_ci = common_shader_ci(
                "Cube GS",
                ShaderSource::FilePath(Path::new("assets/cube_geometry.gsh")),
                ShaderType::Geometry,
                convert_ps_output_to_gamma,
                &shader_source_factory,
            );

            device.create_shader(&shader_ci).unwrap()
        };

        // Create a pixel shader
        let pixel_shader = {
            let shader_ci = common_shader_ci(
                "Cube PS",
                ShaderSource::FilePath(Path::new("assets/cube_geometry.psh")),
                ShaderType::Pixel,
                convert_ps_output_to_gamma,
                &shader_source_factory,
            );

            device.create_shader(&shader_ci).unwrap()
        };

        // Define immutable sampler for g_Texture. Immutable samplers should be used whenever possible
        let sampler_desc = SamplerDesc::new("Cube texture sampler")
            .min_filter(FilterType::Linear)
            .mag_filter(FilterType::Linear)
            .mip_filter(FilterType::Linear)
            .address_u(TextureAddressMode::Clamp)
            .address_v(TextureAddressMode::Clamp)
            .address_w(TextureAddressMode::Clamp);

        // Pipeline state object encompasses configuration of all GPU stages
        let pso_create_info = GraphicsPipelineStateCreateInfo::new(
            "Cube PSO",
            GraphicsPipelineDesc::new(
                BlendStateDesc::default(),
                RasterizerStateDesc::default()
                    // Cull back faces
                    .cull_mode(CullMode::Back),
                DepthStencilStateDesc::default()
                    // Enable depth testing
                    .depth_enable(true),
                GraphicsPipelineRenderTargets::default() // This tutorial will render to a single render target
                    .num_render_targets(1)
                    // Set render target format which is the format of the swap chain's color buffer
                    .rtv_format::<0>(swap_chain_desc.color_buffer_format)
                    // Set depth buffer format which is the format of the swap chain's back buffer
                    .dsv_format(swap_chain_desc.depth_buffer_format),
            )
            // Primitive topology defines what kind of primitives will be rendered by this pipeline state
            .primitive_topology(PrimitiveTopology::TriangleList)
            // Define vertex shader input layout
            .set_input_layouts([
                // Attribute 0 - vertex position
                LayoutElement::new(0, 3, ValueType::Float32).is_normalized(false),
                // Attribute 1 - texture coordinates
                LayoutElement::new(0, 2, ValueType::Float32).is_normalized(false),
            ]),
        )
        .vertex_shader(&vertex_shader)
        .geometry_shader(&geometry_shader)
        .pixel_shader(&pixel_shader)
        // Define variable type that will be used by default
        .default_variable_type(ShaderResourceVariableType::Static)
        // Shader variables should typically be mutable, which means they are expected
        // to change on a per-instance basis
        .set_shader_resource_variables([ShaderResourceVariableDesc::new(
            "g_Texture",
            ShaderResourceVariableType::Mutable,
            ShaderTypes::Pixel,
        )])
        .set_immutable_samplers([ImmutableSamplerDesc::new(
            ShaderTypes::Pixel,
            "g_Texture",
            &sampler_desc,
        )]);

        let pso = device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        // Create dynamic uniform buffer that will store shader constants
        let shader_constants = create_uniform_buffer(
            &device,
            std::mem::size_of::<Constants>() as u64,
            "Shader constants CB",
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

            let texture = device
                .create_texture(
                    &TextureDesc::new(
                        "DGLogo",
                        TextureDimension::Texture2D,
                        image.width(),
                        image.height(),
                        TextureFormat::RGBA8_UNORM_SRGB,
                    )
                    .bind_flags(BindFlags::ShaderResource)
                    .usage(Usage::Immutable),
                    &[&TextureSubResource::new_cpu(
                        image.as_bytes(),
                        image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
                    )],
                    None,
                )
                .unwrap();

            // Get shader resource view from the texture
            texture
                .get_default_view(TextureViewType::ShaderResource)
                .unwrap()
        };

        srb.get_variable_by_name("g_Texture", ShaderTypes::Pixel)
            .unwrap()
            .set(&texture_srv, SetShaderResourceFlags::None);

        let textured_cube = TexturedCube::new(
            &device,
            GeometryPrimitiveVertexFlags::PosTex,
            BindFlags::VertexBuffer,
            BufferMode::Undefined,
            BindFlags::IndexBuffer,
            BufferMode::Undefined,
        )
        .unwrap();

        GeometryShader {
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

        let swap_chain_desc = swap_chain.get_desc();

        let view_proj_matrix = {
            let swap_chain_desc = swap_chain.get_desc();

            // Get pretransform matrix that rotates the scene according the surface orientation
            let srf_pre_transform =
                get_surface_pretransform_matrix(&swap_chain_desc, &glam::Vec3::new(0.0, 0.0, 1.0));

            // Get projection matrix adjusted to the current screen orientation
            let proj = get_adjusted_projection_matrix(
                &swap_chain_desc,
                std::f32::consts::PI / 4.0,
                0.1,
                100.0,
            );

            // Set cube view matrix
            let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 5.0));

            proj * srf_pre_transform * view
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

        {
            // Map the buffer and write current world-view-projection matrix
            let mut constants = immediate_context
                .map_buffer_write(&self.vertex_shader_constants, MapFlags::Discard);

            let buffer_write = unsafe { constants.as_mut() };
            *buffer_write = Constants {
                world_view_proj: (view_proj_matrix * self.rotation_matrix).to_cols_array(),
                viewport_size: [
                    swap_chain_desc.width as f32,
                    swap_chain_desc.height as f32,
                    1.0 / swap_chain_desc.width as f32,
                    1.0 / swap_chain_desc.height as f32,
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
        immediate_context.set_pipeline_state(&self.pipeline_state);

        // Commit shader resources. RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode
        // makes sure that resources are transitioned to required states.
        immediate_context
            .commit_shader_resources(&self.srb, ResourceStateTransitionMode::Transition);

        let draw_attribs = DrawIndexedAttribs::new(36, ValueType::Uint32)
            // Verify the state of vertex and index buffers
            .flags(DrawFlags::VerifyAll);

        immediate_context.draw_indexed(&draw_attribs);
    }

    fn get_name() -> &'static str {
        "Tutorial07: GeometryShader"
    }
}

fn main() {
    native_app::main::<SampleApp<GeometryShader>>().unwrap()
}
