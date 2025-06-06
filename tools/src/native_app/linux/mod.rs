use diligent::graphics_types::RenderDeviceType;
use std::io::{Error, ErrorKind};

use super::{app::App, app_settings::AppSettings};

#[cfg(feature = "vulkan")]
mod linux_xcb;

#[cfg(feature = "opengl")]
mod linux_x11;

pub fn main<Application>(settings: Application::AppSettings) -> Result<(), std::io::Error>
where
    Application: App,
{
    let device_type = settings.get_render_device_type();
    match device_type {
        #[cfg(feature = "vulkan")]
        RenderDeviceType::VULKAN => linux_xcb::main::<Application>(settings),
        #[cfg(feature = "opengl")]
        RenderDeviceType::GL => linux_x11::main::<Application>(settings),
        #[allow(unreachable_patterns)]
        _ => Err(Error::new(
            ErrorKind::Other,
            format!(
                "Render device type {} is not available on linux",
                device_type.to_string()
            ),
        )),
    }
}
