use app::App;
pub mod app;
pub mod app_settings;

pub mod events;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub type NativeWindow = linux::NativeWindow;

#[cfg(target_os = "linux")]
pub fn main<Application>() -> Result<(), std::io::Error>
where
    Application: App,
{
    let settings = Application::parse_settings_from_cli();
    linux::main::<Application>(settings)
}
