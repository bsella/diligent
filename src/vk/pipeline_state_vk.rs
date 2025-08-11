use std::ops::Deref;

use crate::pipeline_state::PipelineState;

#[repr(transparent)]
pub struct PipelineStateVk<'a> {
    pipeline_state: &'a PipelineState,
}

impl Deref for PipelineStateVk<'_> {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        self.pipeline_state
    }
}

impl<'a> From<&'a PipelineState> for PipelineStateVk<'a> {
    fn from(value: &'a PipelineState) -> Self {
        PipelineStateVk {
            pipeline_state: value,
        }
    }
}

impl PipelineStateVk<'_> {
    pub fn get_render_pass(&self) -> diligent_sys::IRenderPassVk {
        todo!()
        //unsafe {
        //    (*self.virtual_functions)
        //        .PipelineStateVk
        //        .GetRenderPass
        //        .unwrap_unchecked()(self.sys_ptr)
        //}
    }

    pub fn get_vk_pipeline(&self) -> diligent_sys::VkPipeline {
        unsafe_member_call!(self, PipelineStateVk, GetVkPipeline,)
    }
}
