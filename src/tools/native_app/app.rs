use crate::{bindings::NativeWindow, core::engine_factory::EngineCreateInfo};

use super::{app_settings::AppSettings, events::EventHandler};

pub enum GoldenImageMode {
    None,
    Capture,
    Compare,
    CompareUpdate,
}

pub trait App {
    type AppSettings: AppSettings;

    fn parse_settings_from_cli() -> Self::AppSettings;

    fn new(
        app_settings: Self::AppSettings,
        engine_create_info: EngineCreateInfo,
        window: Option<&NativeWindow>,
        initial_width: u16,
        initial_height: u16,
    ) -> Self;

    fn run<EH: EventHandler>(self, event_handler: EH) -> Result<(), std::io::Error>;
}
