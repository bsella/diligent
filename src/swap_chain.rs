use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert;

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

#[derive(Builder)]
pub struct SwapChainDesc {
    pub width: u32,

    pub height: u32,

    #[builder(default = TextureFormat::RGBA8_UNORM_SRGB)]
    pub color_buffer_format: TextureFormat,

    #[builder(default = TextureFormat::D32_FLOAT)]
    pub depth_buffer_format: TextureFormat,

    #[builder(default = SwapChainUsageFlags::RenderTarget)]
    pub usage: SwapChainUsageFlags,

    #[builder(default = SurfaceTransform::Optimal)]
    pub pre_transform: SurfaceTransform,

    #[builder(default = 2)]
    pub buffer_count: u32,

    #[builder(default = 1.0)]
    pub default_depth_value: f32,

    #[builder(default = 0)]
    pub default_stencil_value: u8,

    #[builder(default = true)]
    pub is_primary: bool,
}

impl From<&SwapChainDesc> for diligent_sys::SwapChainDesc {
    fn from(value: &SwapChainDesc) -> Self {
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

impl From<&diligent_sys::SwapChainDesc> for SwapChainDesc {
    fn from(value: &diligent_sys::SwapChainDesc) -> Self {
        SwapChainDesc {
            width: value.Width,
            height: value.Height,
            color_buffer_format: value.ColorBufferFormat.into(),
            depth_buffer_format: value.DepthBufferFormat.into(),
            usage: SwapChainUsageFlags::from_bits_retain(value.Usage),
            pre_transform: value.PreTransform.into(),
            buffer_count: value.BufferCount,
            default_depth_value: value.DefaultDepthValue,
            default_stencil_value: value.DefaultStencilValue,
            is_primary: value.IsPrimary,
        }
    }
}

pub struct SwapChain {
    pub(crate) sys_ptr: *mut diligent_sys::ISwapChain,
    virtual_functions: *mut diligent_sys::ISwapChainVtbl,

    object: Object,
}

impl AsRef<Object> for SwapChain {
    fn as_ref(&self) -> &Object {
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
            sys_ptr: swap_chain_ptr,
            virtual_functions: unsafe { (*swap_chain_ptr).pVtbl },
            object: Object::new(swap_chain_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn present(&self, sync_interval: u32) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .Present
                .unwrap_unchecked()(self.sys_ptr, sync_interval)
        }
    }

    pub fn get_desc(&self) -> SwapChainDesc {
        let swap_chain_desc = unsafe {
            (*self.virtual_functions)
                .SwapChain
                .GetDesc
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        };

        swap_chain_desc.into()
    }

    pub fn resize(&self, new_width: u32, new_height: u32, new_transform: SurfaceTransform) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .Resize
                .unwrap_unchecked()(
                self.sys_ptr, new_width, new_height, new_transform.into()
            )
        }
    }

    pub fn set_fullscreen_mode(&self, display_mode: &DisplayModeAttribs) {
        let display_mode = display_mode.into();
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .SetFullscreenMode
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(&display_mode))
        }
    }

    pub fn set_windowed_mode(&self) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .SetWindowedMode
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn set_maximum_frame_latency(&self, max_latency: u32) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .SetMaximumFrameLatency
                .unwrap_unchecked()(self.sys_ptr, max_latency)
        }
    }

    pub fn get_current_back_buffer_rtv(&self) -> TextureView {
        let view = TextureView::new(
            unsafe {
                (*self.virtual_functions)
                    .SwapChain
                    .GetCurrentBackBufferRTV
                    .unwrap_unchecked()(self.sys_ptr)
            },
            std::ptr::null_mut(),
        );

        view.device_object.as_ref().add_ref();

        view
    }

    pub fn get_depth_buffer_dsv(&self) -> TextureView {
        let view = TextureView::new(
            unsafe {
                (*self.virtual_functions)
                    .SwapChain
                    .GetDepthBufferDSV
                    .unwrap_unchecked()(self.sys_ptr)
            },
            std::ptr::null_mut(),
        );

        view.device_object.as_ref().add_ref();

        view
    }
}
