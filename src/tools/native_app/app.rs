use crate::{
    bindings::NativeWindow,
    core::{engine_factory::EngineCreateInfo, graphics_types::RenderDeviceType},
};

use super::events::EventHandler;

pub enum GoldenImageMode {
    None,
    Capture,
    Compare,
    CompareUpdate,
}

pub trait App {
    fn new(
        device_type: RenderDeviceType,
        engine_create_info: EngineCreateInfo,
        window: Option<&NativeWindow>,
        initial_width: u16,
        initial_height: u16,
    ) -> Self;

    fn run<EH: EventHandler>(self, event_handler: EH) -> Result<(), std::io::Error>;
}
