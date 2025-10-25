use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{Buffer, BufferDesc, ResourceState, Texture, TextureDesc, render_device::RenderDevice};

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
struct NativeGLContextAttribsWin32(diligent_sys::NativeGLContextAttribsWin32);

#[cfg(target_os = "windows")]
pub type NativeGLContextAttribs = NativeGLContextAttribsWin32;

#[repr(transparent)]
pub struct RenderDeviceGL<'a>(&'a RenderDevice);

impl<'a> Deref for RenderDeviceGL<'a> {
    type Target = RenderDevice;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a RenderDevice> for RenderDeviceGL<'a> {
    fn from(value: &'a RenderDevice) -> Self {
        RenderDeviceGL(value)
    }
}

impl RenderDeviceGL<'_> {
    pub fn create_texture_from_gl_handle(
        &self,
        gl_handle: u32,
        gl_bind_target: u32,
        tex_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Result<Texture, ()> {
        let mut texture_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDeviceGL,
            CreateTextureFromGLHandle,
            gl_handle,
            gl_bind_target,
            std::ptr::from_ref(tex_desc) as _,
            initial_state.bits(),
            std::ptr::addr_of_mut!(texture_ptr)
        );

        if texture_ptr.is_null() {
            Err(())
        } else {
            Ok(Texture::new(texture_ptr))
        }
    }

    pub fn create_buffer_from_gl_handle(
        &self,
        gl_handle: u32,
        buff_desc: &BufferDesc,
        initial_state: ResourceState,
    ) -> Result<Buffer, ()> {
        let mut buffer_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDeviceGL,
            CreateBufferFromGLHandle,
            gl_handle,
            std::ptr::from_ref(buff_desc) as _,
            initial_state.bits(),
            std::ptr::addr_of_mut!(buffer_ptr)
        );

        if buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Buffer::new(buffer_ptr))
        }
    }

    pub fn create_dummy_texture(
        &self,
        tex_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Result<Texture, ()> {
        let mut texture_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDeviceGL,
            CreateDummyTexture,
            std::ptr::from_ref(tex_desc) as _,
            initial_state.bits(),
            std::ptr::addr_of_mut!(texture_ptr)
        );

        if texture_ptr.is_null() {
            Err(())
        } else {
            Ok(Texture::new(texture_ptr))
        }
    }

    // TODO
    #[cfg(target_os = "windows")]
    pub fn get_native_gl_context_attribs(&self) -> NativeGLContextAttribs {}
}
