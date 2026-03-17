use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use static_assertions::const_assert_eq;

use crate::{
    device_object::{DeviceObject, DeviceObjectAttribs},
    graphics_types::{AccessFlags, PipelineStageFlags, ResourceState, TextureFormat},
};

define_ported!(RenderPass, diligent_sys::IRenderPass, DeviceObject);

impl RenderPass {
    pub fn desc(&self) -> &RenderPassDesc<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const RenderPassDesc) }
    }
}

#[derive(Clone, Copy)]
pub enum AttachmentLoadOperation {
    Load,
    Clear,
    Discard,
}
const_assert_eq!(diligent_sys::ATTACHMENT_LOAD_OP_COUNT, 3);

impl From<AttachmentLoadOperation> for diligent_sys::ATTACHMENT_LOAD_OP {
    fn from(value: AttachmentLoadOperation) -> Self {
        (match value {
            AttachmentLoadOperation::Load => diligent_sys::ATTACHMENT_LOAD_OP_LOAD,
            AttachmentLoadOperation::Clear => diligent_sys::ATTACHMENT_LOAD_OP_CLEAR,
            AttachmentLoadOperation::Discard => diligent_sys::ATTACHMENT_LOAD_OP_DISCARD,
        }) as _
    }
}

impl From<diligent_sys::ATTACHMENT_LOAD_OP> for AttachmentLoadOperation {
    fn from(value: diligent_sys::ATTACHMENT_LOAD_OP) -> Self {
        match value as _ {
            diligent_sys::ATTACHMENT_LOAD_OP_LOAD => AttachmentLoadOperation::Load,
            diligent_sys::ATTACHMENT_LOAD_OP_CLEAR => AttachmentLoadOperation::Clear,
            diligent_sys::ATTACHMENT_LOAD_OP_DISCARD => AttachmentLoadOperation::Discard,
            _ => panic!("Unknown ATTACHMENT_LOAD_OP value"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AttachmentStoreOperation {
    Store,
    Discard,
}
const_assert_eq!(diligent_sys::ATTACHMENT_STORE_OP_COUNT, 2);

impl From<AttachmentStoreOperation> for diligent_sys::ATTACHMENT_STORE_OP {
    fn from(value: AttachmentStoreOperation) -> Self {
        (match value {
            AttachmentStoreOperation::Store => diligent_sys::ATTACHMENT_STORE_OP_STORE,
            AttachmentStoreOperation::Discard => diligent_sys::ATTACHMENT_STORE_OP_DISCARD,
        }) as _
    }
}

impl From<diligent_sys::ATTACHMENT_STORE_OP> for AttachmentStoreOperation {
    fn from(value: diligent_sys::ATTACHMENT_STORE_OP) -> Self {
        match value as _ {
            diligent_sys::ATTACHMENT_STORE_OP_STORE => AttachmentStoreOperation::Store,
            diligent_sys::ATTACHMENT_STORE_OP_DISCARD => AttachmentStoreOperation::Discard,
            _ => panic!("Unknown ATTACHMENT_STORE_OP value"),
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct RenderPassAttachmentDesc(diligent_sys::RenderPassAttachmentDesc);

#[bon::bon]
impl RenderPassAttachmentDesc {
    #[builder(derive(Clone))]
    pub fn new(
        format: Option<TextureFormat>,

        #[builder(default = 1)] sample_count: u8,

        #[builder(default = AttachmentLoadOperation::Load)] load_op: AttachmentLoadOperation,
        #[builder(default = AttachmentStoreOperation::Store)] store_op: AttachmentStoreOperation,

        #[builder(default = AttachmentLoadOperation::Load)]
        stencil_load_op: AttachmentLoadOperation,
        #[builder(default = AttachmentStoreOperation::Store)]
        stencil_store_op: AttachmentStoreOperation,

        initial_state: Option<ResourceState>,
        final_state: Option<ResourceState>,
    ) -> Self {
        Self(diligent_sys::RenderPassAttachmentDesc {
            Format: format.map_or(diligent_sys::TEX_FORMAT_UNKNOWN as _, |f| f.into()),
            SampleCount: sample_count,
            LoadOp: load_op.into(),
            StoreOp: store_op.into(),
            StencilLoadOp: stencil_load_op.into(),
            StencilStoreOp: stencil_store_op.into(),
            InitialState: initial_state
                .map_or(diligent_sys::RESOURCE_STATE_UNKNOWN as _, |state| {
                    state.bits()
                }),
            FinalState: final_state.map_or(diligent_sys::RESOURCE_STATE_UNKNOWN as _, |state| {
                state.bits()
            }),
        })
    }
}

impl RenderPassAttachmentDesc {
    pub fn format(&self) -> Option<TextureFormat> {
        TextureFormat::from_sys(self.0.Format)
    }
    pub fn sample_count(&self) -> u8 {
        self.0.SampleCount
    }
    pub fn load_op(&self) -> AttachmentLoadOperation {
        self.0.LoadOp.into()
    }
    pub fn store_op(&self) -> AttachmentStoreOperation {
        self.0.StoreOp.into()
    }
    pub fn stencil_load_op(&self) -> AttachmentLoadOperation {
        self.0.StencilLoadOp.into()
    }
    pub fn stencil_store_op(&self) -> AttachmentStoreOperation {
        self.0.StencilStoreOp.into()
    }
    pub fn initial_state(&self) -> Option<ResourceState> {
        ResourceState::from_sys(self.0.InitialState)
    }
    pub fn final_state(&self) -> Option<ResourceState> {
        ResourceState::from_sys(self.0.FinalState)
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AttachmentReference(diligent_sys::AttachmentReference);

#[bon::bon]
impl AttachmentReference {
    #[builder(derive(Clone))]
    pub fn new(index: usize, state: Option<ResourceState>) -> Self {
        Self(diligent_sys::AttachmentReference {
            AttachmentIndex: index as u32,
            State: state.map_or(diligent_sys::RESOURCE_STATE_UNKNOWN as _, |state| {
                state.bits()
            }),
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ShadingRateAttachment(diligent_sys::ShadingRateAttachment);

#[bon::bon]
impl ShadingRateAttachment {
    #[builder(derive(Clone))]
    pub fn new(attachment: AttachmentReference, tile_size: [u32; 2]) -> Self {
        Self(diligent_sys::ShadingRateAttachment {
            Attachment: attachment.0,
            TileSize: tile_size,
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct SubpassDesc<
    'input_attachments,
    'rt_attachments,
    'resolve_attachments,
    'depth_stencil_attachment,
    'preserve_attachments,
    'shading_rate_attachment,
>(
    diligent_sys::SubpassDesc,
    PhantomData<(
        &'input_attachments (),
        &'rt_attachments (),
        &'resolve_attachments (),
        &'depth_stencil_attachment (),
        &'preserve_attachments (),
        &'shading_rate_attachment (),
    )>,
);

#[bon::bon]
impl<
    'input_attachments,
    'rt_attachments,
    'resolve_attachments,
    'depth_stencil_attachment,
    'preserve_attachments,
    'shading_rate_attachment,
>
    SubpassDesc<
        'input_attachments,
        'rt_attachments,
        'resolve_attachments,
        'depth_stencil_attachment,
        'preserve_attachments,
        'shading_rate_attachment,
    >
{
    #[builder(derive(Clone))]
    pub fn new(
        #[builder(default = &[])] input_attachments: &'input_attachments [AttachmentReference],
        render_target_attachments: &'rt_attachments [AttachmentReference],
        resolve_attachments: Option<&'resolve_attachments [AttachmentReference]>,

        depth_stencil_attachment: Option<&'depth_stencil_attachment AttachmentReference>,

        #[builder(default = &[])] preserve_attachments: &'preserve_attachments [u32],

        shading_rate_attachment: Option<&'shading_rate_attachment ShadingRateAttachment>,
    ) -> Self {
        Self(
            diligent_sys::SubpassDesc {
                InputAttachmentCount: input_attachments.len() as u32,
                pInputAttachments: input_attachments
                    .first()
                    .map_or(std::ptr::null(), |att| std::ptr::from_ref(&att.0)),
                RenderTargetAttachmentCount: render_target_attachments.len() as u32,
                pRenderTargetAttachments: render_target_attachments
                    .first()
                    .map_or(std::ptr::null(), |att| std::ptr::from_ref(&att.0)),
                pResolveAttachments: resolve_attachments.map_or(std::ptr::null(), |atts| {
                    atts.first()
                        .map_or(std::ptr::null(), |att| std::ptr::from_ref(&att.0))
                }),
                pDepthStencilAttachment: depth_stencil_attachment
                    .map_or(std::ptr::null(), |att: &AttachmentReference| {
                        std::ptr::from_ref(&att.0)
                    }),
                PreserveAttachmentCount: preserve_attachments.len() as u32,
                pPreserveAttachments: preserve_attachments.as_ptr(),
                pShadingRateAttachment: shading_rate_attachment
                    .map_or(std::ptr::null(), |att| std::ptr::from_ref(&att.0)),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct SubpassDependencyDesc(diligent_sys::SubpassDependencyDesc);

#[bon::bon]
impl SubpassDependencyDesc {
    #[builder(derive(Clone))]
    pub fn new(
        src_subpass_index: usize,
        dst_subpass_index: usize,

        #[builder(default = PipelineStageFlags::Undefined)] src_stage_mask: PipelineStageFlags,
        #[builder(default = PipelineStageFlags::Undefined)] dst_stage_mask: PipelineStageFlags,

        #[builder(default = AccessFlags::None)] src_access_mask: AccessFlags,
        #[builder(default = AccessFlags::None)] dst_access_mask: AccessFlags,
    ) -> Self {
        Self(diligent_sys::SubpassDependencyDesc {
            SrcSubpass: src_subpass_index as u32,
            DstSubpass: dst_subpass_index as u32,
            SrcStageMask: src_stage_mask.bits(),
            DstStageMask: dst_stage_mask.bits(),
            SrcAccessMask: src_access_mask.bits(),
            DstAccessMask: dst_access_mask.bits(),
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct RenderPassDesc<
    'name,
    'render_passes,
    'subpasses,
    'dependencies,
    'input_attachments,
    'rt_attachments,
    'resolve_attachments,
    'depth_stencil_attachment,
    'preserve_attachments,
    'shading_rate_attachment,
>(
    pub(crate) diligent_sys::RenderPassDesc,
    PhantomData<(
        &'name (),
        &'render_passes (),
        &'subpasses (),
        &'dependencies (),
        &'input_attachments (),
        &'rt_attachments (),
        &'resolve_attachments (),
        &'depth_stencil_attachment (),
        &'preserve_attachments (),
        &'shading_rate_attachment (),
    )>,
);

#[bon::bon]
impl<
    'name,
    'render_passes,
    'subpasses,
    'dependencies,
    'input_attachments,
    'rt_attachments,
    'resolve_attachments,
    'depth_stencil_attachment,
    'preserve_attachments,
    'shading_rate_attachment,
>
    RenderPassDesc<
        'name,
        'render_passes,
        'subpasses,
        'dependencies,
        'input_attachments,
        'rt_attachments,
        'resolve_attachments,
        'depth_stencil_attachment,
        'preserve_attachments,
        'shading_rate_attachment,
    >
{
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,
        attachments: &'render_passes [RenderPassAttachmentDesc],
        subpasses: &'subpasses [SubpassDesc<
            'input_attachments,
            'rt_attachments,
            'resolve_attachments,
            'depth_stencil_attachment,
            'preserve_attachments,
            'shading_rate_attachment,
        >],
        dependencies: &'dependencies [SubpassDependencyDesc],
    ) -> Self {
        Self(
            diligent_sys::RenderPassDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
                },
                AttachmentCount: attachments.len() as u32,
                pAttachments: attachments.first().map_or(std::ptr::null(), |att| &att.0),
                SubpassCount: subpasses.len() as u32,
                pSubpasses: subpasses.first().map_or(std::ptr::null(), |subp| &subp.0),
                DependencyCount: dependencies.len() as u32,
                pDependencies: dependencies.first().map_or(std::ptr::null(), |dep| &dep.0),
            },
            PhantomData,
        )
    }
}

impl RenderPassDesc<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn attachments(&self) -> &[RenderPassAttachmentDesc] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.pAttachments as *const _,
                self.0.AttachmentCount as usize,
            )
        }
    }

    pub fn subpasses(&self) -> &[SubpassDesc<'_, '_, '_, '_, '_, '_>] {
        unsafe {
            std::slice::from_raw_parts(self.0.pSubpasses as *const _, self.0.SubpassCount as usize)
        }
    }
}

impl Deref for RenderPassDesc<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}
