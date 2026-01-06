use crate::blas::BottomLevelAS;

define_ported!(
    BottomLevelASVk,
    diligent_sys::IBottomLevelASVk,
    diligent_sys::IBottomLevelASVkMethods : 2,
    BottomLevelAS
);

impl BottomLevelASVk {
    pub fn get_vk_blas(&self) -> diligent_sys::VkAccelerationStructureKHR {
        unsafe_member_call!(self, BottomLevelASVk, GetVkBLAS)
    }

    pub fn get_vk_device_address(&self) -> u64 {
        unsafe_member_call!(self, BottomLevelASVk, GetVkDeviceAddress)
    }
}
