use crate::swap_chain::SwapChain;

define_ported!(
    SwapChainVk,
    diligent_sys::ISwapChainVk,
    diligent_sys::ISwapChainVkMethods : 2,
    SwapChain
);

impl SwapChainVk {
    pub fn get_vk_surface(&self) -> diligent_sys::VkSurfaceKHR {
        unsafe_member_call!(self, SwapChainVk, GetVkSurface)
    }

    pub fn get_vk_swap_chain(&self) -> diligent_sys::VkSwapchainKHR {
        unsafe_member_call!(self, SwapChainVk, GetVkSwapChain)
    }
}
