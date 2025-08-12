use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::fence::Fence;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IFenceVkMethods>(),
    std::mem::size_of::<*const ()>()
);

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
