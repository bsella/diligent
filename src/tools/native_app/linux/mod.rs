use super::app::App;

#[cfg(feature = "vulkan")]
mod linux_xcb;

#[cfg(feature = "vulkan")]
pub type NativeWindow = linux_xcb::NativeWindow;

#[cfg(not(feature = "vulkan"))]
mod linux_xlib;

#[cfg(feature = "vulkan")]
pub fn main<Application>(settings: Application::AppSettings) -> Result<(), std::io::Error>
where
    Application: App,
{
    linux_xcb::main::<Application>(settings)
}
