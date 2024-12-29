pub enum GoldenImageMode {
    None,
    Capture,
    Compare,
    CompareUpdate,
}

pub trait App {
    fn get_title(&self) -> &str;
    fn update(&mut self, current_time: f64, elapsed_time: f64);
    fn render(&mut self);
    fn present(&mut self);
    fn window_resize(&mut self, width: i32, height: i32);
    fn get_desired_initial_window_size(&self) -> (i32, i32) {
        (0, 0)
    }
    fn get_golden_image_mode(&self) -> GoldenImageMode {
        GoldenImageMode::None
    }

    fn is_ready(&self) -> bool {
        false
    }
}
