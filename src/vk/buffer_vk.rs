use crate::buffer::Buffer;

define_ported!(
    BufferVk,
    diligent_sys::IBufferVk,
    diligent_sys::IBufferVkMethods : 4,
    Buffer
);

impl BufferVk {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBuffer {
        unsafe_member_call!(self, BufferVk, GetVkBuffer)
    }

    pub fn set_access_flags(&self, access_flags: diligent_sys::VkAccessFlags) {
        unsafe_member_call!(self, BufferVk, SetAccessFlags, access_flags)
    }

    pub fn get_access_flags(&self) -> diligent_sys::VkAccessFlags {
        unsafe_member_call!(self, BufferVk, GetAccessFlags)
    }

    pub fn get_vk_device_address(&self) -> diligent_sys::VkDeviceAddress {
        unsafe_member_call!(self, BufferVk, GetVkDeviceAddress)
    }
}
