use crate::device_memory::DeviceMemory;

define_ported!(
    DeviceMemoryVk,
    diligent_sys::IDeviceMemoryVk,
    diligent_sys::IDeviceMemoryVkMethods : 1,
    DeviceMemory
);

impl DeviceMemoryVk {
    pub fn get_range(&self, offset: u64, size: u64) -> diligent_sys::DeviceMemoryRangeVk {
        unsafe_member_call!(self, DeviceMemoryVk, GetRange, offset, size)
    }
}
