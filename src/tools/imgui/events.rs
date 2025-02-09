pub trait ImguiEventHandler<EventType> {
    fn handle_event(&mut self, event: &EventType) -> bool;
}
