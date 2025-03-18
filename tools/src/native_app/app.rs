use diligent::{engine_factory::EngineCreateInfo, platforms::native_window::NativeWindow};

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
    ) -> Self;

    fn run(
        self,
        event_handler: impl EventHandler,
        update_window_title_cb: impl Fn(&str),
    ) -> Result<(), std::io::Error>;
}
