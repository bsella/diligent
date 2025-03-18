use imgui::Io;

use crate::native_app::events::{EventResult, MouseButton};

pub fn imgui_handle_event(io: &mut Io, event: EventResult) -> EventResult {
    match event {
        EventResult::MouseMove { x, y } => {
            io.mouse_pos = [x as f32, y as f32];
            if io.want_capture_mouse {
                EventResult::Continue
            } else {
                event
            }
        }
        EventResult::MouseDown { ref button } => {
            match button {
                MouseButton::Left => io.mouse_down[0] = true,
                MouseButton::Right => io.mouse_down[2] = true,
                MouseButton::Middle => io.mouse_down[1] = true,
            }
            if io.want_capture_mouse {
                EventResult::Continue
            } else {
                event
            }
        }

        EventResult::MouseUp { ref button } => {
            match button {
                MouseButton::Left => io.mouse_down[0] = false,
                MouseButton::Right => io.mouse_down[2] = false,
                MouseButton::Middle => io.mouse_down[1] = false,
            }
            if io.want_capture_mouse {
                EventResult::Continue
            } else {
                event
            }
        }

        EventResult::Resize { width, height } => {
            io.display_size = [width as f32, height as f32];
            event
        }

        // TODO Key events
        _ => event,
    }
}
