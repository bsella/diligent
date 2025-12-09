use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::render_pass::RenderPass;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IRenderPassVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct RenderPassVk(diligent_sys::IRenderPassVk);

impl Deref for RenderPassVk {
    type Target = RenderPass;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IRenderPass as *const RenderPass)
        }
    }
}

impl RenderPassVk {
    pub fn get_vk_render_pass(&self) -> diligent_sys::VkRenderPass {
        unsafe_member_call!(self, RenderPassVk, GetVkRenderPass)
    }
}
