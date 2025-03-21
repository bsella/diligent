use diligent::{
    accessories::get_render_device_type_string,
    api_info::API_VERSION,
    device_context::ResourceStateTransitionMode,
    engine_factory::{AsEngineFactory, EngineCreateInfo},
    graphics_types::{AdapterType, GraphicsAdapterInfo, RenderDeviceType, SurfaceTransform},
    platforms::native_window::NativeWindow,
    swap_chain::{SwapChain, SwapChainDesc},
};

use diligent_tools::{
    imgui::{
        events::imgui_handle_event,
        renderer::{ImguiRenderer, ImguiRendererCreateInfo},
    },
    native_app::{
        app::{App, GoldenImageMode},
        events::{EventHandler, EventResult},
    },
};

use imgui::WindowFlags;

#[cfg(feature = "vulkan")]
use diligent::vk::engine_factory_vk::{get_engine_factory_vk, EngineFactoryVk, EngineVkCreateInfo};

#[cfg(feature = "opengl")]
use diligent::{
    gl::engine_factory_gl::{get_engine_factory_gl, EngineFactoryOpenGL, EngineGLCreateInfo},
    graphics_types::DeviceFeatureState,
};

use crate::sample_app_settings::SampleAppSettings;

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
    #[cfg(feature = "vulkan")]
    VULKAN(EngineFactoryVk),
    #[cfg(feature = "opengl")]
    OPENGL(EngineFactoryOpenGL),
}

impl AsEngineFactory for EngineFactory {
    fn as_engine_factory(&self) -> &diligent::engine_factory::EngineFactory {
        match self {
            #[cfg(feature = "vulkan")]
            Self::VULKAN(factory) => factory.as_engine_factory(),
            #[cfg(feature = "opengl")]
            Self::OPENGL(factory) => factory.as_engine_factory(),
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
            #[cfg(feature = "d3d11")]
            RenderDeviceType::D3D11 => panic!(),
            #[cfg(feature = "d3d12")]
            RenderDeviceType::D3D12 => panic!(),
            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => EngineFactory::OPENGL(get_engine_factory_gl()),
            //RenderDeviceType::GLES => panic!(),
            #[cfg(feature = "vulkan")]
            RenderDeviceType::VULKAN => EngineFactory::VULKAN(get_engine_factory_vk()),
            #[cfg(feature = "metal")]
            RenderDeviceType::METAL => panic!(),
            #[cfg(feature = "webgpu")]
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
                #[cfg(feature = "vulkan")]
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
                #[cfg(feature = "opengl")]
                EngineFactory::OPENGL(engine_factory) => {
                    if app_settings.non_separable_progs {
                        engine_create_info.features.separable_programs =
                            DeviceFeatureState::Disabled;
                    }

                    if engine_create_info.num_deferred_contexts != 0 {
                        panic!("Deferred contexts are not supported in OpenGL mode");
                    }

                    let engine_gl_create_info =
                        EngineGLCreateInfo::new(window.unwrap(), engine_create_info);

                    let (device, immediate_context, swap_chain) = engine_factory
                        .create_device_and_swap_chain_gl(&engine_gl_create_info, &swap_chain_desc)
                        .unwrap();

                    (device, vec![immediate_context], Vec::new(), swap_chain)
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
        update_window_title_cb: impl Fn(&str),
    ) -> Result<(), std::io::Error> {
        let start_time = std::time::Instant::now();

        let mut last_time = start_time;

        let app_title = String::from(GenericSample::get_name())
            + " ("
            + get_render_device_type_string(&self.app_settings.device_type, false)
            + ", API "
            + format!("{API_VERSION}").as_str()
            + ")";

        update_window_title_cb(app_title.as_str());

        let mut filtered_frame_time = 0.0;

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
