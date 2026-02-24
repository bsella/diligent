#[cfg(target_os = "windows")]
use std::os::raw::c_void;

#[repr(transparent)]
pub struct NativeWindow(pub(crate) diligent_sys::NativeWindow);

impl NativeWindow {
    #[cfg(target_os = "linux")]
    pub fn new(
        window_id: u32,
        x_display: *mut (),
        xcb_connection: *mut (),
        wayland_surface: *mut (),
    ) -> Self {
        Self(diligent_sys::NativeWindow {
            WindowId: window_id,
            pDisplay: x_display as _,
            pXCBConnection: xcb_connection as _,
            pWaylandSurface: wayland_surface as _,
        })
    }

    #[cfg(target_os = "windows")]
    pub fn new(hwnd: *mut c_void) -> Self {
        Self(diligent_sys::NativeWindow { hWnd: hwnd })
    }
}
