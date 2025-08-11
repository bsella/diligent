use std::ops::Deref;

use crate::{buffer::Buffer, device_context::DeviceContext, texture::Texture};

pub struct DeviceContextVk<'a> {
    sys_ptr: *mut diligent_sys::IDeviceContextVk,
    virtual_functions: *mut diligent_sys::IDeviceContextVkVtbl,

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
            sys_ptr: value.sys_ptr as *mut diligent_sys::IDeviceContextVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IDeviceContextVk)).pVtbl
            },
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
            texture.sys_ptr,
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
            buffer.sys_ptr,
            new_access_flags
        )
    }

    pub fn get_vk_command_buffer(&self) -> diligent_sys::VkCommandBuffer {
        unsafe_member_call!(self, DeviceContextVk, GetVkCommandBuffer,)
    }
}
