use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::frame_buffer::Framebuffer;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IFramebufferVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct FramebufferVk<'a> {
    framebuffer: &'a Framebuffer,
}

impl Deref for FramebufferVk<'_> {
    type Target = Framebuffer;
    fn deref(&self) -> &Self::Target {
        self.framebuffer
    }
}

impl<'a> From<&'a Framebuffer> for FramebufferVk<'a> {
    fn from(value: &'a Framebuffer) -> Self {
        FramebufferVk { framebuffer: value }
    }
}

impl FramebufferVk<'_> {
    pub fn get_vk_framebuffer(&self) -> diligent_sys::VkFramebuffer {
        todo!()
        //unsafe_member_call!(self, FramebufferVk, GetVkFramebuffer)
    }
}
