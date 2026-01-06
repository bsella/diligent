use crate::buffer::Buffer;

define_ported!(
    BufferGL,
    diligent_sys::IBufferGL,
    diligent_sys::IBufferGLMethods : 1,
    Buffer
);

impl BufferGL {
    pub fn get_buffer_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, BufferGL, GetGLBufferHandle)
    }
}
