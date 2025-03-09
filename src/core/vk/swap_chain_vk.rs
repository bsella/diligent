use crate::core::{
    object::{AsObject, Object},
    swap_chain::SwapChain,
};

pub struct SwapChainVk<'a> {
    swap_chain_ptr: *mut diligent_sys::ISwapChainVk,
    virtual_functions: *mut diligent_sys::ISwapChainVkVtbl,

    swap_chain: &'a SwapChain,
}

impl AsObject for SwapChainVk<'_> {
    fn as_object(&self) -> &Object {
        self.swap_chain.as_object()
    }
}

impl<'a> From<&'a SwapChain> for SwapChainVk<'a> {
    fn from(value: &'a SwapChain) -> Self {
        SwapChainVk {
            swap_chain: value,
            swap_chain_ptr: value.swap_chain as *mut diligent_sys::ISwapChainVk,
            virtual_functions: unsafe {
                (*(value.swap_chain as *mut diligent_sys::ISwapChainVk)).pVtbl
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
                .unwrap_unchecked()(self.swap_chain_ptr)
        }
    }

    pub fn get_vk_swap_chain(&self) -> diligent_sys::VkSwapchainKHR {
        unsafe {
            (*self.virtual_functions)
                .SwapChainVk
                .GetVkSwapChain
                .unwrap_unchecked()(self.swap_chain_ptr)
        }
    }
}
