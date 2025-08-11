use std::ops::Deref;

use crate::blas::BottomLevelAS;

pub struct BottomLevelASVk<'a> {
    sys_ptr: *mut diligent_sys::IBottomLevelASVk,
    virtual_functions: *mut diligent_sys::IBottomLevelASVkVtbl,

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
        BottomLevelASVk {
            blas: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IBottomLevelASVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IBottomLevelASVk)).pVtbl
            },
        }
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
