use std::ops::Deref;

use crate::pipeline_state::PipelineState;

pub struct PipelineStateVk<'a> {
    sys_ptr: *mut diligent_sys::IPipelineStateVk,
    virtual_functions: *mut diligent_sys::IPipelineStateVkVtbl,

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
            sys_ptr: value.sys_ptr as *mut diligent_sys::IPipelineStateVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IPipelineStateVk)).pVtbl
            },
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
        unsafe {
            (*self.virtual_functions)
                .PipelineStateVk
                .GetVkPipeline
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
