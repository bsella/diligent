use std::ops::Deref;

use crate::buffer_view::BufferView;

pub struct BufferViewVk<'a> {
    sys_ptr: *mut diligent_sys::IBufferViewVk,
    virtual_functions: *mut diligent_sys::IBufferViewVkVtbl,

    buffer_view: &'a BufferView<'a>,
}

impl<'a> Deref for BufferViewVk<'a> {
    type Target = BufferView<'a>;
    fn deref(&self) -> &Self::Target {
        self.buffer_view
    }
}

impl<'a> From<&'a BufferView<'a>> for BufferViewVk<'a> {
    fn from(value: &'a BufferView) -> Self {
        BufferViewVk {
            buffer_view: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IBufferViewVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IBufferViewVk)).pVtbl
            },
        }
    }
}

impl BufferViewVk<'_> {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBufferView {
        unsafe {
            (*self.virtual_functions)
                .BufferViewVk
                .GetVkBufferView
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
