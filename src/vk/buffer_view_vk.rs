use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::buffer_view::BufferView;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferViewVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct BufferViewVk(diligent_sys::IBufferViewVk);

impl Deref for BufferViewVk {
    type Target = BufferView;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IBufferView as *const BufferView)
        }
    }
}

impl BufferViewVk {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBufferView {
        unsafe_member_call!(self, BufferViewVk, GetVkBufferView)
    }
}
