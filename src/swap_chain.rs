use std::{marker::PhantomData, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::{const_assert, const_assert_eq};

use crate::{
    graphics_types::{DisplayModeAttribs, SurfaceTransform, TextureFormat},
    object::Object,
    texture_view::TextureView,
};

bitflags! {
    #[derive(Clone,Copy)]
    pub struct SwapChainUsageFlags: diligent_sys::SWAP_CHAIN_USAGE_FLAGS {
        const None            = diligent_sys::SWAP_CHAIN_USAGE_NONE as diligent_sys::SWAP_CHAIN_USAGE_FLAGS;
        const RenderTarget    = diligent_sys::SWAP_CHAIN_USAGE_RENDER_TARGET as diligent_sys::SWAP_CHAIN_USAGE_FLAGS;
        const ShaderResource  = diligent_sys::SWAP_CHAIN_USAGE_SHADER_RESOURCE as diligent_sys::SWAP_CHAIN_USAGE_FLAGS;
        const InputAttachment = diligent_sys::SWAP_CHAIN_USAGE_INPUT_ATTACHMENT as diligent_sys::SWAP_CHAIN_USAGE_FLAGS;
        const CopySource      = diligent_sys::SWAP_CHAIN_USAGE_COPY_SOURCE as diligent_sys::SWAP_CHAIN_USAGE_FLAGS;
    }
}
const_assert!(diligent_sys::SWAP_CHAIN_USAGE_LAST == 8);

impl Default for SwapChainUsageFlags {
    fn default() -> Self {
        SwapChainUsageFlags::None
    }
}

#[repr(transparent)]
pub struct SwapChainDesc<'a>(
    *const diligent_sys::SwapChainDesc,
    PhantomData<&'a SwapChain>,
);

impl SwapChainDesc<'_> {
    pub fn width(&self) -> u32 {
        unsafe { (*self.0).Width }
    }
    pub fn height(&self) -> u32 {
        unsafe { (*self.0).Height }
    }
    pub fn color_buffer_format(&self) -> TextureFormat {
        unsafe { (*self.0).ColorBufferFormat }.into()
    }
    pub fn depth_buffer_format(&self) -> TextureFormat {
        unsafe { (*self.0).DepthBufferFormat }.into()
    }
    pub fn usage(&self) -> SwapChainUsageFlags {
        SwapChainUsageFlags::from_bits_retain(unsafe { (*self.0).Usage })
    }
    pub fn pre_transform(&self) -> SurfaceTransform {
        unsafe { (*self.0).PreTransform }.into()
    }
    pub fn buffer_count(&self) -> u32 {
        unsafe { (*self.0).BufferCount }
    }
    pub fn default_depth_value(&self) -> f32 {
        unsafe { (*self.0).DefaultDepthValue }
    }
    pub fn default_stencil_value(&self) -> u8 {
        unsafe { (*self.0).DefaultStencilValue }
    }
    pub fn is_primary(&self) -> bool {
        unsafe { (*self.0).IsPrimary }
    }
}

#[derive(Builder)]
pub struct SwapChainCreateInfo {
    width: u32,

    height: u32,

    #[builder(default = TextureFormat::RGBA8_UNORM_SRGB)]
    color_buffer_format: TextureFormat,

    #[builder(default = TextureFormat::D32_FLOAT)]
    depth_buffer_format: TextureFormat,

    #[builder(default = SwapChainUsageFlags::RenderTarget)]
    usage: SwapChainUsageFlags,

    #[builder(default = SurfaceTransform::Optimal)]
    pre_transform: SurfaceTransform,

    #[builder(default = 2)]
    buffer_count: u32,

    #[builder(default = 1.0)]
    default_depth_value: f32,

    #[builder(default = 0)]
    default_stencil_value: u8,

    #[builder(default = true)]
    is_primary: bool,
}

impl From<&SwapChainCreateInfo> for diligent_sys::SwapChainDesc {
    fn from(value: &SwapChainCreateInfo) -> Self {
        diligent_sys::SwapChainDesc {
            Width: value.width,
            Height: value.height,
            ColorBufferFormat: value.color_buffer_format.into(),
            DepthBufferFormat: value.depth_buffer_format.into(),
            Usage: value.usage.bits(),
            PreTransform: value.pre_transform.into(),
            BufferCount: value.buffer_count,
            DefaultDepthValue: value.default_depth_value,
            DefaultStencilValue: value.default_stencil_value,
            IsPrimary: value.is_primary,
        }
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ISwapChainMethods>(),
    8 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct SwapChain {
    object: Object,
}

impl Deref for SwapChain {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl SwapChain {
    pub(crate) fn new(swap_chain_ptr: *mut diligent_sys::ISwapChain) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::ISwapChain>()
        );

        SwapChain {
            object: Object::new(swap_chain_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn present(&self, sync_interval: u32) {
        unsafe_member_call!(self, SwapChain, Present, sync_interval)
    }

    pub fn get_desc(&self) -> SwapChainDesc<'_> {
        let sys_ptr = unsafe_member_call!(self, SwapChain, GetDesc);
        SwapChainDesc(sys_ptr, PhantomData)
    }

    pub fn resize(&self, new_width: u32, new_height: u32, new_transform: SurfaceTransform) {
        unsafe_member_call!(
            self,
            SwapChain,
            Resize,
            new_width,
            new_height,
            new_transform.into()
        )
    }

    pub fn set_fullscreen_mode(&self, display_mode: &DisplayModeAttribs) {
        let display_mode = display_mode.into();
        unsafe_member_call!(
            self,
            SwapChain,
            SetFullscreenMode,
            std::ptr::from_ref(&display_mode)
        )
    }

    pub fn set_windowed_mode(&self) {
        unsafe_member_call!(self, SwapChain, SetWindowedMode)
    }

    pub fn set_maximum_frame_latency(&self, max_latency: u32) {
        unsafe_member_call!(self, SwapChain, SetMaximumFrameLatency, max_latency)
    }

    pub fn get_current_back_buffer_rtv(&self) -> TextureView {
        let texture_view_ptr = unsafe_member_call!(self, SwapChain, GetCurrentBackBufferRTV);

        let view = TextureView::new(texture_view_ptr);

        view.add_ref();

        view
    }

    pub fn get_depth_buffer_dsv(&self) -> TextureView {
        let texture_view_ptr = unsafe_member_call!(self, SwapChain, GetDepthBufferDSV);

        let view = TextureView::new(texture_view_ptr);

        view.add_ref();

        view
    }
}
