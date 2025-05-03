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
        window: window,
        value_list: &[x::ConfigWindow::X(100), x::ConfigWindow::Y(100)],
    });

    connection.flush()?;

    loop {
        match connection.wait_for_event()? {
            xcb::Event::X(x::Event::Expose(_event)) => {
                break;
            }
            _ => {}
        };
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
        match event {
            xcb::Event::X(x::Event::ClientMessage(message_event)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = message_event.data() {
                    if atom == self.atom_delete_window.resource_id() {
                        return Event::Quit;
                    }
                }
                return Event::Continue;
            }

            xcb::Event::X(x::Event::KeyPress(key_event)) => {
                let keysym = self
                    .keyboard_state
                    .key_get_one_sym(xkb::Keycode::new(key_event.detail() as _));

                match keysym.raw() {
                    xkb::keysyms::KEY_a => Event::KeyPress(Key::A),
                    xkb::keysyms::KEY_b => Event::KeyPress(Key::B),
                    xkb::keysyms::KEY_c => Event::KeyPress(Key::C),
                    xkb::keysyms::KEY_d => Event::KeyPress(Key::D),
                    xkb::keysyms::KEY_e => Event::KeyPress(Key::E),
                    xkb::keysyms::KEY_f => Event::KeyPress(Key::F),
                    xkb::keysyms::KEY_g => Event::KeyPress(Key::G),
                    xkb::keysyms::KEY_h => Event::KeyPress(Key::H),
                    xkb::keysyms::KEY_i => Event::KeyPress(Key::I),
                    xkb::keysyms::KEY_j => Event::KeyPress(Key::J),
                    xkb::keysyms::KEY_k => Event::KeyPress(Key::K),
                    xkb::keysyms::KEY_l => Event::KeyPress(Key::L),
                    xkb::keysyms::KEY_m => Event::KeyPress(Key::M),
                    xkb::keysyms::KEY_n => Event::KeyPress(Key::N),
                    xkb::keysyms::KEY_o => Event::KeyPress(Key::O),
                    xkb::keysyms::KEY_p => Event::KeyPress(Key::P),
                    xkb::keysyms::KEY_q => Event::KeyPress(Key::Q),
                    xkb::keysyms::KEY_r => Event::KeyPress(Key::R),
                    xkb::keysyms::KEY_s => Event::KeyPress(Key::S),
                    xkb::keysyms::KEY_t => Event::KeyPress(Key::T),
                    xkb::keysyms::KEY_u => Event::KeyPress(Key::U),
                    xkb::keysyms::KEY_v => Event::KeyPress(Key::V),
                    xkb::keysyms::KEY_w => Event::KeyPress(Key::W),
                    xkb::keysyms::KEY_x => Event::KeyPress(Key::X),
                    xkb::keysyms::KEY_y => Event::KeyPress(Key::Y),
                    xkb::keysyms::KEY_z => Event::KeyPress(Key::Z),
                    xkb::keysyms::KEY_1 => Event::KeyPress(Key::_1),
                    xkb::keysyms::KEY_2 => Event::KeyPress(Key::_2),
                    xkb::keysyms::KEY_3 => Event::KeyPress(Key::_3),
                    xkb::keysyms::KEY_4 => Event::KeyPress(Key::_4),
                    xkb::keysyms::KEY_5 => Event::KeyPress(Key::_5),
                    xkb::keysyms::KEY_6 => Event::KeyPress(Key::_6),
                    xkb::keysyms::KEY_7 => Event::KeyPress(Key::_7),
                    xkb::keysyms::KEY_8 => Event::KeyPress(Key::_8),
                    xkb::keysyms::KEY_9 => Event::KeyPress(Key::_9),
                    xkb::keysyms::KEY_0 => Event::KeyPress(Key::_0),
                    xkb::keysyms::KEY_minus => Event::KeyPress(Key::Minus),
                    xkb::keysyms::KEY_equal => Event::KeyPress(Key::Equals),
                    xkb::keysyms::KEY_braceleft => Event::KeyPress(Key::LeftBrace),
                    xkb::keysyms::KEY_braceright => Event::KeyPress(Key::RightBrace),
                    xkb::keysyms::KEY_comma => Event::KeyPress(Key::Comma),
                    xkb::keysyms::KEY_period => Event::KeyPress(Key::Period),
                    xkb::keysyms::KEY_slash => Event::KeyPress(Key::Slash),
                    xkb::keysyms::KEY_semicolon => Event::KeyPress(Key::Semicolon),
                    xkb::keysyms::KEY_quotedbl => Event::KeyPress(Key::Quote),
                    xkb::keysyms::KEY_asterisk => Event::KeyPress(Key::Asterisk),
                    xkb::keysyms::KEY_backslash => Event::KeyPress(Key::Backslash),
                    xkb::keysyms::KEY_Return => Event::KeyPress(Key::Enter),
                    xkb::keysyms::KEY_BackSpace => Event::KeyPress(Key::Backspace),
                    xkb::keysyms::KEY_uparrow => Event::KeyPress(Key::Up),
                    xkb::keysyms::KEY_downarrow => Event::KeyPress(Key::Down),
                    xkb::keysyms::KEY_leftarrow => Event::KeyPress(Key::Left),
                    xkb::keysyms::KEY_rightarrow => Event::KeyPress(Key::Right),
                    xkb::keysyms::KEY_Control_L => Event::KeyPress(Key::LeftCtrl),
                    xkb::keysyms::KEY_Alt_L => Event::KeyPress(Key::LeftAlt),
                    xkb::keysyms::KEY_Shift_L => Event::KeyPress(Key::LeftShift),
                    xkb::keysyms::KEY_Super_L => Event::KeyPress(Key::LeftSuper),
                    xkb::keysyms::KEY_Control_R => Event::KeyPress(Key::RightCtrl),
                    xkb::keysyms::KEY_Alt_R => Event::KeyPress(Key::RightAlt),
                    xkb::keysyms::KEY_Shift_R => Event::KeyPress(Key::RightShift),
                    xkb::keysyms::KEY_Super_R => Event::KeyPress(Key::RightSuper),
                    xkb::keysyms::KEY_Insert => Event::KeyPress(Key::Insert),
                    xkb::keysyms::KEY_Delete => Event::KeyPress(Key::Delete),
                    xkb::keysyms::KEY_Home => Event::KeyPress(Key::Home),
                    xkb::keysyms::KEY_End => Event::KeyPress(Key::End),
                    xkb::keysyms::KEY_Page_Up => Event::KeyPress(Key::PageUp),
                    xkb::keysyms::KEY_Page_Down => Event::KeyPress(Key::PageDown),
                    xkb::keysyms::KEY_F1 => Event::KeyPress(Key::F1),
                    xkb::keysyms::KEY_F2 => Event::KeyPress(Key::F2),
                    xkb::keysyms::KEY_F3 => Event::KeyPress(Key::F3),
                    xkb::keysyms::KEY_F4 => Event::KeyPress(Key::F4),
                    xkb::keysyms::KEY_F5 => Event::KeyPress(Key::F5),
                    xkb::keysyms::KEY_F6 => Event::KeyPress(Key::F6),
                    xkb::keysyms::KEY_F7 => Event::KeyPress(Key::F7),
                    xkb::keysyms::KEY_F8 => Event::KeyPress(Key::F8),
                    xkb::keysyms::KEY_F9 => Event::KeyPress(Key::F9),
                    xkb::keysyms::KEY_F10 => Event::KeyPress(Key::F10),
                    xkb::keysyms::KEY_F11 => Event::KeyPress(Key::F11),
                    xkb::keysyms::KEY_F12 => Event::KeyPress(Key::F12),
                    xkb::keysyms::KEY_Escape => Event::KeyPress(Key::Esc),
                    xkb::keysyms::KEY_Tab => Event::KeyPress(Key::Tab),
                    xkb::keysyms::KEY_space => Event::KeyPress(Key::Space),

                    _ => Event::KeyPress(Key::Unknown),
                }
            }

            xcb::Event::X(x::Event::KeyRelease(key_event)) => {
                let keysym = self
                    .keyboard_state
                    .key_get_one_sym(xkb::Keycode::new(key_event.detail() as _));

                match keysym.raw() {
                    xkb::keysyms::KEY_a => Event::KeyRelease(Key::A),
                    xkb::keysyms::KEY_b => Event::KeyRelease(Key::B),
                    xkb::keysyms::KEY_c => Event::KeyRelease(Key::C),
                    xkb::keysyms::KEY_d => Event::KeyRelease(Key::D),
                    xkb::keysyms::KEY_e => Event::KeyRelease(Key::E),
                    xkb::keysyms::KEY_f => Event::KeyRelease(Key::F),
                    xkb::keysyms::KEY_g => Event::KeyRelease(Key::G),
                    xkb::keysyms::KEY_h => Event::KeyRelease(Key::H),
                    xkb::keysyms::KEY_i => Event::KeyRelease(Key::I),
                    xkb::keysyms::KEY_j => Event::KeyRelease(Key::J),
                    xkb::keysyms::KEY_k => Event::KeyRelease(Key::K),
                    xkb::keysyms::KEY_l => Event::KeyRelease(Key::L),
                    xkb::keysyms::KEY_m => Event::KeyRelease(Key::M),
                    xkb::keysyms::KEY_n => Event::KeyRelease(Key::N),
                    xkb::keysyms::KEY_o => Event::KeyRelease(Key::O),
                    xkb::keysyms::KEY_p => Event::KeyRelease(Key::P),
                    xkb::keysyms::KEY_q => Event::KeyRelease(Key::Q),
                    xkb::keysyms::KEY_r => Event::KeyRelease(Key::R),
                    xkb::keysyms::KEY_s => Event::KeyRelease(Key::S),
                    xkb::keysyms::KEY_t => Event::KeyRelease(Key::T),
                    xkb::keysyms::KEY_u => Event::KeyRelease(Key::U),
                    xkb::keysyms::KEY_v => Event::KeyRelease(Key::V),
                    xkb::keysyms::KEY_w => Event::KeyRelease(Key::W),
                    xkb::keysyms::KEY_x => Event::KeyRelease(Key::X),
                    xkb::keysyms::KEY_y => Event::KeyRelease(Key::Y),
                    xkb::keysyms::KEY_z => Event::KeyRelease(Key::Z),
                    xkb::keysyms::KEY_1 => Event::KeyRelease(Key::_1),
                    xkb::keysyms::KEY_2 => Event::KeyRelease(Key::_2),
                    xkb::keysyms::KEY_3 => Event::KeyRelease(Key::_3),
                    xkb::keysyms::KEY_4 => Event::KeyRelease(Key::_4),
                    xkb::keysyms::KEY_5 => Event::KeyRelease(Key::_5),
                    xkb::keysyms::KEY_6 => Event::KeyRelease(Key::_6),
                    xkb::keysyms::KEY_7 => Event::KeyRelease(Key::_7),
                    xkb::keysyms::KEY_8 => Event::KeyRelease(Key::_8),
                    xkb::keysyms::KEY_9 => Event::KeyRelease(Key::_9),
                    xkb::keysyms::KEY_0 => Event::KeyRelease(Key::_0),
                    xkb::keysyms::KEY_minus => Event::KeyRelease(Key::Minus),
                    xkb::keysyms::KEY_equal => Event::KeyRelease(Key::Equals),
                    xkb::keysyms::KEY_comma => Event::KeyRelease(Key::Comma),
                    xkb::keysyms::KEY_period => Event::KeyRelease(Key::Period),
                    xkb::keysyms::KEY_slash => Event::KeyRelease(Key::Slash),
                    xkb::keysyms::KEY_semicolon => Event::KeyRelease(Key::Semicolon),
                    xkb::keysyms::KEY_quotedbl => Event::KeyRelease(Key::Quote),
                    xkb::keysyms::KEY_asterisk => Event::KeyRelease(Key::Asterisk),
                    xkb::keysyms::KEY_backslash => Event::KeyRelease(Key::Backslash),
                    xkb::keysyms::KEY_Return => Event::KeyRelease(Key::Enter),
                    xkb::keysyms::KEY_BackSpace => Event::KeyRelease(Key::Backspace),
                    xkb::keysyms::KEY_uparrow => Event::KeyRelease(Key::Up),
                    xkb::keysyms::KEY_downarrow => Event::KeyRelease(Key::Down),
                    xkb::keysyms::KEY_leftarrow => Event::KeyRelease(Key::Left),
                    xkb::keysyms::KEY_rightarrow => Event::KeyRelease(Key::Right),
                    xkb::keysyms::KEY_Control_L => Event::KeyRelease(Key::LeftCtrl),
                    xkb::keysyms::KEY_Alt_L => Event::KeyRelease(Key::LeftAlt),
                    xkb::keysyms::KEY_Shift_L => Event::KeyRelease(Key::LeftShift),
                    xkb::keysyms::KEY_Super_L => Event::KeyRelease(Key::LeftSuper),
                    xkb::keysyms::KEY_Control_R => Event::KeyRelease(Key::RightCtrl),
                    xkb::keysyms::KEY_Alt_R => Event::KeyRelease(Key::RightAlt),
                    xkb::keysyms::KEY_Shift_R => Event::KeyRelease(Key::RightShift),
                    xkb::keysyms::KEY_Super_R => Event::KeyRelease(Key::RightSuper),
                    xkb::keysyms::KEY_Insert => Event::KeyRelease(Key::Insert),
                    xkb::keysyms::KEY_Delete => Event::KeyRelease(Key::Delete),
                    xkb::keysyms::KEY_Home => Event::KeyRelease(Key::Home),
                    xkb::keysyms::KEY_End => Event::KeyRelease(Key::End),
                    xkb::keysyms::KEY_Page_Up => Event::KeyRelease(Key::PageUp),
                    xkb::keysyms::KEY_Page_Down => Event::KeyRelease(Key::PageDown),
                    xkb::keysyms::KEY_F1 => Event::KeyRelease(Key::F1),
                    xkb::keysyms::KEY_F2 => Event::KeyRelease(Key::F2),
                    xkb::keysyms::KEY_F3 => Event::KeyRelease(Key::F3),
                    xkb::keysyms::KEY_F4 => Event::KeyRelease(Key::F4),
                    xkb::keysyms::KEY_F5 => Event::KeyRelease(Key::F5),
                    xkb::keysyms::KEY_F6 => Event::KeyRelease(Key::F6),
                    xkb::keysyms::KEY_F7 => Event::KeyRelease(Key::F7),
                    xkb::keysyms::KEY_F8 => Event::KeyRelease(Key::F8),
                    xkb::keysyms::KEY_F9 => Event::KeyRelease(Key::F9),
                    xkb::keysyms::KEY_F10 => Event::KeyRelease(Key::F10),
                    xkb::keysyms::KEY_F11 => Event::KeyRelease(Key::F11),
                    xkb::keysyms::KEY_F12 => Event::KeyRelease(Key::F12),
                    xkb::keysyms::KEY_Escape => Event::KeyRelease(Key::Esc),
                    xkb::keysyms::KEY_Tab => Event::KeyRelease(Key::Tab),
                    xkb::keysyms::KEY_space => Event::KeyRelease(Key::Space),

                    _ => Event::KeyRelease(Key::Unknown),
                }
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
