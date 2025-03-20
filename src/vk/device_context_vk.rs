use crate::{buffer::Buffer, device_context::DeviceContext, object::Object, texture::Texture};

pub struct DeviceContextVk<'a> {
    device_context_ptr: *mut diligent_sys::IDeviceContextVk,
    virtual_functions: *mut diligent_sys::IDeviceContextVkVtbl,

    device_context: &'a DeviceContext,
}

impl AsRef<Object> for DeviceContextVk<'_> {
    fn as_ref(&self) -> &Object {
        self.device_context.as_ref()
    }
}

impl<'a> From<&'a DeviceContext> for DeviceContextVk<'a> {
    fn from(value: &'a DeviceContext) -> Self {
        DeviceContextVk {
            device_context: value,
            device_context_ptr: value.sys_ptr as *mut diligent_sys::IDeviceContextVk,
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
        unsafe {
            (*self.virtual_functions)
                .DeviceContextVk
                .TransitionImageLayout
                .unwrap_unchecked()(self.device_context_ptr, texture.sys_ptr, new_layout)
        }
    }

    pub fn buffer_memory_barrier(
        &self,
        buffer: Buffer,
        new_access_flags: diligent_sys::VkAccessFlags,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContextVk
                .BufferMemoryBarrier
                .unwrap_unchecked()(
                self.device_context_ptr, buffer.sys_ptr, new_access_flags
            )
        }
    }

    pub fn get_vk_command_buffer(&self) -> diligent_sys::VkCommandBuffer {
        unsafe {
            (*self.virtual_functions)
                .DeviceContextVk
                .GetVkCommandBuffer
                .unwrap_unchecked()(self.device_context_ptr)
        }
    }
}
