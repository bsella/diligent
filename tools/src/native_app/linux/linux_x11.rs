use std::{
    ffi::CString,
    io::{Error, ErrorKind},
};

use crate::native_app::{
    app::App,
    app_settings::AppSettings,
    events::{Event, EventHandler, Key, MouseButton},
};

use diligent::{platforms::native_window::NativeWindow, EngineCreateInfo};
use x11::{keysym::*, xlib::*};

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

    fn handle_event(&mut self, event: &x11::xlib::XEvent) -> Event {
        match unsafe { event.type_ } {
            x11::xlib::ConfigureNotify => {
                let xce = unsafe { &event.configure };

                if xce.width != 0 && xce.height != 0 {
                    Event::Resize {
                        width: xce.width as u16,
                        height: xce.height as u16,
                    }
                } else {
                    Event::Continue
                }
            }

            x11::xlib::ButtonPress => {
                match unsafe { event.button.button } {
                    x11::xlib::Button1 => Event::MouseDown {
                        button: MouseButton::Left,
                    },

                    x11::xlib::Button2 => Event::MouseDown {
                        button: MouseButton::Middle,
                    },

                    x11::xlib::Button3 => Event::MouseDown {
                        button: MouseButton::Right,
                    },

                    x11::xlib::Button4 => Event::MouseWheel { up: true },

                    x11::xlib::Button5 => Event::MouseWheel { up: false },

                    // Unknown mouse button ?
                    _ => Event::MouseDown {
                        button: MouseButton::Left,
                    },
                }
            }

            x11::xlib::ButtonRelease => {
                match unsafe { event.button.button } {
                    x11::xlib::Button1 => Event::MouseUp {
                        button: MouseButton::Left,
                    },

                    x11::xlib::Button2 => Event::MouseUp {
                        button: MouseButton::Middle,
                    },

                    x11::xlib::Button3 => Event::MouseUp {
                        button: MouseButton::Right,
                    },

                    x11::xlib::Button4 => Event::MouseWheel { up: true },

                    x11::xlib::Button5 => Event::MouseWheel { up: false },

                    // Unknown mouse button ?
                    _ => Event::MouseUp {
                        button: MouseButton::Left,
                    },
                }
            }

            x11::xlib::MotionNotify => {
                let xme = unsafe { &event.motion };

                Event::MouseMove {
                    x: xme.x as i16,
                    y: xme.y as i16,
                }
            }

            x11::xlib::KeymapNotify => {
                let mapping_ptr = std::ptr::from_ref(unsafe { &event.mapping });
                unsafe {
                    XRefreshKeyboardMapping(mapping_ptr as _);
                }
                Event::Continue
            }
            x11::xlib::KeyPress => {
                let keysum = unsafe { XKeycodeToKeysym(self.display, event.key.keycode as _, 0) };

                #[allow(non_upper_case_globals)]
                match keysum as _ {
                    XK_a => Event::KeyPress(Key::A),
                    XK_b => Event::KeyPress(Key::B),
                    XK_c => Event::KeyPress(Key::C),
                    XK_d => Event::KeyPress(Key::D),
                    XK_e => Event::KeyPress(Key::E),
                    XK_f => Event::KeyPress(Key::F),
                    XK_g => Event::KeyPress(Key::G),
                    XK_h => Event::KeyPress(Key::H),
                    XK_i => Event::KeyPress(Key::I),
                    XK_j => Event::KeyPress(Key::J),
                    XK_k => Event::KeyPress(Key::K),
                    XK_l => Event::KeyPress(Key::L),
                    XK_m => Event::KeyPress(Key::M),
                    XK_n => Event::KeyPress(Key::N),
                    XK_o => Event::KeyPress(Key::O),
                    XK_p => Event::KeyPress(Key::P),
                    XK_q => Event::KeyPress(Key::Q),
                    XK_r => Event::KeyPress(Key::R),
                    XK_s => Event::KeyPress(Key::S),
                    XK_t => Event::KeyPress(Key::T),
                    XK_u => Event::KeyPress(Key::U),
                    XK_v => Event::KeyPress(Key::V),
                    XK_w => Event::KeyPress(Key::W),
                    XK_x => Event::KeyPress(Key::X),
                    XK_y => Event::KeyPress(Key::Y),
                    XK_z => Event::KeyPress(Key::Z),
                    XK_1 => Event::KeyPress(Key::_1),
                    XK_2 => Event::KeyPress(Key::_2),
                    XK_3 => Event::KeyPress(Key::_3),
                    XK_4 => Event::KeyPress(Key::_4),
                    XK_5 => Event::KeyPress(Key::_5),
                    XK_6 => Event::KeyPress(Key::_6),
                    XK_7 => Event::KeyPress(Key::_7),
                    XK_8 => Event::KeyPress(Key::_8),
                    XK_9 => Event::KeyPress(Key::_9),
                    XK_0 => Event::KeyPress(Key::_0),
                    XK_minus => Event::KeyPress(Key::Minus),
                    XK_equal => Event::KeyPress(Key::Equals),
                    XK_braceleft => Event::KeyPress(Key::LeftBrace),
                    XK_braceright => Event::KeyPress(Key::RightBrace),
                    XK_comma => Event::KeyPress(Key::Comma),
                    XK_period => Event::KeyPress(Key::Period),
                    XK_slash => Event::KeyPress(Key::Slash),
                    XK_semicolon => Event::KeyPress(Key::Semicolon),
                    XK_quotedbl => Event::KeyPress(Key::Apostrophe),
                    XK_asterisk => Event::KeyPress(Key::KeypadMultiply),
                    XK_backslash => Event::KeyPress(Key::Backslash),
                    XK_Return => Event::KeyPress(Key::Enter),
                    XK_BackSpace => Event::KeyPress(Key::Backspace),
                    XK_uparrow => Event::KeyPress(Key::Up),
                    XK_downarrow => Event::KeyPress(Key::Down),
                    XK_leftarrow => Event::KeyPress(Key::Left),
                    XK_rightarrow => Event::KeyPress(Key::Right),
                    XK_Control_L => Event::KeyPress(Key::LeftCtrl),
                    XK_Alt_L => Event::KeyPress(Key::LeftAlt),
                    XK_Shift_L => Event::KeyPress(Key::LeftShift),
                    XK_Super_L => Event::KeyPress(Key::LeftSuper),
                    XK_Control_R => Event::KeyPress(Key::RightCtrl),
                    XK_Alt_R => Event::KeyPress(Key::RightAlt),
                    XK_Shift_R => Event::KeyPress(Key::RightShift),
                    XK_Super_R => Event::KeyPress(Key::RightSuper),
                    XK_Insert => Event::KeyPress(Key::Insert),
                    XK_Delete => Event::KeyPress(Key::Delete),
                    XK_Home => Event::KeyPress(Key::Home),
                    XK_End => Event::KeyPress(Key::End),
                    XK_Page_Up => Event::KeyPress(Key::PageUp),
                    XK_Page_Down => Event::KeyPress(Key::PageDown),
                    XK_F1 => Event::KeyPress(Key::F1),
                    XK_F2 => Event::KeyPress(Key::F2),
                    XK_F3 => Event::KeyPress(Key::F3),
                    XK_F4 => Event::KeyPress(Key::F4),
                    XK_F5 => Event::KeyPress(Key::F5),
                    XK_F6 => Event::KeyPress(Key::F6),
                    XK_F7 => Event::KeyPress(Key::F7),
                    XK_F8 => Event::KeyPress(Key::F8),
                    XK_F9 => Event::KeyPress(Key::F9),
                    XK_F10 => Event::KeyPress(Key::F10),
                    XK_F11 => Event::KeyPress(Key::F11),
                    XK_F12 => Event::KeyPress(Key::F12),
                    XK_Escape => Event::KeyPress(Key::Esc),
                    XK_Tab => Event::KeyPress(Key::Tab),
                    XK_space => Event::KeyPress(Key::Space),

                    _ => Event::KeyPress(Key::Unknown),
                }
            }
            x11::xlib::KeyRelease => {
                let keysum = unsafe { XKeycodeToKeysym(self.display, event.key.keycode as _, 0) };

                #[allow(non_upper_case_globals)]
                match keysum as _ {
                    XK_a => Event::KeyRelease(Key::A),
                    XK_b => Event::KeyRelease(Key::B),
                    XK_c => Event::KeyRelease(Key::C),
                    XK_d => Event::KeyRelease(Key::D),
                    XK_e => Event::KeyRelease(Key::E),
                    XK_f => Event::KeyRelease(Key::F),
                    XK_g => Event::KeyRelease(Key::G),
                    XK_h => Event::KeyRelease(Key::H),
                    XK_i => Event::KeyRelease(Key::I),
                    XK_j => Event::KeyRelease(Key::J),
                    XK_k => Event::KeyRelease(Key::K),
                    XK_l => Event::KeyRelease(Key::L),
                    XK_m => Event::KeyRelease(Key::M),
                    XK_n => Event::KeyRelease(Key::N),
                    XK_o => Event::KeyRelease(Key::O),
                    XK_p => Event::KeyRelease(Key::P),
                    XK_q => Event::KeyRelease(Key::Q),
                    XK_r => Event::KeyRelease(Key::R),
                    XK_s => Event::KeyRelease(Key::S),
                    XK_t => Event::KeyRelease(Key::T),
                    XK_u => Event::KeyRelease(Key::U),
                    XK_v => Event::KeyRelease(Key::V),
                    XK_w => Event::KeyRelease(Key::W),
                    XK_x => Event::KeyRelease(Key::X),
                    XK_y => Event::KeyRelease(Key::Y),
                    XK_z => Event::KeyRelease(Key::Z),
                    XK_1 => Event::KeyRelease(Key::_1),
                    XK_2 => Event::KeyRelease(Key::_2),
                    XK_3 => Event::KeyRelease(Key::_3),
                    XK_4 => Event::KeyRelease(Key::_4),
                    XK_5 => Event::KeyRelease(Key::_5),
                    XK_6 => Event::KeyRelease(Key::_6),
                    XK_7 => Event::KeyRelease(Key::_7),
                    XK_8 => Event::KeyRelease(Key::_8),
                    XK_9 => Event::KeyRelease(Key::_9),
                    XK_0 => Event::KeyRelease(Key::_0),
                    XK_minus => Event::KeyRelease(Key::Minus),
                    XK_equal => Event::KeyRelease(Key::Equals),
                    XK_braceleft => Event::KeyRelease(Key::LeftBrace),
                    XK_braceright => Event::KeyRelease(Key::RightBrace),
                    XK_comma => Event::KeyRelease(Key::Comma),
                    XK_period => Event::KeyRelease(Key::Period),
                    XK_slash => Event::KeyRelease(Key::Slash),
                    XK_semicolon => Event::KeyRelease(Key::Semicolon),
                    XK_quotedbl => Event::KeyRelease(Key::Apostrophe),
                    XK_asterisk => Event::KeyRelease(Key::KeypadMultiply),
                    XK_backslash => Event::KeyRelease(Key::Backslash),
                    XK_Return => Event::KeyRelease(Key::Enter),
                    XK_BackSpace => Event::KeyRelease(Key::Backspace),
                    XK_uparrow => Event::KeyRelease(Key::Up),
                    XK_downarrow => Event::KeyRelease(Key::Down),
                    XK_leftarrow => Event::KeyRelease(Key::Left),
                    XK_rightarrow => Event::KeyRelease(Key::Right),
                    XK_Control_L => Event::KeyRelease(Key::LeftCtrl),
                    XK_Alt_L => Event::KeyRelease(Key::LeftAlt),
                    XK_Shift_L => Event::KeyRelease(Key::LeftShift),
                    XK_Super_L => Event::KeyRelease(Key::LeftSuper),
                    XK_Control_R => Event::KeyRelease(Key::RightCtrl),
                    XK_Alt_R => Event::KeyRelease(Key::RightAlt),
                    XK_Shift_R => Event::KeyRelease(Key::RightShift),
                    XK_Super_R => Event::KeyRelease(Key::RightSuper),
                    XK_Insert => Event::KeyRelease(Key::Insert),
                    XK_Delete => Event::KeyRelease(Key::Delete),
                    XK_Home => Event::KeyRelease(Key::Home),
                    XK_End => Event::KeyRelease(Key::End),
                    XK_Page_Up => Event::KeyRelease(Key::PageUp),
                    XK_Page_Down => Event::KeyRelease(Key::PageDown),
                    XK_F1 => Event::KeyRelease(Key::F1),
                    XK_F2 => Event::KeyRelease(Key::F2),
                    XK_F3 => Event::KeyRelease(Key::F3),
                    XK_F4 => Event::KeyRelease(Key::F4),
                    XK_F5 => Event::KeyRelease(Key::F5),
                    XK_F6 => Event::KeyRelease(Key::F6),
                    XK_F7 => Event::KeyRelease(Key::F7),
                    XK_F8 => Event::KeyRelease(Key::F8),
                    XK_F9 => Event::KeyRelease(Key::F9),
                    XK_F10 => Event::KeyRelease(Key::F10),
                    XK_F11 => Event::KeyRelease(Key::F11),
                    XK_F12 => Event::KeyRelease(Key::F12),
                    XK_Escape => Event::KeyRelease(Key::Esc),
                    XK_Tab => Event::KeyRelease(Key::Tab),
                    XK_space => Event::KeyRelease(Key::Space),

                    _ => Event::KeyRelease(Key::Unknown),
                }
            }

            _ => Event::Continue,
        }
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

    let native_window = NativeWindow::new(win as u32, display as _, std::ptr::null_mut());

    let result = Application::new(settings, EngineCreateInfo::default(), native_window).run(
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
