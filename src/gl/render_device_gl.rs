use crate::{
    Boxed, BoxedFromNulError, Buffer, BufferDesc, ResourceState, Texture, TextureDesc,
    render_device::RenderDevice,
};

#[cfg(target_os = "windows")]
#[repr(transparent)]
pub struct NativeGLContextAttribsWin32(diligent_sys::NativeGLContextAttribsWin32);

#[cfg(target_os = "windows")]
pub type NativeGLContextAttribs = NativeGLContextAttribsWin32;

#[cfg(target_os = "windows")]
const IRENDER_DEVICE_GL_METHODS_COUNT: usize = 4;

#[cfg(not(target_os = "windows"))]
const IRENDER_DEVICE_GL_METHODS_COUNT: usize = 3;

define_ported!(
    RenderDeviceGL,
    diligent_sys::IRenderDeviceGL,
    diligent_sys::IRenderDeviceGLMethods : IRENDER_DEVICE_GL_METHODS_COUNT,
    RenderDevice
);

impl RenderDeviceGL {
    pub fn create_texture_from_gl_handle(
        &self,
        gl_handle: u32,
        gl_bind_target: u32,
        tex_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Texture>, BoxedFromNulError> {
        let mut texture_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDeviceGL,
            CreateTextureFromGLHandle,
            gl_handle,
            gl_bind_target,
            &tex_desc.0,
            initial_state.bits(),
            &mut texture_ptr
        );

        Boxed::new(texture_ptr)
    }

    pub fn create_buffer_from_gl_handle(
        &self,
        gl_handle: u32,
        buff_desc: &BufferDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Buffer>, BoxedFromNulError> {
        let mut buffer_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDeviceGL,
            CreateBufferFromGLHandle,
            gl_handle,
            &buff_desc.0,
            initial_state.bits(),
            &mut buffer_ptr
        );

        Boxed::new(buffer_ptr)
    }

    pub fn create_dummy_texture(
        &self,
        tex_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Texture>, BoxedFromNulError> {
        let mut texture_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDeviceGL,
            CreateDummyTexture,
            &tex_desc.0,
            initial_state.bits(),
            &mut texture_ptr
        );

        Boxed::new(texture_ptr)
    }

    #[cfg(target_os = "windows")]
    pub fn get_native_gl_context_attribs(&self) -> NativeGLContextAttribs {
        NativeGLContextAttribsWin32(unsafe_member_call!(
            self,
            RenderDeviceGL,
            GetNativeGLContextAttribs
        ))
    }
}
