use std::ops::Deref;

use crate::buffer_view::BufferView;

#[repr(transparent)]
pub struct BufferViewVk<'a> {
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
        BufferViewVk { buffer_view: value }
    }
}

impl BufferViewVk<'_> {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBufferView {
        unsafe_member_call!(self, BufferViewVk, GetVkBufferView,)
    }
}
