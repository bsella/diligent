use diligent::graphics_types::RenderDeviceType;

pub trait AppSettings {
    fn get_render_device_type(&self) -> RenderDeviceType;
    fn get_window_dimensions(&self) -> (u16, u16);
}
