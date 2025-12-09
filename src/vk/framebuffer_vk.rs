use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::frame_buffer::Framebuffer;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IFramebufferVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct FramebufferVk(diligent_sys::IFramebufferVk);

impl Deref for FramebufferVk {
    type Target = Framebuffer;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IFramebuffer
                as *const Framebuffer)
        }
    }
}

impl FramebufferVk {
    pub fn get_vk_framebuffer(&self) -> diligent_sys::VkFramebuffer {
        unsafe_member_call!(self, FramebufferVk, GetVkFramebuffer)
    }
}
