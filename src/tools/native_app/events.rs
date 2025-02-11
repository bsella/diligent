pub enum MouseButton {
    Left,
    Right,
    Middle,
}
pub enum EventResult {
    MouseMove { x: i16, y: i16 },
    MouseDown { button: MouseButton },
    MouseUp { button: MouseButton },
    MouseWheel { up: bool },
    KeyPress { keycode: u8 },
    KeyRelease { keycode: u8 },
    Resize { width: u16, height: u16 },
    Continue,
    Quit,
}

pub trait EventHandler {
    type EventType;

    fn poll_event(&self) -> Option<Self::EventType>;
    fn handle_event(&mut self, event: &Self::EventType) -> EventResult;
}
