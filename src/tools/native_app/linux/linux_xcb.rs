use std::os::raw::c_void;

use xcb::{x, Xid};

use crate::{
    bindings,
    core::{engine_factory::EngineFactoryImplementation, vk::engine_factory_vk::EngineFactoryVk},
    tools::native_app::app::{ApiImplementation, App},
};

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

fn xcb_main<Application>() -> xcb::Result<()>
where
    Application: App,
{
    let (connection, window, atom_delete_window) = init_connection_and_window()?;

    let native_window = bindings::NativeWindow {
        WindowId: window.resource_id(),
        pXCBConnection: connection.get_raw_conn() as *mut c_void,
        pDisplay: std::ptr::null_mut(),
    };

    let api = ApiImplementation::Vulkan;

    let mut app = match api {
        ApiImplementation::Vulkan => {
            let engine_create_info =
                <EngineFactoryVk as EngineFactoryImplementation>::EngineCreateInfo::default();
            Application::new::<EngineFactoryVk>(engine_create_info, Some(&native_window))
        }
        ApiImplementation::OpenGL => panic!(),
    };

    // TODO init vulkan

    connection.flush()?;

    // TODO golden image mode

    // TODO timer and title

    let mut current_width = 1024;
    let mut current_height = 768;

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
                        if (configure_event.width() != current_width)
                            || (configure_event.height() != current_height)
                        {
                            current_width = configure_event.width();
                            current_height = configure_event.height();
                            if current_width > 0 && current_height > 0 {
                                app.window_resize(current_height as u32, current_height as u32);
                            }
                        }
                    }
                    _ => break 'xcb_events,
                },
                None => break 'xcb_events,
            }
        }

        // TODO implement timer
        app.update(0.0, 0.0);

        app.render();

        app.present();

        // TODO update title
    }

    Ok(())
}

pub(super) fn main<Application>() -> Result<(), std::io::Error>
where
    Application: App,
{
    xcb_main::<Application>()
        .map_err(|xcb_error| std::io::Error::new(std::io::ErrorKind::Other, xcb_error.to_string()))
}
