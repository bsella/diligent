use crate::core::{
    device_context::DeviceContext, render_device::RenderDevice, swap_chain::SwapChain,
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
            render_device,
            immediate_contexts,
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
