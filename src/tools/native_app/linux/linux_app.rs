pub trait LinuxApp {
    #[cfg(feature = "VULKAN_SUPPORTED")]
    fn init_vulkan(&self, connection: &xcb::Connection, window: &xcb::x::Window);

    #[cfg(feature = "VULKAN_SUPPORTED")]
    fn handle_xcb_event<EventType>(event: EventType)
    where
        EventType: xcb::GeEvent;
}
