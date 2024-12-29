use xcb::{x, Xid};

fn init_connection_and_window() -> xcb::Result<(xcb::Connection, x::Window, x::Atom)> {
    let width = 1024;
    let height = 768;

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
        width: width,
        height: height,
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
        window: window,
        property: atom_wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[atom_wm_delete_window],
    });

    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: b"Test",
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

fn xcb_main() -> xcb::Result<()> {
    // TODO create the application

    let (connection, window, atom_delete_window) = init_connection_and_window()?;

    // TODO init vulkan

    connection.flush()?;

    // TODO golden image mode

    // TODO timer and title

    'main: loop {
        'xcb_events: loop {
            match connection.poll_for_event()? {
                Some(event) => match event {
                    xcb::Event::X(x::Event::ClientMessage(message_event)) => {
                        if let x::ClientMessageData::Data32([atom, ..]) = message_event.data() {
                            if atom == atom_delete_window.resource_id() {
                                break 'main;
                            }
                        }
                    }
                    xcb::Event::X(x::Event::KeyRelease(key_event)) => {
                        // TODO
                    }
                    xcb::Event::X(x::Event::DestroyNotify(_destroy_event)) => {
                        break 'main;
                    }

                    xcb::Event::X(x::Event::ConfigureNotify(configure_event)) => {
                        // TODO
                    }
                    _ => {}
                },
                None => break 'xcb_events,
            }
        }

        // TODO update + render

        // TODO update title
    }

    Ok(())
}

pub(super) fn main() -> Result<(), std::io::Error> {
    xcb_main()
        .map_err(|xcb_error| std::io::Error::new(std::io::ErrorKind::Other, xcb_error.to_string()))
}
