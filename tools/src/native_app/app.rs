use diligent::EngineCreateInfo;

use super::app_settings::AppSettings;

pub enum GoldenImageMode {
    None,
    Capture,
    Compare,
    CompareUpdate,
}

pub trait App {
    type AppSettings: AppSettings;

    fn new(app_settings: Self::AppSettings, engine_create_info: EngineCreateInfo) -> Self;

    fn run(self) -> Result<(), std::io::Error>;
}
