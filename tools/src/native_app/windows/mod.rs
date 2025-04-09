use super::{app::App, app_settings::AppSettings, events::EventHandler, events::EventResult};

use diligent::{engine_factory::EngineCreateInfo, platforms::native_window::NativeWindow};

use windows::{
    core::*,
    Win32::{
        Foundation::*, Graphics::Gdi::ValidateRect, System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

struct Win32EventHandler {
    _hwnd: HWND,
    resize_param: LPARAM,
    resized: bool,
}

impl<'a> EventHandler for Win32EventHandler {
    type EventType = MSG;

    fn poll_event(&self) -> Option<Self::EventType> {
        if self.resized {
            return Some(MSG {
                message: WM_SIZE,
                lParam: self.resize_param,
                ..Default::default()
            });
        }

        let mut msg = std::mem::MaybeUninit::<MSG>::uninit();

        if unsafe { PeekMessageW(msg.as_mut_ptr(), None, 0, 0, PM_REMOVE).as_bool() } {
            let msg = unsafe { msg.assume_init() };
            unsafe {
                let _ = TranslateMessage(std::ptr::addr_of!(msg));
                DispatchMessageW(std::ptr::addr_of!(msg));
            }

            Some(msg)
        } else {
            None
        }
    }

    fn handle_event(&mut self, event: &Self::EventType) -> EventResult {
        match event.message {
            WM_QUIT => EventResult::Quit,
            WM_MOUSEMOVE => EventResult::MouseMove {
                x: (event.lParam.0 & 0xffff) as i16,
                y: ((event.lParam.0 >> 16) & 0xffff) as i16,
            },
            WM_LBUTTONDOWN => EventResult::MouseDown {
                button: super::events::MouseButton::Left,
            },
            WM_LBUTTONUP => EventResult::MouseUp {
                button: super::events::MouseButton::Left,
            },
            WM_RBUTTONDOWN => EventResult::MouseDown {
                button: super::events::MouseButton::Right,
            },
            WM_RBUTTONUP => EventResult::MouseUp {
                button: super::events::MouseButton::Right,
            },
            WM_MBUTTONDOWN => EventResult::MouseDown {
                button: super::events::MouseButton::Middle,
            },
            WM_MBUTTONUP => EventResult::MouseUp {
                button: super::events::MouseButton::Middle,
            },
            WM_SIZE => {
                self.resized = false;

                EventResult::Resize {
                    width: (event.lParam.0 & 0xffff) as _,
                    height: ((event.lParam.0 >> 16) & 0xffff) as _,
                }
            }
            WM_NCMOUSEMOVE => {
                unsafe {
                    SetWindowLongPtrA(event.hwnd, GWL_USERDATA, std::ptr::from_ref(self) as _);
                }
                EventResult::Continue
            }
            _ => {
                //println!("{}", event.message);
                EventResult::Continue
            }
        }
    }
}

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
                unsafe { GetWindowLongPtrA(hwnd, GWL_USERDATA) } as *mut Win32EventHandler;

            if !event_handler_ptr.is_null() {
                unsafe {
                    (*event_handler_ptr).resized = true;
                    (*event_handler_ptr).resize_param = lparam;
                }
            }

            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}

pub fn main<Application>(
    settings: Application::AppSettings,
) -> std::result::Result<(), std::io::Error>
where
    Application: App,
{
    let instance = unsafe { GetModuleHandleW(None) }?;

    debug_assert!(!instance.0.is_null());

    let instance = HINSTANCE(instance.0);

    let window_class = w!("DiligentWindow");

    let wc = WNDCLASSW {
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }?,
        hInstance: instance,
        lpszClassName: window_class,

        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(handle_message),
        ..Default::default()
    };

    let atom = unsafe { RegisterClassW(&wc) };
    debug_assert!(atom != 0);

    let (width, height) = settings.get_window_dimensions();

    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!(""),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width as i32,
            height as i32,
            None,
            None,
            Some(instance),
            None,
        )
    }?;

    Application::new(
        settings,
        EngineCreateInfo::default(),
        Some(&NativeWindow(hwnd.0)),
    )
    .run(
        Win32EventHandler {
            _hwnd: hwnd,
            resize_param: LPARAM::default(),
            resized: false,
        },
        &|title: &str| unsafe {
            let _ = SetWindowTextW(hwnd, &HSTRING::from(title));
        },
    )
}
