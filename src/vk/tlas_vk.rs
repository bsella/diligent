use crate::tlas::TopLevelAS;

define_ported!(
    TopLevelASVk,
    diligent_sys::ITopLevelASVk,
    diligent_sys::ITopLevelASVkMethods : 2,
    TopLevelAS
);

impl TopLevelASVk {
    pub fn get_vk_tlas(&self) -> diligent_sys::VkAccelerationStructureKHR {
        unsafe_member_call!(self, TopLevelASVk, GetVkTLAS)
    }

    pub fn get_vk_device_address(&self) -> diligent_sys::VkDeviceAddress {
        unsafe_member_call!(self, TopLevelASVk, GetVkDeviceAddress)
    }
}
