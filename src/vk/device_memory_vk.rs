use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::device_memory::DeviceMemory;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceMemoryVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceMemoryVk(diligent_sys::IDeviceMemoryVk);

impl Deref for DeviceMemoryVk {
    type Target = DeviceMemory;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IDeviceMemory
                as *const DeviceMemory)
        }
    }
}

impl DeviceMemoryVk {
    pub fn get_range(&self, offset: u64, size: u64) -> diligent_sys::DeviceMemoryRangeVk {
        unsafe_member_call!(self, DeviceMemoryVk, GetRange, offset, size)
    }
}
