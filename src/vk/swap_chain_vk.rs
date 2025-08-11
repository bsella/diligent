use std::ops::Deref;

use crate::swap_chain::SwapChain;

pub struct SwapChainVk<'a> {
    sys_ptr: *mut diligent_sys::ISwapChainVk,
    virtual_functions: *mut diligent_sys::ISwapChainVkVtbl,

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
        SwapChainVk {
            swap_chain: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::ISwapChainVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::ISwapChainVk)).pVtbl
            },
        }
    }
}

impl SwapChainVk<'_> {
    pub fn get_vk_surface(&self) -> diligent_sys::VkSurfaceKHR {
        unsafe {
            (*self.virtual_functions)
                .SwapChainVk
                .GetVkSurface
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_vk_swap_chain(&self) -> diligent_sys::VkSwapchainKHR {
        unsafe {
            (*self.virtual_functions)
                .SwapChainVk
                .GetVkSwapChain
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
