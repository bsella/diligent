use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::buffer::Buffer;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferVkMethods>(),
    4 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct BufferVk(diligent_sys::IBufferVk);

impl Deref for BufferVk {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IBuffer as *const Buffer) }
    }
}

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
