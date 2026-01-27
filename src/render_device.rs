use std::{ffi::CStr, os::raw::c_void};

use crate::{
    Boxed, BoxedFromNulError, ResourceMappingCreateInfo, SparseTextureFormatInfo,
    TextureFormatInfoExt, TilePipelineStateCreateInfo,
    blas::{BottomLevelAS, BottomLevelASDesc},
    buffer::{Buffer, BufferDesc},
    data_blob::DataBlob,
    device_context::{DeferredDeviceContext, DeviceContext},
    device_memory::{DeviceMemory, DeviceMemoryCreateInfo},
    engine_factory::EngineFactory,
    fence::{Fence, FenceDesc},
    frame_buffer::{Framebuffer, FramebufferDesc},
    graphics_types::{
        DeviceFeatures, GraphicsAdapterInfo, RenderDeviceType, TextureFormat, Version,
    },
    object::Object,
    pipeline_resource_signature::{PipelineResourceSignature, PipelineResourceSignatureDesc},
    pipeline_state::{
        ComputePipelineState, ComputePipelineStateCreateInfo, GraphicsPipelineState,
        GraphicsPipelineStateCreateInfo, RayTracingPipelineState,
        RayTracingPipelineStateCreateInfo, TilePipelineState,
    },
    pipeline_state_cache::{PipelineStateCache, PipelineStateCacheCreateInfo},
    query::{
        GetSysQueryType, Query, QueryDataBinaryOcclusion, QueryDataDuration, QueryDataOcclusion,
        QueryDataPipelineStatistics, QueryDataTimestamp,
    },
    render_pass::{RenderPass, RenderPassDesc, RenderTargetAttachments},
    resource_mapping::ResourceMapping,
    sampler::{Sampler, SamplerDesc},
    shader::{Shader, ShaderCreateInfo, ShaderCreateInfoWrapper},
    shader_binding_table::{ShaderBindingTable, ShaderBindingTableDesc},
    texture::{Texture, TextureDesc, TextureDimension, TextureSubResource},
    tlas::{TopLevelAS, TopLevelASDesc},
};

#[repr(transparent)]
pub struct RenderDeviceInfo(diligent_sys::RenderDeviceInfo);
impl RenderDeviceInfo {
    pub fn device_type(&self) -> RenderDeviceType {
        match self.0.Type {
            #[cfg(feature = "d3d11")]
            diligent_sys::RENDER_DEVICE_TYPE_D3D11 => RenderDeviceType::D3D11,
            #[cfg(feature = "d3d12")]
            diligent_sys::RENDER_DEVICE_TYPE_D3D12 => RenderDeviceType::D3D12,

            #[cfg(feature = "opengl")]
            diligent_sys::RENDER_DEVICE_TYPE_GL => RenderDeviceType::GL,

            #[cfg(feature = "vulkan")]
            diligent_sys::RENDER_DEVICE_TYPE_VULKAN => RenderDeviceType::VULKAN,

            #[cfg(feature = "metal")]
            diligent_sys::RENDER_DEVICE_TYPE_METAL => RenderDeviceType::METAL,

            #[cfg(feature = "webgpu")]
            diligent_sys::RENDER_DEVICE_TYPE_WEBGPU => RenderDeviceType::WEBGPU,

            _ => panic!("Unknown RENDER_DEVICE_TYPE value"),
        }
    }

    pub fn api_version(&self) -> Version {
        Version {
            major: self.0.APIVersion.Major,
            minor: self.0.APIVersion.Minor,
        }
    }

    pub fn features(&self) -> &DeviceFeatures {
        unsafe { std::mem::transmute(&self.0.Features) }
    }
    // TODO
    //NDCAttribs NDC DEFAULT_INITIALIZER({});
    //RenderDeviceShaderVersionInfo MaxShaderVersion DEFAULT_INITIALIZER({});
}

define_ported!(
    RenderDevice,
    diligent_sys::IRenderDevice,
    diligent_sys::IRenderDeviceMethods : 29,
    Object
);

impl RenderDevice {
    pub fn create_buffer(
        &self,
        buffer_desc: &BufferDesc,
    ) -> Result<Boxed<Buffer>, BoxedFromNulError> {
        let mut buffer_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateBuffer,
            &buffer_desc.0,
            std::ptr::null(),
            &mut buffer_ptr
        );

        Boxed::new(buffer_ptr)
    }

    pub fn create_buffer_with_data<T: ?Sized>(
        &self,
        buffer_desc: &BufferDesc,
        buffer_data: &T,
        device_context: Option<&DeviceContext>,
    ) -> Result<Boxed<Buffer>, BoxedFromNulError> {
        let mut buffer_ptr = std::ptr::null_mut();

        let buffer_data = diligent_sys::BufferData {
            pData: std::ptr::from_ref(buffer_data) as *const c_void,
            DataSize: std::mem::size_of_val(buffer_data) as u64,
            pContext: device_context.map_or(std::ptr::null_mut(), |context| context.sys_ptr()),
        };

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateBuffer,
            &buffer_desc.0,
            &buffer_data,
            &mut buffer_ptr
        );

        Boxed::new(buffer_ptr)
    }

    pub fn create_shader(
        &self,
        shader_ci: &ShaderCreateInfo,
    ) -> Result<Boxed<Shader>, Option<Boxed<DataBlob>>> {
        let mut shader_ptr = std::ptr::null_mut();
        let mut data_blob_ptr = std::ptr::null_mut();

        let shader_ci_wrapper = ShaderCreateInfoWrapper::from(shader_ci);

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateShader,
            std::ptr::from_ref(&shader_ci_wrapper),
            &mut shader_ptr,
            &mut data_blob_ptr
        );

        Boxed::new(shader_ptr).map_err(|_| Some(Boxed::new(data_blob_ptr).unwrap()))
    }

    pub fn create_texture(
        &self,
        texture_desc: &TextureDesc,
        subresources: &[TextureSubResource],
        device_context: Option<&DeviceContext>,
    ) -> Result<Boxed<Texture>, BoxedFromNulError> {
        let mut texture_ptr = std::ptr::null_mut();

        let texture_data = diligent_sys::TextureData {
            NumSubresources: subresources.len() as u32,
            pSubResources: subresources
                .first()
                .map_or(std::ptr::null_mut(), |subresource| {
                    std::ptr::from_ref(subresource) as _
                }),
            pContext: device_context.map_or(std::ptr::null_mut(), |c| c.sys_ptr() as _),
        };

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateTexture,
            &texture_desc.0,
            if device_context.is_none() && subresources.is_empty() {
                std::ptr::null()
            } else {
                &texture_data
            },
            &mut texture_ptr
        );

        Boxed::new(texture_ptr)
    }

    pub fn create_sampler(
        &self,
        sampler_desc: &SamplerDesc,
    ) -> Result<Boxed<Sampler>, BoxedFromNulError> {
        let mut sampler_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateSampler,
            &sampler_desc.0,
            &mut sampler_ptr
        );

        Boxed::new(sampler_ptr)
    }

    pub fn create_resource_mapping(
        &self,
        resource_mapping_ci: &ResourceMappingCreateInfo,
    ) -> Result<Boxed<ResourceMapping>, BoxedFromNulError> {
        let mut resource_mapping_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateResourceMapping,
            &resource_mapping_ci.0,
            &mut resource_mapping_ptr
        );

        Boxed::new(resource_mapping_ptr)
    }

    pub fn create_graphics_pipeline_state(
        &self,
        pipeline_ci: &GraphicsPipelineStateCreateInfo,
    ) -> Result<Boxed<GraphicsPipelineState>, BoxedFromNulError> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateGraphicsPipelineState,
            &pipeline_ci.0,
            &mut pipeline_state_ptr
        );

        Boxed::new(pipeline_state_ptr)
    }

    pub fn create_compute_pipeline_state(
        &self,
        pipeline_ci: &ComputePipelineStateCreateInfo,
    ) -> Result<Boxed<ComputePipelineState>, BoxedFromNulError> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateComputePipelineState,
            &pipeline_ci.0,
            &mut pipeline_state_ptr
        );

        Boxed::new(pipeline_state_ptr)
    }

    pub fn create_ray_tracing_pipeline_state(
        &self,
        pipeline_ci: &RayTracingPipelineStateCreateInfo,
    ) -> Result<Boxed<RayTracingPipelineState>, BoxedFromNulError> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateRayTracingPipelineState,
            &pipeline_ci.0,
            &mut pipeline_state_ptr
        );

        Boxed::new(pipeline_state_ptr)
    }

    pub fn create_tile_pipeline_state(
        &self,
        pipeline_ci: &TilePipelineStateCreateInfo,
    ) -> Result<Boxed<TilePipelineState>, BoxedFromNulError> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateTilePipelineState,
            &pipeline_ci.0,
            &mut pipeline_state_ptr
        );

        Boxed::new(pipeline_state_ptr)
    }

    pub fn create_fence(&self, fence_desc: &FenceDesc) -> Result<Boxed<Fence>, BoxedFromNulError> {
        let mut fence_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateFence,
            &fence_desc.0,
            &mut fence_ptr
        );

        Boxed::new(fence_ptr)
    }

    fn create_query<QueryDataType: GetSysQueryType>(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataType>>, BoxedFromNulError> {
        let query_desc = diligent_sys::QueryDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            Type: QueryDataType::QUERY_TYPE,
        };

        let mut query_ptr = std::ptr::null_mut();

        unsafe_member_call!(self, RenderDevice, CreateQuery, &query_desc, &mut query_ptr);

        Boxed::new(query_ptr)
    }

    pub fn create_query_occlusion(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataOcclusion>>, BoxedFromNulError> {
        self.create_query(name)
    }

    pub fn create_query_binary_occlusion(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataBinaryOcclusion>>, BoxedFromNulError> {
        self.create_query(name)
    }

    pub fn create_query_timestamp(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataTimestamp>>, BoxedFromNulError> {
        self.create_query(name)
    }

    pub fn create_query_pipeline_statistics(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataPipelineStatistics>>, BoxedFromNulError> {
        self.create_query(name)
    }

    pub fn create_query_duration(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataDuration>>, BoxedFromNulError> {
        self.create_query(name)
    }

    pub fn create_render_pass(
        &self,
        desc: &RenderPassDesc,
    ) -> Result<Boxed<RenderPass>, BoxedFromNulError> {
        let attachments = desc
            .attachments
            .iter()
            .map(|att| diligent_sys::RenderPassAttachmentDesc {
                Format: att
                    .format
                    .map_or(diligent_sys::TEX_FORMAT_UNKNOWN as _, |format| {
                        format.into()
                    }),
                SampleCount: att.sample_count,
                LoadOp: att.load_op.into(),
                StoreOp: att.store_op.into(),
                StencilLoadOp: att.stencil_load_op.into(),
                StencilStoreOp: att.stencil_store_op.into(),
                InitialState: att
                    .initial_state
                    .as_ref()
                    .map_or(diligent_sys::RESOURCE_STATE_UNKNOWN as _, |state| {
                        state.bits()
                    }),
                FinalState: att
                    .final_state
                    .as_ref()
                    .map_or(diligent_sys::RESOURCE_STATE_UNKNOWN as _, |state| {
                        state.bits()
                    }),
            })
            .collect::<Vec<_>>();

        struct SubpassWrapper {
            input_attachments: Vec<diligent_sys::AttachmentReference>,
            preserve_attachments: Vec<u32>,
            render_target_attachments: Vec<diligent_sys::AttachmentReference>,
            resolve_attachments: Vec<diligent_sys::AttachmentReference>,
            depth_stencil_attachments: Vec<diligent_sys::AttachmentReference>,
            shading_rate_attachments: Vec<diligent_sys::ShadingRateAttachment>,
        }

        let subpasses = desc
            .subpasses
            .iter()
            .map(|subpass| SubpassWrapper {
                input_attachments: subpass
                    .input_attachments
                    .iter()
                    .map(|att| att.into())
                    .collect(),
                preserve_attachments: subpass.preserve_attachments.clone(),
                render_target_attachments: match &subpass.render_target_attachments {
                    RenderTargetAttachments::RenderTargets(render_targets) => {
                        render_targets.iter().map(|att| att.into()).collect()
                    }
                    RenderTargetAttachments::RenderTargetsAndResolve(
                        render_targets_and_resolve,
                    ) => render_targets_and_resolve
                        .iter()
                        .map(|(att, _resolve)| att.into())
                        .collect(),
                },
                resolve_attachments: match &subpass.render_target_attachments {
                    RenderTargetAttachments::RenderTargets(_) => Vec::new(),
                    RenderTargetAttachments::RenderTargetsAndResolve(
                        render_targets_and_resolve,
                    ) => render_targets_and_resolve
                        .iter()
                        .map(|(_att, resolve)| resolve.into())
                        .collect(),
                },
                depth_stencil_attachments: subpass
                    .depth_stencil_attachment
                    .iter()
                    .map(|att| att.into())
                    .collect(),

                shading_rate_attachments: subpass
                    .shading_rate_attachment
                    .iter()
                    .map(|att| diligent_sys::ShadingRateAttachment {
                        Attachment: (&att.attachment).into(),
                        TileSize: att.tile_size,
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();

        let subpasses = subpasses
            .iter()
            .map(|subpass| diligent_sys::SubpassDesc {
                InputAttachmentCount: subpass.input_attachments.len() as u32,
                pInputAttachments: subpass.input_attachments.as_ptr(),
                PreserveAttachmentCount: subpass.preserve_attachments.len() as u32,
                pPreserveAttachments: subpass.preserve_attachments.as_ptr(),
                RenderTargetAttachmentCount: subpass.render_target_attachments.len() as u32,
                pRenderTargetAttachments: subpass.render_target_attachments.as_ptr(),
                pResolveAttachments: subpass.resolve_attachments.as_ptr(),
                pDepthStencilAttachment: subpass.depth_stencil_attachments.as_ptr(),
                pShadingRateAttachment: subpass.shading_rate_attachments.as_ptr(),
            })
            .collect::<Vec<_>>();

        let dependencies = desc
            .dependencies
            .iter()
            .map(|dep| diligent_sys::SubpassDependencyDesc {
                SrcSubpass: dep.src_subpass_index as u32,
                DstSubpass: dep.dst_subpass_index as u32,
                SrcStageMask: dep.src_stage_mask.bits(),
                DstStageMask: dep.dst_stage_mask.bits(),
                SrcAccessMask: dep.src_access_mask.bits(),
                DstAccessMask: dep.dst_access_mask.bits(),
            })
            .collect::<Vec<_>>();

        let desc = diligent_sys::RenderPassDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: desc
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            AttachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            SubpassCount: subpasses.len() as u32,
            pSubpasses: subpasses.as_ptr(),
            DependencyCount: dependencies.len() as u32,
            pDependencies: dependencies.as_ptr(),
        };

        let mut render_pass_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateRenderPass,
            &desc,
            &mut render_pass_ptr
        );

        Boxed::new(render_pass_ptr)
    }

    pub fn create_framebuffer(
        &self,
        desc: &FramebufferDesc,
    ) -> Result<Boxed<Framebuffer>, BoxedFromNulError> {
        let mut frame_buffer_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateFramebuffer,
            &desc.0,
            &mut frame_buffer_ptr
        );

        Boxed::new(frame_buffer_ptr)
    }

    pub fn create_blas(
        &self,
        desc: &BottomLevelASDesc,
    ) -> Result<Boxed<BottomLevelAS>, BoxedFromNulError> {
        let mut blas_ptr = std::ptr::null_mut();
        unsafe_member_call!(self, RenderDevice, CreateBLAS, &desc.0, &mut blas_ptr);

        Boxed::new(blas_ptr)
    }

    pub fn create_tlas(
        &self,
        desc: &TopLevelASDesc,
    ) -> Result<Boxed<TopLevelAS>, BoxedFromNulError> {
        let mut tlas_ptr = std::ptr::null_mut();
        unsafe_member_call!(self, RenderDevice, CreateTLAS, &desc.0, &mut tlas_ptr);

        Boxed::new(tlas_ptr)
    }

    pub fn create_sbt(
        &self,
        desc: &ShaderBindingTableDesc,
    ) -> Result<Boxed<ShaderBindingTable>, BoxedFromNulError> {
        let mut sbt_ptr = std::ptr::null_mut();
        unsafe_member_call!(self, RenderDevice, CreateSBT, &desc.0, &mut sbt_ptr);

        Boxed::new(sbt_ptr)
    }

    pub fn create_pipeline_resource_signature(
        &self,
        desc: &PipelineResourceSignatureDesc,
    ) -> Result<Boxed<PipelineResourceSignature>, BoxedFromNulError> {
        let mut prs_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreatePipelineResourceSignature,
            &desc.0,
            &mut prs_ptr
        );

        Boxed::new(prs_ptr)
    }

    pub fn create_device_memory(
        &self,
        create_info: &DeviceMemoryCreateInfo,
    ) -> Result<Boxed<DeviceMemory>, BoxedFromNulError> {
        let mut device_memory_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateDeviceMemory,
            &create_info.0,
            &mut device_memory_ptr
        );

        Boxed::new(device_memory_ptr)
    }

    pub fn create_pipeline_state_cache<T>(
        &self,
        create_info: &PipelineStateCacheCreateInfo<T>,
    ) -> Result<Boxed<PipelineStateCache>, BoxedFromNulError> {
        let mut pso_cache_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreatePipelineStateCache,
            &create_info.0,
            &mut pso_cache_ptr
        );

        Boxed::new(pso_cache_ptr)
    }

    pub fn create_deferred_context(
        &self,
    ) -> Result<Boxed<DeferredDeviceContext>, BoxedFromNulError> {
        let mut deferred_context_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateDeferredContext,
            &mut deferred_context_ptr
        );

        Boxed::new(deferred_context_ptr)
    }

    pub fn get_adapter_info(&self) -> &GraphicsAdapterInfo {
        let info = unsafe_member_call!(self, RenderDevice, GetAdapterInfo);
        unsafe { &*(info as *const GraphicsAdapterInfo) }
    }

    pub fn get_device_info(&self) -> &RenderDeviceInfo {
        let info = unsafe_member_call!(self, RenderDevice, GetDeviceInfo);
        unsafe { &*(info as *const RenderDeviceInfo) }
    }

    pub fn is_texture_format_supported(&self, format: TextureFormat) -> bool {
        let info = unsafe_member_call!(self, RenderDevice, GetTextureFormatInfo, format.into());
        unsafe { (*info).Supported }
    }

    pub fn get_texture_format_info_ext(&self, format: TextureFormat) -> &TextureFormatInfoExt {
        let info = unsafe_member_call!(self, RenderDevice, GetTextureFormatInfoExt, format.into())
            as *const TextureFormatInfoExt;
        unsafe { &*info }
    }

    pub fn get_sparse_texture_format_info(
        &self,
        format: TextureFormat,
        dimension: TextureDimension,
        sample_count: u32,
    ) -> SparseTextureFormatInfo {
        SparseTextureFormatInfo::new(unsafe_member_call!(
            self,
            RenderDevice,
            GetSparseTextureFormatInfo,
            format.into(),
            dimension.into(),
            sample_count
        ))
    }

    pub fn release_stale_resources(&self, force_release: bool) {
        unsafe_member_call!(self, RenderDevice, ReleaseStaleResources, force_release)
    }

    pub fn idle_gpu(&self) {
        unsafe_member_call!(self, RenderDevice, IdleGPU)
    }

    pub fn get_engine_factory(&self) -> &EngineFactory {
        let ptr = unsafe_member_call!(self, RenderDevice, GetEngineFactory);
        unsafe { &*(ptr as *const EngineFactory) }
    }

    //TODO pub fn get_shader_compilation_thread_pool();
}
