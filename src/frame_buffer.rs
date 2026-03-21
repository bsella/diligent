use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use crate::{
    Ported,
    device_object::{
        DeviceObject, DeviceObjectAttribs, ResourceStateNoTransition, ResourceStateTransition,
        ResourceStateVerify,
    },
    render_pass::RenderPass,
    texture_view::TextureView,
};

define_ported!(Framebuffer, diligent_sys::IFramebuffer, DeviceObject);

#[repr(transparent)]
#[derive(Clone)]
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
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,
        render_pass: &'render_pass RenderPass,
        attachments: &'texture_views [&'texture_view TextureView],
        #[builder(default = 0)] width: u32,
        #[builder(default = 0)] height: u32,
        #[builder(default = 0)] num_array_slices: u32,
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

impl FramebufferDesc<'_, '_, '_, '_> {
    pub fn render_pass(&self) -> &RenderPass {
        unsafe { &*(self.0.pRenderPass as *const RenderPass) }
    }
    pub fn attachments(&self) -> &[&TextureView] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.ppAttachments as *const &TextureView,
                self.0.AttachmentCount as usize,
            )
        }
    }
    pub fn width(&self) -> u32 {
        self.0.Width
    }
    pub fn height(&self) -> u32 {
        self.0.Height
    }
    pub fn num_array_slices(&self) -> u32 {
        self.0.NumArraySlices
    }
}

impl Framebuffer {
    pub fn desc(&self) -> &FramebufferDesc<'_, '_, '_, '_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const &FramebufferDesc) }
    }
}

impl Framebuffer {
    pub fn transition_state(&mut self) -> ResourceStateTransition<'_, Framebuffer> {
        ResourceStateTransition::new(self)
    }
    pub fn verify_state(&self) -> ResourceStateVerify<'_, Framebuffer> {
        ResourceStateVerify::new(self)
    }
    pub fn no_state_transition(&self) -> ResourceStateNoTransition<'_, Framebuffer> {
        ResourceStateNoTransition::new(self)
    }
}

// # Safety : Access to Framebuffer can be thread safe
unsafe impl Sync for Framebuffer {}
