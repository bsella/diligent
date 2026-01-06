use crate::render_pass::RenderPass;

define_ported!(
    RenderPassVk,
    diligent_sys::IRenderPassVk,
    diligent_sys::IRenderPassVkMethods : 1,
    RenderPass
);

impl RenderPassVk {
    pub fn get_vk_render_pass(&self) -> diligent_sys::VkRenderPass {
        unsafe_member_call!(self, RenderPassVk, GetVkRenderPass)
    }
}
