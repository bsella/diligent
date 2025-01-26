use crate::bindings;

use super::{
    object::{AsObject, Object},
    texture_view::TextureView,
};

impl Default for bindings::SwapChainDesc {
    fn default() -> Self {
        bindings::SwapChainDesc {
            Width: 0,
            Height: 0,
            ColorBufferFormat: bindings::TEX_FORMAT_RGBA8_UNORM_SRGB as u16,
            DepthBufferFormat: bindings::TEX_FORMAT_D32_FLOAT as u16,
            Usage: bindings::SWAP_CHAIN_USAGE_RENDER_TARGET,
            PreTransform: bindings::SURFACE_TRANSFORM_OPTIMAL,
            BufferCount: 2,
            DefaultDepthValue: 1.0,
            DefaultStencilValue: 0,
            IsPrimary: true,
        }
    }
}

pub struct SwapChain {
    pub(crate) swap_chain: *mut bindings::ISwapChain,
    virtual_functions: *mut bindings::ISwapChainVtbl,

    object: Object,
}

impl AsObject for SwapChain {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl SwapChain {
    pub(crate) fn new(swap_chain_ptr: *mut bindings::ISwapChain) -> Self {
        SwapChain {
            swap_chain: swap_chain_ptr,
            virtual_functions: unsafe { (*swap_chain_ptr).pVtbl },
            object: Object::new(swap_chain_ptr as *mut bindings::IObject),
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

    pub fn get_desc(&self) -> &bindings::SwapChainDesc {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .GetDesc
                .unwrap_unchecked()(self.swap_chain)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn resize(
        &self,
        new_width: u32,
        new_height: u32,
        new_transform: Option<bindings::SURFACE_TRANSFORM>,
    ) {
        unsafe {
            (*self.virtual_functions)
                .SwapChain
                .Resize
                .unwrap_unchecked()(
                self.swap_chain,
                new_width,
                new_height,
                new_transform.unwrap_or(bindings::SURFACE_TRANSFORM_OPTIMAL),
            )
        }
    }

    pub fn set_fullscreen_mode(&self, display_mode: &bindings::DisplayModeAttribs) {
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
