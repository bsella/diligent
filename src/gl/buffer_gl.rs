use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::buffer::Buffer;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct BufferGL<'a>(&'a Buffer);

impl<'a> Deref for BufferGL<'a> {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a Buffer> for BufferGL<'a> {
    fn from(value: &'a Buffer) -> Self {
        BufferGL(value)
    }
}

impl BufferGL<'_> {
    pub fn get_buffer_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, BufferGL, GetGLBufferHandle)
    }
}
