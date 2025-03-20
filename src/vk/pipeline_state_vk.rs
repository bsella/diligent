use crate::{device_object::DeviceObject, pipeline_state::PipelineState};

pub struct PipelineStateVk<'a> {
    pipeline_state_ptr: *mut diligent_sys::IPipelineStateVk,
    virtual_functions: *mut diligent_sys::IPipelineStateVkVtbl,

    pipeline_state: &'a PipelineState,
}

impl AsRef<DeviceObject> for PipelineStateVk<'_> {
    fn as_ref(&self) -> &DeviceObject {
        self.pipeline_state.as_ref()
    }
}

impl<'a> From<&'a PipelineState> for PipelineStateVk<'a> {
    fn from(value: &'a PipelineState) -> Self {
        PipelineStateVk {
            pipeline_state: value,
            pipeline_state_ptr: value.sys_ptr as *mut diligent_sys::IPipelineStateVk,
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
        //        .unwrap_unchecked()(self.pipeline_state_ptr)
        //}
    }

    pub fn get_vk_pipeline(&self) -> diligent_sys::VkPipeline {
        unsafe {
            (*self.virtual_functions)
                .PipelineStateVk
                .GetVkPipeline
                .unwrap_unchecked()(self.pipeline_state_ptr)
        }
    }
}
