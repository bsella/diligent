use std::{
    ffi::CString,
    io::{Error, ErrorKind},
};

use crate::{
    core::engine_factory::EngineCreateInfo,
    tools::native_app::{
        app::App,
        app_settings::AppSettings,
        events::{EventHandler, EventResult},
    },
};

use super::native_window::NativeWindow;

struct X11EventHandler {
    display: *mut x11::xlib::Display,
}

impl EventHandler for X11EventHandler {
    type EventType = x11::xlib::XEvent;

    fn poll_event(&self) -> Option<x11::xlib::XEvent> {
        let mut xev = std::mem::MaybeUninit::uninit();
        if unsafe { x11::xlib::XCheckMaskEvent(self.display, 0xFFFFFFFF, xev.as_mut_ptr()) != 0 } {
            Some(unsafe { xev.assume_init() })
        } else {
            None
        }
    }

    fn handle_event(&mut self, _event: &x11::xlib::XEvent) -> EventResult {
        EventResult::Continue
    }
}

pub(super) fn main<Application>(settings: Application::AppSettings) -> Result<(), std::io::Error>
where
    Application: App,
{
    let (width, height) = settings.get_window_dimensions();

    let (win, display) = unsafe {
        let display = x11::xlib::XOpenDisplay(std::ptr::null());

        #[rustfmt::skip]
        let visual_attribs =
        [
            x11::glx::GLX_RENDER_TYPE,    x11::glx::GLX_RGBA_BIT,
            x11::glx::GLX_DRAWABLE_TYPE,  x11::glx::GLX_WINDOW_BIT,
            x11::glx::GLX_DOUBLEBUFFER,   1,

            // The largest available total RGBA color buffer size (sum of GLX_RED_SIZE, 
            // GLX_GREEN_SIZE, GLX_BLUE_SIZE, and GLX_ALPHA_SIZE) of at least the minimum
            // size specified for each color component is preferred.
            x11::glx::GLX_RED_SIZE,       8,
            x11::glx::GLX_GREEN_SIZE,     8,
            x11::glx::GLX_BLUE_SIZE,      8,
            x11::glx::GLX_ALPHA_SIZE,     8,

            // The largest available depth buffer of at least GLX_DEPTH_SIZE size is preferred
            x11::glx::GLX_DEPTH_SIZE,     24,

            x11::glx::GLX_SAMPLE_BUFFERS, 0,

            // Setting GLX_SAMPLES to 1 results in 2x MS backbuffer, which is 
            // against the spec that states:
            //     if GLX SAMPLE BUFFERS is zero, then GLX SAMPLES will also be zero
            // GLX_SAMPLES, 1,

            0
        ];

        let mut fbcount = 0;

        let fbc = x11::glx::glXChooseFBConfig(
            display,
            x11::xlib::XDefaultScreen(display),
            visual_attribs.as_ptr(),
            std::ptr::addr_of_mut!(fbcount),
        );

        if fbc.is_null() {
            return Err(Error::new(
                ErrorKind::Other,
                "Failed to retrieve a framebuffer config",
            ));
        }

        let vi = x11::glx::glXGetVisualFromFBConfig(display, *fbc);

        let mut swa = x11::xlib::XSetWindowAttributes {
            colormap: x11::xlib::XCreateColormap(
                display,
                x11::xlib::XRootWindow(display, (*vi).screen),
                (*vi).visual,
                x11::xlib::AllocNone,
            ),
            border_pixel: 0,
            event_mask: x11::xlib::StructureNotifyMask
                | x11::xlib::ExposureMask
                | x11::xlib::KeyPressMask
                | x11::xlib::KeyReleaseMask
                | x11::xlib::ButtonPressMask
                | x11::xlib::ButtonReleaseMask
                | x11::xlib::PointerMotionMask,

            background_pixel: 0,
            background_pixmap: 0,
            backing_pixel: 0,
            backing_planes: 0,
            backing_store: 0,
            bit_gravity: 0,
            border_pixmap: 0,
            cursor: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            save_under: 0,
            win_gravity: 0,
        };

        let win = x11::xlib::XCreateWindow(
            display,
            x11::xlib::XRootWindow(display, (*vi).screen),
            0,
            0,
            width as u32,
            height as u32,
            0,
            (*vi).depth,
            x11::xlib::InputOutput as u32,
            (*vi).visual,
            x11::xlib::CWBorderPixel | x11::xlib::CWColormap | x11::xlib::CWEventMask,
            std::ptr::addr_of_mut!(swa),
        );

        if win == 0 {
            return Err(Error::new(ErrorKind::Other, "Failed to create window."));
        }

        {
            let size_hints = x11::xlib::XAllocSizeHints();
            (*size_hints).flags = x11::xlib::PMinSize;
            (*size_hints).min_width = 320;
            (*size_hints).min_height = 240;
            x11::xlib::XSetWMNormalHints(display, win, size_hints);
            x11::xlib::XFree(size_hints as *mut std::ffi::c_void);
        }

        x11::xlib::XMapWindow(display, win);

        let gl_x_create_context_attribs_arb = {
            // Create an oldstyle context first, to get the correct function pointer for glXCreateContextAttribsARB
            let ctx_old = x11::glx::glXCreateContext(display, vi, std::ptr::null_mut(), 1);
            let gl_x_create_context_attribs_arb =
                x11::glx::glXGetProcAddress(c"glXCreateContextAttribsARB".as_ptr() as *const u8);
            x11::glx::glXMakeCurrent(display, 0, std::ptr::null_mut());
            x11::glx::glXDestroyContext(display, ctx_old);

            gl_x_create_context_attribs_arb
        };

        if gl_x_create_context_attribs_arb.is_none() {
            return Err(Error::new(
                ErrorKind::Other,
                "glXCreateContextAttribsARB entry point not found. Aborting.",
            ));
        }

        let gl_x_create_context_attribs_arb = std::mem::transmute::<_, fn(_, _, _, _, _) -> _>(
            gl_x_create_context_attribs_arb.unwrap(),
        );

        let mut flags = x11::glx::arb::GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB;
        #[cfg(debug_assertions)]
        {
            flags |= x11::glx::arb::GLX_CONTEXT_DEBUG_BIT_ARB
        };

        let major_version = 4;
        let minor_version = 3;

        let context_attribs = [
            x11::glx::arb::GLX_CONTEXT_MAJOR_VERSION_ARB,
            major_version,
            x11::glx::arb::GLX_CONTEXT_MINOR_VERSION_ARB,
            minor_version,
            x11::glx::arb::GLX_CONTEXT_FLAGS_ARB,
            flags,
            0, //
        ];

        let ctx: x11::glx::GLXContext = gl_x_create_context_attribs_arb(
            display,
            *fbc,
            std::ptr::null::<x11::glx::GLXContext>(),
            1,
            context_attribs.as_ptr(),
        );
        if ctx.is_null() {
            return Err(Error::new(ErrorKind::Other, "Failed to create GL context."));
        }

        x11::xlib::XFree(fbc as *mut std::ffi::c_void);

        x11::glx::glXMakeCurrent(display, win, ctx);

        (win, display)
    };

    let native_window = NativeWindow::X11 {
        window_id: win as u32,
        display,
    };

    let result = Application::new(settings, EngineCreateInfo::default(), Some(&native_window)).run(
        X11EventHandler { display },
        |title| unsafe {
            let cstring = CString::new(title).unwrap();
            x11::xlib::XStoreName(display, win, cstring.as_ptr());
        },
    );

    unsafe {
        let ctx = x11::glx::glXGetCurrentContext();
        x11::glx::glXMakeCurrent(display, 0, std::ptr::null_mut());
        x11::glx::glXDestroyContext(display, ctx);
        x11::xlib::XDestroyWindow(display, win);
        x11::xlib::XCloseDisplay(display);
    }

    result
}
