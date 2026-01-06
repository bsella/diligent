use crate::buffer_view::BufferView;

define_ported!(
    BufferViewVk,
    diligent_sys::IBufferViewVk,
    diligent_sys::IBufferViewVkMethods : 1,
    BufferView
);

impl BufferViewVk {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBufferView {
        unsafe_member_call!(self, BufferViewVk, GetVkBufferView)
    }
}
