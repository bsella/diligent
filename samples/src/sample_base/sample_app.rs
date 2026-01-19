use std::{collections::VecDeque, ops::Deref};

use diligent::*;

use diligent_tools::{
    imgui::{
        events::imgui_handle_event,
        renderer::{ImguiRenderer, ImguiRendererCreateInfo},
    },
    native_app::{
        app::{App, GoldenImageMode},
        events::Event,
        Window,
    },
};

#[cfg(feature = "vulkan")]
use diligent::vk::engine_factory_vk::{
    get_engine_factory_vk, DeviceFeaturesVk, EngineFactoryVk, EngineVkCreateInfo,
};

#[cfg(feature = "opengl")]
use diligent::gl::engine_factory_gl::{
    get_engine_factory_gl, EngineFactoryOpenGL, EngineGLCreateInfo,
};

#[cfg(feature = "d3d11")]
use diligent::d3d11::engine_factory_d3d11::{
    get_engine_factory_d3d11, D3D11ValidationFlags, EngineD3D11CreateInfo, EngineFactoryD3D11,
};

#[cfg(feature = "d3d12")]
use diligent::d3d12::engine_factory_d3d12::{
    get_engine_factory_d3d12, EngineD3D12CreateInfo, EngineFactoryD3D12,
};

#[allow(unused_imports)]
use crate::sample_base::sample;

use super::{
    sample::SampleBase,
    sample_app_settings::{parse_sample_app_settings, SampleAppSettings},
};

struct SampleWindow<W: Window> {
    window: W,
    swap_chain: Boxed<SwapChain>,
    imgui_renderer: ImguiRenderer,
}

pub struct SampleApp<Sample: SampleBase, W: Window> {
    _golden_image_mode: GoldenImageMode,
    _golden_pixel_tolerance: u32,

    sample: Sample,

    app_settings: SampleAppSettings,

    graphics_adapter: Option<GraphicsAdapterInfo>,

    display_modes: Vec<DisplayModeAttribs>,
    display_modes_strings: Vec<String>,
    selected_display_mode: usize,

    fullscreen_mode: bool,

    windows: VecDeque<SampleWindow<W>>,
}

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

impl<GenericSample: SampleBase, W: Window> SampleApp<GenericSample, W> {
    fn window_resize(&mut self, width: u32, height: u32, swap_chain: &SwapChain) {
        self.sample.pre_window_resize();

        swap_chain.resize(width, height, SurfaceTransform::Optimal);

        let swap_chain_desc = swap_chain.desc();

        self.sample.window_resize(swap_chain_desc);
    }

    fn update(&mut self, current_time: f64, elapsed_time: f64) {
        self.sample.update(current_time, elapsed_time);
    }

    fn update_ui(&mut self, sample_window: &mut SampleWindow<W>) {
        let ui = sample_window.imgui_renderer.new_frame();

        let swap_chain_desc = sample_window.swap_chain.desc();

        let adapters_wnd_width = swap_chain_desc.width().min(330);

        if self.app_settings.show_adapters_dialog {
            if let Some(_window_token) = ui
                .window("Adapters")
                .size([adapters_wnd_width as f32, 0.0], imgui::Condition::Always)
                .position(
                    [
                        (swap_chain_desc.width() as f32 - adapters_wnd_width as f32).max(10.0)
                            - 10.0,
                        10.0,
                    ],
                    imgui::Condition::Always,
                )
                .flags(imgui::WindowFlags::NO_RESIZE)
                .collapsed(true, imgui::Condition::FirstUseEver)
                .begin()
            {
                if let Some(adapter) = &self.graphics_adapter {
                    if adapter.adapter_type() != AdapterType::Unknown {
                        ui.text_disabled(format!(
                            "Adapter: {} ({} MB)",
                            adapter.description().to_str().unwrap(),
                            adapter.memory().local_memory() >> 20
                        ));
                    }
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
                        sample_window.swap_chain.set_windowed_mode();
                    }
                } else if !self.display_modes.is_empty() && ui.button("Go Full Screen") {
                    self.sample.release_swap_chain_buffers();

                    let display_mode = self.display_modes.get(self.selected_display_mode).unwrap();
                    self.fullscreen_mode = true;
                    sample_window.swap_chain.set_fullscreen_mode(display_mode);
                }

                // If you're noticing any difference in frame rate when you enable vsync,
                // this is because of the window title update. This also happens on the
                // main DiligentSamples repository.
                ui.checkbox("VSync", &mut self.app_settings.vsync);
            }
        }
        self.sample.update_ui(ui);
    }

    fn render(&self, swap_chain: &SwapChain) {
        let context = self.sample.get_immediate_context();
        context.clear_stats();

        let rtv = swap_chain.get_current_back_buffer_rtv().unwrap();
        let dsv = swap_chain.get_depth_buffer_dsv().unwrap();

        context.set_render_targets(&[rtv], Some(dsv), ResourceStateTransitionMode::Transition);

        self.sample.render(swap_chain);

        // Restore default render target in case the sample has changed it
        context.set_render_targets(&[rtv], Some(dsv), ResourceStateTransitionMode::Transition);
    }

    fn present(&self, swap_chain: &SwapChain) {
        // TODO screen capture

        swap_chain.present(if self.app_settings.vsync { 1 } else { 0 });

        // TODO screen capture
    }

    fn create_device_and_contexts(
        app_settings: &SampleAppSettings,
        engine_factory: &EngineFactory,
        engine_create_info: EngineCreateInfo,
    ) -> (
        Boxed<RenderDevice>,
        Vec<Boxed<ImmediateDeviceContext>>,
        Vec<Boxed<DeferredDeviceContext>>,
    ) {
        match &engine_factory {
            #[cfg(feature = "vulkan")]
            EngineFactory::Vulkan(engine_factory) => {
                let mut engine_vk_create_info = EngineVkCreateInfo::new(engine_create_info);

                if app_settings.vk_compatibility {
                    engine_vk_create_info.features_vk = DeviceFeaturesVk::new(
                        DeviceFeatureState::Disabled,
                        DeviceFeatureState::Disabled,
                    );
                };

                GenericSample::modify_engine_init_info(
                    &mut sample::EngineCreateInfo::EngineVkCreateInfo(&mut engine_vk_create_info),
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
                let mut engine_d3d11_create_info =
                    EngineD3D11CreateInfo::new(D3D11ValidationFlags::None, engine_create_info);

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
                let mut engine_d3d12_create_info = EngineD3D12CreateInfo::new(engine_create_info);

                GenericSample::modify_engine_init_info(
                    &mut sample::EngineCreateInfo::EngineD3D12CreateInfo(
                        &mut engine_d3d12_create_info,
                    ),
                );

                engine_factory
                    .create_device_and_contexts(&engine_d3d12_create_info)
                    .unwrap()
            }
        }
    }

    fn create_swap_chains(
        app_settings: &SampleAppSettings,
        engine_factory: &EngineFactory,
        device: &RenderDevice,
        swap_chain_ci: &SwapChainCreateInfo,
        immediate_context: &ImmediateDeviceContext,
    ) -> VecDeque<SampleWindow<W>> {
        let window = W::create(app_settings.width, app_settings.height);

        let imgui_renderer = ImguiRenderer::new(
            &ImguiRendererCreateInfo::builder()
                .device(device)
                .back_buffer_format(swap_chain_ci.color_buffer_format())
                .depth_buffer_format(swap_chain_ci.depth_buffer_format())
                .initial_width(app_settings.width as f32)
                .initial_height(app_settings.height as f32)
                .build(),
        );

        match &engine_factory {
            #[cfg(feature = "vulkan")]
            EngineFactory::Vulkan(engine_factory) => {
                let swap_chain = engine_factory
                    .create_swap_chain(device, immediate_context, swap_chain_ci, &window.native())
                    .unwrap();

                VecDeque::from([SampleWindow {
                    swap_chain,
                    window,
                    imgui_renderer,
                }])
            }
            #[cfg(feature = "opengl")]
            EngineFactory::OpenGL(_engine_factory) => {
                unreachable!()
            }
            #[cfg(feature = "d3d11")]
            EngineFactory::D3D11(engine_factory) => {
                let swap_chain = engine_factory
                    .create_swap_chain(
                        device,
                        immediate_context,
                        swap_chain_ci,
                        &FullScreenModeDesc::default(),
                        &window.native(),
                    )
                    .unwrap();

                VecDeque::from([SampleWindow {
                    swap_chain,
                    window,
                    imgui_renderer,
                }])
            }
            #[cfg(feature = "d3d12")]
            EngineFactory::D3D12(engine_factory) => {
                let swap_chain = engine_factory
                    .create_swap_chain(
                        device,
                        immediate_context,
                        swap_chain_ci,
                        &FullScreenModeDesc::default(),
                        &window.native(),
                    )
                    .unwrap();

                VecDeque::from([SampleWindow {
                    swap_chain,
                    window,
                    imgui_renderer,
                }])
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn create_device_and_contexts_and_swap_chains(
        app_settings: &SampleAppSettings,
        engine_factory: &EngineFactory,
        swap_chain_ci: &SwapChainCreateInfo,
        engine_create_info: EngineCreateInfo,
    ) -> (
        Boxed<RenderDevice>,
        Vec<Boxed<ImmediateDeviceContext>>,
        Vec<Boxed<DeferredDeviceContext>>,
        VecDeque<SampleWindow<W>>,
    ) {
        match &engine_factory {
            #[cfg(feature = "opengl")]
            EngineFactory::OpenGL(engine_factory) => {
                let window = W::create(app_settings.width, app_settings.height);

                if engine_create_info.num_deferred_contexts != 0 {
                    panic!("Deferred contexts are not supported in OpenGL mode");
                }

                let mut engine_gl_create_info =
                    EngineGLCreateInfo::new(window.native(), engine_create_info);

                GenericSample::modify_engine_init_info(
                    &mut sample::EngineCreateInfo::EngineGLCreateInfo(&mut engine_gl_create_info),
                );

                let (device, immediate_context, swap_chain) = engine_factory
                    .create_device_and_swap_chain_gl(&engine_gl_create_info, swap_chain_ci)
                    .unwrap();

                let imgui_renderer = ImguiRenderer::new(
                    &ImguiRendererCreateInfo::builder()
                        .device(&device)
                        .back_buffer_format(swap_chain_ci.color_buffer_format())
                        .depth_buffer_format(swap_chain_ci.depth_buffer_format())
                        .initial_width(app_settings.width as f32)
                        .initial_height(app_settings.height as f32)
                        .build(),
                );

                (
                    device,
                    vec![immediate_context],
                    Vec::new(),
                    VecDeque::from([SampleWindow {
                        swap_chain,
                        window,
                        imgui_renderer,
                    }]),
                )
            }
            #[cfg(any(feature = "vulkan", feature = "d3d11", feature = "d3d12"))]
            _ => {
                let (device, immediate_contexts, defered_contexts) =
                    Self::create_device_and_contexts(
                        app_settings,
                        engine_factory,
                        engine_create_info,
                    );
                let sample_windows = Self::create_swap_chains(
                    app_settings,
                    engine_factory,
                    &device,
                    swap_chain_ci,
                    immediate_contexts.first().unwrap(),
                );
                (device, immediate_contexts, defered_contexts, sample_windows)
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
}

impl<GenericSample: SampleBase, W: Window> App for SampleApp<GenericSample, W> {
    type AppSettings = SampleAppSettings;

    fn new(app_settings: SampleAppSettings, mut engine_create_info: EngineCreateInfo) -> Self {
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

            if adapter_index.is_none() {
                if let Some((index, _best_adapter)) =
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

        let (device, immediate_contexts, deferred_contexts, windows) =
            Self::create_device_and_contexts_and_swap_chains(
                &app_settings,
                &engine_factory,
                &swap_chain_ci,
                engine_create_info,
            );

        let sample = GenericSample::new(
            &engine_factory,
            device,
            immediate_contexts,
            deferred_contexts,
            &swap_chain_ci,
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

        SampleApp::<GenericSample, W> {
            _golden_image_mode: GoldenImageMode::None,
            _golden_pixel_tolerance: 0,

            sample,

            app_settings,

            graphics_adapter: adapter,

            display_modes,
            display_modes_strings,
            selected_display_mode: 0,

            fullscreen_mode: false,

            windows,
        }
    }

    fn run(mut self) -> Result<(), std::io::Error> {
        let start_time = std::time::Instant::now();

        let mut last_time = start_time;

        let app_title = String::from(GenericSample::get_name())
            + " ("
            + self.app_settings.device_type.to_string().as_str()
            + ", API "
            + format!("{API_VERSION}").as_str()
            + ")";

        self.windows
            .iter()
            .for_each(|sample_window| sample_window.window.set_title(app_title.as_str()));

        let mut filtered_frame_time = 0.0;

        loop {
            let elapsed_time = {
                let now = std::time::Instant::now();

                let current_time = now.duration_since(start_time).as_secs_f64();
                let elapsed_time = now.duration_since(last_time).as_secs_f64();

                self.update(current_time, elapsed_time);

                last_time = now;

                elapsed_time
            };

            let mut next_windows: VecDeque<SampleWindow<W>> = self.windows.drain(..).collect();

            'window: for mut sample_window in next_windows.drain(..) {
                while let Some(event) = sample_window.window.poll_event() {
                    let event = sample_window.window.handle_event(&event);
                    match event {
                        Event::Quit => {
                            drop(sample_window);
                            continue 'window;
                        }
                        Event::Continue => {}
                        Event::Resize { width, height } => self.window_resize(
                            width as u32,
                            height as u32,
                            &sample_window.swap_chain,
                        ),
                        _ => {}
                    }

                    let event = imgui_handle_event(sample_window.imgui_renderer.io_mut(), event);

                    self.sample.handle_event(event);
                }

                self.render(&sample_window.swap_chain);

                if self.app_settings.show_ui {
                    self.update_ui(&mut sample_window);
                    sample_window.imgui_renderer.render(
                        self.sample.get_immediate_context(),
                        self.sample.get_render_device(),
                    );
                }

                self.present(&sample_window.swap_chain);

                {
                    let filter_scale = 0.2;
                    filtered_frame_time =
                        filtered_frame_time * (1.0 - filter_scale) + filter_scale * elapsed_time;

                    sample_window.window.set_title(
                        format!(
                            "{app_title} - {:.1} ms ({:.1} fps)",
                            filtered_frame_time * 1000.0,
                            1.0 / filtered_frame_time
                        )
                        .as_str(),
                    );
                }

                self.windows.push_back(sample_window);
            }

            if self.windows.is_empty() {
                break;
            }
        }

        Ok(())
    }
}

pub fn main<Sample: SampleBase>() -> Result<(), std::io::Error> {
    let settings = parse_sample_app_settings();

    let engine_ci = EngineCreateInfo::default();

    #[cfg(target_os = "windows")]
    {
        use diligent_tools::native_app::windows::Win32Window;
        SampleApp::<Sample, Win32Window>::new(settings, engine_ci).run()
    }
    #[cfg(target_os = "linux")]
    {
        let device_type = settings.device_type;
        match device_type {
            #[cfg(feature = "vulkan")]
            RenderDeviceType::VULKAN => {
                use diligent_tools::native_app::linux::xcb::XCBWindow;

                SampleApp::<Sample, XCBWindow>::new(settings, engine_ci).run()
            }

            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => {
                use diligent_tools::native_app::linux::x11::X11Window;

                SampleApp::<Sample, X11Window>::new(settings, engine_ci).run()
            }

            #[allow(unreachable_patterns)]
            _ => Err(std::io::Error::other(format!(
                "Render device type {device_type} is not available on linux",
            ))),
        }
    }
}
