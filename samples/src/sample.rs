use diligent::{
    device_context::{DeferredDeviceContext, ImmediateDeviceContext},
    engine_factory::EngineFactory,
    graphics_types::SurfaceTransform,
    render_device::RenderDevice,
    swap_chain::{SwapChain, SwapChainDesc},
};

use diligent_tools::native_app::events::EventResult;
use imgui::Ui;

// Returns projection matrix adjusted to the current screen orientation
pub fn get_adjusted_projection_matrix(
    swap_chain_desc: &SwapChainDesc,
    fov_y: f32,
    near_plane: f32,
    far_plane: f32,
) -> glam::Mat4 {
    let aspect_ratio = swap_chain_desc.width as f32 / swap_chain_desc.height as f32;

    let fov = match swap_chain_desc.pre_transform {
        SurfaceTransform::Rotate90
        | SurfaceTransform::Rotate270
        | SurfaceTransform::HorizontalMirrorRotate90
        | SurfaceTransform::HorizontalMirrorRotate270 => {
            // When the screen is rotated, vertical FOV becomes horizontal FOV
            fov_y * aspect_ratio
        }

        _ => fov_y,
    };

    glam::Mat4::perspective_lh(fov, aspect_ratio, near_plane, far_plane)
}

// Returns pretransform matrix that matches the current screen rotation
pub fn get_surface_pretransform_matrix(
    swap_chain_desc: &SwapChainDesc,
    camera_view_axis: &glam::Vec3,
) -> glam::Mat4 {
    match swap_chain_desc.pre_transform
    {
        SurfaceTransform::Rotate90 =>
            // The image content is rotated 90 degrees clockwise.
            glam::Mat4::from_quat(glam::Quat::from_axis_angle(*camera_view_axis, -std::f32::consts::PI / 2.0)),

            SurfaceTransform::Rotate180 =>
        // The image content is rotated 180 degrees clockwise.
        glam::Mat4::from_quat(glam::Quat::from_axis_angle(*camera_view_axis, -std::f32::consts::PI)),

        SurfaceTransform::Rotate270 =>
        // The image content is rotated 270 degrees clockwise.
        glam::Mat4::from_quat(glam::Quat::from_axis_angle(*camera_view_axis, -std::f32::consts::PI* 3.0 / 2.0)),

        SurfaceTransform::Optimal=>
            panic!("SURFACE_TRANSFORM_OPTIMAL is only valid as parameter during swap chain initialization."),

        SurfaceTransform::HorizontalMirror|
        SurfaceTransform::HorizontalMirrorRotate90|
        SurfaceTransform::HorizontalMirrorRotate180|
        SurfaceTransform::HorizontalMirrorRotate270 =>
            panic!("Mirror transforms are not supported"),

        _=> glam::Mat4::IDENTITY
    }
}

pub trait SampleBase {
    fn new(
        engine_factory: &EngineFactory,
        render_device: RenderDevice,
        immediate_contexts: Vec<ImmediateDeviceContext>,
        deferred_contexts: Vec<DeferredDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self;

    fn get_render_device(&self) -> &RenderDevice;

    fn get_immediate_context(&self) -> &ImmediateDeviceContext;

    fn render(&self, _swap_chain: &SwapChain) {}

    fn update(&mut self, _current_time: f64, _elapsed_time: f64) {}

    fn update_ui(&mut self, _ui: &mut Ui) {}

    fn get_name() -> &'static str;

    fn pre_window_resize(&mut self) {}

    fn window_resize(&mut self, _width: u32, _height: u32) {}

    fn handle_event(&mut self, _event: EventResult) {}
}
