use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::texture::Texture;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureGLMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TextureGL(diligent_sys::ITextureGL);

impl Deref for TextureGL {
    type Target = Texture;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ITexture as *const Texture)
        }
    }
}

impl TextureGL {
    pub fn get_texture_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, TextureGL, GetGLTextureHandle)
    }

    pub fn get_bind_target(&self) -> diligent_sys::GLenum {
        unsafe_member_call!(self, TextureGL, GetBindTarget)
    }
}
