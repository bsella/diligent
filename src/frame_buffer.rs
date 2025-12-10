use std::{ffi::CString, ops::Deref};

use crate::{device_object::DeviceObject, render_pass::RenderPass, texture_view::TextureView};

#[repr(transparent)]
pub struct Framebuffer(diligent_sys::IFramebuffer);

impl Deref for Framebuffer {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IDeviceObject
                as *const DeviceObject)
        }
    }
}

pub struct FramebufferDesc<'a> {
    pub name: Option<CString>,

    pub render_pass: &'a RenderPass,

    pub attachments: Vec<&'a TextureView>,
    pub width: u32,
    pub height: u32,
    pub num_array_slices: u32,
}

impl Framebuffer {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IFramebuffer {
        std::ptr::from_ref(&self.0) as _
    }
}
