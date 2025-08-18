use std::{ffi::CString, ops::Deref};

use static_assertions::const_assert_eq;

use crate::{device_object::DeviceObject, render_pass::RenderPass, texture_view::TextureView};

#[repr(transparent)]
pub struct Framebuffer(DeviceObject);

impl Deref for Framebuffer {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.0
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
    pub(crate) fn new(sys_ptr: *mut diligent_sys::IFramebuffer) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IDeviceObject>(),
            std::mem::size_of::<diligent_sys::IFramebuffer>()
        );
        Self(DeviceObject::new(
            sys_ptr as *mut diligent_sys::IDeviceObject,
        ))
    }
}
