use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::shader::Shader;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IShaderGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ShaderGL(diligent_sys::IShaderGL);

impl Deref for ShaderGL {
    type Target = Shader;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IShader as *const Shader) }
    }
}

impl ShaderGL {
    pub fn get_shader_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, ShaderGL, GetGLShaderHandle)
    }
}
