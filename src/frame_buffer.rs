use std::ffi::CString;

use crate::{render_pass::RenderPass, texture_view::TextureView};

use super::device_object::DeviceObject;

pub struct Framebuffer {
    pub(crate) sys_ptr: *mut diligent_sys::IFramebuffer,
    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for Framebuffer {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

pub struct FramebufferDesc<'a> {
    pub name: CString,

    pub render_pass: &'a RenderPass,

    pub attachments: Vec<&'a TextureView>,
    pub width: u32,
    pub height: u32,
    pub num_array_slices: u32,
}

impl Framebuffer {
    //pub(crate) fn new(sys_ptr: *mut diligent_sys::IFramebuffer) -> Self {
    //    // Both base and derived classes have exactly the same size.
    //    // This means that we can up-cast to the base class without worrying about layout offset between both classes
    //    const_assert!(
    //        std::mem::size_of::<diligent_sys::IDeviceObject>()
    //            == std::mem::size_of::<diligent_sys::IFramebuffer>()
    //    );
    //    Framebuffer {
    //        sys_ptr,
    //        device_object: DeviceObject::new(sys_ptr as *mut diligent_sys::IDeviceObject),
    //    }
    //}

    pub fn get_desc(&self) -> FramebufferDesc {
        todo!()
    }
}
