use std::{
    ops::BitAnd,
    pin::Pin,
    sync::{Arc, Mutex, Weak},
};

use crate::native_app::Window;

use super::events::{Event, Key};

use diligent::platforms::native_window::NativeWindow;

use windows::{
    Win32::{
        Foundation::*, Graphics::Gdi::ValidateRect, System::LibraryLoader::GetModuleHandleW,
        UI::Input::KeyboardAndMouse::*, UI::WindowsAndMessaging::*,
    },
    core::*,
};

extern "system" fn handle_message(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_PAINT => {
            unsafe {
                let _ = ValidateRect(Some(hwnd), None);
            }
            LRESULT(0)
        }
        WM_SIZE => {
            let event_handler_ptr =
                unsafe { GetWindowLongPtrA(hwnd, GWL_USERDATA) } as *mut WindowHackData;

            if !event_handler_ptr.is_null() {
                unsafe {
                    (*event_handler_ptr).resized = true;
                    (*event_handler_ptr).resize_param = lparam;
                }
            }

            LRESULT(0)
        }

        WM_CLOSE => {
            let event_handler_ptr =
                unsafe { GetWindowLongPtrA(hwnd, GWL_USERDATA) } as *mut WindowHackData;

            if !event_handler_ptr.is_null() {
                unsafe {
                    (*event_handler_ptr).closing = true;
                }
            }

            LRESULT(0)
        }

        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}

struct Win32WindowManager {
    instance: HINSTANCE,

    window_class_name: PCWSTR,
}

unsafe impl Sync for Win32WindowManager {}
unsafe impl Send for Win32WindowManager {}

static WINDOW_MANAGER: Mutex<Weak<Win32WindowManager>> = Mutex::new(Weak::new());

struct WindowHackData {
    resize_param: LPARAM,
    resized: bool,
    closing: bool,
}

pub struct Win32Window {
    hwnd: HWND,

    hack: Pin<Box<WindowHackData>>,

    // This increments the static shared lazily-created window manager
    // and keep it until all the windows are dropped
    _window_manager: Arc<Win32WindowManager>,
}

impl Window for Win32Window {
    fn set_title(&self, title: &str) {
        unsafe {
            let _ = SetWindowTextW(self.hwnd, &HSTRING::from(title));
        }
    }

    fn native(&self) -> NativeWindow {
        NativeWindow::new(self.hwnd.0)
    }

    fn create(width: u32, height: u32) -> Self {
        let window_manager = {
            let mut weak = WINDOW_MANAGER.lock().unwrap();
            if let Some(window_manager) = weak.upgrade() {
                // The window manager exists and is being used by a window
                window_manager
            } else {
                // The window manager does not exist : create it
                let instance = unsafe { GetModuleHandleW(None) }.unwrap();

                debug_assert!(!instance.0.is_null());

                let instance = HINSTANCE(instance.0);

                let window_class_name = w!("DiligentWindow");

                let window_class = WNDCLASSW {
                    hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }.unwrap(),
                    hInstance: instance,
                    lpszClassName: window_class_name,

                    style: CS_HREDRAW | CS_VREDRAW,
                    lpfnWndProc: Some(handle_message),
                    ..Default::default()
                };

                let atom = unsafe { RegisterClassW(&window_class) };
                debug_assert!(atom != 0);

                let window_manger = Arc::new(Win32WindowManager {
                    instance,
                    window_class_name,
                });

                *weak = Arc::downgrade(&window_manger);

                window_manger
            }
        };

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window_manager.window_class_name,
                w!(""),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                Some(window_manager.instance),
                None,
            )
        }
        .unwrap();

        let hack = Box::pin(WindowHackData {
            resize_param: LPARAM::default(),
            resized: false,
            closing: false,
        });

        unsafe {
            SetWindowLongPtrA(
                hwnd,
                GWL_USERDATA,
                std::ptr::from_ref(Pin::as_ref(&hack).get_ref()) as _,
            )
        };

        Win32Window {
            hwnd,
            hack,
            _window_manager: window_manager,
        }
    }

    fn handle_event(&mut self) -> Option<Event> {
        let event = {
            if self.hack.resized {
                return Some(MSG {
                    message: WM_SIZE,
                    lParam: self.hack.resize_param,
                    ..Default::default()
                });
            }

            if self.hack.closing {
                return Some(MSG {
                    message: WM_CLOSE,
                    ..Default::default()
                });
            }

            let mut msg = std::mem::MaybeUninit::<MSG>::uninit();

            if unsafe { PeekMessageW(msg.as_mut_ptr(), Some(self.hwnd), 0, 0, PM_REMOVE).as_bool() }
            {
                let msg = unsafe { msg.assume_init() };
                unsafe {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

                Some(msg)
            } else {
                None
            }
        };

        event.map(|event| {
            match event.message {
                WM_CLOSE => Event::Quit,
                WM_MOUSEMOVE => Event::MouseMove {
                    x: (event.lParam.0 & 0xffff) as i16,
                    y: ((event.lParam.0 >> 16) & 0xffff) as i16,
                },
                WM_LBUTTONDOWN => Event::MouseDown {
                    button: super::events::MouseButton::Left,
                },
                WM_LBUTTONUP => Event::MouseUp {
                    button: super::events::MouseButton::Left,
                },
                WM_RBUTTONDOWN => Event::MouseDown {
                    button: super::events::MouseButton::Right,
                },
                WM_RBUTTONUP => Event::MouseUp {
                    button: super::events::MouseButton::Right,
                },
                WM_MBUTTONDOWN => Event::MouseDown {
                    button: super::events::MouseButton::Middle,
                },
                WM_MBUTTONUP => Event::MouseUp {
                    button: super::events::MouseButton::Middle,
                },
                WM_SIZE => {
                    self.hack.resized = false;

                    Event::Resize {
                        width: (event.lParam.0 & 0xffff) as _,
                        height: ((event.lParam.0 >> 16) & 0xffff) as _,
                    }
                }

                WM_KEYDOWN => match VIRTUAL_KEY(event.wParam.0 as u16) {
                    VK_A => Event::KeyPress(Key::A),
                    VK_B => Event::KeyPress(Key::B),
                    VK_C => Event::KeyPress(Key::C),
                    VK_D => Event::KeyPress(Key::D),
                    VK_E => Event::KeyPress(Key::E),
                    VK_F => Event::KeyPress(Key::F),
                    VK_G => Event::KeyPress(Key::G),
                    VK_H => Event::KeyPress(Key::H),
                    VK_I => Event::KeyPress(Key::I),
                    VK_J => Event::KeyPress(Key::J),
                    VK_K => Event::KeyPress(Key::K),
                    VK_L => Event::KeyPress(Key::L),
                    VK_M => Event::KeyPress(Key::M),
                    VK_N => Event::KeyPress(Key::N),
                    VK_O => Event::KeyPress(Key::O),
                    VK_P => Event::KeyPress(Key::P),
                    VK_Q => Event::KeyPress(Key::Q),
                    VK_R => Event::KeyPress(Key::R),
                    VK_S => Event::KeyPress(Key::S),
                    VK_T => Event::KeyPress(Key::T),
                    VK_U => Event::KeyPress(Key::U),
                    VK_V => Event::KeyPress(Key::V),
                    VK_W => Event::KeyPress(Key::W),
                    VK_X => Event::KeyPress(Key::X),
                    VK_Y => Event::KeyPress(Key::Y),
                    VK_Z => Event::KeyPress(Key::Z),
                    VK_1 => Event::KeyPress(Key::_1),
                    VK_2 => Event::KeyPress(Key::_2),
                    VK_3 => Event::KeyPress(Key::_3),
                    VK_4 => Event::KeyPress(Key::_4),
                    VK_5 => Event::KeyPress(Key::_5),
                    VK_6 => Event::KeyPress(Key::_6),
                    VK_7 => Event::KeyPress(Key::_7),
                    VK_8 => Event::KeyPress(Key::_8),
                    VK_9 => Event::KeyPress(Key::_9),
                    VK_0 => Event::KeyPress(Key::_0),
                    VK_OEM_MINUS => Event::KeyPress(Key::Minus),
                    VK_OEM_NEC_EQUAL => Event::KeyPress(Key::Equals),
                    VK_OEM_4 => Event::KeyPress(Key::LeftBrace),
                    VK_OEM_6 => Event::KeyPress(Key::RightBrace),
                    VK_OEM_COMMA => Event::KeyPress(Key::Comma),
                    VK_OEM_PERIOD => Event::KeyPress(Key::Period),
                    VK_OEM_2 => Event::KeyPress(Key::Slash),
                    VK_OEM_1 => Event::KeyPress(Key::Semicolon),
                    VK_OEM_7 => Event::KeyPress(Key::Apostrophe),
                    VK_MULTIPLY => Event::KeyPress(Key::KeypadMultiply),
                    VK_OEM_5 => Event::KeyPress(Key::Backslash),
                    VK_RETURN => Event::KeyPress(Key::Enter),
                    VK_BACK => Event::KeyPress(Key::Backspace),
                    VK_UP => Event::KeyPress(Key::Up),
                    VK_DOWN => Event::KeyPress(Key::Down),
                    VK_LEFT => Event::KeyPress(Key::Left),
                    VK_RIGHT => Event::KeyPress(Key::Right),
                    VK_MENU => {
                        if event.lParam.0.bitand(1 << 24) == 0 {
                            Event::KeyPress(Key::LeftAlt)
                        } else {
                            Event::KeyPress(Key::RightAlt)
                        }
                    }
                    VK_LMENU => Event::KeyPress(Key::LeftAlt),
                    VK_RMENU => Event::KeyPress(Key::RightAlt),
                    VK_SHIFT => {
                        let scancode = event.lParam.0.bitand(0x00ff0000) >> 16;
                        if unsafe { MapVirtualKeyW(scancode as _, MAPVK_VSC_TO_VK_EX) }
                            == VK_LSHIFT.0 as _
                        {
                            Event::KeyPress(Key::LeftShift)
                        } else {
                            Event::KeyPress(Key::RightShift)
                        }
                    }
                    VK_LSHIFT => Event::KeyPress(Key::LeftShift),
                    VK_RSHIFT => Event::KeyPress(Key::RightShift),
                    VK_LWIN => Event::KeyPress(Key::LeftSuper),
                    VK_CONTROL => {
                        if event.lParam.0.bitand(1 << 24) == 0 {
                            Event::KeyPress(Key::LeftCtrl)
                        } else {
                            Event::KeyPress(Key::RightCtrl)
                        }
                    }
                    VK_LCONTROL => Event::KeyPress(Key::LeftCtrl),
                    VK_RCONTROL => Event::KeyPress(Key::RightCtrl),
                    VK_INSERT => Event::KeyPress(Key::Insert),
                    VK_DELETE => Event::KeyPress(Key::Delete),
                    VK_HOME => Event::KeyPress(Key::Home),
                    VK_END => Event::KeyPress(Key::End),
                    VK_PRIOR => Event::KeyPress(Key::PageUp),
                    VK_NEXT => Event::KeyPress(Key::PageDown),
                    VK_F1 => Event::KeyPress(Key::F1),
                    VK_F2 => Event::KeyPress(Key::F2),
                    VK_F3 => Event::KeyPress(Key::F3),
                    VK_F4 => Event::KeyPress(Key::F4),
                    VK_F5 => Event::KeyPress(Key::F5),
                    VK_F6 => Event::KeyPress(Key::F6),
                    VK_F7 => Event::KeyPress(Key::F7),
                    VK_F8 => Event::KeyPress(Key::F8),
                    VK_F9 => Event::KeyPress(Key::F9),
                    VK_F10 => Event::KeyPress(Key::F10),
                    VK_F11 => Event::KeyPress(Key::F11),
                    VK_F12 => Event::KeyPress(Key::F12),
                    VK_ESCAPE => Event::KeyPress(Key::Esc),
                    VK_TAB => Event::KeyPress(Key::Tab),
                    VK_SPACE => Event::KeyPress(Key::Space),
                    _ => {
                        println!("{}", event.wParam.0);
                        Event::KeyPress(Key::Unknown)
                    }
                },
                WM_KEYUP => match VIRTUAL_KEY(event.wParam.0 as u16) {
                    VK_A => Event::KeyRelease(Key::A),
                    VK_B => Event::KeyRelease(Key::B),
                    VK_C => Event::KeyRelease(Key::C),
                    VK_D => Event::KeyRelease(Key::D),
                    VK_E => Event::KeyRelease(Key::E),
                    VK_F => Event::KeyRelease(Key::F),
                    VK_G => Event::KeyRelease(Key::G),
                    VK_H => Event::KeyRelease(Key::H),
                    VK_I => Event::KeyRelease(Key::I),
                    VK_J => Event::KeyRelease(Key::J),
                    VK_K => Event::KeyRelease(Key::K),
                    VK_L => Event::KeyRelease(Key::L),
                    VK_M => Event::KeyRelease(Key::M),
                    VK_N => Event::KeyRelease(Key::N),
                    VK_O => Event::KeyRelease(Key::O),
                    VK_P => Event::KeyRelease(Key::P),
                    VK_Q => Event::KeyRelease(Key::Q),
                    VK_R => Event::KeyRelease(Key::R),
                    VK_S => Event::KeyRelease(Key::S),
                    VK_T => Event::KeyRelease(Key::T),
                    VK_U => Event::KeyRelease(Key::U),
                    VK_V => Event::KeyRelease(Key::V),
                    VK_W => Event::KeyRelease(Key::W),
                    VK_X => Event::KeyRelease(Key::X),
                    VK_Y => Event::KeyRelease(Key::Y),
                    VK_Z => Event::KeyRelease(Key::Z),
                    VK_1 => Event::KeyRelease(Key::_1),
                    VK_2 => Event::KeyRelease(Key::_2),
                    VK_3 => Event::KeyRelease(Key::_3),
                    VK_4 => Event::KeyRelease(Key::_4),
                    VK_5 => Event::KeyRelease(Key::_5),
                    VK_6 => Event::KeyRelease(Key::_6),
                    VK_7 => Event::KeyRelease(Key::_7),
                    VK_8 => Event::KeyRelease(Key::_8),
                    VK_9 => Event::KeyRelease(Key::_9),
                    VK_0 => Event::KeyRelease(Key::_0),
                    VK_OEM_MINUS => Event::KeyRelease(Key::Minus),
                    VK_OEM_NEC_EQUAL => Event::KeyRelease(Key::Equals),
                    VK_OEM_4 => Event::KeyRelease(Key::LeftBrace),
                    VK_OEM_6 => Event::KeyRelease(Key::RightBrace),
                    VK_OEM_COMMA => Event::KeyRelease(Key::Comma),
                    VK_OEM_PERIOD => Event::KeyRelease(Key::Period),
                    VK_OEM_2 => Event::KeyRelease(Key::Slash),
                    VK_OEM_1 => Event::KeyRelease(Key::Semicolon),
                    VK_OEM_7 => Event::KeyRelease(Key::Apostrophe),
                    VK_MULTIPLY => Event::KeyRelease(Key::KeypadMultiply),
                    VK_OEM_5 => Event::KeyRelease(Key::Backslash),
                    VK_RETURN => Event::KeyRelease(Key::Enter),
                    VK_BACK => Event::KeyRelease(Key::Backspace),
                    VK_UP => Event::KeyRelease(Key::Up),
                    VK_DOWN => Event::KeyRelease(Key::Down),
                    VK_LEFT => Event::KeyRelease(Key::Left),
                    VK_RIGHT => Event::KeyRelease(Key::Right),
                    VK_MENU => {
                        if event.lParam.0.bitand(1 << 24) == 0 {
                            Event::KeyRelease(Key::LeftAlt)
                        } else {
                            Event::KeyRelease(Key::RightAlt)
                        }
                    }
                    VK_LMENU => Event::KeyRelease(Key::LeftAlt),
                    VK_RMENU => Event::KeyRelease(Key::RightAlt),
                    VK_SHIFT => {
                        let scancode = event.lParam.0.bitand(0x00ff0000) >> 16;
                        if unsafe { MapVirtualKeyW(scancode as _, MAPVK_VSC_TO_VK_EX) }
                            == VK_LSHIFT.0 as _
                        {
                            Event::KeyRelease(Key::LeftShift)
                        } else {
                            Event::KeyRelease(Key::RightShift)
                        }
                    }
                    VK_LSHIFT => Event::KeyRelease(Key::LeftShift),
                    VK_RSHIFT => Event::KeyRelease(Key::RightShift),
                    VK_LWIN => Event::KeyRelease(Key::LeftSuper),
                    VK_CONTROL => {
                        if event.lParam.0.bitand(1 << 24) == 0 {
                            Event::KeyRelease(Key::LeftCtrl)
                        } else {
                            Event::KeyRelease(Key::RightCtrl)
                        }
                    }
                    VK_LCONTROL => Event::KeyRelease(Key::LeftCtrl),
                    VK_RCONTROL => Event::KeyRelease(Key::RightCtrl),
                    VK_INSERT => Event::KeyRelease(Key::Insert),
                    VK_DELETE => Event::KeyRelease(Key::Delete),
                    VK_HOME => Event::KeyRelease(Key::Home),
                    VK_END => Event::KeyRelease(Key::End),
                    VK_PRIOR => Event::KeyRelease(Key::PageUp),
                    VK_NEXT => Event::KeyRelease(Key::PageDown),
                    VK_F1 => Event::KeyRelease(Key::F1),
                    VK_F2 => Event::KeyRelease(Key::F2),
                    VK_F3 => Event::KeyRelease(Key::F3),
                    VK_F4 => Event::KeyRelease(Key::F4),
                    VK_F5 => Event::KeyRelease(Key::F5),
                    VK_F6 => Event::KeyRelease(Key::F6),
                    VK_F7 => Event::KeyRelease(Key::F7),
                    VK_F8 => Event::KeyRelease(Key::F8),
                    VK_F9 => Event::KeyRelease(Key::F9),
                    VK_F10 => Event::KeyRelease(Key::F10),
                    VK_F11 => Event::KeyRelease(Key::F11),
                    VK_F12 => Event::KeyRelease(Key::F12),
                    VK_ESCAPE => Event::KeyRelease(Key::Esc),
                    VK_TAB => Event::KeyRelease(Key::Tab),
                    VK_SPACE => Event::KeyRelease(Key::Space),
                    _ => Event::KeyRelease(Key::Unknown),
                },
                _ => {
                    //println!("{}", event.message);
                    Event::Continue
                }
            }
        })
    }
}

impl Drop for Win32Window {
    fn drop(&mut self) {
        let _ = unsafe { DestroyWindow(self.hwnd) };
    }
}

impl Drop for Win32WindowManager {
    fn drop(&mut self) {
        let _ = unsafe { UnregisterClassW(self.window_class_name, Some(self.instance)) };
    }
}
