use std::ops::Deref;

use crate::{buffer::Buffer, device_context::DeviceContext, texture::Texture};

#[repr(transparent)]
pub struct DeviceContextVk<'a> {
    device_context: &'a DeviceContext,
}

impl Deref for DeviceContextVk<'_> {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        self.device_context
    }
}

impl<'a> From<&'a DeviceContext> for DeviceContextVk<'a> {
    fn from(value: &'a DeviceContext) -> Self {
        DeviceContextVk {
            device_context: value,
        }
    }
}

impl DeviceContextVk<'_> {
    pub fn transition_image_layout(
        &self,
        texture: &Texture,
        new_layout: diligent_sys::VkImageLayout,
    ) {
        unsafe_member_call!(
            self,
            DeviceContextVk,
            TransitionImageLayout,
            texture.sys_ptr as _,
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
            buffer.sys_ptr as _,
            new_access_flags
        )
    }

    pub fn get_vk_command_buffer(&self) -> diligent_sys::VkCommandBuffer {
        unsafe_member_call!(self, DeviceContextVk, GetVkCommandBuffer,)
    }
}
