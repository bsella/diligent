use crate::{bindings::NativeWindow, core::engine_factory::EngineFactoryImplementation};

pub enum GoldenImageMode {
    None,
    Capture,
    Compare,
    CompareUpdate,
}

pub enum ApiImplementation {
    Vulkan,
    OpenGL,
}

pub trait App {
    fn new<EngineFactory: EngineFactoryImplementation>(
        engine_create_info: EngineFactory::EngineCreateInfo,
        window: Option<&NativeWindow>,
    ) -> Self;

    fn get_title(&self) -> &str;
    fn update(&mut self, current_time: f64, elapsed_time: f64);
    fn render(&self);
    fn present(&mut self);
    fn window_resize(&mut self, width: u32, height: u32);
    fn get_desired_initial_window_size(&self) -> Option<(i32, i32)> {
        None
    }
    fn get_golden_image_mode(&self) -> GoldenImageMode {
        GoldenImageMode::None
    }

    fn is_ready(&self) -> bool {
        false
    }
}
