use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::buffer::Buffer;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct BufferGL(diligent_sys::IBufferGL);

impl Deref for BufferGL {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IBuffer as *const Buffer) }
    }
}

impl BufferGL {
    pub fn get_buffer_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, BufferGL, GetGLBufferHandle)
    }
}
