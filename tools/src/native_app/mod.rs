use app::App;
use diligent::platforms::native_window::NativeWindow;
pub mod app;
pub mod app_settings;

pub mod events;

pub trait Window {
    fn native(&self) -> NativeWindow;

    fn set_title(&self, title: &str);
}

pub trait WindowManager {
    type Window<'manager>: Window
    where
        Self: 'manager;

    fn new() -> Self;

    fn create_window(&self, width: u32, height: u32) -> Self::Window<'_>;
}

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
