use crate::fence::Fence;

define_ported!(
    FenceVk,
    diligent_sys::IFenceVk,
    diligent_sys::IFenceVkMethods : 1,
    Fence
);

impl FenceVk {
    pub fn get_vk_semaphore(&self) -> diligent_sys::VkSemaphore {
        unsafe_member_call!(self, FenceVk, GetVkSemaphore)
    }
}
