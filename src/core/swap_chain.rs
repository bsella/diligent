use bitflags::bitflags;
use static_assertions::const_assert;

use super::{
    graphics_types::SurfaceTransform,
    object::{AsObject, Object},
    texture_view::TextureView,
};

bitflags! {
    pub struct SwapChainUsageFlags: diligent_sys::_SWAP_CHAIN_USAGE_FLAGS {
        const None            = diligent_sys::SWAP_CHAIN_USAGE_NONE;
        const RenderTarget    = diligent_sys::SWAP_CHAIN_USAGE_RENDER_TARGET;
        const ShaderResource  = diligent_sys::SWAP_CHAIN_USAGE_SHADER_RESOURCE;
        const InputAttachment = diligent_sys::SWAP_CHAIN_USAGE_INPUT_ATTACHMENT;
        const CopySource      = diligent_sys::SWAP_CHAIN_USAGE_COPY_SOURCE;
    }
}
const_assert!(diligent_sys::SWAP_CHAIN_USAGE_LAST == 8);

pub struct SwapChainDesc {
    pub width: u32,
    pub height: u32,
    pub color_buffer_format: diligent_sys::TEXTURE_FORMAT,
    pub depth_buffer_format: diligent_sys::TEXTURE_FORMAT,
    pub usage: SwapChainUsageFlags,
    pub pre_transform: SurfaceTransform,
    pub buffer_count: u32,
    pub default_depth_value: f32,
    pub default_stencil_value: u8,
    pub is_primary: bool,
}

impl SwapChainDesc {
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn color_buffer_format(
        mut self,
        color_buffer_format: diligent_sys::TEXTURE_FORMAT,
    ) -> Self {
        self.color_buffer_format = color_buffer_format;
        self
    }
    pub fn depth_buffer_format(
        mut self,
        depth_buffer_format: diligent_sys::TEXTURE_FORMAT,
    ) -> Self {
        self.depth_buffer_format = depth_buffer_format;
        self
    }
    pub fn usage(mut self, usage: SwapChainUsageFlags) -> Self {
        self.usage = usage;
        self
    }
    pub fn pre_transform(mut self, pre_transform: SurfaceTransform) -> Self {
        self.pre_transform = pre_transform;
        self
    }
    pub fn buffer_count(mut self, buffer_count: u32) -> Self {
        self.buffer_count = buffer_count;
        self
    }
    pub fn default_depth_value(mut self, default_depth_value: f32) -> Self {
        self.default_depth_value = default_depth_value;
        self
    }
    pub fn default_stencil_value(mut self, default_stencil_value: u8) -> Self {
        self.default_stencil_value = default_stencil_value;
        self
    }
    pub fn is_primary(mut self, is_primary: bool) -> Self {
        self.is_primary = is_primary;
        self
    }
}

impl Default for SwapChainDesc {
    fn default() -> Self {
        SwapChainDesc {
            width: 0,
            height: 0,
            color_buffer_format: diligent_sys::TEX_FORMAT_RGBA8_UNORM_SRGB as u16,
            depth_buffer_format: diligent_sys::TEX_FORMAT_D32_FLOAT as u16,
            usage: SwapChainUsageFlags::RenderTarget,
            pre_transform: SurfaceTransform::Optimal,
            buffer_count: 2,
            default_depth_value: 1.0,
            default_stencil_value: 0,
            is_primary: true,
        }
    }
}

impl From<&SwapChainDesc> for diligent_sys::SwapChainDesc {
    fn from(value: &SwapChainDesc) -> Self {
        diligent_sys::SwapChainDesc {
            Width: value.width,
            Height: value.height,
            ColorBufferFormat: value.color_buffer_format,
            DepthBufferFormat: value.depth_buffer_format,
            Usage: value.usage.bits(),
            PreTransform: diligent_sys::SURFACE_TRANSFORM::from(&value.pre_transform),
            BufferCount: value.buffer_count,
            DefaultDepthValue: value.default_depth_value,
            DefaultStencilValue: value.default_stencil_value,
            IsPrimary: value.is_primary,
        }
    }
}

impl From<&diligent_sys::SwapChainDesc> for SwapChainDesc {
    fn from(value: &diligent_sys::SwapChainDesc) -> Self {
        SwapChainDesc {
            width: value.Width,
            height: value.Height,
            color_buffer_format: value.ColorBufferFormat,
            depth_buffer_format: value.DepthBufferFormat,
            usage: SwapChainUsageFlags::from_bits_retain(value.Usage),
            pre_transform: SurfaceTransform::from(&value.PreTransform),
            buffer_count: value.BufferCount,
            default_depth_value: value.DefaultDepthValue,
            default_stencil_value: value.DefaultStencilValue,
            is_primary: value.IsPrimary,
        }
    }
}

pub struct SwapChain {
    pub(crate) swap_chain: *mut diligent_sys::ISwapChain,
    virtual_functions: *mut diligent_sys::ISwapChainVtbl,

    object: Object,
}

impl AsObject for SwapChain {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl SwapChain {
    pub(crate) fn new(swap_chain_ptr: *mut diligent_sys::ISwapChain) -> Self {
        SwapChain {
            swap_chain: swap_chain_ptr,
            virtual_functions: unsafe { (*swap_chain_ptr).pVtbl },
            object: Object::new(swap_chain_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn present(&self, sync_interval: u32) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .Present
                .unwrap_unchecked()(self.swap_chain, sync_interval)
        }
    }

    pub fn get_desc(&self) -> SwapChainDesc {
        let swap_chain_desc = unsafe {
            (*self.virtual_functions)
                .SwapChain
                .GetDesc
                .unwrap_unchecked()(self.swap_chain)
            .as_ref()
            .unwrap_unchecked()
        };

        SwapChainDesc::from(swap_chain_desc)
    }

    pub fn resize(&self, new_width: u32, new_height: u32, new_transform: SurfaceTransform) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .Resize
                .unwrap_unchecked()(
                self.swap_chain,
                new_width,
                new_height,
                diligent_sys::SURFACE_TRANSFORM::from(&new_transform),
            )
        }
    }

    pub fn set_fullscreen_mode(&self, display_mode: &diligent_sys::DisplayModeAttribs) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .SetFullscreenMode
                .unwrap_unchecked()(self.swap_chain, std::ptr::from_ref(display_mode))
        }
    }

    pub fn set_windowed_mode(&self) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .SetWindowedMode
                .unwrap_unchecked()(self.swap_chain)
        }
    }

    pub fn set_maximum_frame_latency(&self, max_latency: u32) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .SetMaximumFrameLatency
                .unwrap_unchecked()(self.swap_chain, max_latency)
        }
    }

    pub fn get_current_back_buffer_rtv(&self) -> TextureView {
        let view = TextureView::new(
            unsafe {
                (*self.virtual_functions)
                    .SwapChain
                    .GetCurrentBackBufferRTV
                    .unwrap_unchecked()(self.swap_chain)
            },
            std::ptr::null_mut(),
        );

        view.device_object.as_object().add_ref();

        view
    }

    pub fn get_depth_buffer_dsv(&self) -> TextureView {
        let view = TextureView::new(
            unsafe {
                (*self.virtual_functions)
                    .SwapChain
                    .GetDepthBufferDSV
                    .unwrap_unchecked()(self.swap_chain)
            },
            std::ptr::null_mut(),
        );

        view.device_object.as_object().add_ref();

        view
    }
}
