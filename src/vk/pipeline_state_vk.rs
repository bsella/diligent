use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::pipeline_state::PipelineState;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IPipelineStateVkMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct PipelineStateVk<'a>(&'a PipelineState);

impl Deref for PipelineStateVk<'_> {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a PipelineState> for PipelineStateVk<'a> {
    fn from(value: &'a PipelineState) -> Self {
        PipelineStateVk(value)
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
        unsafe_member_call!(self, PipelineStateVk, GetVkPipeline)
    }
}
