use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::swap_chain::SwapChain;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ISwapChainVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct SwapChainVk(diligent_sys::ISwapChainVk);

impl Deref for SwapChainVk {
    type Target = SwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ISwapChain as *const SwapChain)
        }
    }
}

impl SwapChainVk {
    pub fn get_vk_surface(&self) -> diligent_sys::VkSurfaceKHR {
        unsafe_member_call!(self, SwapChainVk, GetVkSurface)
    }

    pub fn get_vk_swap_chain(&self) -> diligent_sys::VkSwapchainKHR {
        unsafe_member_call!(self, SwapChainVk, GetVkSwapChain)
    }
}
