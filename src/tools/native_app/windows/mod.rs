use super::app::App;

pub struct NativeWindow {
    hwnd: *mut std::ffi::c_void,
}

impl From<&NativeWindow> for diligent_sys::NativeWindow {
    fn from(value: &NativeWindow) -> Self {
        diligent_sys::NativeWindow { hWnd: value.hwnd }
    }
}

pub fn main<Application>(_settings: Application::AppSettings) -> Result<(), std::io::Error>
where
    Application: App,
{
    //win32::main::<Application>(settings)
    Ok(())
}
