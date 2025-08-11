use std::ops::Deref;

use crate::swap_chain::SwapChain;

#[repr(transparent)]
pub struct SwapChainVk<'a> {
    swap_chain: &'a SwapChain,
}

impl Deref for SwapChainVk<'_> {
    type Target = SwapChain;
    fn deref(&self) -> &Self::Target {
        self.swap_chain
    }
}

impl<'a> From<&'a SwapChain> for SwapChainVk<'a> {
    fn from(value: &'a SwapChain) -> Self {
        SwapChainVk { swap_chain: value }
    }
}

impl SwapChainVk<'_> {
    pub fn get_vk_surface(&self) -> diligent_sys::VkSurfaceKHR {
        unsafe_member_call!(self, SwapChainVk, GetVkSurface,)
    }

    pub fn get_vk_swap_chain(&self) -> diligent_sys::VkSwapchainKHR {
        unsafe_member_call!(self, SwapChainVk, GetVkSwapChain,)
    }
}
