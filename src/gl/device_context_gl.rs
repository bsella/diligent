use crate::{device_context::DeviceContext, gl::swap_chain_gl::SwapChainGL};

define_ported!(
    DeviceContextGL,
    diligent_sys::IDeviceContextGL,
    diligent_sys::IDeviceContextGLMethods : 3,
    DeviceContext
);

impl DeviceContextGL {
    pub fn update_current_gl_context(&self) -> bool {
        unsafe_member_call!(self, DeviceContextGL, UpdateCurrentGLContext)
    }

    pub fn purge_current_gl_context_caches(&self) {
        unsafe_member_call!(self, DeviceContextGL, PurgeCurrentGLContextCaches)
    }

    pub fn set_swap_chain(&self, swap_chain: &SwapChainGL) {
        unsafe_member_call!(
            self,
            DeviceContextGL,
            SetSwapChain,
            std::ptr::from_ref(&swap_chain.0) as *mut _
        )
    }
}
