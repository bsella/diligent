use crate::shader::Shader;

define_ported!(
    ShaderGL,
    diligent_sys::IShaderGL,
    diligent_sys::IShaderGLMethods : 1,
    Shader
);

impl ShaderGL {
    pub fn get_shader_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, ShaderGL, GetGLShaderHandle)
    }
}
