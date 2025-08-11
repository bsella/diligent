use std::ops::Deref;

use crate::buffer::Buffer;

pub struct BufferVk<'a> {
    sys_ptr: *mut diligent_sys::IBufferVk,
    virtual_functions: *mut diligent_sys::IBufferVkVtbl,

    buffer: &'a Buffer,
}

impl<'a> Deref for BufferVk<'a> {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl<'a> From<&'a Buffer> for BufferVk<'a> {
    fn from(value: &'a Buffer) -> Self {
        BufferVk {
            buffer: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IBufferVk,
            virtual_functions: unsafe { (*(value.sys_ptr as *mut diligent_sys::IBufferVk)).pVtbl },
        }
    }
}

impl BufferVk<'_> {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBuffer {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .GetVkBuffer
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn set_access_flags(&self, access_flags: diligent_sys::VkAccessFlags) {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .SetAccessFlags
                .unwrap_unchecked()(self.sys_ptr, access_flags)
        }
    }

    pub fn get_access_flags(&self) -> diligent_sys::VkAccessFlags {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .GetAccessFlags
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_vk_device_address(&self) -> diligent_sys::VkDeviceAddress {
        unsafe {
            (*self.virtual_functions)
                .BufferVk
                .GetVkDeviceAddress
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
