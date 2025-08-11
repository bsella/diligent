use std::ops::Deref;

use crate::tlas::TopLevelAS;

pub struct TopLevelASVk<'a> {
    sys_ptr: *mut diligent_sys::ITopLevelASVk,
    virtual_functions: *mut diligent_sys::ITopLevelASVkVtbl,

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
        TopLevelASVk {
            tlas: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::ITopLevelASVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::ITopLevelASVk)).pVtbl
            },
        }
    }
}

impl TopLevelASVk<'_> {
    pub fn get_vk_tlas(&self) -> diligent_sys::VkAccelerationStructureKHR {
        unsafe_member_call!(self, TopLevelASVk, GetVkTLAS,)
    }

    pub fn get_vk_device_address(&self) -> diligent_sys::VkDeviceAddress {
        unsafe_member_call!(self, TopLevelASVk, GetVkDeviceAddress,)
    }
}
