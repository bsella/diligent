use crate::{bindings::NativeWindow, core::engine_factory::EngineFactoryImplementation};

use super::events::EventHandler;

pub enum GoldenImageMode {
    None,
    Capture,
    Compare,
    CompareUpdate,
}

pub enum ApiImplementation {
    Vulkan,
    OpenGL,
}

pub trait App {
    fn new<EngineFactory: EngineFactoryImplementation>(
        engine_create_info: EngineFactory::EngineCreateInfo,
        window: Option<&NativeWindow>,
    ) -> Self;

    fn run<EH: EventHandler>(self, event_handler: EH) -> Result<(), std::io::Error>;
}
