use crate::{
    bindings::{self, NativeWindow},
    core::{
        device_context::{DeviceContext, ResourceStateTransitionMode},
        engine_factory::EngineFactoryImplementation,
        render_device::RenderDevice,
        swap_chain::SwapChain,
    },
    tools::native_app::app::{App, GoldenImageMode},
};

pub struct Sample {
    render_device: RenderDevice,
    immediate_contexts: Vec<DeviceContext>,
    _deferred_contexts: Vec<DeviceContext>,
}

pub trait SampleBase {
    fn new(
        render_device: RenderDevice,
        immediate_contexts: Vec<DeviceContext>,
        deferred_contexts: Vec<DeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self;

    fn get_render_device(&self) -> &RenderDevice;

    fn get_immediate_context(&self) -> &DeviceContext;

    fn render(&self, swap_chain: &SwapChain);
    fn update(&self, _current_time: f64, _elapsed_time: f64);
    fn get_name() -> &'static str;
    fn pre_window_resize(&mut self);
    fn window_resize(&mut self, _width: u32, _height: u32);
}

impl SampleBase for Sample {
    fn new(
        render_device: RenderDevice,
        immediate_contexts: Vec<DeviceContext>,
        deferred_contexts: Vec<DeviceContext>,
        _swap_chain: &SwapChain,
    ) -> Self {
        Sample {
            render_device: render_device,
            immediate_contexts: immediate_contexts,
            _deferred_contexts: deferred_contexts,
        }
    }

    fn get_render_device(&self) -> &RenderDevice {
        &self.render_device
    }

    fn get_immediate_context(&self) -> &DeviceContext {
        self.immediate_contexts.first().unwrap()
    }

    fn render(&self, _swap_chain: &SwapChain) {}
    fn update(&self, _current_time: f64, _elapsed_time: f64) {}
    fn get_name() -> &'static str {
        ""
    }
    fn pre_window_resize(&mut self) {}
    fn window_resize(&mut self, _width: u32, _height: u32) {}
}

pub struct SampleApp<Sample: SampleBase> {
    app_title: String,
    swap_chain: SwapChain,

    _golden_image_mode: GoldenImageMode,
    _golden_pixel_tolerance: u32,

    sample: Sample,

    vsync: bool,

    current_time: f64,
}

impl<GenericSample: SampleBase> App for SampleApp<GenericSample> {
    fn new<EngineFactory: EngineFactoryImplementation>(
        engine_create_info: EngineFactory::EngineCreateInfo,
        window: Option<&NativeWindow>,
    ) -> Self {
        let swap_chain_desc = bindings::SwapChainDesc::default();

        let engine_factory = EngineFactory::get();

        let (render_device, immediate_contexts, deferred_contexts) = engine_factory
            .create_device_and_contexts(engine_create_info)
            .unwrap();

        let swap_chain = engine_factory
            .create_swap_chain(
                &render_device,
                immediate_contexts.first().unwrap(),
                &swap_chain_desc,
                window,
            )
            .unwrap();

        let sample = GenericSample::new(
            render_device,
            immediate_contexts,
            deferred_contexts,
            &swap_chain,
        );

        SampleApp::<GenericSample> {
            app_title: GenericSample::get_name().to_string(),
            swap_chain: swap_chain,

            _golden_image_mode: GoldenImageMode::None,
            _golden_pixel_tolerance: 0,

            sample: sample,

            vsync: false,

            current_time: 0.0,
        }
    }

    fn get_title(&self) -> &str {
        self.app_title.as_str()
    }

    fn update(&mut self, current_time: f64, elapsed_time: f64) {
        self.current_time = current_time;

        // TODO : update app settings

        // TODO Imgui

        self.sample.update(current_time, elapsed_time);
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

        // TODO Imgui
    }

    fn present(&mut self) {
        // TODO screen capture

        self.swap_chain.present(if self.vsync { 1 } else { 0 });

        // TODO screen capture
    }

    fn window_resize(&mut self, width: u32, height: u32) {
        self.sample.pre_window_resize();

        self.swap_chain.resize(width, height, None);

        let swap_chain_desc = self.swap_chain.get_desc();

        self.sample
            .window_resize(swap_chain_desc.Width, swap_chain_desc.Height);
    }
}
