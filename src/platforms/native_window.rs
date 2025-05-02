#[cfg(target_os = "windows")]
pub struct NativeWindow(pub *mut std::os::raw::c_void);

#[cfg(target_os = "windows")]
impl From<&NativeWindow> for diligent_sys::NativeWindow {
    fn from(value: &NativeWindow) -> Self {
        diligent_sys::NativeWindow { hWnd: value.0 }
    }
}

#[derive(Clone, Copy)]
#[cfg(target_os = "linux")]
pub enum NativeWindow {
    #[cfg(feature = "vulkan")]
    XCB {
        window_id: u32,
        connection: *mut std::ffi::c_void,
    },
    #[cfg(feature = "opengl")]
    X11 {
        window_id: u32,
        display: *mut std::ffi::c_void,
    },
}

#[cfg(target_os = "linux")]
impl From<&NativeWindow> for diligent_sys::NativeWindow {
    fn from(value: &NativeWindow) -> Self {
        match *value {
            #[cfg(feature = "vulkan")]
            NativeWindow::XCB {
                window_id,
                connection,
            } => diligent_sys::NativeWindow {
                WindowId: window_id,
                pXCBConnection: connection,
                pDisplay: std::ptr::null_mut(),
            },
            #[cfg(feature = "opengl")]
            NativeWindow::X11 { window_id, display } => diligent_sys::NativeWindow {
                WindowId: window_id,
                pXCBConnection: std::ptr::null_mut(),
                pDisplay: display,
            },
        }
    }
}
