pub trait AppSettings {
    fn get_window_dimensions(&self) -> (u16, u16);
}
