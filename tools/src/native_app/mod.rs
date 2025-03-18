use app::App;
pub mod app;
pub mod app_settings;

pub mod events;

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

pub fn main<Application>() -> Result<(), std::io::Error>
where
    Application: App,
{
    let settings = Application::parse_settings_from_cli();
    platform::main::<Application>(settings)
}
