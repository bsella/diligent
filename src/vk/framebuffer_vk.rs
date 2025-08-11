use std::ops::Deref;

use crate::frame_buffer::Framebuffer;

pub struct FramebufferVk<'a> {
    #[allow(dead_code)]
    sys_ptr: *mut diligent_sys::IFramebufferVk,
    #[allow(dead_code)]
    virtual_functions: *mut diligent_sys::IFramebufferVkVtbl,

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
        FramebufferVk {
            framebuffer: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IFramebufferVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IFramebufferVk)).pVtbl
            },
        }
    }
}

impl FramebufferVk<'_> {
    pub fn get_vk_framebuffer(&self) -> diligent_sys::VkFramebuffer {
        todo!()
        //unsafe_member_call!(self, FramebufferVk, GetVkFramebuffer,)
    }
}
