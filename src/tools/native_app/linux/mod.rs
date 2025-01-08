use super::app::App;

mod linux_app;

#[cfg(feature = "VULKAN_SUPPORTED")]
mod linux_xcb;

#[cfg(not(feature = "VULKAN_SUPPORTED"))]
mod linux_xlib;

#[cfg(feature = "VULKAN_SUPPORTED")]
pub fn main<Application>() -> Result<(), std::io::Error>
where
    Application: App,
{
    linux_xcb::main::<Application>()
}
