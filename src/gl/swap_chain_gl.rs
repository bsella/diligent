use crate::SwapChain;

define_ported!(
    SwapChainGL,
    diligent_sys::ISwapChainGL,
    diligent_sys::ISwapChainGLMethods : 1,
    SwapChain
);

impl SwapChainGL {
    pub fn get_default_fbo(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, SwapChainGL, GetDefaultFBO)
    }
}
