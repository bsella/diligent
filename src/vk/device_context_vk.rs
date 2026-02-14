use crate::{buffer::Buffer, device_context::DeviceContext, texture::Texture};

use crate::Ported;

define_ported!(
    DeviceContextVk,
    diligent_sys::IDeviceContextVk,
    diligent_sys::IDeviceContextVkMethods : 3,
    DeviceContext
);

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
