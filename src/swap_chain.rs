use std::ops::Deref;

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{
    BindFlags,
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
const_assert_eq!(diligent_sys::SWAP_CHAIN_USAGE_LAST, 8);

impl Default for SwapChainUsageFlags {
    fn default() -> Self {
        SwapChainUsageFlags::None
    }
}

impl SwapChainUsageFlags {
    pub fn to_bind_flags(&self) -> BindFlags {
        let mut result = BindFlags::None;

        if self.contains(SwapChainUsageFlags::RenderTarget) {
            result |= BindFlags::RenderTarget
        }

        if self.contains(SwapChainUsageFlags::ShaderResource) {
            result |= BindFlags::ShaderResource
        }

        if self.contains(SwapChainUsageFlags::InputAttachment) {
            result |= BindFlags::InputAttachement
        }

        result
    }
}

#[repr(transparent)]
pub struct SwapChainDesc(pub(crate) diligent_sys::SwapChainDesc);

impl SwapChainDesc {
    pub fn width(&self) -> u32 {
        self.0.Width
    }
    pub fn height(&self) -> u32 {
        self.0.Height
    }
    pub fn color_buffer_format(&self) -> Option<TextureFormat> {
        TextureFormat::from_sys(self.0.ColorBufferFormat)
    }
    pub fn depth_buffer_format(&self) -> Option<TextureFormat> {
        TextureFormat::from_sys(self.0.DepthBufferFormat)
    }
    pub fn usage(&self) -> SwapChainUsageFlags {
        SwapChainUsageFlags::from_bits_retain(self.0.Usage)
    }
    pub fn pre_transform(&self) -> SurfaceTransform {
        self.0.PreTransform.into()
    }
    pub fn buffer_count(&self) -> u32 {
        self.0.BufferCount
    }
    pub fn default_depth_value(&self) -> f32 {
        self.0.DefaultDepthValue
    }
    pub fn default_stencil_value(&self) -> u8 {
        self.0.DefaultStencilValue
    }
    pub fn is_primary(&self) -> bool {
        self.0.IsPrimary
    }
}

#[repr(transparent)]
pub struct SwapChainCreateInfo(pub(crate) SwapChainDesc);

impl Deref for SwapChainCreateInfo {
    type Target = SwapChainDesc;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[bon::bon]
impl SwapChainCreateInfo {
    #[builder]
    pub fn new(
        width: u32,

        height: u32,

        #[builder(required)]
        #[builder(default = Some(TextureFormat::RGBA8_UNORM_SRGB))]
        color_buffer_format: Option<TextureFormat>,

        #[builder(required)]
        #[builder(default = Some(TextureFormat::D32_FLOAT))]
        depth_buffer_format: Option<TextureFormat>,

        #[builder(default = SwapChainUsageFlags::RenderTarget)] usage: SwapChainUsageFlags,

        #[builder(default = SurfaceTransform::Optimal)] pre_transform: SurfaceTransform,

        #[builder(default = 2)] buffer_count: u32,

        #[builder(default = 1.0)] default_depth_value: f32,

        #[builder(default = 0)] default_stencil_value: u8,

        #[builder(default = true)] is_primary: bool,
    ) -> Self {
        Self(SwapChainDesc(diligent_sys::SwapChainDesc {
            Width: width,
            Height: height,
            ColorBufferFormat: color_buffer_format
                .map_or(diligent_sys::TEX_FORMAT_UNKNOWN as _, TextureFormat::into),
            DepthBufferFormat: depth_buffer_format
                .map_or(diligent_sys::TEX_FORMAT_UNKNOWN as _, TextureFormat::into),
            Usage: usage.bits(),
            PreTransform: pre_transform.into(),
            BufferCount: buffer_count,
            DefaultDepthValue: default_depth_value,
            DefaultStencilValue: default_stencil_value,
            IsPrimary: is_primary,
        }))
    }
}

define_ported!(
    SwapChain,
    diligent_sys::ISwapChain,
    diligent_sys::ISwapChainMethods : 8,
    Object
);

impl SwapChain {
    pub fn present(&self, sync_interval: u32) {
        unsafe_member_call!(self, SwapChain, Present, sync_interval)
    }

    pub fn desc(&self) -> &SwapChainDesc {
        let sys_ptr = unsafe_member_call!(self, SwapChain, GetDesc);
        unsafe { &*(sys_ptr as *const SwapChainDesc) }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32, new_transform: SurfaceTransform) {
        unsafe_member_call!(
            self,
            SwapChain,
            Resize,
            new_width,
            new_height,
            new_transform.into()
        )
    }

    pub fn set_fullscreen_mode(&mut self, display_mode: &DisplayModeAttribs) {
        unsafe_member_call!(self, SwapChain, SetFullscreenMode, &display_mode.0)
    }

    pub fn set_windowed_mode(&mut self) {
        unsafe_member_call!(self, SwapChain, SetWindowedMode)
    }

    pub fn set_maximum_frame_latency(&mut self, max_latency: u32) {
        unsafe_member_call!(self, SwapChain, SetMaximumFrameLatency, max_latency)
    }

    pub fn get_current_back_buffer_rtv(&self) -> Option<&TextureView> {
        let texture_view_ptr = unsafe_member_call!(self, SwapChain, GetCurrentBackBufferRTV);

        if texture_view_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(texture_view_ptr as *const TextureView) })
        }
    }

    pub fn get_depth_buffer_dsv(&self) -> Option<&TextureView> {
        let texture_view_ptr = unsafe_member_call!(self, SwapChain, GetDepthBufferDSV);

        if texture_view_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(texture_view_ptr as *const TextureView) })
        }
    }
}
