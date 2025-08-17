use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::buffer_view::BufferView;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferViewVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct BufferViewVk<'a>(&'a BufferView<'a>);

impl<'a> Deref for BufferViewVk<'a> {
    type Target = BufferView<'a>;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a BufferView<'a>> for BufferViewVk<'a> {
    fn from(value: &'a BufferView) -> Self {
        BufferViewVk(value)
    }
}

impl BufferViewVk<'_> {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBufferView {
        unsafe_member_call!(self, BufferViewVk, GetVkBufferView)
    }
}
