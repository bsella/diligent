use diligent::{
    core::{
        accessories::linear_to_srgba,
        buffer::{Buffer, BufferDesc},
        device_context::{
            DeviceContext, DrawFlags, DrawIndexedAttribs, ResourceStateTransitionMode,
            SetVertexBufferFlags,
        },
        engine_factory::EngineFactory,
        graphics_types::{
            BindFlags, CpuAccessFlags, MapFlags, PrimitiveTopology, SetShaderResourceFlags,
            ShaderType, Usage, ValueType,
        },
        input_layout::LayoutElement,
        pipeline_state::{
            BlendStateDesc, DepthStencilStateDesc, GraphicsPipelineDesc,
            GraphicsPipelineStateCreateInfo, PipelineState, RasterizerStateDesc,
        },
        render_device::RenderDevice,
        shader::{ShaderCompileFlags, ShaderCreateInfo, ShaderLanguage, ShaderSource},
        shader_resource_binding::ShaderResourceBinding,
        shader_resource_variable::ShaderResourceVariableType,
        swap_chain::SwapChain,
    },
    samples::{
        sample::{get_adjusted_projection_matrix, get_surface_pretransform_matrix, SampleBase},
        sample_app::SampleApp,
    },
    tools::native_app,
};

struct Cube {
    render_device: RenderDevice,
    immediate_contexts: Vec<DeviceContext>,

    convert_ps_output_to_gamma: bool,

    pipeline_state: PipelineState,
    vertex_shader_constant_buffer: Buffer,
    cube_vertex_buffer: Buffer,
    cube_index_buffer: Buffer,
    srb: ShaderResourceBinding,

    world_view_matrix: glam::Mat4,
}

impl SampleBase for Cube {
    fn get_render_device(&self) -> &RenderDevice {
        &self.render_device
    }

    fn get_immediate_context(&self) -> &DeviceContext {
        self.immediate_contexts.first().unwrap()
    }

    fn new(
        engine_factory: &EngineFactory,
        render_device: RenderDevice,
        immediate_contexts: Vec<DeviceContext>,
        _deferred_contexts: Vec<DeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
        let swap_chain_desc = swap_chain.get_desc();

        // If the swap chain color buffer format is a non-sRGB UNORM format,
        // we need to manually convert pixel shader output to gamma space.
        let convert_ps_output_to_gamma = swap_chain_desc.color_buffer_format
            == diligent_sys::TEX_FORMAT_RGBA8_UNORM as diligent_sys::TEXTURE_FORMAT
            || swap_chain_desc.color_buffer_format
                == diligent_sys::TEX_FORMAT_BGRA8_UNORM as diligent_sys::TEXTURE_FORMAT;

        // In this tutorial, we will load shaders from file. To be able to do that,
        // we need to create a shader source stream factory
        let shader_source_factory = engine_factory
            .create_default_shader_source_stream_factory(&[])
            .unwrap();

        let shader_ci = ShaderCreateInfo::new(
            c"Cube VS",
            ShaderSource::FilePath(c"cube.vsh"),
            ShaderType::Vertex,
        )
        .entry_point(c"main")
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
        .add_macro(
            c"CONVERT_PS_OUTPUT_TO_GAMMA",
            if convert_ps_output_to_gamma {
                c"1"
            } else {
                c"0"
            },
        )
        .shader_source_input_stream_factory(Some(&shader_source_factory));

        // Create a vertex shader
        let vertex_shader = render_device.create_shader(&shader_ci).unwrap();

        // Create dynamic uniform buffer that will store our transformation matrix
        // Dynamic buffers can be frequently updated by the CPU
        let vertex_shader_constant_buffer = render_device
            .create_buffer(
                &BufferDesc::new(
                    c"VS constants CB",
                    (std::mem::size_of::<f32>() * 4 * 4) as u64,
                )
                .usage(Usage::Dynamic)
                .bind_flags(BindFlags::UniformBuffer)
                .cpu_access_flags(CpuAccessFlags::Write),
            )
            .unwrap();

        // Create a pixel shader
        let pixel_shader = render_device
            .create_shader(
                &shader_ci
                    .name(c"Cube PS")
                    .source(ShaderSource::FilePath(c"cube.psh"))
                    .shader_type(ShaderType::Pixel),
            )
            .unwrap();

        // Pipeline state object encompasses configuration of all GPU stages
        let pso_create_info = GraphicsPipelineStateCreateInfo::new(
            // Pipeline state name is used by the engine to report issues.
            c"Cube PSO",
            GraphicsPipelineDesc::new(
                BlendStateDesc::default(),
                RasterizerStateDesc::default()
                    // Cull back faces
                    .cull_mode(diligent::core::pipeline_state::CullMode::Back),
                DepthStencilStateDesc::default()
                    // Enable depth testing
                    .depth_enable(true),
            )
            // This tutorial will render to a single render target
            .num_render_targets(1)
            // Set render target format which is the format of the swap chain's color buffer
            .rtv_format::<0>(swap_chain_desc.color_buffer_format as diligent_sys::_TEXTURE_FORMAT)
            // Set depth buffer format which is the format of the swap chain's back buffer
            .dsv_format(swap_chain_desc.depth_buffer_format as diligent_sys::_TEXTURE_FORMAT)
            // Primitive topology defines what kind of primitives will be rendered by this pipeline state
            .primitive_topology(PrimitiveTopology::TriangleList)
            // Define vertex shader input layout
            // Attribute 0 - vertex position
            .add_input_layout(LayoutElement::new(0, 0, 3, ValueType::Float32).is_normalized(false))
            // Attribute 1 - vertex color
            .add_input_layout(LayoutElement::new(1, 0, 4, ValueType::Float32).is_normalized(false)),
        )
        // Define variable type that will be used by default
        .default_variable_type(ShaderResourceVariableType::Static)
        .vertex_shader(&vertex_shader)
        .pixel_shader(&pixel_shader);

        let pipeline_state = render_device
            .create_graphics_pipeline_state(&pso_create_info)
            .unwrap();

        // Since we did not explicitly specify the type for 'Constants' variable, default
        // type (SHADER_RESOURCE_VARIABLE_TYPE_STATIC) will be used. Static variables never
        // change and are bound directly through the pipeline state object.
        pipeline_state
            .get_static_variable_by_name(ShaderType::Vertex, c"Constants")
            .unwrap()
            .set(&vertex_shader_constant_buffer, SetShaderResourceFlags::None);

        // Create a shader resource binding object and bind all static resources in it
        let srb = pipeline_state.create_shader_resource_binding(true).unwrap();

        // Layout of this structure matches the one we defined in the pipeline state
        #[repr(C)]
        struct Vertex {
            pos: [f32; 3],
            color: [f32; 4],
        }

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
        const CUBE_VERTS : [Vertex; 8] = [
            Vertex{ pos:[-1.0, -1.0, -1.0], color:[1.0, 0.0, 0.0, 1.0] },
            Vertex{ pos:[-1.0,  1.0, -1.0], color:[0.0, 1.0, 0.0, 1.0] },
            Vertex{ pos:[ 1.0,  1.0, -1.0], color:[0.0, 0.0, 1.0, 1.0] },
            Vertex{ pos:[ 1.0, -1.0, -1.0], color:[1.0, 1.0, 1.0, 1.0] },
            Vertex{ pos:[-1.0, -1.0,  1.0], color:[1.0, 1.0, 0.0, 1.0] },
            Vertex{ pos:[-1.0,  1.0,  1.0], color:[0.0, 1.0, 1.0, 1.0] },
            Vertex{ pos:[ 1.0,  1.0,  1.0], color:[1.0, 0.0, 1.0, 1.0] },
            Vertex{ pos:[ 1.0, -1.0,  1.0], color:[0.2, 0.2, 0.2, 1.0] },
        ];

        // Create a vertex buffer that stores cube vertices
        let cube_vertex_buffer = {
            let vertex_buffer_desc = BufferDesc::new(
                c"Cube vertex buffer",
                std::mem::size_of_val(&CUBE_VERTS) as u64,
            )
            .usage(Usage::Immutable)
            .bind_flags(BindFlags::VertexBuffer);
            render_device
                .create_buffer_with_data(&vertex_buffer_desc, &CUBE_VERTS, None)
                .unwrap()
        };

        #[rustfmt::skip]
        const INDICES : [u32; 36] =
        [
            2,0,1, 2,3,0,
            4,6,5, 4,7,6,
            0,7,4, 0,3,7,
            1,0,4, 1,4,5,
            1,5,2, 5,6,2,
            3,6,7, 3,2,6
        ];

        let cube_index_buffer = {
            let vertex_buffer_desc =
                BufferDesc::new(c"Cube index buffer", std::mem::size_of_val(&INDICES) as u64)
                    .usage(Usage::Immutable)
                    .bind_flags(BindFlags::IndexBuffer);
            render_device
                .create_buffer_with_data(&vertex_buffer_desc, &INDICES, None)
                .unwrap()
        };

        Cube {
            convert_ps_output_to_gamma,
            pipeline_state,
            cube_vertex_buffer,
            cube_index_buffer,
            immediate_contexts,
            render_device,
            srb,
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

        let mut rtv = swap_chain.get_current_back_buffer_rtv();
        let mut dsv = swap_chain.get_depth_buffer_dsv();

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
            &mut rtv,
            &clear_color,
            ResourceStateTransitionMode::Transition,
        );
        immediate_context.clear_depth(&mut dsv, 1.0, ResourceStateTransitionMode::Transition);

        {
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

            let model_view_proj = proj * srf_pre_transform * self.world_view_matrix;

            {
                // Map the buffer and write current world-view-projection matrix
                let constant_buffer_data = immediate_context
                    .map_buffer_write(&self.vertex_shader_constant_buffer, MapFlags::Discard);

                unsafe {
                    std::ptr::copy_nonoverlapping(
                        std::ptr::from_ref(&model_view_proj),
                        constant_buffer_data.get_ptr_mut(),
                        1,
                    );
                }
            }
        }

        // Bind vertex and index buffers
        immediate_context.set_vertex_buffers(
            &[&self.cube_vertex_buffer],
            &[0],
            ResourceStateTransitionMode::Transition,
            SetVertexBufferFlags::Reset,
        );
        immediate_context.set_index_buffer(
            &self.cube_index_buffer,
            0,
            ResourceStateTransitionMode::Transition,
        );

        // Set the pipeline state
        immediate_context.set_pipeline_state(&self.pipeline_state);

        // Commit shader resources. RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode
        // makes sure that resources are transitioned to required states.
        immediate_context
            .commit_shader_resources(&self.srb, ResourceStateTransitionMode::Transition);

        immediate_context.draw_indexed(
            &DrawIndexedAttribs::new(36, ValueType::Uint32)
                // Verify the state of vertex and index buffers
                .flags(DrawFlags::VerifyAll),
        );
    }

    fn get_name() -> &'static str {
        "Tutorial02: Cube"
    }
}

fn main() {
    native_app::main::<SampleApp<Cube>>().unwrap()
}
