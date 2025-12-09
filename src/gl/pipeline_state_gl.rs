use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{ShaderType, pipeline_state::PipelineState};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IPipelineStateGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct PipelineStateGL(diligent_sys::IPipelineStateGL);

impl Deref for PipelineStateGL {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IPipelineState
                as *const PipelineState)
        }
    }
}

impl PipelineStateGL {
    pub fn get_pipeline_state_handle(&self, state: ShaderType) -> diligent_sys::GLuint {
        unsafe_member_call!(self, PipelineStateGL, GetGLProgramHandle, state.into())
    }
}
