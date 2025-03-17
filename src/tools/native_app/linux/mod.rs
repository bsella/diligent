use std::io::{Error, ErrorKind};

use crate::core::{accessories::get_render_device_type_string, graphics_types::RenderDeviceType};

use super::{app::App, app_settings::AppSettings};

#[cfg(feature = "vulkan")]
mod linux_xcb;

#[cfg(feature = "opengl")]
mod linux_x11;

mod native_window;

pub type NativeWindow = native_window::NativeWindow;

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
                get_render_device_type_string(device_type, false)
            ),
        )),
    }
}
