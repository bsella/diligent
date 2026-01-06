use crate::texture::Texture;

define_ported!(
    TextureGL,
    diligent_sys::ITextureGL,
    diligent_sys::ITextureGLMethods : 2,
    Texture
);

impl TextureGL {
    pub fn get_texture_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, TextureGL, GetGLTextureHandle)
    }

    pub fn get_bind_target(&self) -> diligent_sys::GLenum {
        unsafe_member_call!(self, TextureGL, GetBindTarget)
    }
}
