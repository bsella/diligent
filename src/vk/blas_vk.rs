use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::blas::BottomLevelAS;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBottomLevelASVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct BottomLevelASVk(diligent_sys::IBottomLevelASVk);

impl Deref for BottomLevelASVk {
    type Target = BottomLevelAS;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IBottomLevelAS
                as *const BottomLevelAS)
        }
    }
}

impl BottomLevelASVk {
    pub fn get_vk_blas(&self) -> diligent_sys::VkAccelerationStructureKHR {
        unsafe_member_call!(self, BottomLevelASVk, GetVkBLAS)
    }

    pub fn get_vk_device_address(&self) -> u64 {
        unsafe_member_call!(self, BottomLevelASVk, GetVkDeviceAddress)
    }
}
