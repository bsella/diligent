use std::path::Path;

use diligent::{graphics_utilities::linear_to_srgba, *};

use diligent_tools::native_app;

use diligent_samples::sample_base::{
    sample::{get_adjusted_projection_matrix, get_surface_pretransform_matrix, SampleBase},
    sample_app::SampleApp,
};

struct Texturing {
    device: Boxed<RenderDevice>,
    immediate_context: Boxed<ImmediateDeviceContext>,

    convert_ps_output_to_gamma: bool,

    pipeline_state: Boxed<GraphicsPipelineState>,
    vertex_shader_constant_buffer: Boxed<Buffer>,
    cube_vertex_buffer: Boxed<Buffer>,
    cube_index_buffer: Boxed<Buffer>,
    srb: Boxed<ShaderResourceBinding>,

    _texture_srv: Boxed<TextureView>,

    world_view_matrix: glam::Mat4,
}

impl SampleBase for Texturing {
    fn get_render_device(&self) -> &RenderDevice {
        &self.device
    }
    fn get_immediate_context(&self) -> &ImmediateDeviceContext {
        &self.immediate_context
    }

    fn new(
        engine_factory: &EngineFactory,
        device: Boxed<RenderDevice>,
        immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        _deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        swap_chain: &SwapChain,
    ) -> Self {
        let swap_chain_desc = swap_chain.get_desc();

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = matches!(
            swap_chain_desc.color_buffer_format(),
            TextureFormat::RGBA8_UNORM | TextureFormat::BGRA8_UNORM,
        );

        // In this tutorial, we will load shaders from file. To be able to do that,
        // we need to create a shader source stream factory
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
        let vertex_shader = device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Cube VS")
                    .source(ShaderSource::FilePath(Path::new("assets/cube_texture.vsh")))
                    .shader_type(ShaderType::Vertex)
                    .build(),
            )
            .unwrap();

        // Create dynamic uniform buffer that will store our transformation matrix
        // Dynamic buffers can be frequently updated by the CPU
        let vertex_shader_constant_buffer = device
            .create_buffer(
                &BufferDesc::builder()
                    .name("VS constants CB")
                    .size((std::mem::size_of::<glam::Mat4>()) as u64)
                    .usage(Usage::Dynamic)
                    .bind_flags(BindFlags::UniformBuffer)
                    .cpu_access_flags(CpuAccessFlags::Write)
                    .build(),
            )
            .unwrap();

        // Create a pixel shader
        let pixel_shader = device
            .create_shader(
                &shader_ci
                    .clone()
                    .name("Cube PS")
                    .source(ShaderSource::FilePath(Path::new("assets/cube_texture.psh")))
                    .shader_type(ShaderType::Pixel)
                    .build(),
            )
            .unwrap();

        // Define immutable sampler for g_Texture. Immutable samplers should be used whenever possible
        let sampler_desc = SamplerDesc::builder()
            .name("Cube texture sampler")
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

        // Pipeline state object encompasses configuration of all GPU stages
        let pso_create_info = PipelineStateCreateInfo::builder()
            // Define variable type that will be used by default
            .default_variable_type(ShaderResourceVariableType::Static)
            // Shader variables should typically be mutable, which means they are expected
            // to change on a per-instance basis
            .shader_resource_variables([ShaderResourceVariableDesc::builder()
                .name("g_Texture")
                .variable_type(ShaderResourceVariableType::Mutable)
                .shader_stages(ShaderTypes::Pixel)
                .build()])
            // Define immutable sampler for g_Texture. Immutable samplers should be used whenever possible
            .immutable_samplers([ImmutableSamplerDesc::new(
                ShaderTypes::Pixel,
                "g_Texture",
                &sampler_desc,
            )])
            .name("Cube PSO")
            .graphics()
            .graphics_pipeline_desc(
                GraphicsPipelineDesc::builder()
                    .rasterizer_desc(rasterizer_desc)
                    .depth_stencil_desc(depth_desc)
                    .output(
                        GraphicsPipelineRenderTargets::builder() // This tutorial will render to a single render target
                            .num_render_targets(1)
                            // Set render target format which is the format of the swap chain's color buffer
                            .rtv_formats(rtv_formats)
                            // Set depth buffer format which is the format of the swap chain's back buffer
                            .dsv_format(swap_chain_desc.depth_buffer_format())
                            .build(),
                    )
                    // Primitive topology defines what kind of primitives will be rendered by this pipeline state
                    .primitive_topology(PrimitiveTopology::TriangleList)
                    // Define vertex shader input layout
                    .input_layouts([
                        // Attribute 0 - vertex position
                        LayoutElement::builder().slot(0).f32_3().build(),
                        // Attribute 1 - vertex color
                        LayoutElement::builder().slot(0).f32_2().build(),
                    ])
                    .build(),
            )
            .vertex_shader(&vertex_shader)
            .pixel_shader(&pixel_shader)
            .build();

        let pipeline_state = device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        // Since we did not explicitly specify the type for 'Constants' variable, default
        // type (SHADER_RESOURCE_VARIABLE_TYPE_STATIC) will be used. Static variables never
        // change and are bound directly through the pipeline state object.
        pipeline_state
            .get_static_variable_by_name(ShaderType::Vertex, "Constants")
            .unwrap()
            .set(&vertex_shader_constant_buffer, SetShaderResourceFlags::None);

        // Create a shader resource binding object and bind all static resources in it
        let srb = pipeline_state.create_shader_resource_binding(true).unwrap();

        // Layout of this structure matches the one we defined in the pipeline state
        #[repr(C)]
        struct Vertex {
            pos: [f32; 3],
            uv: [f32; 2],
        }

        // Create a vertex buffer that stores cube vertices
        let cube_vertex_buffer = {
            // Cube vertices

            //      (-1,+1,+1)________________(+1,+1,+1)
            //               /|              /|
            //              / |             / |
            //             /  |            /  |
            //            /   |           /   |
            //(-1,-1,+1) /____|__________/(+1,-1,+1)
            //           |    |__________|____|
            //           |   /(-1,+1,-1) |    /(+1,+1,-1)
            //           |  /            |   /
            //           | /             |  /
            //           |/              | /
            //           /_______________|/
            //        (-1,-1,-1)       (+1,-1,-1)
            #[rustfmt::skip]
            const CUBE_VERTS : [Vertex; 24] = [
                Vertex{pos : [-1.0, -1.0, -1.0], uv: [0.0, 1.0]},
                Vertex{pos : [-1.0,  1.0, -1.0], uv: [0.0, 0.0]},
                Vertex{pos : [ 1.0,  1.0, -1.0], uv: [1.0, 0.0]},
                Vertex{pos : [ 1.0, -1.0, -1.0], uv: [1.0, 1.0]},

                Vertex{pos : [-1.0, -1.0, -1.0], uv: [0.0, 1.0]},
                Vertex{pos : [-1.0, -1.0,  1.0], uv: [0.0, 0.0]},
                Vertex{pos : [ 1.0, -1.0,  1.0], uv: [1.0, 0.0]},
                Vertex{pos : [ 1.0, -1.0, -1.0], uv: [1.0, 1.0]},

                Vertex{pos : [ 1.0, -1.0, -1.0], uv: [0.0, 1.0]},
                Vertex{pos : [ 1.0, -1.0,  1.0], uv: [1.0, 1.0]},
                Vertex{pos : [ 1.0,  1.0,  1.0], uv: [1.0, 0.0]},
                Vertex{pos : [ 1.0,  1.0, -1.0], uv: [0.0, 0.0]},

                Vertex{pos : [ 1.0,  1.0, -1.0], uv: [0.0, 1.0]},
                Vertex{pos : [ 1.0,  1.0,  1.0], uv: [0.0, 0.0]},
                Vertex{pos : [-1.0,  1.0,  1.0], uv: [1.0, 0.0]},
                Vertex{pos : [-1.0,  1.0, -1.0], uv: [1.0, 1.0]},

                Vertex{pos : [-1.0,  1.0, -1.0], uv: [1.0, 0.0]},
                Vertex{pos : [-1.0,  1.0,  1.0], uv: [0.0, 0.0]},
                Vertex{pos : [-1.0, -1.0,  1.0], uv: [0.0, 1.0]},
                Vertex{pos : [-1.0, -1.0, -1.0], uv: [1.0, 1.0]},

                Vertex{pos : [-1.0, -1.0,  1.0], uv: [1.0, 1.0]},
                Vertex{pos : [ 1.0, -1.0,  1.0], uv: [0.0, 1.0]},
                Vertex{pos : [ 1.0,  1.0,  1.0], uv: [0.0, 0.0]},
                Vertex{pos : [-1.0,  1.0,  1.0], uv: [1.0, 0.0]},
            ];

            let vertex_buffer_desc = BufferDesc::builder()
                .name("Cube vertex buffer")
                .size(std::mem::size_of_val(&CUBE_VERTS) as u64)
                .usage(Usage::Immutable)
                .bind_flags(BindFlags::VertexBuffer)
                .build();
            device
                .create_buffer_with_data(&vertex_buffer_desc, &CUBE_VERTS, None)
                .unwrap()
        };

        let cube_index_buffer = {
            #[rustfmt::skip]
            const INDICES : [u32; 36] =
            [
                2,0,1,    2,3,0,
                4,6,5,    4,7,6,
                8,10,9,   8,11,10,
                12,14,13, 12,15,14,
                16,18,17, 16,19,18,
                20,21,22, 20,22,23
            ];

            let vertex_buffer_desc = BufferDesc::builder()
                .name("Cube index buffer")
                .size(std::mem::size_of_val(&INDICES) as u64)
                .usage(Usage::Immutable)
                .bind_flags(BindFlags::IndexBuffer)
                .build();

            device
                .create_buffer_with_data(&vertex_buffer_desc, &INDICES, None)
                .unwrap()
        };

        let texture_srv = {
            let image = image::ImageReader::open("assets/DGLogo.png")
                .unwrap()
                .decode()
                .unwrap();

            let texture_desc = TextureDesc::builder()
                .name("DGLogo")
                .dimension(TextureDimension::Texture2D)
                .width(image.width())
                .height(image.height())
                .format(TextureFormat::RGBA8_UNORM_SRGB)
                .bind_flags(BindFlags::ShaderResource)
                .usage(Usage::Immutable)
                .build();

            let texture = device
                .create_texture(
                    &texture_desc,
                    &[&TextureSubResource::builder()
                        .from_host(
                            image.as_bytes(),
                            image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
                        )
                        .build()],
                    None,
                )
                .unwrap();

            // Get shader resource view from the texture
            Boxed::<TextureView>::from_ref(
                texture
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
            )
        };

        // Set texture SRV in the SRB
        srb.get_variable_by_name("g_Texture", ShaderTypes::Pixel)
            .unwrap()
            .set(&texture_srv, SetShaderResourceFlags::None);

        Texturing {
            device,
            convert_ps_output_to_gamma,
            pipeline_state,
            cube_vertex_buffer,
            cube_index_buffer,
            immediate_context: immediate_contexts.into_iter().nth(0).unwrap(),
            srb,
            _texture_srv: texture_srv,
            vertex_shader_constant_buffer,
            world_view_matrix: glam::Mat4::IDENTITY,
        }
    }

    fn update(&mut self, current_time: f64, _elapsed_time: f64) {
        // Apply rotation
        let cube_model_transform = glam::Mat4::from_rotation_x(-std::f32::consts::PI * 0.1)
            * glam::Mat4::from_rotation_y(current_time as f32);

        // Camera is at (0, 0, -5) looking along the Z axis
        let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 5.0));

        // Compute world-view-projection matrix
        self.world_view_matrix = view * cube_model_transform;
    }

    fn render(&self, swap_chain: &SwapChain) {
        let immediate_context = self.get_immediate_context();

        let rtv = swap_chain.get_current_back_buffer_rtv().unwrap();
        let dsv = swap_chain.get_depth_buffer_dsv().unwrap();

        // Clear the back buffer
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

        immediate_context.clear_depth(dsv, 1.0, ResourceStateTransitionMode::Transition);

        {
            let swap_chain_desc = swap_chain.get_desc();

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

            let model_view_proj = proj * srf_pre_transform * self.world_view_matrix;

            {
                // Map the buffer and write current world-view-projection matrix
                let mut constant_buffer_data = immediate_context
                    .map_buffer_write(&self.vertex_shader_constant_buffer, MapFlags::Discard);

                *unsafe { constant_buffer_data.as_mut() } = model_view_proj;
            }
        }

        // Bind vertex and index buffers
        immediate_context.set_vertex_buffers(
            &[(&self.cube_vertex_buffer, 0)],
            ResourceStateTransitionMode::Transition,
            SetVertexBufferFlags::Reset,
        );
        immediate_context.set_index_buffer(
            &self.cube_index_buffer,
            0,
            ResourceStateTransitionMode::Transition,
        );

        // Set the pipeline state in the immediate context
        let graphics = immediate_context.set_graphics_pipeline_state(&self.pipeline_state);

        // Commit shader resources. RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode
        // makes sure that resources are transitioned to required states.
        immediate_context
            .commit_shader_resources(&self.srb, ResourceStateTransitionMode::Transition);

        graphics.draw_indexed(
            &DrawIndexedAttribs::builder()
                .num_indices(36)
                .index_type(ValueType::Uint32)
                // Verify the state of vertex and index buffers
                .flags(DrawFlags::VerifyAll)
                .build(),
        );
    }

    fn get_name() -> &'static str {
        "Tutorial03: Texturing"
    }
}

fn main() {
    native_app::main::<SampleApp<Texturing>>().unwrap()
}
