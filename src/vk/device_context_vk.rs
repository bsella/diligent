use crate::core::{
    buffer::Buffer,
    device_context::DeviceContext,
    object::{AsObject, Object},
    texture::Texture,
};

pub struct DeviceContextVk<'a> {
    device_context_ptr: *mut diligent_sys::IDeviceContextVk,
    virtual_functions: *mut diligent_sys::IDeviceContextVkVtbl,

    device_context: &'a DeviceContext,
}

impl AsObject for DeviceContextVk<'_> {
    fn as_object(&self) -> &Object {
        self.device_context.as_object()
    }
}

impl<'a> From<&'a DeviceContext> for DeviceContextVk<'a> {
    fn from(value: &'a DeviceContext) -> Self {
        DeviceContextVk {
            device_context: value,
            device_context_ptr: value.device_context as *mut diligent_sys::IDeviceContextVk,
            virtual_functions: unsafe {
                (*(value.device_context as *mut diligent_sys::IDeviceContextVk)).pVtbl
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
                .unwrap_unchecked()(self.device_context_ptr, texture.texture, new_layout)
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
                self.device_context_ptr, buffer.buffer, new_access_flags
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
