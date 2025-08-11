use std::ops::Deref;

use crate::fence::Fence;

#[repr(transparent)]
pub struct FenceVk<'a> {
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
        FenceVk { fence: value }
    }
}

impl FenceVk<'_> {
    pub fn get_vk_semaphore(&self) -> diligent_sys::VkSemaphore {
        unsafe_member_call!(self, FenceVk, GetVkSemaphore,)
    }
}
