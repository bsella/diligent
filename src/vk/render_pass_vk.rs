use std::ops::Deref;

use crate::render_pass::RenderPass;

#[repr(transparent)]
pub struct RenderPassVk<'a> {
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
        RenderPassVk { render_pass: value }
    }
}

impl RenderPassVk<'_> {
    pub fn get_vk_render_pass(&self) -> diligent_sys::VkRenderPass {
        unsafe_member_call!(self, RenderPassVk, GetVkRenderPass,)
    }
}
