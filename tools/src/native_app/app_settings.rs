use diligent::RenderDeviceType;

pub trait AppSettings {
    fn get_render_device_type(&self) -> RenderDeviceType;
    fn get_window_dimensions(&self) -> (u32, u32);
}
