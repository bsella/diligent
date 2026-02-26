use diligent::platforms::native_window::NativeWindow;

use crate::native_app::events::Event;

pub mod events;

pub trait Window {
    fn native(&self) -> NativeWindow;

    fn set_title(&self, title: &str);

    fn handle_event(&mut self) -> Option<Event>;
}

pub trait WindowManager {
    fn create_window(width: u32, height: u32) -> Box<dyn Window>;
}

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
pub mod windows;
