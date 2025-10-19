use std::{ops::Div, path::Path};

use diligent::{graphics_utilities::linear_to_srgba, *};

use diligent_samples::sample_base::{sample::SampleBase, sample_app::SampleApp};
use diligent_tools::native_app;
use rand::distr::uniform::{UniformFloat, UniformSampler};

#[repr(C)]
struct ParticleAttribs {
    f2_pos: [f32; 2],
    f2_new_pos: [f32; 2],

    f2_speed: [f32; 2],
    f2_new_speed: [f32; 2],

    f_size: f32,
    f_temperature: f32,
    i_num_collisions: i32,
    f_padding0: f32,
}

fn create_render_particle_pso(
    factory: &EngineFactory,
    device: &RenderDevice,
    swap_chain_desc: &SwapChainDesc,
    convert_ps_output_to_gamma: bool,
) -> GraphicsPipelineState {
    let mut rtv_formats = std::array::from_fn(|_| None);
    rtv_formats[0] = Some(swap_chain_desc.color_buffer_format());

    let mut render_targets = std::array::from_fn(|_| RenderTargetBlendDesc::default());

    render_targets[0] = RenderTargetBlendDesc::builder()
        .blend_enable(true)
        .src_blend(BlendFactor::SrcAlpha)
        .dest_blend(BlendFactor::InvSrcAlpha)
        .build();

    let graphics_pso_desc = GraphicsPipelineDesc::builder()
        .blend_desc(
            BlendStateDesc::builder()
                .render_targets(render_targets)
                .build(),
        )
        .rasterizer_desc(
            RasterizerStateDesc::builder()
                // Disable back face culling
                .cull_mode(CullMode::None)
                .build(),
        )
        // Disable depth testing
        .depth_stencil_desc(DepthStencilStateDesc::builder().depth_enable(false).build())
        .output(
            GraphicsPipelineRenderTargets::builder()
                .num_render_targets(1)
                .rtv_formats(rtv_formats)
                .dsv_format(swap_chain_desc.depth_buffer_format())
                .build(),
        )
        .primitive_topology(PrimitiveTopology::TriangleStrip)
        .build();

    // Create a shader source stream factory to load shaders from files.
    let shader_source_factory = factory
        .create_default_shader_source_stream_factory(&[])
        .unwrap();

    let shader_ci = ShaderCreateInfo::builder()
        // Tell the system that the shader source code is in HLSL.
        // For OpenGL, the engine will convert this into GLSL under the hood.
        .source_language(ShaderLanguage::HLSL)
        .compiler(ShaderCompiler::DXC)
        .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
        .shader_source_input_stream_factory(&shader_source_factory)
        // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
        .use_combined_texture_samplers(true)
        // Presentation engine always expects input in gamma space. Normally, pixel shader output is
        // converted from linear to gamma space by the GPU. However, some platforms (e.g. Android in GLES mode,
        // or Emscripten in WebGL mode) do not support gamma-correction. In this case the application
        // has to do the conversion manually.
        .macros(vec![(
            "CONVERT_PS_OUTPUT_TO_GAMMA",
            if convert_ps_output_to_gamma { "1" } else { "0" },
        )]);

    let vertex_shader = device
        .create_shader(
            &shader_ci
                .clone()
                .shader_type(ShaderType::Vertex)
                .name("Particle VS")
                .source(ShaderSource::FilePath(Path::new("assets/particle.vsh")))
                .entry_point("main")
                .build(),
        )
        .unwrap();

    let pixel_shader = device
        .create_shader(
            &shader_ci
                .clone()
                .shader_type(ShaderType::Pixel)
                .name("Particle PS")
                .source(ShaderSource::FilePath(Path::new("assets/particle.psh")))
                .entry_point("main")
                .build(),
        )
        .unwrap();

    let pso_ci = PipelineStateCreateInfo::builder()
        .shader_resource_variables([ShaderResourceVariableDesc::builder()
            .name("g_Particles")
            // Shader variables should typically be mutable, which means they are expected
            // to change on a per-instance basis
            .variable_type(ShaderResourceVariableType::Mutable)
            .shader_stages(ShaderTypes::Vertex)
            .build()])
        // Define variable type that will be used by default
        .default_variable_type(ShaderResourceVariableType::Static)
        // Pipeline state name is used by the engine to report issues.
        .name("Render particles PSO")
        // This is a graphics pipeline
        .graphics()
        .graphics_pipeline_desc(graphics_pso_desc)
        .vertex_shader(&vertex_shader)
        .pixel_shader(&pixel_shader)
        .build();

    device.create_graphics_pipeline_state(&pso_ci).unwrap()
}

fn create_update_particle_pso(
    factory: &EngineFactory,
    device: &RenderDevice,
    thread_group_size: u32,
) -> (
    ComputePipelineState,
    ComputePipelineState,
    ComputePipelineState,
    ComputePipelineState,
) {
    // Create a shader source stream factory to load shaders from files.
    let shader_source_factory = factory
        .create_default_shader_source_stream_factory(&[])
        .unwrap();

    let mut macros = vec![("THREAD_GROUP_SIZE", format!("{thread_group_size}"))];

    let shader_ci = ShaderCreateInfo::builder()
        // Tell the system that the shader source code is in HLSL.
        // For OpenGL, the engine will convert this into GLSL under the hood.
        .source_language(ShaderLanguage::HLSL)
        // OpenGL backend requires emulated combined HLSL texture samplers (g_Texture + g_Texture_sampler combination)
        .use_combined_texture_samplers(true)
        .compiler(ShaderCompiler::DXC)
        .compile_flags(ShaderCompileFlags::PackMatrixRowMajor)
        .shader_source_input_stream_factory(&shader_source_factory)
        .shader_type(ShaderType::Compute)
        .entry_point("main");

    let reset_particle_lists_cs = device
        .create_shader(
            &shader_ci
                .clone()
                .name("Reset particle lists CS")
                .source(ShaderSource::FilePath(Path::new(
                    "assets/reset_particle_lists.csh",
                )))
                .macros(macros.clone())
                .build(),
        )
        .unwrap();

    let move_particles_cs = device
        .create_shader(
            &shader_ci
                .clone()
                .name("Move particles CS")
                .source(ShaderSource::FilePath(Path::new(
                    "assets/move_particles.csh",
                )))
                .macros(macros.clone())
                .build(),
        )
        .unwrap();

    let collide_particles_cs = device
        .create_shader(
            &shader_ci
                .clone()
                .name("Collide particles CS")
                .source(ShaderSource::FilePath(Path::new(
                    "assets/collide_particles.csh",
                )))
                .macros(macros.clone())
                .build(),
        )
        .unwrap();

    macros.push(("UPDATE_SPEED", String::from("1")));

    let updated_speed_cs = device
        .create_shader(
            &shader_ci
                .clone()
                .name("Update particle speed CS")
                .source(ShaderSource::FilePath(Path::new(
                    "assets/collide_particles.csh",
                )))
                .macros(macros)
                .build(),
        )
        .unwrap();

    let compute_pso_create_info = PipelineStateCreateInfo::builder()
        .default_variable_type(ShaderResourceVariableType::Mutable)
        .shader_resource_variables([ShaderResourceVariableDesc::builder()
            .name("Constants")
            .variable_type(ShaderResourceVariableType::Static)
            .shader_stages(ShaderTypes::Compute)
            .build()]);

    (
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name("Reset particle lists PSO")
                    .compute()
                    .shader(&reset_particle_lists_cs)
                    .build(),
            )
            .unwrap(),
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name("Move particles PSO")
                    .compute()
                    .shader(&move_particles_cs)
                    .build(),
            )
            .unwrap(),
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name("Collidse particles PSO")
                    .compute()
                    .shader(&collide_particles_cs)
                    .build(),
            )
            .unwrap(),
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name("Update particle speed PSO")
                    .compute()
                    .shader(&updated_speed_cs)
                    .build(),
            )
            .unwrap(),
    )
}

fn create_particle_buffers(num_particles: u32, device: &RenderDevice) -> (Buffer, Buffer, Buffer) {
    let buffer_desc = BufferDesc::builder()
        .name("Particle attribs buffer")
        .bind_flags(BindFlags::ShaderResource | BindFlags::UnorderedAccess)
        .mode(BufferMode::Structured)
        .usage(Usage::Default);

    const MAX_PARTICLE_SIZE: f32 = 0.05;

    let size = f32::min(MAX_PARTICLE_SIZE, 0.7 / f32::sqrt(num_particles as f32));

    let mut rng = rand::rng();

    let pos_distr = UniformFloat::<f32>::new(-1.0, 1.0).unwrap();
    let size_distr = UniformFloat::<f32>::new(0.5, 1.0).unwrap();

    let particle_data = Vec::from_iter(
        std::iter::repeat_with(|| ParticleAttribs {
            f2_new_pos: [pos_distr.sample(&mut rng), pos_distr.sample(&mut rng)],

            f2_new_speed: [
                pos_distr.sample(&mut rng) * size * 5.0,
                pos_distr.sample(&mut rng) * size * 5.0,
            ],

            f_size: size_distr.sample(&mut rng) * size,

            f2_pos: [0.0, 0.0],
            f2_speed: [0.0, 0.0],

            f_temperature: 0.0,
            i_num_collisions: 0,

            f_padding0: 0.0,
        })
        .take(num_particles as usize),
    );

    let particle_attribs_buffer = device
        .create_buffer_with_data(
            &buffer_desc
                .clone()
                .element_byte_stride(std::mem::size_of::<ParticleAttribs>() as u32)
                .size(std::mem::size_of::<ParticleAttribs>() as u64 * num_particles as u64)
                .build(),
            particle_data.as_slice(),
            None,
        )
        .unwrap();

    let buffer_desc = buffer_desc
        .element_byte_stride(std::mem::size_of::<i32>() as u32)
        .size(std::mem::size_of::<i32>() as u64 * num_particles as u64)
        .build();

    let particle_list_heads_buffer = device.create_buffer(&buffer_desc).unwrap();
    let particle_lists_buffer = device.create_buffer(&buffer_desc).unwrap();

    (
        particle_attribs_buffer,
        particle_list_heads_buffer,
        particle_lists_buffer,
    )
}

fn create_constant_buffer(device: &RenderDevice) -> Buffer {
    device
        .create_buffer(
            &BufferDesc::builder()
                .name("Constants buffer")
                .usage(Usage::Dynamic)
                .bind_flags(BindFlags::UniformBuffer)
                .cpu_access_flags(CpuAccessFlags::Write)
                .size(std::mem::size_of::<[f32; 4]>() as u64 * 2)
                .build(),
        )
        .unwrap()
}

struct ComputeShader {
    immediate_context: ImmediateDeviceContext,

    render_particle_pso: GraphicsPipelineState,
    render_particle_srb: ShaderResourceBinding,

    reset_particle_lists_pso: ComputePipelineState,
    reset_particle_lists_srb: ShaderResourceBinding,

    move_particles_pso: ComputePipelineState,
    move_particles_srb: ShaderResourceBinding,

    collide_particles_pso: ComputePipelineState,
    collide_particles_srb: ShaderResourceBinding,

    update_particle_speed_pso: ComputePipelineState,

    constants: Buffer,
    _particle_attribs_buffer: Buffer,
    _particle_list_heads_buffer: Buffer,
    _particle_lists_buffer: Buffer,

    num_particles: i32,

    simulation_speed: f32,

    clear_color: [f32; 4],

    time_delta: f32,

    thread_group_size: u32,
}

impl SampleBase for ComputeShader {
    fn get_immediate_context(&self) -> &ImmediateDeviceContext {
        &self.immediate_context
    }

    fn new(
        engine_factory: &EngineFactory,
        device: &RenderDevice,
        immediate_contexts: Vec<ImmediateDeviceContext>,
        _deferred_contexts: Vec<DeferredDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self {
        let swap_chain_desc = swap_chain.get_desc();

        let mut clear_color = [0.350, 0.350, 0.350, 1.0];

        let convert_ps_output_to_gamma = matches!(
            swap_chain_desc.color_buffer_format(),
            TextureFormat::RGBA8_UNORM | TextureFormat::BGRA8_UNORM
        );

        if convert_ps_output_to_gamma {
            clear_color = linear_to_srgba(clear_color);
        }

        let render_particle_pso = create_render_particle_pso(
            engine_factory,
            device,
            swap_chain_desc,
            convert_ps_output_to_gamma,
        );

        let thread_group_size = 256;

        let (
            reset_particle_lists_pso,
            move_particles_pso,
            collide_particles_pso,
            update_particle_speed_pso,
        ) = create_update_particle_pso(engine_factory, device, thread_group_size);

        let constants = create_constant_buffer(device);

        reset_particle_lists_pso
            .get_static_variable_by_name(ShaderType::Compute, "Constants")
            .unwrap()
            .set(&constants, SetShaderResourceFlags::None);
        move_particles_pso
            .get_static_variable_by_name(ShaderType::Compute, "Constants")
            .unwrap()
            .set(&constants, SetShaderResourceFlags::None);
        collide_particles_pso
            .get_static_variable_by_name(ShaderType::Compute, "Constants")
            .unwrap()
            .set(&constants, SetShaderResourceFlags::None);
        update_particle_speed_pso
            .get_static_variable_by_name(ShaderType::Compute, "Constants")
            .unwrap()
            .set(&constants, SetShaderResourceFlags::None);
        render_particle_pso
            .get_static_variable_by_name(ShaderType::Vertex, "Constants")
            .unwrap()
            .set(&constants, SetShaderResourceFlags::None);

        let num_particles = 2000;

        let (attribs_buffer, list_heads_buffer, lists_buffer) =
            create_particle_buffers(num_particles, device);

        let attribs_buffer_srv = attribs_buffer
            .get_default_view(BufferViewType::ShaderResource)
            .unwrap();
        let attribs_buffer_uav = attribs_buffer
            .get_default_view(BufferViewType::UnorderedAccess)
            .unwrap();

        let list_heads_buffer_uav = list_heads_buffer
            .get_default_view(BufferViewType::UnorderedAccess)
            .unwrap();
        let lists_buffer_uav = lists_buffer
            .get_default_view(BufferViewType::UnorderedAccess)
            .unwrap();
        let list_heads_buffer_srv = list_heads_buffer
            .get_default_view(BufferViewType::ShaderResource)
            .unwrap();
        let lists_buffer_srv = lists_buffer
            .get_default_view(BufferViewType::ShaderResource)
            .unwrap();

        let reset_particle_lists_srb = reset_particle_lists_pso
            .create_shader_resource_binding(true)
            .unwrap();

        reset_particle_lists_srb
            .get_variable_by_name("g_ParticleListHead", ShaderTypes::Compute)
            .unwrap()
            .set(&list_heads_buffer_uav, SetShaderResourceFlags::None);

        let render_particle_srb = render_particle_pso
            .create_shader_resource_binding(true)
            .unwrap();

        render_particle_srb
            .get_variable_by_name("g_Particles", ShaderTypes::Vertex)
            .unwrap()
            .set(&attribs_buffer_srv, SetShaderResourceFlags::None);

        let move_particles_srb = move_particles_pso
            .create_shader_resource_binding(true)
            .unwrap();
        move_particles_srb
            .get_variable_by_name("g_Particles", ShaderTypes::Compute)
            .unwrap()
            .set(&attribs_buffer_uav, SetShaderResourceFlags::None);
        move_particles_srb
            .get_variable_by_name("g_ParticleListHead", ShaderTypes::Compute)
            .unwrap()
            .set(&list_heads_buffer_uav, SetShaderResourceFlags::None);
        move_particles_srb
            .get_variable_by_name("g_ParticleLists", ShaderTypes::Compute)
            .unwrap()
            .set(&lists_buffer_uav, SetShaderResourceFlags::None);

        let collide_particles_srb = collide_particles_pso
            .create_shader_resource_binding(true)
            .unwrap();
        collide_particles_srb
            .get_variable_by_name("g_Particles", ShaderTypes::Compute)
            .unwrap()
            .set(&attribs_buffer_uav, SetShaderResourceFlags::None);
        collide_particles_srb
            .get_variable_by_name("g_ParticleListHead", ShaderTypes::Compute)
            .unwrap()
            .set(&list_heads_buffer_srv, SetShaderResourceFlags::None);
        collide_particles_srb
            .get_variable_by_name("g_ParticleLists", ShaderTypes::Compute)
            .unwrap()
            .set(&lists_buffer_srv, SetShaderResourceFlags::None);

        ComputeShader {
            immediate_context: immediate_contexts.into_iter().nth(0).unwrap(),

            render_particle_pso,
            reset_particle_lists_pso,
            move_particles_pso,
            collide_particles_pso,
            update_particle_speed_pso,

            render_particle_srb,
            reset_particle_lists_srb,
            move_particles_srb,
            collide_particles_srb,

            clear_color,
            num_particles: num_particles as i32,
            simulation_speed: 1.0,

            constants,
            _particle_attribs_buffer: attribs_buffer,
            _particle_list_heads_buffer: list_heads_buffer,
            _particle_lists_buffer: lists_buffer,

            time_delta: 0.0,
            thread_group_size,
        }
    }

    fn modify_engine_init_info(
        engine_ci: &mut diligent_samples::sample_base::sample::EngineCreateInfo,
    ) {
        engine_ci
            .features
            .set_compute_shaders(DeviceFeatureState::Enabled);
    }

    fn render(&self, swap_chain: &SwapChain) {
        let immediate_context = self.get_immediate_context();

        let mut rtv = swap_chain.get_current_back_buffer_rtv();
        let mut dsv = swap_chain.get_depth_buffer_dsv();

        // Clear the back buffer
        // Let the engine perform required state transitions
        immediate_context.clear_render_target(
            &mut rtv,
            &self.clear_color,
            ResourceStateTransitionMode::Transition,
        );

        immediate_context.clear_depth(&mut dsv, 1.0, ResourceStateTransitionMode::Transition);

        let swap_chain_desc = swap_chain.get_desc();

        {
            #[repr(C)]
            struct Constants {
                ui_num_particles: u32,
                f_delta_time: f32,
                f_dummy0: f32,
                f_dummy1: f32,
                f2_scale: [f32; 2],
                i2_particle_grid_size: [i32; 2],
            }

            // Map the buffer and write current world-view-projection matrix
            let mut map =
                immediate_context.map_buffer_write::<Constants>(&self.constants, MapFlags::Discard);

            let constants = unsafe { map.as_mut() };
            constants.ui_num_particles = self.num_particles as u32;
            constants.f_delta_time = f32::min(self.time_delta, 1.0 / 60.0) * self.simulation_speed;

            let aspect_ratio = swap_chain_desc.width() as f32 / swap_chain_desc.height() as f32;
            let f2_scale = [f32::sqrt(1.0 / aspect_ratio), f32::sqrt(aspect_ratio)];
            constants.f2_scale = f2_scale;

            let i_particle_grid_width = (f32::sqrt(self.num_particles as f32) / f2_scale[0]) as i32;
            constants.i2_particle_grid_size[0] = i_particle_grid_width;
            constants.i2_particle_grid_size[1] = self.num_particles / i_particle_grid_width;
        }

        let dispatch_attribs = DispatchComputeAttribs::builder()
            .thread_group_count_x(
                (self.num_particles as u32 + self.thread_group_size - 1)
                    .div(self.thread_group_size),
            )
            .build();

        {
            let reset_particle_lists =
                immediate_context.set_compute_pipeline_state(&self.reset_particle_lists_pso);
            immediate_context.commit_shader_resources(
                &self.reset_particle_lists_srb,
                ResourceStateTransitionMode::Transition,
            );
            reset_particle_lists.dispatch_compute(&dispatch_attribs);
        }

        {
            let move_particle_lists =
                immediate_context.set_compute_pipeline_state(&self.move_particles_pso);
            immediate_context.commit_shader_resources(
                &self.move_particles_srb,
                ResourceStateTransitionMode::Transition,
            );
            move_particle_lists.dispatch_compute(&dispatch_attribs);
        }

        {
            let collide_particles =
                immediate_context.set_compute_pipeline_state(&self.collide_particles_pso);
            immediate_context.commit_shader_resources(
                &self.collide_particles_srb,
                ResourceStateTransitionMode::Transition,
            );
            collide_particles.dispatch_compute(&dispatch_attribs);
        }

        {
            let update_particles =
                immediate_context.set_compute_pipeline_state(&self.update_particle_speed_pso);
            // Use the same SRB
            immediate_context.commit_shader_resources(
                &self.collide_particles_srb,
                ResourceStateTransitionMode::Transition,
            );
            update_particles.dispatch_compute(&dispatch_attribs);
        }

        let graphics = immediate_context.set_graphics_pipeline_state(&self.render_particle_pso);

        immediate_context.commit_shader_resources(
            &self.render_particle_srb,
            ResourceStateTransitionMode::Transition,
        );

        graphics.draw(
            &DrawAttribs::builder()
                .num_vertices(4)
                .num_instances(self.num_particles as u32)
                .build(),
        );
    }

    fn update_ui(&mut self, ui: &mut imgui::Ui) {
        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .begin()
        {
            if ui
                .input_int("Num Particles", &mut self.num_particles)
                .build()
            {
                self.num_particles = self.num_particles.clamp(100, 100000);
            }

            ui.slider("Simulation Speed", 0.1, 5.0, &mut self.simulation_speed);
        }
    }

    fn update(&mut self, _current_time: f64, elapsed_time: f64) {
        self.time_delta = elapsed_time as f32;
    }

    fn get_name() -> &'static str {
        "Tutorial14: Compute Shader"
    }
}

fn main() {
    native_app::main::<SampleApp<ComputeShader>>().unwrap()
}
