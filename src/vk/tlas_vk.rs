use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::tlas::TopLevelAS;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITopLevelASVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TopLevelASVk<'a> {
    tlas: &'a TopLevelAS,
}

impl Deref for TopLevelASVk<'_> {
    type Target = TopLevelAS;
    fn deref(&self) -> &Self::Target {
        self.tlas
    }
}

impl<'a> From<&'a TopLevelAS> for TopLevelASVk<'a> {
    fn from(value: &'a TopLevelAS) -> Self {
        TopLevelASVk { tlas: value }
    }
}

impl TopLevelASVk<'_> {
    pub fn get_vk_tlas(&self) -> diligent_sys::VkAccelerationStructureKHR {
        unsafe_member_call!(self, TopLevelASVk, GetVkTLAS)
    }

    pub fn get_vk_device_address(&self) -> diligent_sys::VkDeviceAddress {
        unsafe_member_call!(self, TopLevelASVk, GetVkDeviceAddress)
    }
}
