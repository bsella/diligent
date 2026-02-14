use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use crate::{
    Ported,
    device_object::{DeviceObject, DeviceObjectAttribs},
    render_pass::RenderPass,
    texture_view::TextureView,
};

define_ported!(Framebuffer, diligent_sys::IFramebuffer, DeviceObject);

#[repr(transparent)]
pub struct FramebufferDesc<'name, 'render_pass, 'texture_views, 'texture_view>(
    pub(crate) diligent_sys::FramebufferDesc,
    PhantomData<(
        &'name (),
        &'render_pass (),
        &'texture_views (),
        &'texture_view (),
    )>,
);

impl Deref for FramebufferDesc<'_, '_, '_, '_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name, 'render_pass, 'texture_views, 'texture_view>
    FramebufferDesc<'name, 'render_pass, 'texture_views, 'texture_view>
{
    #[builder]
    pub fn new(
        name: Option<&'name CStr>,
        render_pass: &'render_pass RenderPass,
        attachments: &'texture_views [&'texture_view TextureView],
        width: u32,
        height: u32,
        num_array_slices: u32,
    ) -> Self {
        Self(
            diligent_sys::FramebufferDesc {
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
            },
            PhantomData,
        )
    }
}

impl Framebuffer {
    pub fn desc(&self) -> &FramebufferDesc<'_, '_, '_, '_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const &FramebufferDesc) }
    }
}
