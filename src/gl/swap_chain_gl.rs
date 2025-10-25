use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::SwapChain;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ISwapChainGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct SwapChainGL<'a>(&'a SwapChain);

impl<'a> Deref for SwapChainGL<'a> {
    type Target = SwapChain;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a SwapChain> for SwapChainGL<'a> {
    fn from(value: &'a SwapChain) -> Self {
        SwapChainGL(value)
    }
}

impl SwapChainGL<'_> {
    pub fn get_default_fbo(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, SwapChainGL, GetDefaultFBO)
    }
}
