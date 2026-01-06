use crate::frame_buffer::Framebuffer;

define_ported!(
    FramebufferVk,
    diligent_sys::IFramebufferVk,
    diligent_sys::IFramebufferVkMethods : 1,
    Framebuffer
);

impl FramebufferVk {
    pub fn get_vk_framebuffer(&self) -> diligent_sys::VkFramebuffer {
        unsafe_member_call!(self, FramebufferVk, GetVkFramebuffer)
    }
}
