use std::ops::Deref;

use crate::render_pass::RenderPass;

pub struct RenderPassVk<'a> {
    sys_ptr: *mut diligent_sys::IRenderPassVk,
    virtual_functions: *mut diligent_sys::IRenderPassVkVtbl,

    render_pass: &'a RenderPass,
}

impl Deref for RenderPassVk<'_> {
    type Target = RenderPass;
    fn deref(&self) -> &Self::Target {
        self.render_pass
    }
}

impl<'a> From<&'a RenderPass> for RenderPassVk<'a> {
    fn from(value: &'a RenderPass) -> Self {
        RenderPassVk {
            render_pass: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IRenderPassVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IRenderPassVk)).pVtbl
            },
        }
    }
}

impl RenderPassVk<'_> {
    pub fn get_vk_render_pass(&self) -> diligent_sys::VkRenderPass {
        unsafe_member_call!(self, RenderPassVk, GetVkRenderPass,)
    }
}
