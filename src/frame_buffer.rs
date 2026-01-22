use std::ffi::CStr;

use crate::{device_object::DeviceObject, render_pass::RenderPass, texture_view::TextureView};

define_ported!(Framebuffer, diligent_sys::IFramebuffer, DeviceObject);

#[repr(transparent)]
pub struct FramebufferDesc(pub(crate) diligent_sys::FramebufferDesc);

#[bon::bon]
impl FramebufferDesc {
    #[builder]
    pub fn new(
        name: Option<&CStr>,
        render_pass: &RenderPass,
        attachments: &[&TextureView],
        width: u32,
        height: u32,
        num_array_slices: u32,
    ) -> Self {
        Self(diligent_sys::FramebufferDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            pRenderPass: render_pass.sys_ptr(),
            AttachmentCount: attachments.len() as u32,
            ppAttachments: attachments
                .first()
                .map_or(std::ptr::null(), |_| attachments.as_ptr() as *const *mut _),
            Width: width,
            Height: height,
            NumArraySlices: num_array_slices,
        })
    }
}

impl Framebuffer {
    pub fn desc(&self) -> &FramebufferDesc {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const &FramebufferDesc) }
    }
}
