use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{pipeline_state::PipelineState, vk::render_pass_vk::RenderPassVk};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IPipelineStateVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct PipelineStateVk(diligent_sys::IPipelineStateVk);

impl Deref for PipelineStateVk {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IPipelineState
                as *const PipelineState)
        }
    }
}

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
