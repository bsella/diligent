use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::blas::BottomLevelAS;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBottomLevelASVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct BottomLevelASVk<'a> {
    blas: &'a BottomLevelAS,
}

impl<'a> Deref for BottomLevelASVk<'a> {
    type Target = BottomLevelAS;
    fn deref(&self) -> &Self::Target {
        self.blas
    }
}

impl<'a> From<&'a BottomLevelAS> for BottomLevelASVk<'a> {
    fn from(value: &'a BottomLevelAS) -> Self {
        BottomLevelASVk { blas: value }
    }
}

impl BottomLevelASVk<'_> {
    pub fn get_vk_blas(&self) -> diligent_sys::VkAccelerationStructureKHR {
        unsafe_member_call!(self, BottomLevelASVk, GetVkBLAS,)
    }

    pub fn get_vk_device_address(&self) -> u64 {
        unsafe_member_call!(self, BottomLevelASVk, GetVkDeviceAddress,)
    }
}
