pub enum EventResult {
    Resize { width: u16, height: u16 },
    Continue,
    Quit,
}

pub trait EventHandler {
    type EventType;

    fn poll_event(&self) -> Option<Self::EventType>;
    fn handle_event(&mut self, event: &Self::EventType) -> EventResult;
}
