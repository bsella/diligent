use crate::{
    core::{device_context::DeviceContext, render_device::RenderDevice, swap_chain::SwapChain},
    tools::native_app::events::EventResult,
};

pub trait SampleBase {
    fn new(
        render_device: RenderDevice,
        immediate_contexts: Vec<DeviceContext>,
        deferred_contexts: Vec<DeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self;

    fn get_render_device(&self) -> &RenderDevice;

    fn get_immediate_context(&self) -> &DeviceContext;

    fn render(&self, _swap_chain: &SwapChain) {}
    fn update(&self, _current_time: f64, _elapsed_time: f64) {}
    fn get_name() -> &'static str;
    fn pre_window_resize(&mut self) {}
    fn window_resize(&mut self, _width: u32, _height: u32) {}

    fn handle_event(&mut self, _event: EventResult) {}
}
