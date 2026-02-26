use std::{cell::RefCell, ops::Div, path::Path};

use diligent::{graphics_utilities::linear_to_srgba, *};

use diligent_samples::sample_base::{
    sample::SampleBase,
    sample_app::{self},
};

use imgui::InputTextFlags;
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
) -> Boxed<GraphicsPipelineState> {
    let mut rtv_formats = std::array::from_fn(|_| None);
    rtv_formats[0] = swap_chain_desc.color_buffer_format();

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
                .maybe_dsv_format(swap_chain_desc.depth_buffer_format())
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

    let shader_resource_variables = [ShaderResourceVariableDesc::builder()
        .name(c"g_Particles")
        // Shader variables should typically be mutable, which means they are expected
        // to change on a per-instance basis
        .variable_type(ShaderResourceVariableType::Mutable)
        .shader_stages(ShaderTypes::Vertex)
        .build()];

    let pso_ci = PipelineStateCreateInfo::builder()
        .shader_resource_variables(&shader_resource_variables)
        // Define variable type that will be used by default
        .default_variable_type(ShaderResourceVariableType::Static)
        // Pipeline state name is used by the engine to report issues.
        .name(c"Render particles PSO")
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
    Boxed<ComputePipelineState>,
    Boxed<ComputePipelineState>,
    Boxed<ComputePipelineState>,
    Boxed<ComputePipelineState>,
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

    let binding = [ShaderResourceVariableDesc::builder()
        .name(c"Constants")
        .variable_type(ShaderResourceVariableType::Static)
        .shader_stages(ShaderTypes::Compute)
        .build()];

    let compute_pso_create_info = PipelineStateCreateInfo::builder()
        .default_variable_type(ShaderResourceVariableType::Mutable)
        .shader_resource_variables(&binding);

    (
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name(c"Reset particle lists PSO")
                    .compute()
                    .shader(&reset_particle_lists_cs)
                    .build(),
            )
            .unwrap(),
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name(c"Move particles PSO")
                    .compute()
                    .shader(&move_particles_cs)
                    .build(),
            )
            .unwrap(),
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name(c"Collidse particles PSO")
                    .compute()
                    .shader(&collide_particles_cs)
                    .build(),
            )
            .unwrap(),
        device
            .create_compute_pipeline_state(
                &compute_pso_create_info
                    .clone()
                    .name(c"Update particle speed PSO")
                    .compute()
                    .shader(&updated_speed_cs)
                    .build(),
            )
            .unwrap(),
    )
}

fn create_particle_buffers(
    num_particles: u32,
    device: &RenderDevice,
) -> (Boxed<Buffer>, Boxed<Buffer>, Boxed<Buffer>) {
    let buffer_desc = BufferDesc::builder()
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
                .name(c"Particle attribs buffer")
                .element_byte_stride(std::mem::size_of::<ParticleAttribs>() as u32)
                .size(std::mem::size_of::<ParticleAttribs>() as u64 * num_particles as u64)
                .build(),
            particle_data.as_slice(),
            None,
        )
        .unwrap();

    let buffer_desc = buffer_desc
        .element_byte_stride(std::mem::size_of::<i32>() as u32)
        .size(std::mem::size_of::<i32>() as u64 * num_particles as u64);

    let particle_list_heads_buffer = device
        .create_buffer(&buffer_desc.clone().name(c"Particle list heads").build())
        .unwrap();
    let particle_lists_buffer = device
        .create_buffer(&buffer_desc.name(c"Particle lists").build())
        .unwrap();

    (
        particle_attribs_buffer,
        particle_list_heads_buffer,
        particle_lists_buffer,
    )
}

fn create_constant_buffer(device: &RenderDevice) -> Boxed<Buffer> {
    device
        .create_buffer(
            &BufferDesc::builder()
                .name(c"Constants buffer")
                .usage(Usage::Dynamic)
                .bind_flags(BindFlags::UniformBuffer)
                .cpu_access_flags(CpuAccessFlags::Write)
                .size(std::mem::size_of::<[f32; 4]>() as u64 * 2)
                .build(),
        )
        .unwrap()
}

fn bind_buffers(
    attribs_buffer: &Buffer,
    list_heads_buffer: &Buffer,
    lists_buffer: &Buffer,

    render_particle_srb: &ShaderResourceBinding,
    reset_particle_lists_srb: &ShaderResourceBinding,
    move_particles_srb: &ShaderResourceBinding,
    collide_particles_srb: &ShaderResourceBinding,
) {
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

    reset_particle_lists_srb
        .get_variable_by_name("g_ParticleListHead", ShaderTypes::Compute)
        .unwrap()
        .set(list_heads_buffer_uav, SetShaderResourceFlags::None);
    render_particle_srb
        .get_variable_by_name("g_Particles", ShaderTypes::Vertex)
        .unwrap()
        .set(attribs_buffer_srv, SetShaderResourceFlags::None);

    move_particles_srb
        .get_variable_by_name("g_Particles", ShaderTypes::Compute)
        .unwrap()
        .set(attribs_buffer_uav, SetShaderResourceFlags::None);
    move_particles_srb
        .get_variable_by_name("g_ParticleListHead", ShaderTypes::Compute)
        .unwrap()
        .set(list_heads_buffer_uav, SetShaderResourceFlags::None);
    move_particles_srb
        .get_variable_by_name("g_ParticleLists", ShaderTypes::Compute)
        .unwrap()
        .set(lists_buffer_uav, SetShaderResourceFlags::None);

    collide_particles_srb
        .get_variable_by_name("g_Particles", ShaderTypes::Compute)
        .unwrap()
        .set(attribs_buffer_uav, SetShaderResourceFlags::None);
    collide_particles_srb
        .get_variable_by_name("g_ParticleListHead", ShaderTypes::Compute)
        .unwrap()
        .set(list_heads_buffer_srv, SetShaderResourceFlags::None);
    collide_particles_srb
        .get_variable_by_name("g_ParticleLists", ShaderTypes::Compute)
        .unwrap()
        .set(lists_buffer_srv, SetShaderResourceFlags::None);
}

struct ComputeShader {
    render_particle_pso: Boxed<GraphicsPipelineState>,
    render_particle_srb: RefCell<Boxed<ShaderResourceBinding>>,

    reset_particle_lists_pso: Boxed<ComputePipelineState>,
    reset_particle_lists_srb: RefCell<Boxed<ShaderResourceBinding>>,

    move_particles_pso: Boxed<ComputePipelineState>,
    move_particles_srb: RefCell<Boxed<ShaderResourceBinding>>,

    collide_particles_pso: Boxed<ComputePipelineState>,
    collide_particles_srb: RefCell<Boxed<ShaderResourceBinding>>,

    update_particle_speed_pso: Boxed<ComputePipelineState>,

    constants: Boxed<Buffer>,
    particle_attribs_buffer: Boxed<Buffer>,
    particle_list_heads_buffer: Boxed<Buffer>,
    particle_lists_buffer: Boxed<Buffer>,

    num_particles: i32,

    simulation_speed: f32,

    clear_color: [f32; 4],

    time_delta: f32,

    thread_group_size: u32,
}

impl SampleBase for ComputeShader {
    fn new(
        engine_factory: &EngineFactory,
        device: &RenderDevice,
        _main_context: &ImmediateDeviceContext,
        _immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        _deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        swap_chain_descs: &[&SwapChainDesc],
    ) -> Self {
        // We are only using one swap chain
        let swap_chain_desc = swap_chain_descs[0];

        let mut clear_color = [0.350, 0.350, 0.350, 1.0];

        let convert_ps_output_to_gamma = matches!(
            swap_chain_desc.color_buffer_format(),
            Some(TextureFormat::RGBA8_UNORM) | Some(TextureFormat::BGRA8_UNORM)
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

        let reset_particle_lists_srb = reset_particle_lists_pso
            .create_shader_resource_binding(true)
            .unwrap();

        let render_particle_srb = render_particle_pso
            .create_shader_resource_binding(true)
            .unwrap();

        let move_particles_srb = move_particles_pso
            .create_shader_resource_binding(true)
            .unwrap();

        let collide_particles_srb = collide_particles_pso
            .create_shader_resource_binding(true)
            .unwrap();

        bind_buffers(
            &attribs_buffer,
            &list_heads_buffer,
            &lists_buffer,
            &render_particle_srb,
            &reset_particle_lists_srb,
            &move_particles_srb,
            &collide_particles_srb,
        );

        ComputeShader {
            render_particle_pso,
            reset_particle_lists_pso,
            move_particles_pso,
            collide_particles_pso,
            update_particle_speed_pso,

            render_particle_srb: RefCell::new(render_particle_srb),
            reset_particle_lists_srb: RefCell::new(reset_particle_lists_srb),
            move_particles_srb: RefCell::new(move_particles_srb),
            collide_particles_srb: RefCell::new(collide_particles_srb),

            clear_color,
            num_particles: num_particles as i32,
            simulation_speed: 1.0,

            constants,
            particle_attribs_buffer: attribs_buffer,
            particle_list_heads_buffer: list_heads_buffer,
            particle_lists_buffer: lists_buffer,

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

    fn render(
        &self,
        main_context: Boxed<ImmediateDeviceContext>,
        swap_chain: &mut SwapChain,
    ) -> Boxed<ImmediateDeviceContext> {
        // Clear the back buffer
        // Let the engine perform required state transitions
        {
            let rtv = swap_chain.get_current_back_buffer_rtv_mut().unwrap();
            main_context.clear_render_target(rtv.transition_state(), &self.clear_color);
        }

        {
            let dsv = swap_chain.get_depth_buffer_dsv_mut().unwrap();
            main_context.clear_depth(dsv.transition_state(), 1.0);
        }

        let swap_chain_desc = swap_chain.desc();

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
            let mut constants =
                main_context.map_buffer_write::<Constants>(&self.constants, MapFlags::Discard);

            let constants = &mut constants[0];

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

        let main_context = {
            let reset_particle_lists =
                main_context.set_compute_pipeline_state(&self.reset_particle_lists_pso);
            reset_particle_lists.commit_shader_resources(
                self.reset_particle_lists_srb
                    .borrow_mut()
                    .transition_state(),
            );
            reset_particle_lists.dispatch_compute(&dispatch_attribs);

            reset_particle_lists.finish()
        };

        let main_context = {
            let move_particle_lists =
                main_context.set_compute_pipeline_state(&self.move_particles_pso);
            move_particle_lists
                .commit_shader_resources(self.move_particles_srb.borrow_mut().transition_state());
            move_particle_lists.dispatch_compute(&dispatch_attribs);
            move_particle_lists.finish()
        };

        let main_context = {
            let collide_particles =
                main_context.set_compute_pipeline_state(&self.collide_particles_pso);
            collide_particles.commit_shader_resources(
                self.collide_particles_srb.borrow_mut().transition_state(),
            );
            collide_particles.dispatch_compute(&dispatch_attribs);
            collide_particles.finish()
        };

        let main_context = {
            let update_particles =
                main_context.set_compute_pipeline_state(&self.update_particle_speed_pso);
            // Use the same SRB
            update_particles.commit_shader_resources(
                self.collide_particles_srb.borrow_mut().transition_state(),
            );
            update_particles.dispatch_compute(&dispatch_attribs);
            update_particles.finish()
        };

        let graphics = main_context.set_graphics_pipeline_state(&self.render_particle_pso);

        graphics.commit_shader_resources(self.render_particle_srb.borrow_mut().transition_state());

        graphics.draw(
            &DrawAttribs::builder()
                .num_vertices(4)
                .num_instances(self.num_particles as u32)
                .build(),
        );

        graphics.finish()
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
            if ui
                .input_int("Num Particles", &mut self.num_particles)
                .step(100)
                .step_fast(1000)
                .flags(InputTextFlags::ENTER_RETURNS_TRUE)
                .build()
            {
                self.num_particles = self.num_particles.clamp(100, 100000);

                (
                    self.particle_attribs_buffer,
                    self.particle_list_heads_buffer,
                    self.particle_lists_buffer,
                ) = create_particle_buffers(self.num_particles as u32, device);

                self.reset_particle_lists_srb = RefCell::new(
                    self.reset_particle_lists_pso
                        .create_shader_resource_binding(true)
                        .unwrap(),
                );

                self.render_particle_srb = RefCell::new(
                    self.render_particle_pso
                        .create_shader_resource_binding(true)
                        .unwrap(),
                );

                self.move_particles_srb = RefCell::new(
                    self.move_particles_pso
                        .create_shader_resource_binding(true)
                        .unwrap(),
                );

                self.collide_particles_srb = RefCell::new(
                    self.collide_particles_pso
                        .create_shader_resource_binding(true)
                        .unwrap(),
                );

                bind_buffers(
                    &self.particle_attribs_buffer,
                    &self.particle_list_heads_buffer,
                    &self.particle_lists_buffer,
                    &self.render_particle_srb.borrow(),
                    &self.reset_particle_lists_srb.borrow(),
                    &self.move_particles_srb.borrow(),
                    &self.collide_particles_srb.borrow(),
                );
            }

            ui.slider("Simulation Speed", 0.1, 5.0, &mut self.simulation_speed);
        }
    }

    fn update(
        &mut self,
        _main_context: &ImmediateDeviceContext,
        _current_time: f64,
        elapsed_time: f64,
    ) {
        self.time_delta = elapsed_time as f32;
    }

    fn get_name() -> &'static str {
        "Tutorial14: Compute Shader"
    }
}

fn main() {
    sample_app::main::<ComputeShader>().unwrap()
}
