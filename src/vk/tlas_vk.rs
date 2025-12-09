use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::tlas::TopLevelAS;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITopLevelASVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TopLevelASVk(diligent_sys::ITopLevelASVk);

impl Deref for TopLevelASVk {
    type Target = TopLevelAS;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ITopLevelAS as *const TopLevelAS)
        }
    }
}

impl TopLevelASVk {
    pub fn get_vk_tlas(&self) -> diligent_sys::VkAccelerationStructureKHR {
        unsafe_member_call!(self, TopLevelASVk, GetVkTLAS)
    }

    pub fn get_vk_device_address(&self) -> diligent_sys::VkDeviceAddress {
        unsafe_member_call!(self, TopLevelASVk, GetVkDeviceAddress)
    }
}
