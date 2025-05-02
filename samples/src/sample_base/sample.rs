use std::ops::{Deref, DerefMut};

use diligent::{
    device_context::{DeferredDeviceContext, ImmediateDeviceContext},
    engine_factory::EngineFactory,
    graphics_types::SurfaceTransform,
    render_device::RenderDevice,
    swap_chain::{SwapChain, SwapChainDesc},
};

#[cfg(feature = "vulkan")]
use diligent::vk::engine_factory_vk::EngineVkCreateInfo;

#[cfg(feature = "opengl")]
use diligent::gl::engine_factory_gl::EngineGLCreateInfo;

#[cfg(feature = "d3d11")]
use diligent::d3d11::engine_factory_d3d11::EngineD3D11CreateInfo;

#[cfg(feature = "d3d12")]
use diligent::d3d12::engine_factory_d3d12::EngineD3D12CreateInfo;

use diligent_tools::native_app::events::Event;
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

pub enum EngineCreateInfo<'a> {
    #[cfg(feature = "vulkan")]
    EngineVkCreateInfo(&'a mut EngineVkCreateInfo),
    #[cfg(feature = "opengl")]
    EngineGLCreateInfo(&'a mut EngineGLCreateInfo<'a>),
    #[cfg(feature = "d3d11")]
    EngineD3D11CreateInfo(&'a mut EngineD3D11CreateInfo),
    #[cfg(feature = "d3d12")]
    EngineD3D12CreateInfo(&'a mut EngineD3D12CreateInfo),
}

impl Deref for EngineCreateInfo<'_> {
    type Target = diligent::engine_factory::EngineCreateInfo;
    fn deref(&self) -> &Self::Target {
        match self {
            #[cfg(feature = "vulkan")]
            EngineCreateInfo::EngineVkCreateInfo(vk_create_info) => vk_create_info,
            #[cfg(feature = "opengl")]
            EngineCreateInfo::EngineGLCreateInfo(gl_create_info) => gl_create_info,
            #[cfg(feature = "d3d11")]
            EngineCreateInfo::EngineD3D11CreateInfo(d3d11_create_info) => d3d11_create_info,
            #[cfg(feature = "d3d12")]
            EngineCreateInfo::EngineD3D12CreateInfo(d3d12_create_info) => d3d12_create_info,
        }
    }
}

impl DerefMut for EngineCreateInfo<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            #[cfg(feature = "vulkan")]
            EngineCreateInfo::EngineVkCreateInfo(vk_create_info) => vk_create_info,
            #[cfg(feature = "opengl")]
            EngineCreateInfo::EngineGLCreateInfo(gl_create_info) => gl_create_info,
            #[cfg(feature = "d3d11")]
            EngineCreateInfo::EngineD3D11CreateInfo(d3d11_create_info) => d3d11_create_info,
            #[cfg(feature = "d3d12")]
            EngineCreateInfo::EngineD3D12CreateInfo(d3d12_create_info) => d3d12_create_info,
        }
    }
}

pub trait SampleBase {
    fn new(
        engine_factory: &EngineFactory,
        render_device: &RenderDevice,
        immediate_contexts: Vec<ImmediateDeviceContext>,
        deferred_contexts: Vec<DeferredDeviceContext>,
        swap_chain: &SwapChain,
    ) -> Self;

    fn get_immediate_context(&self) -> &ImmediateDeviceContext;

    fn render(&self, _swap_chain: &SwapChain) {}

    fn update(&mut self, _current_time: f64, _elapsed_time: f64) {}

    fn update_ui(&mut self, _ui: &mut Ui) {}

    fn get_name() -> &'static str;

    fn pre_window_resize(&mut self) {}

    fn window_resize(&mut self, _width: u32, _height: u32) {}

    fn handle_event(&mut self, _event: Event) {}

    fn release_swap_chain_buffers(&mut self) {}

    fn modify_engine_init_info(_engine_ci: &mut EngineCreateInfo) {}
}
