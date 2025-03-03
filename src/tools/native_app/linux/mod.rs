use super::app::App;

#[cfg(feature = "VULKAN_SUPPORTED")]
mod linux_xcb;

#[cfg(not(feature = "VULKAN_SUPPORTED"))]
mod linux_xlib;

#[cfg(feature = "VULKAN_SUPPORTED")]
pub fn main<Application>(settings: Application::AppSettings) -> Result<(), std::io::Error>
where
    Application: App,
{
    linux_xcb::main::<Application>(settings)
}
