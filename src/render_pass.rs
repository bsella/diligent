use std::ffi::CString;

use static_assertions::const_assert;

use crate::{
    device_object::DeviceObject,
    graphics_types::{AccessFlags, PipelineStageFlags, ResourceState, TextureFormat},
};

pub struct RenderPass {
    pub(crate) sys_ptr: *mut diligent_sys::IRenderPass,
    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for RenderPass {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

pub enum AttachmentLoadOperation {
    Load,
    Clear,
    Discard,
}
const_assert!(diligent_sys::ATTACHMENT_LOAD_OP_COUNT == 3);

pub enum AttachmentStoreOperation {
    Store,
    Discard,
}
const_assert!(diligent_sys::ATTACHMENT_STORE_OP_COUNT == 2);

pub struct RenderPassAttachmentDesc {
    pub format: Option<TextureFormat>,

    pub sample_count: u8,

    pub load_op: AttachmentLoadOperation,
    pub store_op: AttachmentStoreOperation,

    pub stencil_load_op: AttachmentLoadOperation,
    pub stencil_store_op: AttachmentStoreOperation,

    pub initial_state: Option<ResourceState>,
    pub final_state: Option<ResourceState>,
}

pub struct AttachmentReference {
    pub attachment_index: usize,

    pub state: Option<ResourceState>,
}

pub enum RenderTargetAttachments {
    RenderTargets(Vec<AttachmentReference>),
    RenderTargetsAndResolve(Vec<(AttachmentReference, AttachmentReference)>),
}

pub struct ShadingRateAttachment {
    pub attachment: AttachmentReference,
    pub tile_size: [u32; 2],
}

pub struct SubpassDesc {
    pub input_attachments: Vec<AttachmentReference>,
    pub render_target_attachments: RenderTargetAttachments,

    pub depth_stencil_attachment: Option<AttachmentReference>,

    pub preserve_attachments: Vec<u32>,

    pub shading_rate_attachment: Option<ShadingRateAttachment>,
}

impl Default for SubpassDesc {
    fn default() -> Self {
        SubpassDesc {
            input_attachments: Vec::new(),
            render_target_attachments: RenderTargetAttachments::RenderTargets(Vec::new()),
            depth_stencil_attachment: None,
            preserve_attachments: Vec::new(),
            shading_rate_attachment: None,
        }
    }
}

pub struct SubpassDependencyDesc {
    pub src_subpass_index: usize,
    pub dst_subpass_index: usize,

    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,

    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}

impl Default for SubpassDependencyDesc {
    fn default() -> Self {
        SubpassDependencyDesc {
            src_subpass_index: 0,
            dst_subpass_index: 0,

            src_stage_mask: PipelineStageFlags::Undefined,
            dst_stage_mask: PipelineStageFlags::Undefined,

            src_access_mask: AccessFlags::None,
            dst_access_mask: AccessFlags::None,
        }
    }
}

pub struct RenderPassDesc {
    pub name: CString,
    attachments: Vec<RenderPassAttachmentDesc>,
    subpasses: Vec<SubpassDesc>,
    dependencies: Vec<SubpassDependencyDesc>,
}

impl RenderPassDesc {
    pub fn new(name: impl AsRef<str>) -> Self {
        RenderPassDesc {
            name: CString::new(name.as_ref()).unwrap(),
            attachments: Vec::new(),
            subpasses: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn attachments(mut self, attachments: Vec<RenderPassAttachmentDesc>) -> Self {
        self.attachments = attachments;
        self
    }
    pub fn subpasses(mut self, subpasses: Vec<SubpassDesc>) -> Self {
        self.subpasses = subpasses;
        self
    }
    pub fn dependencies(mut self, dependencies: Vec<SubpassDependencyDesc>) -> Self {
        self.dependencies = dependencies;
        self
    }
}

impl RenderPass {
    //pub(crate) fn new(sys_ptr: *mut diligent_sys::IRenderPass) -> Self {
    //    // Both base and derived classes have exactly the same size.
    //    // This means that we can up-cast to the base class without worrying about layout offset between both classes
    //    const_assert!(
    //        std::mem::size_of::<diligent_sys::IObject>()
    //            == std::mem::size_of::<diligent_sys::IRenderPass>()
    //    );
    //    RenderPass {
    //        sys_ptr,
    //        device_object: DeviceObject::new(sys_ptr as *mut diligent_sys::IDeviceObject),
    //    }
    //}

    pub fn get_desc(&self) -> RenderPassDesc {
        todo!()
    }
}
