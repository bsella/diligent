use crate::{ShaderType, pipeline_state::PipelineState};

define_ported!(
    PipelineStateGL,
    diligent_sys::IPipelineStateGL,
    diligent_sys::IPipelineStateGLMethods : 1,
    PipelineState
);

impl PipelineStateGL {
    pub fn get_pipeline_state_handle(&self, state: ShaderType) -> diligent_sys::GLuint {
        unsafe_member_call!(self, PipelineStateGL, GetGLProgramHandle, state.into())
    }
}
