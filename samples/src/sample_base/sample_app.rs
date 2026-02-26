use std::{collections::VecDeque, ops::Deref};

use diligent::*;

use diligent_tools::{
    imgui::{
        events::imgui_handle_event,
        renderer::{ImguiRenderer, ImguiRendererCreateInfo},
    },
    native_app::{Window, WindowManager, events::Event},
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

#[allow(unused_imports)]
use crate::sample_base::sample;

use super::{
    sample::SampleBase,
    sample_app_settings::{SampleAppSettings, parse_sample_app_settings},
};

// TODO
#[allow(dead_code)]
enum GoldenImageMode {
    None,
    Capture,
    Compare,
    CompareUpdate,
}

pub struct SampleWindow {
    window: std::boxed::Box<dyn Window>,
    swap_chain: Boxed<SwapChain>,
    imgui_renderer: ImguiRenderer,
}

pub struct SampleApp<Sample: SampleBase> {
    _golden_image_mode: GoldenImageMode,
    _golden_pixel_tolerance: u32,

    device: Boxed<RenderDevice>,
    main_context: Boxed<ImmediateDeviceContext>,

    sample: Sample,

    app_settings: SampleAppSettings,

    graphics_adapter: Option<GraphicsAdapterInfo>,

    display_modes: Vec<DisplayModeAttribs>,
    display_modes_strings: Vec<String>,
    selected_display_mode: usize,

    fullscreen_mode: bool,

    windows: VecDeque<SampleWindow>,
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

impl<GenericSample: SampleBase> SampleApp<GenericSample> {
    fn window_resize(&mut self, width: u32, height: u32, swap_chain: &mut SwapChain) {
        self.sample.pre_window_resize();

        swap_chain.resize(width, height, SurfaceTransform::Optimal);

        let swap_chain_desc = swap_chain.desc();

        self.sample.window_resize(&self.device, swap_chain_desc);
    }

    fn update(&mut self, current_time: f64, elapsed_time: f64) {
        self.sample
            .update(&self.main_context, current_time, elapsed_time);
    }

    fn update_ui(&mut self, swap_chain: &mut SwapChain, ui: &mut imgui::Ui) {
        let swap_chain_desc = swap_chain.desc();

        let adapters_wnd_width = swap_chain_desc.width().min(330);

        if self.app_settings.show_adapters_dialog
            && let Some(_window_token) = ui
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
                    swap_chain.set_windowed_mode();
                }
            } else if !self.display_modes.is_empty() && ui.button("Go Full Screen") {
                self.sample.release_swap_chain_buffers();

                let display_mode = self.display_modes.get(self.selected_display_mode).unwrap();
                self.fullscreen_mode = true;
                swap_chain.set_fullscreen_mode(display_mode);
            }

            // If you're noticing any difference in frame rate when you enable vsync,
            // this is because of the window title update. This also happens on the
            // main DiligentSamples repository.
            ui.checkbox("VSync", &mut self.app_settings.vsync);
        }
        self.sample.update_ui(&self.device, &self.main_context, ui);
    }

    fn present(&self, swap_chain: &SwapChain) {
        // TODO screen capture

        swap_chain.present(if self.app_settings.vsync { 1 } else { 0 });

        // TODO screen capture
    }

    #[allow(clippy::type_complexity)]
    fn create_device_and_contexts_and_swap_chains<WM: WindowManager>(
        app_settings: &SampleAppSettings,
        engine_factory: &EngineFactory,
        swap_chain_ci: &[SwapChainCreateInfo],
        engine_create_info: EngineCreateInfo,
    ) -> (
        Boxed<RenderDevice>,
        Vec<Boxed<ImmediateDeviceContext>>,
        Vec<Boxed<DeferredDeviceContext>>,
        VecDeque<SampleWindow>,
    ) {
        match &engine_factory {
            #[cfg(feature = "opengl")]
            EngineFactory::OpenGL(engine_factory) => {
                if engine_create_info.num_deferred_contexts != 0 {
                    panic!("Deferred contexts are not supported in OpenGL mode");
                }

                if swap_chain_ci.len() > 1 {
                    panic!("The OpenGL backend does not permite creating multiple swapchains");
                }

                let window = WM::create_window(swap_chain_ci[0].width(), swap_chain_ci[0].height());

                let mut engine_gl_create_info =
                    EngineGLCreateInfo::new(window.native(), engine_create_info);

                GenericSample::modify_engine_init_info(
                    &mut sample::EngineCreateInfo::EngineGLCreateInfo(&mut engine_gl_create_info),
                );

                let (device, immediate_context, swap_chain) = engine_factory
                    .create_device_and_swap_chain_gl(&engine_gl_create_info, &swap_chain_ci[0])
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
                    VecDeque::from([SampleWindow {
                        swap_chain,
                        window,
                        imgui_renderer,
                    }]),
                )
            }
            #[cfg(any(feature = "vulkan", feature = "d3d11", feature = "d3d12"))]
            _ => {
                let (device, immediate_contexts, defered_contexts) = match &engine_factory {
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

                let sample_windows = {
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

                    swap_chain_ci
                        .iter()
                        .map(|ci| {
                            let window = WM::create_window(ci.width(), ci.height());

                            let swap_chain = create_swap_chain(ci, window.native());

                            let swap_chain_desc = swap_chain.desc();

                            let imgui_renderer = ImguiRenderer::new(
                                &ImguiRendererCreateInfo::builder()
                                    .device(&device)
                                    .maybe_back_buffer_format(swap_chain_desc.color_buffer_format())
                                    .maybe_depth_buffer_format(
                                        swap_chain_desc.depth_buffer_format(),
                                    )
                                    .initial_width(swap_chain_desc.width() as f32)
                                    .initial_height(swap_chain_desc.height() as f32)
                                    .build(),
                            );
                            SampleWindow {
                                imgui_renderer,
                                swap_chain,
                                window,
                            }
                        })
                        .collect()
                };
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

    fn new<WM: WindowManager>(
        app_settings: SampleAppSettings,
        mut engine_create_info: EngineCreateInfo,
    ) -> Self {
        let swap_chains_ci = GenericSample::make_swap_chains_create_info(&app_settings);

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

        let (device, mut immediate_contexts, deferred_contexts, windows) =
            Self::create_device_and_contexts_and_swap_chains::<WM>(
                &app_settings,
                &engine_factory,
                swap_chains_ci.as_slice(),
                engine_create_info,
            );

        let swap_chain_descs = windows
            .iter()
            .map(|sample_window| sample_window.swap_chain.desc())
            .collect::<Vec<_>>();

        let other_immediate_context = immediate_contexts.split_off(1);
        let main_context = immediate_contexts.remove(0);

        let sample = GenericSample::new(
            &engine_factory,
            &device,
            &main_context,
            other_immediate_context,
            deferred_contexts,
            swap_chain_descs.as_slice(),
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

        SampleApp::<GenericSample> {
            _golden_image_mode: GoldenImageMode::None,
            _golden_pixel_tolerance: 0,

            device,
            main_context,

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

        let mut next_windows = VecDeque::<SampleWindow>::new();

        loop {
            // Update
            let elapsed_time = {
                let now = std::time::Instant::now();

                let current_time = now.duration_since(start_time).as_secs_f64();
                let elapsed_time = now.duration_since(last_time).as_secs_f64();

                self.update(current_time, elapsed_time);

                last_time = now;

                elapsed_time
            };

            std::mem::swap(&mut self.windows, &mut next_windows);

            // Handle events
            'window: for mut sample_window in next_windows.drain(..) {
                let mut imgui_frame = sample_window.imgui_renderer.new_frame();

                while let Some(event) = sample_window.window.handle_event() {
                    match event {
                        Event::Quit => {
                            // sample_window is destroyed here instead of being moved into self.windows
                            continue 'window;
                        }
                        Event::Continue => {}
                        Event::Resize { width, height } => self.window_resize(
                            width as u32,
                            height as u32,
                            &mut sample_window.swap_chain,
                        ),
                        _ => {}
                    }

                    let event = imgui_handle_event(imgui_frame.io_mut(), event);

                    self.sample.handle_event(event);
                }

                // Render
                {
                    self.main_context.clear_stats();

                    let (rtv, dsv) = sample_window.swap_chain.get_current_rtv_and_dsv_mut();

                    let rtv = rtv.unwrap().transition_state();
                    let dsv = dsv.map(|dsv| dsv.transition_state());

                    self.main_context.set_render_targets(&[rtv], dsv);

                    (self.main_context, sample_window.swap_chain) = self
                        .sample
                        .render(self.main_context, sample_window.swap_chain);
                }

                // Render imgui UI
                if self.app_settings.show_ui {
                    self.update_ui(&mut sample_window.swap_chain, imgui_frame.ui_mut());
                    self.main_context = imgui_frame.render(self.main_context, &self.device);
                }

                self.present(&sample_window.swap_chain);

                // Update window title
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

                sample_window.imgui_renderer = imgui_frame.finish();

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
                use diligent_tools::native_app::linux::xcb::XCBWindowManager;

                SampleApp::<Sample>::new::<XCBWindowManager>(settings, engine_ci).run()
            }

            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => {
                use diligent_tools::native_app::linux::x11::X11WindowManager;

                SampleApp::<Sample>::new::<X11WindowManager>(settings, engine_ci).run()
            }

            #[allow(unreachable_patterns)]
            _ => Err(std::io::Error::other(format!(
                "Render device type {device_type} is not available on linux",
            ))),
        }
    }
}
