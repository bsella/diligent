use xcb::{x, Xid};

use crate::{
    core::engine_factory::EngineCreateInfo,
    tools::native_app::{
        app::App,
        app_settings::AppSettings,
        events::{EventHandler, EventResult, MouseButton},
        NativeWindow,
    },
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
}

impl<'a> XcbEventHandler<'a> {
    fn new(connection: &'a xcb::Connection, atom_delete_window: xcb::x::Atom) -> Self {
        XcbEventHandler {
            connection,
            atom_delete_window,
        }
    }
}

impl<'a> EventHandler for XcbEventHandler<'a> {
    type EventType = xcb::Event;

    fn poll_event(&self) -> Option<xcb::Event> {
        self.connection.poll_for_event().unwrap()
    }

    fn handle_event(&mut self, event: &xcb::Event) -> EventResult {
        match event {
            xcb::Event::X(x::Event::ClientMessage(message_event)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = message_event.data() {
                    if atom == self.atom_delete_window.resource_id() {
                        return EventResult::Quit;
                    }
                }
                return EventResult::Continue;
            }

            xcb::Event::X(x::Event::KeyRelease(_key_event)) => {
                // TODO
                EventResult::Continue
            }

            xcb::Event::X(x::Event::DestroyNotify(_destroy_event)) => EventResult::Quit,

            xcb::Event::X(x::Event::ConfigureNotify(configure_event)) => EventResult::Resize {
                width: configure_event.width(),
                height: configure_event.height(),
            },

            xcb::Event::X(xcb::x::Event::MotionNotify(motion_event)) => EventResult::MouseMove {
                x: motion_event.event_x(),
                y: motion_event.event_y(),
            },

            xcb::Event::X(xcb::x::Event::ButtonPress(press_event)) => match press_event.detail() {
                1 => EventResult::MouseDown {
                    button: MouseButton::Left,
                },
                2 => EventResult::MouseDown {
                    button: MouseButton::Right,
                },
                3 => EventResult::MouseDown {
                    button: MouseButton::Middle,
                },
                4 => EventResult::MouseWheel { up: true },
                5 => EventResult::MouseWheel { up: false },
                _ => EventResult::Continue,
            },

            xcb::Event::X(xcb::x::Event::ButtonRelease(release_event)) => {
                match release_event.detail() {
                    1 => EventResult::MouseUp {
                        button: MouseButton::Left,
                    },
                    2 => EventResult::MouseUp {
                        button: MouseButton::Right,
                    },
                    3 => EventResult::MouseUp {
                        button: MouseButton::Middle,
                    },
                    _ => EventResult::Continue,
                }
            }

            _ => EventResult::Continue,
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
        connection: connection.get_raw_conn(),
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

    app.run(
        XcbEventHandler::new(&connection, atom_delete_window),
        update_window_title,
    )
}
