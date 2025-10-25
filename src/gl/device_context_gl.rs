use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{SwapChain, device_context::DeviceContext};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceContextGLMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceContextGL<'a>(&'a DeviceContext);

impl<'a> Deref for DeviceContextGL<'a> {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a DeviceContext> for DeviceContextGL<'a> {
    fn from(value: &'a DeviceContext) -> Self {
        DeviceContextGL(value)
    }
}

impl DeviceContextGL<'_> {
    pub fn update_current_gl_context(&self) -> bool {
        unsafe_member_call!(self, DeviceContextGL, UpdateCurrentGLContext)
    }

    pub fn purge_current_gl_context_caches(&self) {
        unsafe_member_call!(self, DeviceContextGL, PurgeCurrentGLContextCaches)
    }

    pub fn set_swap_chain(&self, swap_chain: &SwapChain) {
        unsafe_member_call!(self, DeviceContextGL, SetSwapChain, swap_chain.sys_ptr as _)
    }
}
