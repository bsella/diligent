use imgui::Io;

use crate::native_app::events::{Event, MouseButton};

pub fn imgui_handle_event(io: &mut Io, event: Event) -> Event {
    match event {
        Event::MouseMove { x, y } => {
            io.mouse_pos = [x as f32, y as f32];
            if io.want_capture_mouse {
                Event::Continue
            } else {
                event
            }
        }
        Event::MouseDown { ref button } => {
            match button {
                MouseButton::Left => io.mouse_down[0] = true,
                MouseButton::Right => io.mouse_down[2] = true,
                MouseButton::Middle => io.mouse_down[1] = true,
            }
            if io.want_capture_mouse {
                Event::Continue
            } else {
                event
            }
        }

        Event::MouseUp { ref button } => {
            match button {
                MouseButton::Left => io.mouse_down[0] = false,
                MouseButton::Right => io.mouse_down[2] = false,
                MouseButton::Middle => io.mouse_down[1] = false,
            }
            if io.want_capture_mouse {
                Event::Continue
            } else {
                event
            }
        }

        Event::Resize { width, height } => {
            io.display_size = [width as f32, height as f32];
            event
        }

        // TODO Key events
        _ => event,
    }
}
