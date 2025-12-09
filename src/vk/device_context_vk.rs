use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{buffer::Buffer, device_context::DeviceContext, texture::Texture};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceContextVkMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceContextVk(diligent_sys::IDeviceContextVk);

impl Deref for DeviceContextVk {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IDeviceContext
                as *const DeviceContext)
        }
    }
}

impl DeviceContextVk {
    pub fn transition_image_layout(
        &self,
        texture: &Texture,
        new_layout: diligent_sys::VkImageLayout,
    ) {
        unsafe_member_call!(
            self,
            DeviceContextVk,
            TransitionImageLayout,
            texture.sys_ptr(),
            new_layout
        )
    }

    pub fn buffer_memory_barrier(
        &self,
        buffer: &Buffer,
        new_access_flags: diligent_sys::VkAccessFlags,
    ) {
        unsafe_member_call!(
            self,
            DeviceContextVk,
            BufferMemoryBarrier,
            buffer.sys_ptr(),
            new_access_flags
        )
    }

    pub fn get_vk_command_buffer(&self) -> diligent_sys::VkCommandBuffer {
        unsafe_member_call!(self, DeviceContextVk, GetVkCommandBuffer)
    }
}
