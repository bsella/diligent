use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::render_pass::RenderPass;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IRenderPassVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct RenderPassVk<'a>(&'a RenderPass);

impl Deref for RenderPassVk<'_> {
    type Target = RenderPass;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a RenderPass> for RenderPassVk<'a> {
    fn from(value: &'a RenderPass) -> Self {
        RenderPassVk(value)
    }
}

impl RenderPassVk<'_> {
    pub fn get_vk_render_pass(&self) -> diligent_sys::VkRenderPass {
        unsafe_member_call!(self, RenderPassVk, GetVkRenderPass)
    }
}
