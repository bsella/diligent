pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _0,

    Minus,
    Equals,

    LeftBrace,
    RightBrace,

    Comma,
    Period,
    Slash,
    Semicolon,
    Quote,
    Asterisk,

    Backslash,
    Enter,
    Backspace,

    Up,
    Down,
    Left,
    Right,

    LeftCtrl,
    LeftAlt,
    LeftShift,
    LeftSuper,

    RightCtrl,
    RightAlt,
    RightShift,
    RightSuper,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Esc,
    Tab,
    Space,

    Unknown,
}

pub enum Event {
    MouseMove { x: i16, y: i16 },
    MouseDown { button: MouseButton },
    MouseUp { button: MouseButton },
    MouseWheel { up: bool },
    KeyPress(Key),
    KeyRelease(Key),
    Resize { width: u16, height: u16 },
    Continue,
    Quit,
}

pub trait EventHandler {
    type EventType;

    fn poll_event(&self) -> Option<Self::EventType>;
    fn handle_event(&mut self, event: &Self::EventType) -> Event;
}
