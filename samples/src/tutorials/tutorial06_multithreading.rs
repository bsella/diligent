use std::{
    collections::VecDeque,
    ops::Deref,
    sync::{
        Arc, Barrier, RwLock,
        mpsc::{Receiver, Sender},
    },
};

#[cfg(feature = "vulkan")]
use diligent::vk::engine_factory_vk::{
    DeviceFeaturesVk, EngineFactoryVk, EngineVkCreateInfo, get_engine_factory_vk,
};

#[cfg(feature = "opengl")]
use diligent::gl::engine_factory_gl::{
    EngineFactoryOpenGL, EngineGLCreateInfo, get_engine_factory_gl,
};

#[cfg(feature = "d3d11")]
use diligent::d3d11::engine_factory_d3d11::{
    D3D11ValidationFlags, EngineD3D11CreateInfo, EngineFactoryD3D11, get_engine_factory_d3d11,
};

#[cfg(feature = "d3d12")]
use diligent::d3d12::engine_factory_d3d12::{
    EngineD3D12CreateInfo, EngineFactoryD3D12, get_engine_factory_d3d12,
};

use diligent::{
    geometry_primitives::GeometryPrimitiveVertexFlags,
    graphics_utilities::{create_uniform_buffer, linear_to_srgba},
    *,
};
use diligent_samples::{
    sample_base::{
        sample::{get_adjusted_projection_matrix, get_surface_pretransform_matrix},
        sample_app_settings::{SampleAppSettings, parse_sample_app_settings},
    },
    textured_cube::{CreatePSOInfo, TexturedCube},
};

enum EngineFactory {
    #[cfg(feature = "vulkan")]
    Vulkan(Boxed<EngineFactoryVk>),
    #[cfg(feature = "opengl")]
    OpenGL(Boxed<EngineFactoryOpenGL>),
    #[cfg(feature = "d3d11")]
    D3D11(Boxed<EngineFactoryD3D11>),
    #[cfg(feature = "d3d12")]
    D3D12(Boxed<EngineFactoryD3D12>),
}

impl Deref for EngineFactory {
    type Target = diligent::EngineFactory;
    fn deref(&self) -> &Self::Target {
        match self {
            #[cfg(feature = "vulkan")]
            Self::Vulkan(factory) => factory,
            #[cfg(feature = "opengl")]
            Self::OpenGL(factory) => factory,
            #[cfg(feature = "d3d11")]
            Self::D3D11(factory) => factory,
            #[cfg(feature = "d3d12")]
            Self::D3D12(factory) => factory,
        }
    }
}

use diligent_tools::{
    imgui::{events::imgui_handle_event, renderer::ImguiRenderer},
    native_app::{Window, WindowManager, events::Event},
};
use image::DynamicImage;
use rand::distr::uniform::{UniformFloat, UniformInt, UniformSampler};

struct SampleWindow {
    window: std::boxed::Box<dyn Window>,

    imgui_renderer: ImguiRenderer,
}

const NUM_TEXTURES: usize = 4;

#[allow(non_camel_case_types)]
type float4x4 = [f32; 4 * 4];

#[repr(C)]
struct InstanceData {
    matrix: float4x4,
    texture_ind: i32,
}

fn populate_instance_buffer(grid_size: usize) -> Vec<InstanceData> {
    let mut rng = rand::rng();

    let scale_distr = UniformFloat::<f32>::new(0.3, 1.0).unwrap();
    let offset_distr = UniformFloat::<f32>::new(-0.15f32, 0.15f32).unwrap();
    let rot_distr = UniformFloat::<f32>::new(-std::f32::consts::PI, std::f32::consts::PI).unwrap();
    let tex_distr = UniformInt::<i32>::new(0, (NUM_TEXTURES - 1) as i32).unwrap();

    let mut instances = Vec::with_capacity(grid_size * grid_size * grid_size);

    let base_scale = 0.6 / grid_size as f32;
    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                // Add random offset from central position in the grid
                let x_offset =
                    2.0 * (x as f32 + 0.5 + offset_distr.sample(&mut rng)) / grid_size as f32 - 1.0;
                let y_offset =
                    2.0 * (y as f32 + 0.5 + offset_distr.sample(&mut rng)) / grid_size as f32 - 1.0;
                let z_offset =
                    2.0 * (z as f32 + 0.5 + offset_distr.sample(&mut rng)) / grid_size as f32 - 1.0;

                // Random scale
                let scale = base_scale * scale_distr.sample(&mut rng);

                // Random rotation
                let rotation = glam::Mat4::from_rotation_z(rot_distr.sample(&mut rng))
                    * glam::Mat4::from_rotation_y(rot_distr.sample(&mut rng))
                    * glam::Mat4::from_rotation_x(rot_distr.sample(&mut rng));

                // Combine rotation, scale and translation
                let matrix = glam::Mat4::from_translation(glam::Vec3 {
                    x: x_offset,
                    y: y_offset,
                    z: z_offset,
                }) * glam::Mat4::from_scale(glam::Vec3 {
                    x: scale,
                    y: scale,
                    z: scale,
                }) * rotation;
                instances.push(InstanceData {
                    matrix: matrix.to_cols_array(),
                    texture_ind: tex_distr.sample(&mut rng),
                });
            }
        }
    }

    instances
}

struct SharedThreadData {
    pso: Boxed<GraphicsPipelineState>,

    textured_cube: TexturedCube,

    instance_constants: Boxed<Buffer>,
    vs_constants: Boxed<Buffer>,

    srb: [Boxed<ShaderResourceBinding>; NUM_TEXTURES],

    instances: Vec<InstanceData>,
}

impl SharedThreadData {
    fn render_subset<Context: GraphicsContext>(
        &self,
        context: Context,
        subset: usize,
        num_subsets: usize,
        swap_chain: &SwapChain,
        view_proj_matrix: &glam::Mat4,
        rotation_matrix: &glam::Mat4,
    ) -> Context {
        // Deferred contexts start in default state. We must bind everything to the context.
        // Render targets are set and transitioned to correct states by the main thread, here we only verify the states.
        {
            let rtv = swap_chain
                .get_current_back_buffer_rtv()
                .unwrap()
                .verify_state();

            let dsv = swap_chain.get_depth_buffer_dsv();

            context
                .borrow()
                .set_render_targets(&[rtv], dsv.map(TextureView::verify_state));
        }

        {
            // Map the buffer and write current world-view-projection matrix

            // Since this is a dynamic buffer, it must be mapped in every context before
            // it can be used even though the matrices are the same.

            let mut cb_constants = context
                .borrow()
                .map_buffer_write(&self.vs_constants, MapFlags::Discard);

            cb_constants[0] = view_proj_matrix;
            cb_constants[1] = rotation_matrix;
        }

        // Bind vertex and index buffers. This must be done for every context
        context.borrow().set_vertex_buffers(
            [(self.textured_cube.vertex_buffer().verify_state(), 0)],
            SetVertexBufferFlags::Reset,
        );
        context
            .borrow()
            .set_index_buffer(self.textured_cube.index_buffer().verify_state(), 0);

        // This is an indexed draw call
        let draw_attrs = DrawIndexedAttribs::builder()
            .index_type(ValueType::Uint32)
            .num_indices(36)
            .flags(DrawFlags::VerifyAll)
            .build();

        let num_instances = self.instances.len();
        let susbset_size = num_instances / num_subsets;
        let start_inst = susbset_size * subset;
        let end_inst = if subset < num_subsets - 1 {
            susbset_size * (subset + 1)
        } else {
            num_instances
        };

        let graphics = context.set_graphics_pipeline_state(&self.pso);

        for instance in &self.instances[start_inst..end_inst] {
            // Shader resources have been explicitly transitioned to correct states, so
            // RESOURCE_STATE_TRANSITION_MODE_TRANSITION mode is not needed.
            // Instead, we use RESOURCE_STATE_TRANSITION_MODE_VERIFY mode to
            // verify that all resources are in correct states. This mode only has effect
            // in debug and development builds.
            graphics
                .borrow()
                .commit_shader_resources(self.srb[instance.texture_ind as usize].verify_state());

            {
                // Map the buffer and write current world-view-projection matrix

                let mut inst_data = graphics
                    .borrow()
                    .map_buffer_write(&self.instance_constants, MapFlags::Discard);

                inst_data[0] = instance.matrix;
            }

            graphics.draw_indexed(&draw_attrs);
        }

        graphics.finish()
    }
}

struct ExclusiveThreadData {
    context: Boxed<DeferredDeviceContext>,
    message_receiver: Receiver<ThreadMessage>,
    command_list_sender: Sender<Boxed<CommandList>>,
}

unsafe impl Send for ExclusiveThreadData {}

enum ThreadMessage {
    Draw {
        swap_chain: Arc<RwLock<Boxed<SwapChain>>>,
        view_proj_matrix: glam::Mat4,
    },
    Stop,
}

struct RunningThread {
    handle: std::thread::JoinHandle<ExclusiveThreadData>,

    message_sender: Sender<ThreadMessage>,
}

impl RunningThread {
    fn pause(self) -> PausedThread {
        self.message_sender.send(ThreadMessage::Stop).unwrap();

        PausedThread {
            data: self.handle.join().unwrap(),
            message_sender: self.message_sender,
        }
    }
}

struct PausedThread {
    data: ExclusiveThreadData,

    message_sender: Sender<ThreadMessage>,
}

impl PausedThread {
    fn run(
        self,
        shared_data: Arc<SharedThreadData>,
        index: usize,
        num_threads: usize,
        rotation_matrix: glam::Mat4,
        execute_command_list_barrier: Arc<Barrier>,
    ) -> RunningThread {
        RunningThread {
            message_sender: self.message_sender,
            handle: std::thread::spawn(move || {
                worker_thread_func(
                    shared_data,
                    self.data,
                    index,
                    num_threads,
                    &rotation_matrix,
                    &execute_command_list_barrier,
                )
            }),
        }
    }
}

fn worker_thread_func(
    shared_data: Arc<SharedThreadData>,
    mut thread_data: ExclusiveThreadData,
    thread_index: usize,
    num_threads: usize,
    rotation_matrix: &glam::Mat4,
    execute_command_list_barrier: &Barrier,
) -> ExclusiveThreadData {
    let command_list_sender = &thread_data.command_list_sender;
    while let Ok(message) = thread_data.message_receiver.recv() {
        match message {
            ThreadMessage::Draw {
                swap_chain,
                view_proj_matrix,
            } => {
                thread_data.context.begin(0);

                thread_data.context = shared_data.render_subset(
                    thread_data.context,
                    1 + thread_index,
                    num_threads + 1,
                    &swap_chain.read().unwrap(),
                    &view_proj_matrix,
                    rotation_matrix,
                );

                let _ =
                    command_list_sender.send(thread_data.context.finish_command_list().unwrap());

                // Call FinishFrame() to release dynamic resources allocated by deferred contexts
                // IMPORTANT: we must wait until the command lists are submitted for execution
                //            because FinishFrame() invalidates all dynamic resources.
                // IMPORTANT: In Metal backend FinishFrame must be called from the same
                //            thread that issued rendering commands.
                execute_command_list_barrier.wait();

                unsafe {
                    thread_data.context.finish_frame();
                }
            }
            ThreadMessage::Stop => break,
        }
    }

    thread_data
}

struct Multithreading {
    running_threads: VecDeque<RunningThread>,
    paused_threads: VecDeque<PausedThread>,

    grid_size: usize,

    swap_chain: Arc<RwLock<Boxed<SwapChain>>>,

    shared_thead_data: Arc<SharedThreadData>,

    _textures_srv: [Boxed<TextureView>; NUM_TEXTURES],

    max_threads: usize,
    num_worker_threads: usize,

    rotation_matrix: glam::Mat4,

    convert_ps_output_to_gamma: bool,

    command_list_receiver: Receiver<Boxed<CommandList>>,

    execute_command_list_barrier: Arc<Barrier>,
}

impl Multithreading {
    fn stop_all_threads(&mut self) {
        for thread in self.running_threads.drain(..) {
            self.paused_threads.push_back(thread.pause());
        }
    }

    fn set_num_threads(&mut self, num_threads: usize) {
        self.stop_all_threads();

        let rotation_matrix = self.rotation_matrix;

        self.execute_command_list_barrier = Arc::new(Barrier::new(num_threads + 1));

        for (index, paused_thread) in self.paused_threads.drain(0..num_threads).enumerate() {
            self.running_threads.push_back(paused_thread.run(
                self.shared_thead_data.clone(),
                index,
                num_threads,
                rotation_matrix,
                self.execute_command_list_barrier.clone(),
            ))
        }
    }
}

impl Drop for Multithreading {
    fn drop(&mut self) {
        self.stop_all_threads();
    }
}

impl Multithreading {
    fn get_name() -> &'static str {
        "Tutorial06: Multithreading"
    }

    fn modify_engine_init_info(
        engine_ci: &mut diligent_samples::sample_base::sample::EngineCreateInfo,
    ) {
        engine_ci.num_deferred_contexts = std::thread::available_parallelism().unwrap().into();
        match engine_ci {
            #[cfg(feature = "vulkan")]
            diligent_samples::sample_base::sample::EngineCreateInfo::EngineVkCreateInfo(ci) => {
                // Enough space for 32x32x32x256 bytes allocations for 3 frames
                ci.dynamic_heap_size = 26 << 20;
            }
        }
    }

    fn release_swap_chain_buffers(&mut self) {}

    fn update_ui(
        &mut self,
        _device: &RenderDevice,
        _main_context: &ImmediateDeviceContext,
        ui: &mut imgui::Ui,
    ) {
        if let Some(_window_token) = ui
            .window("Settings")
            .always_auto_resize(true)
            .position([10.0, 10.0], imgui::Condition::Always)
            .begin()
        {
            if ui.slider("Grid Size", 1, 32, &mut self.grid_size) {
                //self.rendering_context.instances = populate_instance_buffer(self.grid_size);
            }
            if ui.slider(
                "Worker Threads",
                0,
                self.max_threads,
                &mut self.num_worker_threads,
            ) {
                self.set_num_threads(self.num_worker_threads);
            }
        }
    }

    fn new(
        engine_factory: &diligent::EngineFactory,
        device: &RenderDevice,
        main_context: &ImmediateDeviceContext,
        _immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        deferred_contexts: Vec<Boxed<diligent::DeferredDeviceContext>>,
        swap_chain: Boxed<SwapChain>,
    ) -> Self {
        // We are only using one swap chain
        let swap_chain_desc = swap_chain.desc();

        let max_threads = deferred_contexts.len();
        let num_worker_threads = max_threads.min(4);

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

        let pso = TexturedCube::create_pipeline_state(
            CreatePSOInfo::new(
                device,
                swap_chain_desc.color_buffer_format(),
                swap_chain_desc.depth_buffer_format(),
                &shader_source_factory,
                "assets/cube_multithreading.vsh",
                "assets/cube_multithreading.psh",
                GeometryPrimitiveVertexFlags::PosTex,
                [],
                1,
            ),
            convert_ps_output_to_gamma,
        )
        .unwrap();

        // Create dynamic uniform buffer that will store our transformation matrix
        // Dynamic buffers can be frequently updated by the CPU
        let mut vs_constants = create_uniform_buffer(
            device,
            (std::mem::size_of::<float4x4>() * 2) as u64,
            c"VS constants CB",
            diligent::Usage::Dynamic,
            BindFlags::UniformBuffer,
            CpuAccessFlags::Write,
        )
        .unwrap();

        let mut instance_constants = create_uniform_buffer(
            device,
            std::mem::size_of::<float4x4>() as u64,
            c"Instance constants CB",
            diligent::Usage::Dynamic,
            BindFlags::UniformBuffer,
            CpuAccessFlags::Write,
        )
        .unwrap();

        // Since we did not explicitly specify the type for 'Constants' and 'InstanceData' variables,
        // default type (ShaderResourceVariableType::Static) will be used. Static variables
        // never change and are bound directly to the pipeline state object.
        pso.get_static_variable_by_name(ShaderType::Vertex, "Constants")
            .unwrap()
            .set(&vs_constants, SetShaderResourceFlags::None);
        pso.get_static_variable_by_name(ShaderType::Vertex, "InstanceData")
            .unwrap()
            .set(&instance_constants, SetShaderResourceFlags::None);

        let mut textured_cube = TexturedCube::new(
            device,
            GeometryPrimitiveVertexFlags::PosTex,
            BindFlags::VertexBuffer,
            None,
            BindFlags::IndexBuffer,
            None,
        )
        .unwrap();

        let mut textures = {
            let images: [DynamicImage; NUM_TEXTURES] = std::array::from_fn(|i| i).map(|tex_id| {
                image::ImageReader::open(format!("assets/DGLogo{tex_id}.png"))
                    .unwrap()
                    .decode()
                    .unwrap()
            });

            let texture_desc = TextureDesc::builder()
                .name(c"DGLogo")
                .dimension(TextureDimension::Texture2D)
                .width(images[0].width())
                .height(images[0].height())
                .format(TextureFormat::RGBA8_UNORM_SRGB)
                .bind_flags(BindFlags::ShaderResource)
                .usage(Usage::Default)
                .build();

            let texture_data = images.each_ref().map(|image| {
                TextureSubResource::builder()
                    .from_host(
                        image.as_bytes(),
                        image.width() as u64 * std::mem::size_of::<[u8; 4]>() as u64,
                    )
                    .build()
            });

            texture_data.map(|texture_data| {
                device
                    .create_texture(&texture_desc, &[texture_data], None)
                    .unwrap()
            })
        };

        let (vertex_buffer, index_buffer) = textured_cube.vertex_and_index_buffer_mut();

        {
            let [texture0, texture1, texture2, texture3] = &mut textures;
            // Explicitly transition the buffers to StateTransitionFlags::UpdateState state
            let barriers = [
                StateTransitionDesc::builder()
                    .resource(&mut vs_constants)
                    .new_state(ResourceState::ConstantBuffer)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(&mut instance_constants)
                    .new_state(ResourceState::ConstantBuffer)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                // Explicitly transition vertex and index buffers to required states
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
                    .resource(texture0)
                    .new_state(ResourceState::ShaderResource)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(texture1)
                    .new_state(ResourceState::ShaderResource)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(texture2)
                    .new_state(ResourceState::ShaderResource)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
                StateTransitionDesc::builder()
                    .resource(texture3)
                    .new_state(ResourceState::ShaderResource)
                    .flags(StateTransitionFlags::UpdateState)
                    .build(),
            ];

            // Execute all barriers
            main_context.transition_resource_states(&barriers);
        }

        let textures_srv = textures.map(|texture| {
            // Get shader resource view from the texture array
            Boxed::<TextureView>::from_ref(
                texture
                    .get_default_view(TextureViewType::ShaderResource)
                    .unwrap(),
            )
        });

        let srb = std::array::from_fn(|_| pso.create_shader_resource_binding(true).unwrap());
        srb.iter().enumerate().for_each(|(index, srb)| {
            srb.get_variable_by_name("g_Texture", ShaderTypes::Pixel)
                .unwrap()
                .set(&textures_srv[index], SetShaderResourceFlags::None)
        });

        let grid_size = 5;
        let instances = populate_instance_buffer(grid_size);

        let (command_list_sender, command_list_receiver) =
            std::sync::mpsc::channel::<Boxed<CommandList>>();

        let paused_threads = deferred_contexts
            .into_iter()
            .map(|context| {
                let (message_sender, message_receiver) =
                    std::sync::mpsc::channel::<ThreadMessage>();

                PausedThread {
                    data: ExclusiveThreadData {
                        context,
                        message_receiver,
                        command_list_sender: command_list_sender.clone(),
                    },
                    message_sender,
                }
            })
            .collect();

        let shared_thread_data = SharedThreadData {
            pso,
            instance_constants,
            instances,
            textured_cube,
            srb,
            vs_constants,
        };

        let mut sample = Self {
            rotation_matrix: glam::Mat4::IDENTITY,

            _textures_srv: textures_srv,
            grid_size,
            max_threads,
            num_worker_threads,

            swap_chain: Arc::new(RwLock::new(swap_chain)),

            shared_thead_data: Arc::new(shared_thread_data),

            paused_threads,
            running_threads: VecDeque::new(),

            convert_ps_output_to_gamma,

            command_list_receiver,

            execute_command_list_barrier: Arc::new(Barrier::new(num_worker_threads)),
        };

        sample.set_num_threads(num_worker_threads);

        sample
    }

    fn update(
        &mut self,
        _main_context: &ImmediateDeviceContext,
        current_time: f64,
        _elapsed_time: f64,
    ) {
        // Apply rotation
        self.rotation_matrix = glam::Mat4::from_rotation_y(current_time as f32)
            * glam::Mat4::from_rotation_x(-current_time as f32 * 0.25);
    }

    fn render(
        &mut self,
        main_context: Boxed<ImmediateDeviceContext>,
    ) -> Boxed<ImmediateDeviceContext> {
        // Exclusive access to the swap chain in this thread
        {
            let mut swap_chain = self.swap_chain.write().unwrap();

            let (rtv, dsv) = swap_chain.get_current_rtv_and_dsv_mut();

            let rtv = rtv.unwrap().transition_state();
            let dsv = dsv.map(|v| v.transition_state());

            main_context.set_render_targets(std::slice::from_ref(&rtv), dsv.clone());

            // Clear the back buffer
            {
                let clear_color = {
                    let clear_color = [0.35, 0.35, 0.35, 1.0];

                    if self.convert_ps_output_to_gamma {
                        // If manual gamma correction is required, we need to clear the render target with sRGB color
                        linear_to_srgba(clear_color)
                    } else {
                        clear_color
                    }
                };

                main_context.clear_render_target(rtv, &clear_color);
            }

            if let Some(dsv) = dsv {
                main_context.clear_depth(dsv, 1.0);
            }
        }

        let swap_chain = self.swap_chain.read().unwrap();
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
            let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 4.0))
                * glam::Mat4::from_rotation_x(-0.6);

            proj * srf_pre_transform * view
        };

        // Tell all the worker threads to render
        self.running_threads.iter().for_each(|thread| {
            thread
                .message_sender
                .send(ThreadMessage::Draw {
                    swap_chain: self.swap_chain.clone(),
                    view_proj_matrix,
                })
                .unwrap()
        });

        // Also render in the main thread
        let main_context = self.shared_thead_data.render_subset(
            main_context,
            0,
            self.num_worker_threads + 1,
            &self.swap_chain.read().unwrap(),
            &view_proj_matrix,
            &self.rotation_matrix,
        );

        let mut command_lists = Vec::new();
        // Get all the command lists from the threads
        for _ in 0..self.running_threads.len() {
            command_lists.push(self.command_list_receiver.recv().unwrap());
        }

        let command_lists_refs: Vec<_> = command_lists
            .iter()
            .map(|cmd_list| cmd_list.deref())
            .collect();

        main_context.execute_command_lists(command_lists_refs.as_slice());

        self.execute_command_list_barrier.wait();

        main_context
    }
}

struct MultithreadingApp {
    device: Boxed<RenderDevice>,
    main_context: Boxed<ImmediateDeviceContext>,

    sample: Multithreading,

    app_settings: SampleAppSettings,

    graphics_adapter: Option<GraphicsAdapterInfo>,

    display_modes: Vec<DisplayModeAttribs>,
    display_modes_strings: Vec<String>,
    selected_display_mode: usize,

    fullscreen_mode: bool,

    window: SampleWindow,
}

impl MultithreadingApp {
    #[allow(clippy::type_complexity)]
    fn create_device_and_contexts_and_swap_chains(
        app_settings: &SampleAppSettings,
        engine_factory: &EngineFactory,
        swap_chain_ci: &SwapChainCreateInfo,
        engine_create_info: EngineCreateInfo,
        window_manager: &mut impl WindowManager,
    ) -> (
        Boxed<RenderDevice>,
        Vec<Boxed<ImmediateDeviceContext>>,
        Vec<Boxed<DeferredDeviceContext>>,
        SampleWindow,
        Boxed<SwapChain>,
    ) {
        match &engine_factory {
            #[cfg(feature = "opengl")]
            EngineFactory::OpenGL(engine_factory) => {
                if engine_create_info.num_deferred_contexts != 0 {
                    panic!("Deferred contexts are not supported in OpenGL mode");
                }

                let window = WM::create_window(swap_chain_ci.width(), swap_chain_ci.height());

                let mut engine_gl_create_info =
                    EngineGLCreateInfo::new(window.native(), engine_create_info);

                GenericSample::modify_engine_init_info(
                    &mut sample::EngineCreateInfo::EngineGLCreateInfo(&mut engine_gl_create_info),
                );

                let (device, immediate_context, swap_chain) = engine_factory
                    .create_device_and_swap_chain_gl(&engine_gl_create_info, swap_chain_ci)
                    .unwrap();

                let swap_chain_desc = swap_chain.desc();

                let imgui_renderer = ImguiRenderer::new(
                    &ImguiRendererCreateInfo::builder()
                        .device(&device)
                        .maybe_back_buffer_format(swap_chain_desc.color_buffer_format())
                        .maybe_depth_buffer_format(swap_chain_desc.depth_buffer_format())
                        .initial_width(swap_chain_desc.width() as f32)
                        .initial_height(swap_chain_desc.height() as f32)
                        .build(),
                );

                (
                    device,
                    vec![immediate_context],
                    vec![],
                    vec![SampleWindow {
                        swap_chain,
                        window,
                        imgui_renderer,
                    }],
                )
            }
            #[cfg(any(feature = "vulkan", feature = "d3d11", feature = "d3d12"))]
            _ => {
                let (device, immediate_contexts, defered_contexts) = match &engine_factory {
                    #[cfg(feature = "vulkan")]
                    EngineFactory::Vulkan(engine_factory) => {
                        use diligent_samples::sample_base::sample;

                        let mut engine_vk_create_info = EngineVkCreateInfo::new(engine_create_info);

                        if app_settings.vk_compatibility {
                            engine_vk_create_info.features_vk = DeviceFeaturesVk::new(
                                DeviceFeatureState::Disabled,
                                DeviceFeatureState::Disabled,
                            );
                        };

                        Multithreading::modify_engine_init_info(
                            &mut sample::EngineCreateInfo::EngineVkCreateInfo(
                                &mut engine_vk_create_info,
                            ),
                        );

                        engine_factory
                            .create_device_and_contexts(&engine_vk_create_info)
                            .unwrap()
                    }
                    #[cfg(feature = "opengl")]
                    EngineFactory::OpenGL(_engine_factory) => {
                        // The device, and contexts are created with the swapchain in the same function later
                        unreachable!();
                    }
                    #[cfg(feature = "d3d11")]
                    EngineFactory::D3D11(engine_factory) => {
                        let mut engine_d3d11_create_info = EngineD3D11CreateInfo::new(
                            D3D11ValidationFlags::None,
                            engine_create_info,
                        );

                        GenericSample::modify_engine_init_info(
                            &mut sample::EngineCreateInfo::EngineD3D11CreateInfo(
                                &mut engine_d3d11_create_info,
                            ),
                        );

                        engine_factory
                            .create_device_and_contexts(&engine_d3d11_create_info)
                            .unwrap()
                    }
                    #[cfg(feature = "d3d12")]
                    EngineFactory::D3D12(engine_factory) => {
                        let mut engine_d3d12_create_info =
                            EngineD3D12CreateInfo::new(engine_create_info);

                        GenericSample::modify_engine_init_info(
                            &mut sample::EngineCreateInfo::EngineD3D12CreateInfo(
                                &mut engine_d3d12_create_info,
                            ),
                        );

                        engine_factory
                            .create_device_and_contexts(&engine_d3d12_create_info)
                            .unwrap()
                    }
                };

                let immediate_context = immediate_contexts.first().unwrap();

                let create_swap_chain = |swap_chain_ci, native_window| match &engine_factory {
                    #[cfg(feature = "vulkan")]
                    EngineFactory::Vulkan(engine_factory) => engine_factory
                        .create_swap_chain(
                            &device,
                            immediate_context,
                            swap_chain_ci,
                            &native_window,
                        )
                        .unwrap(),
                    #[cfg(feature = "opengl")]
                    EngineFactory::OpenGL(_engine_factory) => {
                        unreachable!()
                    }
                    #[cfg(feature = "d3d11")]
                    EngineFactory::D3D11(engine_factory) => engine_factory
                        .create_swap_chain(
                            device,
                            immediate_context,
                            swap_chain_ci,
                            &FullScreenModeDesc::default(),
                            &native_window,
                        )
                        .unwrap(),
                    #[cfg(feature = "d3d12")]
                    EngineFactory::D3D12(engine_factory) => engine_factory
                        .create_swap_chain(
                            device,
                            immediate_context,
                            swap_chain_ci,
                            &FullScreenModeDesc::default(),
                            &native_window,
                        )
                        .unwrap(),
                };

                use diligent_tools::imgui::renderer::{ImguiRenderer, ImguiRendererCreateInfo};

                let window =
                    window_manager.create_window(swap_chain_ci.width(), swap_chain_ci.height());

                let swap_chain = create_swap_chain(swap_chain_ci, window.native());

                let swap_chain_desc = swap_chain.desc();

                let imgui_renderer = ImguiRenderer::new(
                    &ImguiRendererCreateInfo::builder()
                        .device(&device)
                        .maybe_back_buffer_format(swap_chain_desc.color_buffer_format())
                        .maybe_depth_buffer_format(swap_chain_desc.depth_buffer_format())
                        .initial_width(swap_chain_desc.width() as f32)
                        .initial_height(swap_chain_desc.height() as f32)
                        .build(),
                );
                (
                    device,
                    immediate_contexts,
                    defered_contexts,
                    SampleWindow {
                        imgui_renderer,
                        window,
                    },
                    swap_chain,
                )
            }
        }
    }

    #[allow(unused_variables)]
    fn display_modes(
        engine_factory: &EngineFactory,
        app_settings: &SampleAppSettings,
        engine_create_info: &EngineCreateInfo,
    ) -> Vec<DisplayModeAttribs> {
        match &engine_factory {
            #[cfg(feature = "vulkan")]
            EngineFactory::Vulkan(_engine_factory) => Vec::new(),
            #[cfg(feature = "opengl")]
            EngineFactory::OpenGL(_engine_factory) => Vec::new(),
            #[cfg(feature = "d3d11")]
            EngineFactory::D3D11(engine_factory) => {
                match (&app_settings.adapter_type, app_settings.adapter_index) {
                    (AdapterType::Software, _) | (_, None) => Vec::new(),
                    (_, Some(adapter_index)) => engine_factory.enumerate_display_modes(
                        engine_create_info.graphics_api_version,
                        adapter_index as u32,
                        0,
                        TextureFormat::RGBA8_UNORM_SRGB,
                    ),
                }
            }
            #[cfg(feature = "d3d12")]
            EngineFactory::D3D12(engine_factory) => {
                match (&app_settings.adapter_type, app_settings.adapter_index) {
                    (AdapterType::Software, _) | (_, None) => Vec::new(),
                    (_, Some(adapter_index)) => engine_factory.enumerate_display_modes(
                        engine_create_info.graphics_api_version,
                        adapter_index as u32,
                        0,
                        TextureFormat::RGBA8_UNORM_SRGB,
                    ),
                }
            }
        }
    }

    fn new(
        app_settings: SampleAppSettings,
        mut engine_create_info: EngineCreateInfo,
        window_manager: &mut impl WindowManager,
    ) -> Self {
        let swap_chain_ci = SwapChainCreateInfo::builder()
            .width(app_settings.width)
            .height(app_settings.height)
            .build();

        fn find_adapter(
            mut adapter_index: Option<usize>,
            adapter_type: AdapterType,
            adapters: &[GraphicsAdapterInfo],
        ) -> Option<usize> {
            let mut adapter_type = adapter_type;

            if let Some(adap_id) = adapter_index {
                if adap_id < adapters.len() {
                    adapter_type = adapters.get(adap_id).unwrap().adapter_type();
                } else {
                    //LOG_ERROR_MESSAGE("Adapter ID (", AdapterId, ") is invalid. Only ", Adapters.size(), " compatible ", (Adapters.size() == 1 ? "adapter" : "adapters"), " present in the system");
                    adapter_index = None;
                }
            }

            if adapter_index.is_none() && adapter_type != AdapterType::Unknown {
                adapter_index = adapters
                    .iter()
                    .position(|adapter| adapter.adapter_type() == adapter_type);
            };

            if adapter_index.is_none()
                && let Some((index, _best_adapter)) =
                    adapters
                        .iter()
                        .enumerate()
                        .max_by(|(_, adapter1), (_, adapter2)| {
                            // Prefer Discrete over Integrated over Software
                            let compare_type =
                                |adapter1: &GraphicsAdapterInfo, adapter2: &GraphicsAdapterInfo| {
                                    adapter1.adapter_type().cmp(&adapter2.adapter_type())
                                };

                            // Select adapter with most memory
                            let get_total_mem = |adapter: &GraphicsAdapterInfo| {
                                adapter.memory().local_memory()
                                    + adapter.memory().host_visible_memory()
                                    + adapter.memory().unified_memory()
                            };
                            let compare_memory =
                                |adapter1: &GraphicsAdapterInfo, adapter2: &GraphicsAdapterInfo| {
                                    get_total_mem(adapter1).cmp(&get_total_mem(adapter2))
                                };

                            compare_type(adapter1, adapter2)
                                .then(compare_memory(adapter1, adapter2))
                        })
            {
                adapter_index = Some(index);
            }

            if let Some(adapter_index) = adapter_index {
                let adaper_description = adapters.get(adapter_index).unwrap().description();
                println!(
                    "Using adapter {adapter_index}, : '{}'",
                    adaper_description.to_str().unwrap()
                );
            }

            adapter_index
        }

        let engine_factory = match app_settings.device_type {
            #[cfg(feature = "d3d11")]
            RenderDeviceType::D3D11 => EngineFactory::D3D11(get_engine_factory_d3d11()),
            #[cfg(feature = "d3d12")]
            RenderDeviceType::D3D12 => {
                let engine_factory = get_engine_factory_d3d12();
                if !engine_factory.load_d3d12() {
                    panic!("Failed to load Direct3D12");
                }
                EngineFactory::D3D12(engine_factory)
            }
            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => EngineFactory::OpenGL(get_engine_factory_gl()),
            //RenderDeviceType::GLES => panic!(),
            #[cfg(feature = "vulkan")]
            RenderDeviceType::VULKAN => EngineFactory::Vulkan(get_engine_factory_vk()),
            #[cfg(feature = "metal")]
            RenderDeviceType::METAL => todo!(),
            #[cfg(feature = "webgpu")]
            RenderDeviceType::WEBGPU => todo!(),
        };

        #[cfg(feature = "d3d11")]
        {
            engine_create_info.graphics_api_version = Version::new(11, 0);
        }

        #[cfg(feature = "d3d12")]
        {
            engine_create_info.graphics_api_version = Version::new(11, 0);
        }

        let adapters = engine_factory.enumerate_adapters(engine_create_info.graphics_api_version);

        let adapter = if let Some(adapter_index) = find_adapter(
            app_settings.adapter_index,
            app_settings.adapter_type,
            adapters.as_slice(),
        ) {
            engine_create_info.adapter_index.replace(adapter_index);
            adapters.into_iter().nth(adapter_index)
        } else {
            None
        };

        engine_create_info
            .features
            .set_all(DeviceFeatureState::Optional);

        engine_create_info
            .features
            .set_transfer_queue_timestamp_queries(DeviceFeatureState::Disabled);

        let display_modes =
            Self::display_modes(&engine_factory, &app_settings, &engine_create_info);

        let (device, mut immediate_contexts, deferred_contexts, window, swap_chain) =
            Self::create_device_and_contexts_and_swap_chains(
                &app_settings,
                &engine_factory,
                &swap_chain_ci,
                engine_create_info,
                window_manager,
            );

        let other_immediate_context = immediate_contexts.split_off(1);
        let main_context = immediate_contexts.remove(0);

        let sample = Multithreading::new(
            &engine_factory,
            &device,
            &main_context,
            other_immediate_context,
            deferred_contexts,
            swap_chain,
        );

        let display_modes_strings = display_modes
            .iter()
            .map(|display_mode: &DisplayModeAttribs| {
                let refresh_rate = display_mode.refresh_rate_numerator() as f32
                    / display_mode.refresh_rate_denominator() as f32;

                format!(
                    "{}x{}@{refresh_rate} Hz{}",
                    display_mode.width(),
                    display_mode.height(),
                    match display_mode.scaling_mode() {
                        ScalingMode::Unspecified => "",
                        ScalingMode::Centered => " Centered",
                        ScalingMode::Stretched => " Stretched",
                    }
                )
            })
            .collect();

        Self {
            sample,
            device,
            app_settings,
            display_modes,
            display_modes_strings,
            fullscreen_mode: false,
            graphics_adapter: adapter,
            main_context,
            selected_display_mode: 0,
            window,
        }
    }

    fn run(mut self) -> Result<(), std::io::Error> {
        let start_time = std::time::Instant::now();

        let mut last_time = start_time;

        let app_title = String::from(Multithreading::get_name())
            + " ("
            + self.app_settings.device_type.to_string().as_str()
            + ", API "
            + format!("{API_VERSION}").as_str()
            + ")";

        self.window.window.set_title(app_title.as_str());

        let mut filtered_frame_time = 0.0;

        'main: loop {
            // Update
            let elapsed_time = {
                let now = std::time::Instant::now();

                let current_time = now.duration_since(start_time).as_secs_f64();
                let elapsed_time = now.duration_since(last_time).as_secs_f64();

                self.sample
                    .update(&self.main_context, current_time, elapsed_time);

                last_time = now;

                elapsed_time
            };

            // Handle events
            let mut imgui_frame = self.window.imgui_renderer.new_frame();

            while let Some(event) = self.window.window.handle_event() {
                match event {
                    Event::Quit => {
                        break 'main;
                    }
                    Event::Continue => {}
                    Event::Resize { width, height } => {
                        self.sample.swap_chain.write().unwrap().resize(
                            width as u32,
                            height as u32,
                            SurfaceTransform::Optimal,
                        );
                    }
                    _ => {}
                }

                imgui_handle_event(imgui_frame.io_mut(), event);
            }

            // Render
            {
                self.main_context.clear_stats();

                self.main_context = self.sample.render(self.main_context);
            }

            // Render imgui UI
            if self.app_settings.show_ui {
                let width = {
                    let mut swap_chain = self.sample.swap_chain.write().unwrap();

                    let (rtv, dsv) = swap_chain.get_current_rtv_and_dsv_mut();

                    self.main_context.set_render_targets(
                        &[rtv.unwrap().transition_state()],
                        dsv.map(|dsv| dsv.transition_state()),
                    );

                    swap_chain.desc().width()
                };

                let adapters_wnd_width = width.min(330);

                let ui = imgui_frame.ui_mut();

                if self.app_settings.show_adapters_dialog
                    && let Some(_window_token) = ui
                        .window("Adapters")
                        .size([adapters_wnd_width as f32, 0.0], imgui::Condition::Always)
                        .position(
                            [
                                (width as f32 - adapters_wnd_width as f32).max(10.0) - 10.0,
                                10.0,
                            ],
                            imgui::Condition::Always,
                        )
                        .flags(imgui::WindowFlags::NO_RESIZE)
                        .collapsed(true, imgui::Condition::FirstUseEver)
                        .begin()
                {
                    if let Some(adapter) = &self.graphics_adapter
                        && adapter.adapter_type() != AdapterType::Unknown
                    {
                        ui.text_disabled(format!(
                            "Adapter: {} ({} MB)",
                            adapter.description().to_str().unwrap(),
                            adapter.memory().local_memory() >> 20
                        ));
                    }

                    if !self.display_modes.is_empty() {
                        ui.set_next_item_width(220.0);
                        ui.combo(
                            "Display Modes",
                            &mut self.selected_display_mode,
                            self.display_modes_strings.as_slice(),
                            |label| label.into(),
                        );
                    }

                    if self.fullscreen_mode {
                        if ui.button("Go Windowed") {
                            self.sample.release_swap_chain_buffers();
                            self.fullscreen_mode = false;
                            self.sample.swap_chain.write().unwrap().set_windowed_mode();
                        }
                    } else if !self.display_modes.is_empty() && ui.button("Go Full Screen") {
                        self.sample.release_swap_chain_buffers();

                        let display_mode =
                            self.display_modes.get(self.selected_display_mode).unwrap();
                        self.fullscreen_mode = true;
                        self.sample
                            .swap_chain
                            .write()
                            .unwrap()
                            .set_fullscreen_mode(display_mode);
                    }

                    // If you're noticing any difference in frame rate when you enable vsync,
                    // this is because of the window title update. This also happens on the
                    // main DiligentSamples repository.
                    ui.checkbox("VSync", &mut self.app_settings.vsync);
                }
                self.sample.update_ui(&self.device, &self.main_context, ui);
                self.main_context = imgui_frame.render(self.main_context, &self.device);
            }

            self.sample
                .swap_chain
                .read()
                .unwrap()
                .present(if self.app_settings.vsync { 1 } else { 0 });

            // Update window title
            {
                let filter_scale = 0.2;
                filtered_frame_time =
                    filtered_frame_time * (1.0 - filter_scale) + filter_scale * elapsed_time;

                self.window.window.set_title(
                    format!(
                        "{app_title} - {:.1} ms ({:.1} fps)",
                        filtered_frame_time * 1000.0,
                        1.0 / filtered_frame_time
                    )
                    .as_str(),
                );
            }

            self.window.imgui_renderer = imgui_frame.finish();
        }

        Ok(())
    }
}

fn main() {
    let settings = parse_sample_app_settings();

    let engine_ci = EngineCreateInfo::default();

    #[cfg(target_os = "windows")]
    {
        let mut window_manager = diligent_tools::native_app::windows::Win32WindowManager::new();

        MultithreadingApp::new(settings, engine_ci, &mut window_manager)
            .run()
            .unwrap()
    }
    #[cfg(target_os = "linux")]
    {
        let device_type = settings.device_type;
        let window_manager = match device_type {
            #[cfg(feature = "vulkan")]
            RenderDeviceType::VULKAN => {
                Ok(diligent_tools::native_app::linux::xcb::XCBWindowManager::new())
            }

            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => {
                Ok(diligent_tools::native_app::linux::xcb::X11WindowManager::new())
            }

            #[allow(unreachable_patterns)]
            _ => Err(std::io::Error::other(format!(
                "Render device type {device_type} is not available on linux",
            ))),
        };
        MultithreadingApp::new(settings, engine_ci, &mut window_manager.unwrap())
            .run()
            .unwrap()
    }
}
