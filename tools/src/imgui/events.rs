use imgui::Io;

use crate::native_app::events::{Event, Key, MouseButton};

#[rustfmt::skip]
fn handle_key_event(io: &mut imgui::Io, key: Key, down: bool) {
    let is_shift_pressed = io.key_shift;

    match key {
        Key::LeftShift => io.add_key_event(imgui::Key::LeftShift, down),
        Key::RightShift => io.add_key_event(imgui::Key::RightShift, down),
        Key::LeftAlt => io.add_key_event(imgui::Key::LeftAlt, down),
        Key::RightAlt => io.add_key_event(imgui::Key::RightAlt, down),
        Key::LeftCtrl => io.add_key_event(imgui::Key::LeftCtrl, down),
        Key::RightCtrl => io.add_key_event(imgui::Key::RightCtrl, down),
        Key::LeftSuper => io.add_key_event(imgui::Key::LeftSuper, down),
        Key::RightSuper => io.add_key_event(imgui::Key::RightSuper, down),
        Key::A  => if is_shift_pressed {io.add_input_character('A')} else {io.add_input_character('a')},
        Key::B  => if is_shift_pressed {io.add_input_character('B')} else {io.add_input_character('b')},
        Key::C  => if is_shift_pressed {io.add_input_character('C')} else {io.add_input_character('c')},
        Key::D  => if is_shift_pressed {io.add_input_character('D')} else {io.add_input_character('d')},
        Key::E  => if is_shift_pressed {io.add_input_character('E')} else {io.add_input_character('e')},
        Key::F  => if is_shift_pressed {io.add_input_character('F')} else {io.add_input_character('f')},
        Key::G  => if is_shift_pressed {io.add_input_character('G')} else {io.add_input_character('g')},
        Key::H  => if is_shift_pressed {io.add_input_character('H')} else {io.add_input_character('h')},
        Key::I  => if is_shift_pressed {io.add_input_character('I')} else {io.add_input_character('i')},
        Key::J  => if is_shift_pressed {io.add_input_character('J')} else {io.add_input_character('j')},
        Key::K  => if is_shift_pressed {io.add_input_character('K')} else {io.add_input_character('k')},
        Key::L  => if is_shift_pressed {io.add_input_character('L')} else {io.add_input_character('l')},
        Key::M  => if is_shift_pressed {io.add_input_character('M')} else {io.add_input_character('m')},
        Key::N  => if is_shift_pressed {io.add_input_character('N')} else {io.add_input_character('n')},
        Key::O  => if is_shift_pressed {io.add_input_character('O')} else {io.add_input_character('o')},
        Key::P  => if is_shift_pressed {io.add_input_character('P')} else {io.add_input_character('p')},
        Key::Q  => if is_shift_pressed {io.add_input_character('Q')} else {io.add_input_character('q')},
        Key::R  => if is_shift_pressed {io.add_input_character('R')} else {io.add_input_character('r')},
        Key::S  => if is_shift_pressed {io.add_input_character('S')} else {io.add_input_character('s')},
        Key::T  => if is_shift_pressed {io.add_input_character('T')} else {io.add_input_character('t')},
        Key::U  => if is_shift_pressed {io.add_input_character('U')} else {io.add_input_character('u')},
        Key::V  => if is_shift_pressed {io.add_input_character('V')} else {io.add_input_character('v')},
        Key::W  => if is_shift_pressed {io.add_input_character('W')} else {io.add_input_character('w')},
        Key::X  => if is_shift_pressed {io.add_input_character('X')} else {io.add_input_character('x')},
        Key::Y  => if is_shift_pressed {io.add_input_character('Y')} else {io.add_input_character('y')},
        Key::Z  => if is_shift_pressed {io.add_input_character('Z')} else {io.add_input_character('z')},
        Key::_1 => if is_shift_pressed {io.add_input_character('!')} else {io.add_input_character('1')},
        Key::_2 => if is_shift_pressed {io.add_input_character('@')} else {io.add_input_character('2')},
        Key::_3 => if is_shift_pressed {io.add_input_character('#')} else {io.add_input_character('3')},
        Key::_4 => if is_shift_pressed {io.add_input_character('$')} else {io.add_input_character('4')},
        Key::_5 => if is_shift_pressed {io.add_input_character('%')} else {io.add_input_character('5')},
        Key::_6 => if is_shift_pressed {io.add_input_character('^')} else {io.add_input_character('6')},
        Key::_7 => if is_shift_pressed {io.add_input_character('&')} else {io.add_input_character('7')},
        Key::_8 => if is_shift_pressed {io.add_input_character('*')} else {io.add_input_character('8')},
        Key::_9 => if is_shift_pressed {io.add_input_character('(')} else {io.add_input_character('9')},
        Key::_0 => if is_shift_pressed {io.add_input_character(')')} else {io.add_input_character('0')},
        Key::F1 => io.add_key_event(imgui::Key::F1, down),
        Key::F2 => io.add_key_event(imgui::Key::F2, down),
        Key::F3 => io.add_key_event(imgui::Key::F3, down),
        Key::F4 => io.add_key_event(imgui::Key::F4, down),
        Key::F5 => io.add_key_event(imgui::Key::F5, down),
        Key::F6 => io.add_key_event(imgui::Key::F6, down),
        Key::F7 => io.add_key_event(imgui::Key::F7, down),
        Key::F8 => io.add_key_event(imgui::Key::F8, down),
        Key::F9 => io.add_key_event(imgui::Key::F9, down),
        Key::F10 => io.add_key_event(imgui::Key::F10, down),
        Key::F11 => io.add_key_event(imgui::Key::F11, down),
        Key::F12 => io.add_key_event(imgui::Key::F12, down),
        Key::Esc => io.add_key_event(imgui::Key::Escape, down),
        Key::Up => io.add_key_event(imgui::Key::UpArrow, down),
        Key::Down => io.add_key_event(imgui::Key::DownArrow, down),
        Key::Right => io.add_key_event(imgui::Key::RightArrow, down),
        Key::Left => io.add_key_event(imgui::Key::LeftArrow, down),
        Key::Enter => io.add_key_event(imgui::Key::Enter, down),
        Key::Insert => io.add_key_event(imgui::Key::Insert, down),
        Key::Delete => io.add_key_event(imgui::Key::Delete, down),
        Key::Backspace => io.add_key_event(imgui::Key::Backspace, down),
        Key::Home => io.add_key_event(imgui::Key::Home, down),
        Key::Tab => io.add_key_event(imgui::Key::Tab, down),
        Key::End => io.add_key_event(imgui::Key::End, down),
        Key::PageUp => io.add_key_event(imgui::Key::PageUp, down),
        Key::PageDown => io.add_key_event(imgui::Key::PageDown, down),
        Key::Space => io.add_key_event(imgui::Key::Space, down),
        Key::Minus => io.add_key_event(imgui::Key::Minus, down),
        Key::Equals => io.add_key_event(imgui::Key::Equal, down),
        Key::LeftBrace => io.add_key_event(imgui::Key::LeftBracket, down),
        Key::RightBrace => io.add_key_event(imgui::Key::RightBracket, down),
        Key::Comma => io.add_key_event(imgui::Key::Comma, down),
        Key::Period => io.add_key_event(imgui::Key::Period, down),
        Key::Slash => io.add_key_event(imgui::Key::Slash, down),
        Key::Semicolon => io.add_key_event(imgui::Key::Semicolon, down),
        Key::Apostrophe => io.add_key_event(imgui::Key::Apostrophe, down),
        Key::Backslash => io.add_key_event(imgui::Key::Backslash, down),
        Key::Keypad0 => io.add_key_event(imgui::Key::Keypad0, down),
        Key::Keypad1 => io.add_key_event(imgui::Key::Keypad1, down),
        Key::Keypad2 => io.add_key_event(imgui::Key::Keypad2, down),
        Key::Keypad3 => io.add_key_event(imgui::Key::Keypad3, down),
        Key::Keypad4 => io.add_key_event(imgui::Key::Keypad4, down),
        Key::Keypad5 => io.add_key_event(imgui::Key::Keypad5, down),
        Key::Keypad6 => io.add_key_event(imgui::Key::Keypad6, down),
        Key::Keypad7 => io.add_key_event(imgui::Key::Keypad7, down),
        Key::Keypad8 => io.add_key_event(imgui::Key::Keypad8, down),
        Key::Keypad9 => io.add_key_event(imgui::Key::Keypad9, down),
        Key::KeypadDecimal => io.add_key_event(imgui::Key::KeypadDecimal, down),
        Key::KeypadDivide => io.add_key_event(imgui::Key::KeypadDivide, down),
        Key::KeypadMultiply => io.add_key_event(imgui::Key::KeypadMultiply, down),
        Key::KeypadSubtract => io.add_key_event(imgui::Key::KeypadSubtract, down),
        Key::KeypadAdd => io.add_key_event(imgui::Key::KeypadAdd, down),
        Key::KeypadEnter => io.add_key_event(imgui::Key::KeypadEnter, down),
        Key::KeypadEqual => io.add_key_event(imgui::Key::KeypadEqual, down),

        Key::Unknown => panic!("Unknown key"),
    }
}

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

        Event::KeyPress(key) => {
            handle_key_event(io, key, true);

            if io.want_capture_keyboard {
                Event::Continue
            } else {
                event
            }
        }

        Event::KeyRelease(key) => {
            handle_key_event(io, key, false);

            if io.want_capture_keyboard {
                Event::Continue
            } else {
                event
            }
        }
        _ => event,
    }
}
