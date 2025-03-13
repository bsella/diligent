pub enum NativeWindow {
    #[cfg(feature = "vulkan")]
    XCB {
        window_id: u32,
        connection: *mut xcb::ffi::xcb_connection_t,
    },
    #[cfg(feature = "opengl")]
    X11 {
        window_id: u32,
        display: *mut x11::xlib::Display,
    },
}

impl From<&NativeWindow> for diligent_sys::NativeWindow {
    fn from(value: &NativeWindow) -> Self {
        match *value {
            #[cfg(feature = "vulkan")]
            NativeWindow::XCB {
                window_id,
                connection,
            } => diligent_sys::NativeWindow {
                WindowId: window_id,
                pXCBConnection: connection as *mut std::ffi::c_void,
                pDisplay: std::ptr::null_mut(),
            },
            #[cfg(feature = "opengl")]
            NativeWindow::X11 { window_id, display } => diligent_sys::NativeWindow {
                WindowId: window_id,
                pXCBConnection: std::ptr::null_mut(),
                pDisplay: display as *mut std::ffi::c_void,
            },
        }
    }
}
