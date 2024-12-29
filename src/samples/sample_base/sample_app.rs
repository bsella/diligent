use crate::tools::native_app::app::App;

struct SampleApp {
    m_app_title: String,
}

impl App for SampleApp {
    fn get_title(&self) -> &str {
        self.m_app_title.as_str()
    }

    fn update(&mut self, current_time: f64, elapsed_time: f64) {}

    fn render(&mut self) {}

    fn present(&mut self) {}

    fn window_resize(&mut self, width: i32, height: i32) {}
}
