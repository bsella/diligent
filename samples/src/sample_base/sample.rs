use diligent::*;

#[cfg(feature = "vulkan")]
use diligent::vk::engine_factory_vk::EngineVkCreateInfo;

#[cfg(feature = "opengl")]
use diligent::gl::engine_factory_gl::EngineGLCreateInfo;

#[cfg(feature = "d3d11")]
use diligent::d3d11::engine_factory_d3d11::EngineD3D11CreateInfo;

#[cfg(feature = "d3d12")]
use diligent::d3d12::engine_factory_d3d12::EngineD3D12CreateInfo;

use crate::window::native_app::events::Event;
use imgui::Ui;

use crate::sample_base::sample_app_settings::SampleAppSettings;

// Returns projection matrix adjusted to the current screen orientation
pub fn get_adjusted_projection_matrix(
    swap_chain_desc: &SwapChainDesc,
    fov_y: f32,
    near_plane: f32,
    far_plane: f32,
) -> glam::Mat4 {
    let aspect_ratio = swap_chain_desc.width() as f32 / swap_chain_desc.height() as f32;

    let fov = match swap_chain_desc.pre_transform() {
        SurfaceTransform::Rotate90
        | SurfaceTransform::Rotate270
        | SurfaceTransform::HorizontalMirrorRotate90
        | SurfaceTransform::HorizontalMirrorRotate270 => {
            // When the screen is rotated, vertical FOV becomes horizontal FOV
            fov_y * aspect_ratio
        }

        _ => fov_y,
    };

    glam::camera::lh::proj::directx::perspective(fov, aspect_ratio, near_plane, far_plane)
}

// Returns pretransform matrix that matches the current screen rotation
pub fn get_surface_pretransform_matrix(
    pre_transform: SurfaceTransform,
    camera_view_axis: &glam::Vec3,
) -> glam::Mat4 {
    match pre_transform {
        SurfaceTransform::Rotate90 =>
        // The image content is rotated 90 degrees clockwise.
        {
            glam::Mat4::from_quat(glam::Quat::from_axis_angle(
                *camera_view_axis,
                -std::f32::consts::PI / 2.0,
            ))
        }

        SurfaceTransform::Rotate180 =>
        // The image content is rotated 180 degrees clockwise.
        {
            glam::Mat4::from_quat(glam::Quat::from_axis_angle(
                *camera_view_axis,
                -std::f32::consts::PI,
            ))
        }

        SurfaceTransform::Rotate270 =>
        // The image content is rotated 270 degrees clockwise.
        {
            glam::Mat4::from_quat(glam::Quat::from_axis_angle(
                *camera_view_axis,
                -std::f32::consts::PI * 3.0 / 2.0,
            ))
        }

        SurfaceTransform::Optimal => panic!(
            "SURFACE_TRANSFORM_OPTIMAL is only valid as parameter during swap chain initialization."
        ),

        SurfaceTransform::HorizontalMirror
        | SurfaceTransform::HorizontalMirrorRotate90
        | SurfaceTransform::HorizontalMirrorRotate180
        | SurfaceTransform::HorizontalMirrorRotate270 => {
            panic!("Mirror transforms are not supported")
        }

        _ => glam::Mat4::IDENTITY,
    }
}

pub trait SampleBase {
    fn new(
        engine_factory: &EngineFactory,
        render_device: &RenderDevice,
        main_context: &ImmediateDeviceContext,
        other_immediate_contexts: Vec<Boxed<ImmediateDeviceContext>>,
        deferred_contexts: Vec<Boxed<DeferredDeviceContext>>,
        windows: &[&SwapChainDesc],
    ) -> Self;

    fn make_swap_chains_create_info(settings: &SampleAppSettings) -> Vec<SwapChainCreateInfo> {
        // By default, the sample only created one window with one swap chain
        vec![
            SwapChainCreateInfo::builder()
                .width(settings.width)
                .height(settings.height)
                .build(),
        ]
    }

    fn render(
        &self,
        main_context: Boxed<ImmediateDeviceContext>,
        _swap_chain: &mut SwapChain,
    ) -> Boxed<ImmediateDeviceContext> {
        main_context
    }

    fn update(
        &mut self,
        _main_context: &ImmediateDeviceContext,
        _current_time: f64,
        _elapsed_time: f64,
    ) {
    }

    fn update_ui(
        &mut self,
        _device: &RenderDevice,
        _main_context: &ImmediateDeviceContext,
        _ui: &mut Ui,
    ) {
    }

    fn get_name() -> &'static str;

    fn pre_window_resize(&mut self) {}

    fn window_resize(&mut self, _device: &RenderDevice, _new_swap_chain: &SwapChainDesc) {}

    fn handle_event(&mut self, _event: Event) {}

    fn release_swap_chain_buffers(&mut self) {}

    fn required_features() -> DeviceFeatures {
        DeviceFeatures::default()
    }

    fn num_deferred_contexts() -> usize {
        0
    }

    // TODO : replace the following methods with a generic when specialization
    // is stabilized in Rust
    #[cfg(feature = "vulkan")]
    fn modify_engine_init_info_vk(_engine_ci: &mut EngineVkCreateInfo) {}
    #[cfg(feature = "opengl")]
    fn modify_engine_init_info_gl(_engine_ci: &mut EngineGLCreateInfo) {}
    #[cfg(feature = "d3d11")]
    fn modify_engine_init_info_d3d11(_engine_ci: &mut EngineD3D11CreateInfo) {}
    #[cfg(feature = "d3d12")]
    fn modify_engine_init_info_d3d12(_engine_ci: &mut EngineD3D12CreateInfo) {}
}
