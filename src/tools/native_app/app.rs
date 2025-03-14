use crate::core::engine_factory::EngineCreateInfo;

use super::{app_settings::AppSettings, events::EventHandler, NativeWindow};

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
    ) -> Self;

    fn run(
        self,
        event_handler: impl EventHandler,
        update_window_title_cb: impl Fn(&str),
    ) -> Result<(), std::io::Error>;
}
