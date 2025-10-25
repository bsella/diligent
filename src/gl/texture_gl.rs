use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::texture::Texture;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureGLMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TextureGL<'a>(&'a Texture);

impl Deref for TextureGL<'_> {
    type Target = Texture;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a Texture> for TextureGL<'a> {
    fn from(value: &'a Texture) -> Self {
        TextureGL(value)
    }
}

impl TextureGL<'_> {
    pub fn get_texture_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, TextureGL, GetGLTextureHandle)
    }

    pub fn get_bind_target(&self) -> diligent_sys::GLenum {
        unsafe_member_call!(self, TextureGL, GetBindTarget)
    }
}
