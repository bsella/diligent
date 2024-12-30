use crate::bindings;

use super::object::{AsObject, Object};

pub struct SwapChain {
    pub(crate) m_swap_chain: *mut bindings::ISwapChain,
    m_virtual_functions: *mut bindings::ISwapChainVtbl,

    m_object: Object,
}

impl AsObject for SwapChain {
    fn as_object(&self) -> &Object {
        &self.m_object
    }
}

impl SwapChain {
    pub fn present(&self, sync_interval: u32) {
        unsafe {
            (*self.m_virtual_functions)
                .SwapChain
                .Present
                .unwrap_unchecked()(self.m_swap_chain, sync_interval)
        }
    }

    pub fn get_desc(&self) -> &bindings::SwapChainDesc {
        unsafe {
            (*self.m_virtual_functions)
                .SwapChain
                .GetDesc
                .unwrap_unchecked()(self.m_swap_chain)
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
            (*self.m_virtual_functions)
                .SwapChain
                .Resize
                .unwrap_unchecked()(
                self.m_swap_chain,
                new_width,
                new_height,
                new_transform.unwrap_or(bindings::SURFACE_TRANSFORM_OPTIMAL),
            )
        }
    }

    pub fn set_fullscreen_mode(&self, display_mode: &bindings::DisplayModeAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .SwapChain
                .SetFullscreenMode
                .unwrap_unchecked()(self.m_swap_chain, std::ptr::from_ref(display_mode))
        }
    }

    pub fn set_windowed_mode(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .SwapChain
                .SetWindowedMode
                .unwrap_unchecked()(self.m_swap_chain)
        }
    }

    pub fn set_maximum_frame_latency(&self, max_latency: u32) {
        unsafe {
            (*self.m_virtual_functions)
                .SwapChain
                .SetMaximumFrameLatency
                .unwrap_unchecked()(self.m_swap_chain, max_latency)
        }
    }

    pub(crate) fn get_current_back_buffer_rtv(&self) -> *mut bindings::ITextureView {
        unsafe {
            (*self.m_virtual_functions)
                .SwapChain
                .GetCurrentBackBufferRTV
                .unwrap_unchecked()(self.m_swap_chain)
        }
    }

    pub(crate) fn get_depth_buffer_dsv(&self) -> *mut bindings::ITextureView {
        unsafe {
            (*self.m_virtual_functions)
                .SwapChain
                .GetDepthBufferDSV
                .unwrap_unchecked()(self.m_swap_chain)
        }
    }
}
