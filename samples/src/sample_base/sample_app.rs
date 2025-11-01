use std::ops::Deref;

use diligent::{platforms::native_window::NativeWindow, *};

use diligent_tools::{
    imgui::{
        events::imgui_handle_event,
        renderer::{ImguiRenderer, ImguiRendererCreateInfo},
    },
    native_app::{
        app::{App, GoldenImageMode},
        events::{Event, EventHandler},
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

pub struct SampleApp<Sample: SampleBase> {
    swap_chain: Boxed<SwapChain>,

    _golden_image_mode: GoldenImageMode,
    _golden_pixel_tolerance: u32,

    sample: Sample,

    imgui_renderer: ImguiRenderer,

    app_settings: SampleAppSettings,

    graphics_adapter: Option<GraphicsAdapterInfo>,

    display_modes: Vec<DisplayModeAttribs>,
    display_modes_strings: Vec<String>,
    selected_display_mode: usize,

    fullscreen_mode: bool,
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
    fn window_resize(&mut self, width: u32, height: u32) {
        self.sample.pre_window_resize();

        self.swap_chain
            .resize(width, height, SurfaceTransform::Optimal);

        let swap_chain_desc = self.swap_chain.get_desc();

        self.sample.window_resize(swap_chain_desc);
    }

    fn update(&mut self, current_time: f64, elapsed_time: f64) {
        self.sample.update(current_time, elapsed_time);
    }

    fn update_ui(&mut self) {
        let ui = self.imgui_renderer.new_frame();

        let swap_chain_desc = self.swap_chain.get_desc();

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
                        self.swap_chain.set_windowed_mode();
                    }
                } else if !self.display_modes.is_empty() && ui.button("Go Full Screen") {
                    self.sample.release_swap_chain_buffers();

                    let display_mode = self.display_modes.get(self.selected_display_mode).unwrap();
                    self.fullscreen_mode = true;
                    self.swap_chain.set_fullscreen_mode(display_mode);
                }

                // If you're noticing any difference in frame rate when you enable vsync,
                // this is because of the window title update. This also happens on the
                // main DiligentSamples repository.
                ui.checkbox("VSync", &mut self.app_settings.vsync);
            }
        }
        self.sample.update_ui(ui);
    }

    fn render(&self) {
        let context = self.sample.get_immediate_context();
        context.clear_stats();

        let rtv = self.swap_chain.get_current_back_buffer_rtv().unwrap();
        let dsv = self.swap_chain.get_depth_buffer_dsv().unwrap();

        context.set_render_targets(&[rtv], Some(&dsv), ResourceStateTransitionMode::Transition);

        self.sample.render(&self.swap_chain);

        // Restore default render target in case the sample has changed it
        context.set_render_targets(&[&rtv], Some(&dsv), ResourceStateTransitionMode::Transition);
    }

    fn present(&mut self) {
        // TODO screen capture

        self.swap_chain
            .present(if self.app_settings.vsync { 1 } else { 0 });

        // TODO screen capture
    }
}

impl<GenericSample: SampleBase> App for SampleApp<GenericSample> {
    type AppSettings = SampleAppSettings;

    fn parse_settings_from_cli() -> SampleAppSettings {
        parse_sample_app_settings()
    }

    fn new(
        app_settings: SampleAppSettings,
        mut engine_create_info: EngineCreateInfo,
        window: Option<NativeWindow>,
    ) -> Self {
        let swap_chain_ci = SwapChainCreateInfo::builder()
            .width(app_settings.width as u32)
            .height(app_settings.height as u32)
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
            RenderDeviceType::METAL => panic!(),
            #[cfg(feature = "webgpu")]
            RenderDeviceType::WEBGPU => panic!(),
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

        let (device, immediate_contexts, deferred_contexts, swap_chain, display_modes) =
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
                        &mut sample::EngineCreateInfo::EngineVkCreateInfo(
                            &mut engine_vk_create_info,
                        ),
                    );

                    let (device, immediate_contexts, deferred_contexts) = engine_factory
                        .create_device_and_contexts(&engine_vk_create_info)
                        .unwrap();

                    let swap_chain = engine_factory
                        .create_swap_chain(
                            &device,
                            immediate_contexts.first().unwrap(),
                            &swap_chain_ci,
                            window.as_ref(),
                        )
                        .unwrap();

                    (
                        device,
                        immediate_contexts,
                        deferred_contexts,
                        swap_chain,
                        Vec::new(),
                    )
                }
                #[cfg(feature = "opengl")]
                EngineFactory::OpenGL(engine_factory) => {
                    if let Some(window) = window {
                        if app_settings.non_separable_progs {
                            engine_create_info
                                .features
                                .set_separable_programs(DeviceFeatureState::Disabled);
                        }

                        if engine_create_info.num_deferred_contexts != 0 {
                            panic!("Deferred contexts are not supported in OpenGL mode");
                        }

                        let mut engine_gl_create_info =
                            EngineGLCreateInfo::new(window, engine_create_info);

                        GenericSample::modify_engine_init_info(
                            &mut sample::EngineCreateInfo::EngineGLCreateInfo(
                                &mut engine_gl_create_info,
                            ),
                        );

                        let (device, immediate_context, swap_chain) = engine_factory
                            .create_device_and_swap_chain_gl(&engine_gl_create_info, &swap_chain_ci)
                            .unwrap();

                        (
                            device,
                            vec![immediate_context],
                            Vec::new(),
                            swap_chain,
                            Vec::new(),
                        )
                    } else {
                        panic!("")
                    }
                }
                #[cfg(feature = "d3d11")]
                EngineFactory::D3D11(engine_factory) => {
                    let graphics_api_version = engine_create_info.graphics_api_version;

                    let mut engine_d3d11_create_info =
                        EngineD3D11CreateInfo::new(D3D11ValidationFlags::None, engine_create_info);

                    GenericSample::modify_engine_init_info(
                        &mut sample::EngineCreateInfo::EngineD3D11CreateInfo(
                            &mut engine_d3d11_create_info,
                        ),
                    );

                    let (device, immediate_contexts, deferred_contexts) = engine_factory
                        .create_device_and_contexts(&engine_d3d11_create_info)
                        .unwrap();

                    let display_modes =
                        match (&app_settings.adapter_type, app_settings.adapter_index) {
                            (AdapterType::Software, _) | (_, None) => Vec::new(),
                            (_, Some(adapter_index)) => engine_factory.enumerate_display_modes(
                                graphics_api_version,
                                adapter_index as u32,
                                0,
                                TextureFormat::RGBA8_UNORM_SRGB,
                            ),
                        };

                    let swap_chain = engine_factory
                        .create_swap_chain(
                            &device,
                            immediate_contexts.first().unwrap(),
                            &swap_chain_ci,
                            &FullScreenModeDesc::default(),
                            window.as_ref(),
                        )
                        .unwrap();

                    (
                        device,
                        immediate_contexts,
                        deferred_contexts,
                        swap_chain,
                        display_modes,
                    )
                }
                #[cfg(feature = "d3d12")]
                EngineFactory::D3D12(engine_factory) => {
                    let graphics_api_version = engine_create_info.graphics_api_version;

                    let mut engine_d3d12_create_info =
                        EngineD3D12CreateInfo::new(engine_create_info);

                    GenericSample::modify_engine_init_info(
                        &mut sample::EngineCreateInfo::EngineD3D12CreateInfo(
                            &mut engine_d3d12_create_info,
                        ),
                    );

                    let (device, immediate_contexts, deferred_contexts) = engine_factory
                        .create_device_and_contexts(&engine_d3d12_create_info)
                        .unwrap();

                    let display_modes =
                        match (&app_settings.adapter_type, app_settings.adapter_index) {
                            (AdapterType::Software, _) | (_, None) => Vec::new(),
                            (_, Some(adapter_index)) => engine_factory.enumerate_display_modes(
                                graphics_api_version,
                                adapter_index as u32,
                                0,
                                TextureFormat::RGBA8_UNORM_SRGB,
                            ),
                        };

                    let swap_chain = engine_factory
                        .create_swap_chain(
                            &device,
                            immediate_contexts.first().unwrap(),
                            &swap_chain_ci,
                            &FullScreenModeDesc::default(),
                            window.as_ref(),
                        )
                        .unwrap();

                    (
                        device,
                        immediate_contexts,
                        deferred_contexts,
                        swap_chain,
                        display_modes,
                    )
                }
            };

        let sample = GenericSample::new(
            &engine_factory,
            device,
            immediate_contexts,
            deferred_contexts,
            &*swap_chain,
        );

        let swap_chain_desc = swap_chain.get_desc();

        let imgui_renderer = ImguiRenderer::new(
            &ImguiRendererCreateInfo::builder()
                .device(sample.get_render_device())
                .back_buffer_format(swap_chain_desc.color_buffer_format())
                .depth_buffer_format(swap_chain_desc.depth_buffer_format())
                .initial_width(app_settings.width as f32)
                .initial_height(app_settings.height as f32)
                .build(),
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
            swap_chain,

            _golden_image_mode: GoldenImageMode::None,
            _golden_pixel_tolerance: 0,

            sample,

            app_settings,

            imgui_renderer,

            graphics_adapter: adapter,

            display_modes,
            display_modes_strings,
            selected_display_mode: 0,

            fullscreen_mode: false,
        }
    }

    fn run(
        mut self,
        mut event_handler: impl EventHandler,
        update_window_title_cb: impl Fn(&str),
    ) -> Result<(), std::io::Error> {
        let start_time = std::time::Instant::now();

        let mut last_time = start_time;

        let app_title = String::from(GenericSample::get_name())
            + " ("
            + self.app_settings.device_type.to_string().as_str()
            + ", API "
            + format!("{API_VERSION}").as_str()
            + ")";

        update_window_title_cb(app_title.as_str());

        let mut filtered_frame_time = 0.0;

        'main: loop {
            while let Some(event) = event_handler.poll_event() {
                let event = event_handler.handle_event(&event);
                match event {
                    Event::Quit => break 'main,
                    Event::Continue => {}
                    Event::Resize { width, height } => {
                        self.window_resize(width as u32, height as u32)
                    }
                    _ => {}
                }

                let event = imgui_handle_event(self.imgui_renderer.io_mut(), event);

                self.sample.handle_event(event);
            }

            let elapsed_time = {
                let now = std::time::Instant::now();

                let current_time = now.duration_since(start_time).as_secs_f64();
                let elapsed_time = now.duration_since(last_time).as_secs_f64();

                self.update(current_time, elapsed_time);

                last_time = now;

                elapsed_time
            };

            self.render();

            if self.app_settings.show_ui {
                self.update_ui();
                self.imgui_renderer.render(
                    self.sample.get_immediate_context(),
                    self.sample.get_render_device(),
                );
            }

            self.present();

            {
                let filter_scale = 0.2;
                filtered_frame_time =
                    filtered_frame_time * (1.0 - filter_scale) + filter_scale * elapsed_time;

                update_window_title_cb(
                    format!(
                        "{app_title} - {:.1} ms ({:.1} fps)",
                        filtered_frame_time * 1000.0,
                        1.0 / filtered_frame_time
                    )
                    .as_str(),
                );
            }
        }

        Ok(())
    }
}
