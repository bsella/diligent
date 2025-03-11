use super::app::App;

mod linux_xcb;

pub type NativeWindow = linux_xcb::NativeWindow;

pub fn main<Application>(settings: Application::AppSettings) -> Result<(), std::io::Error>
where
    Application: App,
{
    linux_xcb::main::<Application>(settings)
}
