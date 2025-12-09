use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::SwapChain;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ISwapChainGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct SwapChainGL(pub(crate) diligent_sys::ISwapChainGL);

impl Deref for SwapChainGL {
    type Target = SwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ISwapChain as *const SwapChain)
        }
    }
}

impl SwapChainGL {
    pub fn get_default_fbo(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, SwapChainGL, GetDefaultFBO)
    }
}
