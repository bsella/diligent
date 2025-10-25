use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{ShaderType, pipeline_state::PipelineState};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IPipelineStateGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct PipelineStateGL<'a>(&'a PipelineState);

impl<'a> Deref for PipelineStateGL<'a> {
    type Target = PipelineState;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a PipelineState> for PipelineStateGL<'a> {
    fn from(value: &'a PipelineState) -> Self {
        PipelineStateGL(value)
    }
}

impl PipelineStateGL<'_> {
    pub fn get_pipeline_state_handle(&self, state: ShaderType) -> diligent_sys::GLuint {
        unsafe_member_call!(self, PipelineStateGL, GetGLProgramHandle, state.into())
    }
}
