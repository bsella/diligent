use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::device_memory::DeviceMemory;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceMemoryVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceMemoryVk<'a> {
    device_memory: &'a DeviceMemory,
}

impl Deref for DeviceMemoryVk<'_> {
    type Target = DeviceMemory;
    fn deref(&self) -> &Self::Target {
        self.device_memory
    }
}

impl<'a> From<&'a DeviceMemory> for DeviceMemoryVk<'a> {
    fn from(value: &'a DeviceMemory) -> Self {
        DeviceMemoryVk {
            device_memory: value,
        }
    }
}

impl DeviceMemoryVk<'_> {
    pub fn get_range(&self, offset: u64, size: u64) -> diligent_sys::DeviceMemoryRangeVk {
        unsafe_member_call!(self, DeviceMemoryVk, GetRange, offset, size)
    }
}
