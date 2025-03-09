use crate::core::{
    buffer::Buffer,
    device_object::{AsDeviceObject, DeviceObject},
};

pub struct BufferVk<'a> {
    buffer_ptr: *mut diligent_sys::IBufferVk,
    virtual_functions: *mut diligent_sys::IBufferVkVtbl,

    buffer: &'a Buffer,
}

impl AsDeviceObject for BufferVk<'_> {
    fn as_device_object(&self) -> &DeviceObject {
        &self.buffer.as_device_object()
    }
}

impl<'a> From<&'a Buffer> for BufferVk<'a> {
    fn from(value: &'a Buffer) -> Self {
        BufferVk {
            buffer: value,
            buffer_ptr: value.buffer as *mut diligent_sys::IBufferVk,
            virtual_functions: unsafe { (*(value.buffer as *mut diligent_sys::IBufferVk)).pVtbl },
        }
    }
}

impl BufferVk<'_> {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBuffer {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .GetVkBuffer
                .unwrap_unchecked()(self.buffer_ptr)
        }
    }

    pub fn set_access_flags(&self, access_flags: diligent_sys::VkAccessFlags) {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .SetAccessFlags
                .unwrap_unchecked()(self.buffer_ptr, access_flags)
        }
    }

    pub fn get_access_flags(&self) -> diligent_sys::VkAccessFlags {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .GetAccessFlags
                .unwrap_unchecked()(self.buffer_ptr)
        }
    }

    pub fn get_vk_device_address(&self) -> diligent_sys::VkDeviceAddress {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .GetVkDeviceAddress
                .unwrap_unchecked()(self.buffer_ptr)
        }
    }
}
