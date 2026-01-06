use crate::{pipeline_state::PipelineState, vk::render_pass_vk::RenderPassVk};

define_ported!(
    PipelineStateVk,
    diligent_sys::IPipelineStateVk,
    diligent_sys::IPipelineStateVkMethods : 2,
    PipelineState
);

impl PipelineStateVk {
    pub fn get_render_pass(&self) -> Option<&RenderPassVk> {
        let render_pass_ptr = unsafe_member_call!(self, PipelineStateVk, GetRenderPass);
        if render_pass_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(render_pass_ptr as *const RenderPassVk) })
        }
    }

    pub fn get_vk_pipeline(&self) -> diligent_sys::VkPipeline {
        unsafe_member_call!(self, PipelineStateVk, GetVkPipeline)
    }
}
