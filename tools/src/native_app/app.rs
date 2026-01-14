use diligent::{platforms::native_window::NativeWindow, EngineCreateInfo};

use crate::native_app::Window;

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
        window: NativeWindow,
    ) -> Self;

    fn run(
        self,
        event_handler: impl EventHandler,
        window: impl Window,
    ) -> Result<(), std::io::Error>;
}
