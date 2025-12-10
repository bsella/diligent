use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{
    Boxed, Buffer, BufferDesc, ResourceState, Texture, TextureDesc, render_device::RenderDevice,
};

#[cfg(target_os = "windows")]
const_assert_eq!(
    std::mem::size_of::<diligent_sys::IRenderDeviceGLMethods>(),
    4 * std::mem::size_of::<*const ()>()
);

#[cfg(not(target_os = "windows"))]
const_assert_eq!(
    std::mem::size_of::<diligent_sys::IRenderDeviceGLMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[cfg(target_os = "windows")]
#[repr(transparent)]
pub struct NativeGLContextAttribsWin32(diligent_sys::NativeGLContextAttribsWin32);

#[cfg(target_os = "windows")]
pub type NativeGLContextAttribs = NativeGLContextAttribsWin32;

#[repr(transparent)]
pub struct RenderDeviceGL(diligent_sys::IRenderDeviceGL);

impl Deref for RenderDeviceGL {
    type Target = RenderDevice;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IRenderDevice
                as *const RenderDevice)
        }
    }
}

impl RenderDeviceGL {
    pub fn create_texture_from_gl_handle(
        &self,
        gl_handle: u32,
        gl_bind_target: u32,
        tex_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Texture>, ()> {
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

        if texture_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Texture>::new(texture_ptr as _))
        }
    }

    pub fn create_buffer_from_gl_handle(
        &self,
        gl_handle: u32,
        buff_desc: &BufferDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Buffer>, ()> {
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

        if buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Buffer>::new(buffer_ptr as _))
        }
    }

    pub fn create_dummy_texture(
        &self,
        tex_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Texture>, ()> {
        let mut texture_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDeviceGL,
            CreateDummyTexture,
            &tex_desc.0,
            initial_state.bits(),
            &mut texture_ptr
        );

        if texture_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Texture>::new(texture_ptr as _))
        }
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
