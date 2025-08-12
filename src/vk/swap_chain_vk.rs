use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::swap_chain::SwapChain;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ISwapChainVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

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
