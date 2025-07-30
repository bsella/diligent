use diligent::{engine_factory::EngineCreateInfo, platforms::native_window::NativeWindow};

use xcb::{x, Xid};
use xkbcommon::xkb;

use crate::native_app::{
    app::App,
    app_settings::AppSettings,
    events::{Event, EventHandler, Key, MouseButton},
};

fn init_connection_and_window(
    width: u16,
    height: u16,
) -> xcb::Result<(xcb::Connection, x::Window, x::Atom)> {
    let (connection, screen_number) =
        xcb::Connection::connect(None).expect("Unable to make an XCB connection");

    let setup = connection.get_setup();

    let screen = setup.roots().nth(screen_number as usize).unwrap();

    let window = connection.generate_id();

    connection.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width,
        height,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &[
            x::Cw::BackPixel(screen.black_pixel()),
            x::Cw::EventMask(
                x::EventMask::KEY_RELEASE
                    | x::EventMask::KEY_PRESS
                    | x::EventMask::EXPOSURE
                    | x::EventMask::STRUCTURE_NOTIFY
                    | x::EventMask::POINTER_MOTION
                    | x::EventMask::BUTTON_PRESS
                    | x::EventMask::BUTTON_RELEASE,
            ),
        ],
    });

    let cookies = (
        connection.send_request(&x::InternAtom {
            only_if_exists: true,
            name: b"WM_PROTOCOLS",
        }),
        connection.send_request(&x::InternAtom {
            only_if_exists: false,
            name: b"WM_DELETE_WINDOW",
        }),
    );

    let atom_wm_delete_window = connection.wait_for_reply(cookies.1)?.atom();
    let atom_wm_protocols = connection.wait_for_reply(cookies.0)?.atom();

    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: atom_wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[atom_wm_delete_window],
    });

    // TODO : set the XCB_ATOM_WM_NORMAL_HINTS for minimal window size

    connection.send_request(&x::MapWindow { window });

    // Force the x/y coordinates to 100,100 results are identical in consecutive runs
    connection.send_request(&x::ConfigureWindow {
        window,
        value_list: &[x::ConfigWindow::X(100), x::ConfigWindow::Y(100)],
    });

    connection.flush()?;

    loop {
        if let Ok(xcb::Event::X(x::Event::Expose(_event))) = connection.wait_for_event() {
            break;
        }
    }

    Ok((connection, window, atom_wm_delete_window))
}

struct XcbEventHandler<'a> {
    connection: &'a xcb::Connection,
    atom_delete_window: xcb::x::Atom,
    keyboard_state: xkb::State,
}

impl<'a> XcbEventHandler<'a> {
    fn new(
        connection: &'a xcb::Connection,
        atom_delete_window: xcb::x::Atom,
        keyboard_state: xkb::State,
    ) -> Self {
        XcbEventHandler {
            connection,
            atom_delete_window,
            keyboard_state,
        }
    }
}

impl<'a> EventHandler for XcbEventHandler<'a> {
    type EventType = xcb::Event;

    fn poll_event(&self) -> Option<xcb::Event> {
        self.connection.poll_for_event().unwrap()
    }

    fn handle_event(&mut self, event: &xcb::Event) -> Event {
        fn keysym_to_key(keysym: xcb::x::Keysym) -> Key {
            match keysym {
                xkb::keysyms::KEY_a => Key::A,
                xkb::keysyms::KEY_b => Key::B,
                xkb::keysyms::KEY_c => Key::C,
                xkb::keysyms::KEY_d => Key::D,
                xkb::keysyms::KEY_e => Key::E,
                xkb::keysyms::KEY_f => Key::F,
                xkb::keysyms::KEY_g => Key::G,
                xkb::keysyms::KEY_h => Key::H,
                xkb::keysyms::KEY_i => Key::I,
                xkb::keysyms::KEY_j => Key::J,
                xkb::keysyms::KEY_k => Key::K,
                xkb::keysyms::KEY_l => Key::L,
                xkb::keysyms::KEY_m => Key::M,
                xkb::keysyms::KEY_n => Key::N,
                xkb::keysyms::KEY_o => Key::O,
                xkb::keysyms::KEY_p => Key::P,
                xkb::keysyms::KEY_q => Key::Q,
                xkb::keysyms::KEY_r => Key::R,
                xkb::keysyms::KEY_s => Key::S,
                xkb::keysyms::KEY_t => Key::T,
                xkb::keysyms::KEY_u => Key::U,
                xkb::keysyms::KEY_v => Key::V,
                xkb::keysyms::KEY_w => Key::W,
                xkb::keysyms::KEY_x => Key::X,
                xkb::keysyms::KEY_y => Key::Y,
                xkb::keysyms::KEY_z => Key::Z,
                xkb::keysyms::KEY_1 => Key::_1,
                xkb::keysyms::KEY_2 => Key::_2,
                xkb::keysyms::KEY_3 => Key::_3,
                xkb::keysyms::KEY_4 => Key::_4,
                xkb::keysyms::KEY_5 => Key::_5,
                xkb::keysyms::KEY_6 => Key::_6,
                xkb::keysyms::KEY_7 => Key::_7,
                xkb::keysyms::KEY_8 => Key::_8,
                xkb::keysyms::KEY_9 => Key::_9,
                xkb::keysyms::KEY_0 => Key::_0,
                xkb::keysyms::KEY_minus => Key::Minus,
                xkb::keysyms::KEY_equal => Key::Equals,
                xkb::keysyms::KEY_braceleft => Key::LeftBrace,
                xkb::keysyms::KEY_braceright => Key::RightBrace,
                xkb::keysyms::KEY_comma => Key::Comma,
                xkb::keysyms::KEY_period => Key::Period,
                xkb::keysyms::KEY_slash => Key::Slash,
                xkb::keysyms::KEY_semicolon => Key::Semicolon,
                xkb::keysyms::KEY_quotedbl => Key::Quote,
                xkb::keysyms::KEY_asterisk => Key::Asterisk,
                xkb::keysyms::KEY_backslash => Key::Backslash,
                xkb::keysyms::KEY_Return => Key::Enter,
                xkb::keysyms::KEY_BackSpace => Key::Backspace,
                xkb::keysyms::KEY_uparrow => Key::Up,
                xkb::keysyms::KEY_downarrow => Key::Down,
                xkb::keysyms::KEY_leftarrow => Key::Left,
                xkb::keysyms::KEY_rightarrow => Key::Right,
                xkb::keysyms::KEY_Control_L => Key::LeftCtrl,
                xkb::keysyms::KEY_Alt_L => Key::LeftAlt,
                xkb::keysyms::KEY_Shift_L => Key::LeftShift,
                xkb::keysyms::KEY_Super_L => Key::LeftSuper,
                xkb::keysyms::KEY_Control_R => Key::RightCtrl,
                xkb::keysyms::KEY_Alt_R => Key::RightAlt,
                xkb::keysyms::KEY_Shift_R => Key::RightShift,
                xkb::keysyms::KEY_Super_R => Key::RightSuper,
                xkb::keysyms::KEY_Insert => Key::Insert,
                xkb::keysyms::KEY_Delete => Key::Delete,
                xkb::keysyms::KEY_Home => Key::Home,
                xkb::keysyms::KEY_End => Key::End,
                xkb::keysyms::KEY_Page_Up => Key::PageUp,
                xkb::keysyms::KEY_Page_Down => Key::PageDown,
                xkb::keysyms::KEY_F1 => Key::F1,
                xkb::keysyms::KEY_F2 => Key::F2,
                xkb::keysyms::KEY_F3 => Key::F3,
                xkb::keysyms::KEY_F4 => Key::F4,
                xkb::keysyms::KEY_F5 => Key::F5,
                xkb::keysyms::KEY_F6 => Key::F6,
                xkb::keysyms::KEY_F7 => Key::F7,
                xkb::keysyms::KEY_F8 => Key::F8,
                xkb::keysyms::KEY_F9 => Key::F9,
                xkb::keysyms::KEY_F10 => Key::F10,
                xkb::keysyms::KEY_F11 => Key::F11,
                xkb::keysyms::KEY_F12 => Key::F12,
                xkb::keysyms::KEY_Escape => Key::Esc,
                xkb::keysyms::KEY_Tab => Key::Tab,
                xkb::keysyms::KEY_space => Key::Space,

                _ => Key::Unknown,
            }
        }

        match event {
            xcb::Event::X(x::Event::ClientMessage(message_event)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = message_event.data() {
                    if atom == self.atom_delete_window.resource_id() {
                        return Event::Quit;
                    }
                }
                Event::Continue
            }

            xcb::Event::X(x::Event::KeyPress(key_event)) => {
                let keysym = self
                    .keyboard_state
                    .key_get_one_sym(xkb::Keycode::new(key_event.detail() as _));

                Event::KeyPress(keysym_to_key(keysym.raw()))
            }

            xcb::Event::X(x::Event::KeyRelease(key_event)) => {
                let keysym = self
                    .keyboard_state
                    .key_get_one_sym(xkb::Keycode::new(key_event.detail() as _));

                if let Ok(Some(xcb::Event::X(x::Event::KeyPress(next_event)))) =
                    self.connection.poll_for_queued_event()
                {
                    if next_event.time() == key_event.time()
                        && next_event.detail() == key_event.detail()
                    {
                        return Event::Continue;
                    }
                }

                Event::KeyRelease(keysym_to_key(keysym.raw()))
            }

            xcb::Event::X(x::Event::DestroyNotify(_destroy_event)) => Event::Quit,

            xcb::Event::X(x::Event::ConfigureNotify(configure_event)) => Event::Resize {
                width: configure_event.width(),
                height: configure_event.height(),
            },

            xcb::Event::X(xcb::x::Event::MotionNotify(motion_event)) => Event::MouseMove {
                x: motion_event.event_x(),
                y: motion_event.event_y(),
            },

            xcb::Event::X(xcb::x::Event::ButtonPress(press_event)) => match press_event.detail() {
                1 => Event::MouseDown {
                    button: MouseButton::Left,
                },
                2 => Event::MouseDown {
                    button: MouseButton::Right,
                },
                3 => Event::MouseDown {
                    button: MouseButton::Middle,
                },
                4 => Event::MouseWheel { up: true },
                5 => Event::MouseWheel { up: false },
                _ => Event::Continue,
            },

            xcb::Event::X(xcb::x::Event::ButtonRelease(release_event)) => {
                match release_event.detail() {
                    1 => Event::MouseUp {
                        button: MouseButton::Left,
                    },
                    2 => Event::MouseUp {
                        button: MouseButton::Right,
                    },
                    3 => Event::MouseUp {
                        button: MouseButton::Middle,
                    },
                    _ => Event::Continue,
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

    let (connection, window, atom_delete_window) =
        init_connection_and_window(width, height).unwrap();

    let native_window = NativeWindow::XCB {
        window_id: window.resource_id(),
        connection: connection.get_raw_conn() as _,
    };

    let app = Application::new(settings, EngineCreateInfo::default(), Some(&native_window));

    connection.flush().unwrap();

    let update_window_title = |title: &str| {
        connection.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: title.as_bytes(),
        });
        connection.flush().unwrap();
    };

    {
        let mut _dummyu16 = 0u16;
        let mut _dummyyu16 = 0u16;
        let mut _dummyu8 = 0u8;
        let mut _dummyyu8 = 0u8;

        xkb::x11::setup_xkb_extension(
            &connection,
            1,
            0,
            xkb::x11::SetupXkbExtensionFlags::NoFlags,
            &mut _dummyu16,
            &mut _dummyyu16,
            &mut _dummyu8,
            &mut _dummyyu8,
        );
    }

    let xkb_context = xkb::Context::new(xkb::COMPILE_NO_FLAGS);

    let kb_device_id = xkb::x11::get_core_keyboard_device_id(&connection);

    let keymap = xkb::x11::keymap_new_from_device(
        &xkb_context,
        &connection,
        kb_device_id,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    );

    let state = xkb::x11::state_new_from_device(&keymap, &connection, kb_device_id);

    app.run(
        XcbEventHandler::new(&connection, atom_delete_window, state),
        update_window_title,
    )
}
