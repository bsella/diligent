use std::ops::Deref;

use crate::fence::Fence;

pub struct FenceVk<'a> {
    sys_ptr: *mut diligent_sys::IFenceVk,
    virtual_functions: *mut diligent_sys::IFenceVkVtbl,

    fence: &'a Fence,
}

impl Deref for FenceVk<'_> {
    type Target = Fence;
    fn deref(&self) -> &Self::Target {
        self.fence
    }
}

impl<'a> From<&'a Fence> for FenceVk<'a> {
    fn from(value: &'a Fence) -> Self {
        FenceVk {
            fence: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IFenceVk,
            virtual_functions: unsafe { (*(value.sys_ptr as *mut diligent_sys::IFenceVk)).pVtbl },
        }
    }
}

impl FenceVk<'_> {
    pub fn get_vk_semaphore(&self) -> diligent_sys::VkSemaphore {
        unsafe_member_call!(self, FenceVk, GetVkSemaphore,)
    }
}
