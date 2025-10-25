use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::shader::Shader;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IShaderGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ShaderGL<'a>(&'a Shader);

impl<'a> Deref for ShaderGL<'a> {
    type Target = Shader;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a Shader> for ShaderGL<'a> {
    fn from(value: &'a Shader) -> Self {
        ShaderGL(value)
    }
}

impl ShaderGL<'_> {
    pub fn get_shader_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, ShaderGL, GetGLShaderHandle)
    }
}
