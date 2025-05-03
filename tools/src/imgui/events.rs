use imgui::{sys::*, Io};

use crate::native_app::events::{Event, Key, MouseButton};

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

        Event::KeyPress(ref key) => {
            match key {
                Key::LeftShift | Key::RightShift => io.key_shift = true,
                Key::LeftAlt | Key::RightAlt => io.key_alt = true,
                Key::LeftCtrl | Key::RightCtrl => io.key_ctrl = true,
                Key::LeftSuper | Key::RightSuper => io.key_super = true,
                Key::A => io.keys_down['a' as usize] = true,
                Key::B => io.keys_down['b' as usize] = true,
                Key::C => io.keys_down['c' as usize] = true,
                Key::D => io.keys_down['d' as usize] = true,
                Key::E => io.keys_down['e' as usize] = true,
                Key::F => io.keys_down['f' as usize] = true,
                Key::G => io.keys_down['g' as usize] = true,
                Key::H => io.keys_down['h' as usize] = true,
                Key::I => io.keys_down['i' as usize] = true,
                Key::J => io.keys_down['j' as usize] = true,
                Key::K => io.keys_down['k' as usize] = true,
                Key::L => io.keys_down['l' as usize] = true,
                Key::M => io.keys_down['m' as usize] = true,
                Key::N => io.keys_down['n' as usize] = true,
                Key::O => io.keys_down['o' as usize] = true,
                Key::P => io.keys_down['p' as usize] = true,
                Key::Q => io.keys_down['q' as usize] = true,
                Key::R => io.keys_down['r' as usize] = true,
                Key::S => io.keys_down['s' as usize] = true,
                Key::T => io.keys_down['t' as usize] = true,
                Key::U => io.keys_down['u' as usize] = true,
                Key::V => io.keys_down['v' as usize] = true,
                Key::W => io.keys_down['w' as usize] = true,
                Key::X => io.keys_down['x' as usize] = true,
                Key::Y => io.keys_down['y' as usize] = true,
                Key::Z => io.keys_down['z' as usize] = true,

                Key::_1 => io.keys_down['1' as usize] = true,
                Key::_2 => io.keys_down['2' as usize] = true,
                Key::_3 => io.keys_down['3' as usize] = true,
                Key::_4 => io.keys_down['4' as usize] = true,
                Key::_5 => io.keys_down['5' as usize] = true,
                Key::_6 => io.keys_down['6' as usize] = true,
                Key::_7 => io.keys_down['7' as usize] = true,
                Key::_8 => io.keys_down['8' as usize] = true,
                Key::_9 => io.keys_down['9' as usize] = true,
                Key::_0 => io.keys_down['0' as usize] = true,

                Key::F1 => io.keys_down[ImGuiKey_F1 as usize] = true,
                Key::F2 => io.keys_down[ImGuiKey_F2 as usize] = true,
                Key::F3 => io.keys_down[ImGuiKey_F3 as usize] = true,
                Key::F4 => io.keys_down[ImGuiKey_F4 as usize] = true,
                Key::F5 => io.keys_down[ImGuiKey_F5 as usize] = true,
                Key::F6 => io.keys_down[ImGuiKey_F6 as usize] = true,
                Key::F7 => io.keys_down[ImGuiKey_F7 as usize] = true,
                Key::F8 => io.keys_down[ImGuiKey_F8 as usize] = true,
                Key::F9 => io.keys_down[ImGuiKey_F9 as usize] = true,
                Key::F10 => io.keys_down[ImGuiKey_F10 as usize] = true,
                Key::F11 => io.keys_down[ImGuiKey_F11 as usize] = true,
                Key::F12 => io.keys_down[ImGuiKey_F12 as usize] = true,

                Key::Esc => io.keys_down[ImGuiKey_Escape as usize] = true,
                Key::Up => io.keys_down[ImGuiKey_UpArrow as usize] = true,
                Key::Down => io.keys_down[ImGuiKey_DownArrow as usize] = true,
                Key::Right => io.keys_down[ImGuiKey_RightArrow as usize] = true,
                Key::Left => io.keys_down[ImGuiKey_LeftArrow as usize] = true,
                Key::Enter => io.keys_down[ImGuiKey_Enter as usize] = true,
                Key::Insert => io.keys_down[ImGuiKey_Insert as usize] = true,
                Key::Delete => io.keys_down[ImGuiKey_Delete as usize] = true,
                Key::Backspace => io.keys_down[ImGuiKey_Backspace as usize] = true,
                Key::Home => io.keys_down[ImGuiKey_Home as usize] = true,
                Key::Tab => io.keys_down[ImGuiKey_Tab as usize] = true,
                Key::End => io.keys_down[ImGuiKey_End as usize] = true,
                Key::PageUp => io.keys_down[ImGuiKey_PageUp as usize] = true,
                Key::PageDown => io.keys_down[ImGuiKey_PageDown as usize] = true,
                Key::Space => io.keys_down[ImGuiKey_Space as usize] = true,
                _ => {}
            }

            if io.want_capture_keyboard {
                Event::Continue
            } else {
                event
            }
        }

        Event::KeyRelease(ref key) => {
            match key {
                Key::LeftShift | Key::RightShift => io.key_shift = false,
                Key::LeftAlt | Key::RightAlt => io.key_alt = false,
                Key::LeftCtrl | Key::RightCtrl => io.key_ctrl = false,
                Key::LeftSuper | Key::RightSuper => io.key_super = false,
                Key::A => io.keys_down['a' as usize] = false,
                Key::B => io.keys_down['b' as usize] = false,
                Key::C => io.keys_down['c' as usize] = false,
                Key::D => io.keys_down['d' as usize] = false,
                Key::E => io.keys_down['e' as usize] = false,
                Key::F => io.keys_down['f' as usize] = false,
                Key::G => io.keys_down['g' as usize] = false,
                Key::H => io.keys_down['h' as usize] = false,
                Key::I => io.keys_down['i' as usize] = false,
                Key::J => io.keys_down['j' as usize] = false,
                Key::K => io.keys_down['k' as usize] = false,
                Key::L => io.keys_down['l' as usize] = false,
                Key::M => io.keys_down['m' as usize] = false,
                Key::N => io.keys_down['n' as usize] = false,
                Key::O => io.keys_down['o' as usize] = false,
                Key::P => io.keys_down['p' as usize] = false,
                Key::Q => io.keys_down['q' as usize] = false,
                Key::R => io.keys_down['r' as usize] = false,
                Key::S => io.keys_down['s' as usize] = false,
                Key::T => io.keys_down['t' as usize] = false,
                Key::U => io.keys_down['u' as usize] = false,
                Key::V => io.keys_down['v' as usize] = false,
                Key::W => io.keys_down['w' as usize] = false,
                Key::X => io.keys_down['x' as usize] = false,
                Key::Y => io.keys_down['y' as usize] = false,
                Key::Z => io.keys_down['z' as usize] = false,

                Key::_1 => io.keys_down['1' as usize] = false,
                Key::_2 => io.keys_down['2' as usize] = false,
                Key::_3 => io.keys_down['3' as usize] = false,
                Key::_4 => io.keys_down['4' as usize] = false,
                Key::_5 => io.keys_down['5' as usize] = false,
                Key::_6 => io.keys_down['6' as usize] = false,
                Key::_7 => io.keys_down['7' as usize] = false,
                Key::_8 => io.keys_down['8' as usize] = false,
                Key::_9 => io.keys_down['9' as usize] = false,
                Key::_0 => io.keys_down['0' as usize] = false,

                Key::F1 => io.keys_down[ImGuiKey_F1 as usize] = false,
                Key::F2 => io.keys_down[ImGuiKey_F2 as usize] = false,
                Key::F3 => io.keys_down[ImGuiKey_F3 as usize] = false,
                Key::F4 => io.keys_down[ImGuiKey_F4 as usize] = false,
                Key::F5 => io.keys_down[ImGuiKey_F5 as usize] = false,
                Key::F6 => io.keys_down[ImGuiKey_F6 as usize] = false,
                Key::F7 => io.keys_down[ImGuiKey_F7 as usize] = false,
                Key::F8 => io.keys_down[ImGuiKey_F8 as usize] = false,
                Key::F9 => io.keys_down[ImGuiKey_F9 as usize] = false,
                Key::F10 => io.keys_down[ImGuiKey_F10 as usize] = false,
                Key::F11 => io.keys_down[ImGuiKey_F11 as usize] = false,
                Key::F12 => io.keys_down[ImGuiKey_F12 as usize] = false,

                Key::Esc => io.keys_down[ImGuiKey_Escape as usize] = false,
                Key::Up => io.keys_down[ImGuiKey_UpArrow as usize] = false,
                Key::Down => io.keys_down[ImGuiKey_DownArrow as usize] = false,
                Key::Right => io.keys_down[ImGuiKey_RightArrow as usize] = false,
                Key::Left => io.keys_down[ImGuiKey_LeftArrow as usize] = false,
                Key::Enter => io.keys_down[ImGuiKey_Enter as usize] = false,
                Key::Insert => io.keys_down[ImGuiKey_Insert as usize] = false,
                Key::Delete => io.keys_down[ImGuiKey_Delete as usize] = false,
                Key::Backspace => io.keys_down[ImGuiKey_Backspace as usize] = false,
                Key::Home => io.keys_down[ImGuiKey_Home as usize] = false,
                Key::Tab => io.keys_down[ImGuiKey_Tab as usize] = false,
                Key::End => io.keys_down[ImGuiKey_End as usize] = false,
                Key::PageUp => io.keys_down[ImGuiKey_PageUp as usize] = false,
                Key::PageDown => io.keys_down[ImGuiKey_PageDown as usize] = false,
                Key::Space => io.keys_down[ImGuiKey_Space as usize] = false,
                _ => {}
            }

            if io.want_capture_keyboard {
                Event::Continue
            } else {
                event
            }
        }
        _ => event,
    }
}
