use std::ops::Deref;

use crate::device_memory::DeviceMemory;

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
