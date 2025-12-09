use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{device_context::DeviceContext, gl::swap_chain_gl::SwapChainGL};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceContextGLMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceContextGL(diligent_sys::IDeviceContextGL);

impl Deref for DeviceContextGL {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IDeviceContext
                as *const DeviceContext)
        }
    }
}

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
