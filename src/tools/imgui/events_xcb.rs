use std::ops::BitAnd;

use super::events::ImguiEventHandler;

enum KeySymbols {
    COOKIE(xcb::x::GetKeyboardMappingCookie),
    VALUE(xcb::x::GetKeyboardMappingReply),
}

pub struct ImguiXcbEventHandler<'a> {
    io: &'a mut imgui::Io,
    key_symbols: KeySymbols,
    last_timestamp: f32,
}

impl<'a> ImguiXcbEventHandler<'a> {
    pub fn new(io: &'a mut imgui::Io, connection: &xcb::Connection) -> Self {
        let setup = connection.get_setup();
        let min_keycode = setup.min_keycode();
        let max_keycode = setup.max_keycode();

        let cookie = connection.send_request(&xcb::x::GetKeyboardMapping {
            first_keycode: min_keycode,
            count: max_keycode - min_keycode + 1,
        });

        //Keyboard mapping. ImGui will use those indices to peek into the io.KeysDown[] array that we will update during the application lifetime.
        io.key_map[imgui::Key::Tab as usize] = 0x17;
        io.key_map[imgui::Key::LeftArrow as usize] = 0x71;
        io.key_map[imgui::Key::RightArrow as usize] = 0x72;
        io.key_map[imgui::Key::UpArrow as usize] = 0x6F;
        io.key_map[imgui::Key::DownArrow as usize] = 0x74;
        io.key_map[imgui::Key::PageUp as usize] = 0x70;
        io.key_map[imgui::Key::PageDown as usize] = 0x75;
        io.key_map[imgui::Key::Home as usize] = 0x6E;
        io.key_map[imgui::Key::End as usize] = 0x73;
        io.key_map[imgui::Key::Insert as usize] = 0x76;
        io.key_map[imgui::Key::Delete as usize] = 0x77;
        io.key_map[imgui::Key::Backspace as usize] = 0x16;
        //io.key_map[imgui::Key::Space as usize] = 0;//VK_SPACE;
        io.key_map[imgui::Key::Enter as usize] = 0x24;
        io.key_map[imgui::Key::Escape as usize] = 0x09;
        io.key_map[imgui::Key::KeypadEnter as usize] = 0x68;
        io.key_map[imgui::Key::A as usize] = 'A' as u32;
        io.key_map[imgui::Key::C as usize] = 'C' as u32;
        io.key_map[imgui::Key::V as usize] = 'V' as u32;
        io.key_map[imgui::Key::X as usize] = 'X' as u32;
        io.key_map[imgui::Key::Y as usize] = 'Y' as u32;
        io.key_map[imgui::Key::Z as usize] = 'Z' as u32;

        ImguiXcbEventHandler {
            io: io,
            key_symbols: KeySymbols::COOKIE(cookie),
            last_timestamp: 0.0,
        }
    }
}

impl<'a> ImguiEventHandler<xcb::Event> for ImguiXcbEventHandler<'a> {
    fn handle_event(&mut self, event: &xcb::Event) -> bool {
        let handled = match event {
            xcb::Event::X(xcb::x::Event::MotionNotify(motion_event)) => {
                self.io.mouse_pos = [motion_event.event_x() as f32, motion_event.event_y() as f32];
                self.io.want_capture_mouse
            }

            xcb::Event::X(xcb::x::Event::ButtonPress(press_event)) => {
                match press_event.detail() {
                    1 => self.io.mouse_down[0] = true,
                    2 => self.io.mouse_down[2] = true,
                    3 => self.io.mouse_down[1] = true,
                    4 => self.io.mouse_wheel += 1.0,
                    5 => self.io.mouse_wheel -= 1.0,
                    _ => {}
                };
                self.io.want_capture_mouse
            }

            xcb::Event::X(xcb::x::Event::ButtonRelease(release_event)) => {
                match release_event.detail() {
                    1 => self.io.mouse_down[0] = false,
                    2 => self.io.mouse_down[2] = false,
                    3 => self.io.mouse_down[1] = false,
                    _ => {}
                };
                self.io.want_capture_mouse
            }

            xcb::Event::X(xcb::x::Event::KeyRelease(key_event))
            | xcb::Event::X(xcb::x::Event::KeyPress(key_event)) => {
                let is_key_pressed = key_event.response_type().bitand(0x7f) as u32
                    == xcb::x::EventMask::KEY_PRESS.bits();

                self.io.key_ctrl = key_event.state().contains(xcb::x::KeyButMask::CONTROL);
                self.io.key_shift = key_event.state().contains(xcb::x::KeyButMask::SHIFT);
                self.io.key_alt = key_event.state().contains(xcb::x::KeyButMask::MOD1);

                let k = match key_event.detail() {
                    0x09 => self.io.key_map[imgui::Key::Escape as usize],
                    0x6F => self.io.key_map[imgui::Key::UpArrow as usize],
                    0x74 => self.io.key_map[imgui::Key::DownArrow as usize],
                    0x72 => self.io.key_map[imgui::Key::RightArrow as usize],
                    0x71 => self.io.key_map[imgui::Key::LeftArrow as usize],
                    0x24 => self.io.key_map[imgui::Key::Enter as usize],
                    0x76 => self.io.key_map[imgui::Key::Insert as usize],
                    0x77 => self.io.key_map[imgui::Key::Delete as usize],
                    0x16 => self.io.key_map[imgui::Key::Backspace as usize],
                    0x6E => self.io.key_map[imgui::Key::Home as usize],
                    0x17 => self.io.key_map[imgui::Key::Tab as usize],
                    0x73 => self.io.key_map[imgui::Key::End as usize],
                    0x68 => self.io.key_map[imgui::Key::KeypadEnter as usize],
                    0x70 => self.io.key_map[imgui::Key::PageUp as usize],
                    0x75 => self.io.key_map[imgui::Key::PageDown as usize],
                    _ => 0,
                };

                self.io.want_capture_keyboard
            }

            xcb::Event::X(xcb::x::Event::ConfigureNotify(configure_event)) => {
                self.io.display_size = [
                    configure_event.width() as f32,
                    configure_event.height() as f32,
                ];
                false
            }
            _ => false,
        };

        let force_pass_event_to_sample = match event {
            xcb::Event::X(xcb::x::Event::MotionNotify(_))
            | xcb::Event::X(xcb::x::Event::ButtonRelease(_))
            | xcb::Event::X(xcb::x::Event::KeyRelease(_)) => true,
            _ => false,
        };

        !handled || force_pass_event_to_sample
    }
}
