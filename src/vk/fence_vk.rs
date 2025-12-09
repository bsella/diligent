use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::fence::Fence;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IFenceVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct FenceVk(diligent_sys::IFenceVk);

impl Deref for FenceVk {
    type Target = Fence;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IFence as *const Fence) }
    }
}

impl FenceVk {
    pub fn get_vk_semaphore(&self) -> diligent_sys::VkSemaphore {
        unsafe_member_call!(self, FenceVk, GetVkSemaphore)
    }
}
