use imgui::WindowFlags;

use crate::{
    core::{
        accessories::get_render_device_type_string,
        api_info::API_VERSION,
        device_context::ResourceStateTransitionMode,
        engine_factory::{AsEngineFactory, EngineCreateInfo},
        graphics_types::{AdapterType, GraphicsAdapterInfo, RenderDeviceType, SurfaceTransform},
        swap_chain::{SwapChain, SwapChainDesc},
        vk::engine_factory_vk::{get_engine_factory_vk, EngineFactoryVk, EngineVkCreateInfo},
    },
    samples::sample_app_settings::SampleAppSettings,
    tools::{
        imgui::{
            events::imgui_handle_event,
            renderer::{ImguiRenderer, ImguiRendererCreateInfo},
        },
        native_app::{
            app::{App, GoldenImageMode},
            events::{EventHandler, EventResult},
            NativeWindow,
        },
    },
};

use super::{sample::SampleBase, sample_app_settings::parse_sample_app_settings};

pub struct SampleApp<Sample: SampleBase> {
    swap_chain: SwapChain,

    _golden_image_mode: GoldenImageMode,
    _golden_pixel_tolerance: u32,

    sample: Sample,

    current_time: f64,

    imgui_renderer: ImguiRenderer,

    app_settings: SampleAppSettings,

    graphics_adapter: Option<GraphicsAdapterInfo>,
}

enum EngineFactory {
    VULKAN(EngineFactoryVk),
}

impl AsEngineFactory for EngineFactory {
    fn as_engine_factory(&self) -> &crate::core::engine_factory::EngineFactory {
        match self {
            Self::VULKAN(factory) => factory.as_engine_factory(),
        }
    }
}

impl<GenericSample: SampleBase> SampleApp<GenericSample> {
    fn window_resize(&mut self, width: u32, height: u32) {
        self.sample.pre_window_resize();

        self.swap_chain
            .resize(width, height, SurfaceTransform::Optimal);

        let swap_chain_desc = self.swap_chain.get_desc();

        self.sample
            .window_resize(swap_chain_desc.width, swap_chain_desc.height);
    }

    fn update(&mut self, current_time: f64, elapsed_time: f64) {
        self.current_time = current_time;

        // TODO : update app settings

        self.sample.update(current_time, elapsed_time);
    }

    fn update_ui(&mut self) {
        let ui = self.imgui_renderer.new_frame();

        let swap_chain_desc = self.swap_chain.get_desc();

        let adapters_wnd_width = swap_chain_desc.width.min(330);

        if self.app_settings.show_adapters_dialog {
            if let Some(_window_token) = ui
                .window("Adapters")
                .size([adapters_wnd_width as f32, 0.0], imgui::Condition::Always)
                .position(
                    [
                        (swap_chain_desc.width as f32 - adapters_wnd_width as f32).max(10.0) - 10.0,
                        10.0,
                    ],
                    imgui::Condition::Always,
                )
                .flags(WindowFlags::NO_RESIZE)
                .collapsed(true, imgui::Condition::FirstUseEver)
                .begin()
            {
                if let Some(adapter) = &self.graphics_adapter {
                    if adapter.adapter_type != AdapterType::Unknown {
                        ui.text_disabled(format!(
                            "Adapter: {} ({} MB)",
                            adapter.description,
                            adapter.memory.local_memory >> 20
                        ));
                    }
                }

                ui.checkbox("VSync", &mut self.app_settings.vsync);
            }
        }
        self.sample.update_ui(ui);
    }

    fn render(&self) {
        let context = self.sample.get_immediate_context();
        context.clear_stats();

        let rtv = self.swap_chain.get_current_back_buffer_rtv();
        let dsv = self.swap_chain.get_depth_buffer_dsv();

        context.set_render_targets(&[&rtv], Some(&dsv), ResourceStateTransitionMode::Transition);

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
        window: Option<&NativeWindow>,
    ) -> Self {
        let swap_chain_desc = SwapChainDesc::default();

        //#[cfg(any(
        //    feature = "D3D11_SUPPORTED",
        //    feature = "D3D12_SUPPORTED",
        //    feature = "VULKAN_SUPPORTED",
        //    feature = "WEBGPU_SUPPORTED"
        //))]
        fn find_adapter(
            mut adapter_index: Option<usize>,
            adapter_type: &AdapterType,
            adapters: &[GraphicsAdapterInfo],
        ) -> Option<usize> {
            let mut adapter_type = adapter_type;

            if let Some(adap_id) = adapter_index {
                if adap_id < adapters.len() {
                    adapter_type = &adapters.get(adap_id).unwrap().adapter_type;
                } else {
                    //LOG_ERROR_MESSAGE("Adapter ID (", AdapterId, ") is invalid. Only ", Adapters.size(), " compatible ", (Adapters.size() == 1 ? "adapter" : "adapters"), " present in the system");
                    adapter_index = None;
                }
            }

            if adapter_index.is_none() && *adapter_type != AdapterType::Unknown {
                adapter_index = adapters
                    .iter()
                    .position(|adapter| adapter.adapter_type == *adapter_type)
                    .map_or(None, |id| Some(id));
            };

            if adapter_index.is_none() {
                if let Some((index, _best_adapter)) =
                    adapters
                        .iter()
                        .enumerate()
                        .max_by(|(_, &ref adapter1), (_, &ref adapter2)| {
                            // Prefer Discrete over Integrated over Software
                            let compare_type =
                                |adapter1: &GraphicsAdapterInfo, adapter2: &GraphicsAdapterInfo| {
                                    adapter1.adapter_type.cmp(&adapter2.adapter_type)
                                };

                            // Select adapter with most memory
                            let get_total_mem = |adapter: &GraphicsAdapterInfo| {
                                adapter.memory.local_memory
                                    + adapter.memory.host_visible_memory
                                    + adapter.memory.unified_memory
                            };
                            let compare_memory =
                                |adapter1: &GraphicsAdapterInfo, adapter2: &GraphicsAdapterInfo| {
                                    get_total_mem(adapter1).cmp(&get_total_mem(adapter2))
                                };

                            compare_type(&adapter1, &adapter2)
                                .then(compare_memory(&adapter1, &adapter2))
                        })
                {
                    adapter_index = Some(index);
                }
            }

            if let Some(adapter_index) = adapter_index {
                let adaper_description = &adapters.get(adapter_index).unwrap().description;
                println!("Using adapter {adapter_index}, : '{adaper_description}'");
            }

            adapter_index
        }

        let engine_factory = match app_settings.device_type {
            RenderDeviceType::D3D11 => panic!(),
            RenderDeviceType::D3D12 => panic!(),
            RenderDeviceType::GL => panic!(),
            RenderDeviceType::GLES => panic!(),
            RenderDeviceType::VULKAN => EngineFactory::VULKAN(get_engine_factory_vk()),
            RenderDeviceType::METAL => panic!(),
            RenderDeviceType::WEBGPU => panic!(),
        };

        let adapters = engine_factory
            .as_engine_factory()
            .enumerate_adapters(&engine_create_info.graphics_api_version);

        let adapter = if let Some(adapter_index) = find_adapter(
            app_settings.adapter_index,
            &app_settings.adapter_type,
            adapters.as_slice(),
        ) {
            engine_create_info.adapter_index.replace(adapter_index);
            adapters.into_iter().nth(adapter_index)
        } else {
            None
        };

        let (render_device, immediate_contexts, deferred_contexts, swap_chain) =
            match &engine_factory {
                EngineFactory::VULKAN(engine_factory) => {
                    let engine_vk_create_info = EngineVkCreateInfo::new(engine_create_info);

                    let (render_device, immediate_contexts, deferred_contexts) = engine_factory
                        .create_device_and_contexts(&engine_vk_create_info)
                        .unwrap();

                    let swap_chain = engine_factory
                        .create_swap_chain(
                            &render_device,
                            immediate_contexts.first().unwrap(),
                            &swap_chain_desc,
                            window,
                        )
                        .unwrap();

                    (
                        render_device,
                        immediate_contexts,
                        deferred_contexts,
                        swap_chain,
                    )
                }
            };

        let sample = GenericSample::new(
            engine_factory.as_engine_factory(),
            render_device,
            immediate_contexts,
            deferred_contexts,
            &swap_chain,
        );

        let swap_chain_desc = swap_chain.get_desc();

        let imgui_renderer = ImguiRenderer::new(ImguiRendererCreateInfo::new(
            sample.get_render_device(),
            swap_chain_desc.color_buffer_format,
            swap_chain_desc.depth_buffer_format,
            app_settings.width,
            app_settings.height,
        ));

        SampleApp::<GenericSample> {
            swap_chain,

            _golden_image_mode: GoldenImageMode::None,
            _golden_pixel_tolerance: 0,

            sample,

            app_settings,

            current_time: 0.0,

            imgui_renderer,

            graphics_adapter: adapter,
        }
    }

    fn run(
        mut self,
        mut event_handler: impl EventHandler,
        update_window_title_cb: Option<impl Fn(&str) -> ()>,
    ) -> Result<(), std::io::Error> {
        let start_time = std::time::Instant::now();

        let mut last_time = start_time;

        if let Some(update_window) = &update_window_title_cb {
            let app_title = String::from(GenericSample::get_name())
                + " ("
                + get_render_device_type_string(&self.app_settings.device_type, false)
                + ", API "
                + format!("{API_VERSION}").as_str()
                + ")";
            update_window(app_title.as_str());
        }

        'main: loop {
            while let Some(event) = event_handler.poll_event() {
                let event = event_handler.handle_event(&event);
                match event {
                    EventResult::Quit => break 'main,
                    EventResult::Continue => {}
                    EventResult::Resize { width, height } => {
                        self.window_resize(width as u32, height as u32)
                    }
                    _ => {}
                }

                let event = imgui_handle_event(self.imgui_renderer.io_mut(), event);

                self.sample.handle_event(event);
            }

            {
                let now = std::time::Instant::now();

                self.update(
                    now.duration_since(start_time).as_secs_f64(),
                    now.duration_since(last_time).as_secs_f64(),
                );

                last_time = now;
            }

            self.render();

            if self.app_settings.show_ui {
                self.update_ui();
                self.imgui_renderer.render(
                    self.sample.get_immediate_context(),
                    self.sample.get_render_device(),
                );
            }

            self.present();
        }

        Ok(())
    }
}
