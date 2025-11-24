use std::{ffi::CStr, ops::Deref, os::raw::c_void};

use static_assertions::const_assert_eq;

use crate::{
    Boxed, ResourceMappingCreateInfo, SparseTextureFormatInfo, TextureFormatInfoExt,
    TilePipelineStateCreateInfo,
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

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IRenderDeviceMethods>(),
    29 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct RenderDevice(diligent_sys::IRenderDevice);

impl Deref for RenderDevice {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IObject as *const Object) }
    }
}

impl RenderDevice {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IRenderDevice {
        std::ptr::addr_of!(self.0) as _
    }

    pub fn create_buffer(&self, buffer_desc: &BufferDesc) -> Result<Boxed<Buffer>, ()> {
        let mut buffer_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateBuffer,
            std::ptr::from_ref(&buffer_desc.0),
            std::ptr::null(),
            std::ptr::addr_of_mut!(buffer_ptr)
        );

        if buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Buffer>::new(buffer_ptr as _))
        }
    }

    pub fn create_buffer_with_data<T: ?Sized>(
        &self,
        buffer_desc: &BufferDesc,
        buffer_data: &T,
        device_context: Option<&DeviceContext>,
    ) -> Result<Boxed<Buffer>, ()> {
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
            std::ptr::from_ref(&buffer_desc.0),
            std::ptr::from_ref(&buffer_data),
            std::ptr::addr_of_mut!(buffer_ptr)
        );

        if buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Buffer>::new(buffer_ptr as _))
        }
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
            std::ptr::addr_of_mut!(shader_ptr),
            std::ptr::addr_of_mut!(data_blob_ptr)
        );

        if shader_ptr.is_null() {
            if data_blob_ptr.is_null() {
                Err(None)
            } else {
                Err(Some(Boxed::<DataBlob>::new(data_blob_ptr as _)))
            }
        } else {
            Ok(Boxed::<Shader>::new(shader_ptr as _))
        }
    }

    pub fn create_texture(
        &self,
        texture_desc: &TextureDesc,
        subresources: &[&TextureSubResource],
        device_context: Option<&DeviceContext>,
    ) -> Result<Boxed<Texture>, ()> {
        let mut texture_ptr = std::ptr::null_mut();

        let mut subresources: Vec<_> = subresources.iter().map(|&subres| subres.into()).collect();

        let texture_data = diligent_sys::TextureData {
            NumSubresources: subresources.len() as u32,
            pSubResources: subresources.as_mut_ptr(),
            pContext: device_context.map_or(std::ptr::null_mut(), |c| c.sys_ptr() as _),
        };

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateTexture,
            std::ptr::from_ref(&texture_desc.0),
            if device_context.is_none() && subresources.is_empty() {
                std::ptr::null()
            } else {
                std::ptr::addr_of!(texture_data)
            },
            std::ptr::addr_of_mut!(texture_ptr)
        );

        if texture_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Texture>::new(texture_ptr as _))
        }
    }

    pub fn create_sampler(&self, sampler_desc: &SamplerDesc) -> Result<Boxed<Sampler>, ()> {
        let sampler_desc = sampler_desc.into();

        let mut sampler_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateSampler,
            std::ptr::addr_of!(sampler_desc),
            std::ptr::addr_of_mut!(sampler_ptr)
        );

        if sampler_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Sampler>::new(sampler_ptr as _))
        }
    }

    pub fn create_resource_mapping(
        &self,
        resource_mapping_ci: &ResourceMappingCreateInfo,
    ) -> Result<Boxed<ResourceMapping>, ()> {
        let mut resource_mapping_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateResourceMapping,
            std::ptr::from_ref(&resource_mapping_ci.0),
            std::ptr::addr_of_mut!(resource_mapping_ptr)
        );

        if resource_mapping_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<ResourceMapping>::new(resource_mapping_ptr as _))
        }
    }

    pub fn create_graphics_pipeline_state(
        &self,
        pipeline_ci: &GraphicsPipelineStateCreateInfo,
    ) -> Result<Boxed<GraphicsPipelineState>, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateGraphicsPipelineState,
            std::ptr::from_ref(&pipeline_ci.0),
            std::ptr::addr_of_mut!(pipeline_state_ptr)
        );

        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<GraphicsPipelineState>::new(pipeline_state_ptr as _))
        }
    }

    pub fn create_compute_pipeline_state(
        &self,
        pipeline_ci: &ComputePipelineStateCreateInfo,
    ) -> Result<Boxed<ComputePipelineState>, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateComputePipelineState,
            std::ptr::from_ref(&pipeline_ci.0),
            std::ptr::addr_of_mut!(pipeline_state_ptr)
        );

        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<ComputePipelineState>::new(pipeline_state_ptr as _))
        }
    }

    pub fn create_ray_tracing_pipeline_state(
        &self,
        pipeline_ci: &RayTracingPipelineStateCreateInfo,
    ) -> Result<Boxed<RayTracingPipelineState>, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateRayTracingPipelineState,
            std::ptr::from_ref(&pipeline_ci.0),
            std::ptr::addr_of_mut!(pipeline_state_ptr)
        );

        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<RayTracingPipelineState>::new(
                pipeline_state_ptr as _,
            ))
        }
    }

    pub fn create_tile_pipeline_state(
        &self,
        pipeline_ci: &TilePipelineStateCreateInfo,
    ) -> Result<Boxed<TilePipelineState>, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateTilePipelineState,
            std::ptr::from_ref(&pipeline_ci.0),
            std::ptr::addr_of_mut!(pipeline_state_ptr)
        );

        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<TilePipelineState>::new(pipeline_state_ptr as _))
        }
    }

    pub fn create_fence(&self, fence_desc: &FenceDesc) -> Result<Boxed<Fence>, ()> {
        let mut fence_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateFence,
            std::ptr::from_ref(&fence_desc.0),
            std::ptr::addr_of_mut!(fence_ptr)
        );

        if fence_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Fence>::new(fence_ptr as _))
        }
    }

    fn create_query<QueryDataType: GetSysQueryType>(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataType>>, ()> {
        let query_desc = diligent_sys::QueryDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            Type: QueryDataType::QUERY_TYPE,
        };

        let mut query_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDevice,
            CreateQuery,
            std::ptr::from_ref(&query_desc),
            std::ptr::addr_of_mut!(query_ptr)
        );

        if query_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Query<QueryDataType>>::new(query_ptr as _))
        }
    }

    pub fn create_query_occlusion(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataOcclusion>>, ()> {
        self.create_query(name)
    }

    pub fn create_query_binary_occlusion(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataBinaryOcclusion>>, ()> {
        self.create_query(name)
    }

    pub fn create_query_timestamp(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataTimestamp>>, ()> {
        self.create_query(name)
    }

    pub fn create_query_pipeline_statistics(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataPipelineStatistics>>, ()> {
        self.create_query(name)
    }

    pub fn create_query_duration(
        &self,
        name: Option<&CStr>,
    ) -> Result<Boxed<Query<QueryDataDuration>>, ()> {
        self.create_query(name)
    }

    pub fn create_render_pass(&self, desc: &RenderPassDesc) -> Result<Boxed<RenderPass>, ()> {
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
            std::ptr::from_ref(&desc),
            std::ptr::addr_of_mut!(render_pass_ptr)
        );

        if render_pass_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<RenderPass>::new(render_pass_ptr as _))
        }
    }

    pub fn create_framebuffer(&self, desc: &FramebufferDesc) -> Result<Boxed<Framebuffer>, ()> {
        let texture_views = desc
            .attachments
            .iter()
            .map(|view| view.sys_ptr())
            .collect::<Vec<_>>();

        let desc = diligent_sys::FramebufferDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: desc
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            pRenderPass: desc.render_pass.sys_ptr(),
            AttachmentCount: texture_views.len() as u32,
            ppAttachments: texture_views.as_ptr() as _,
            Width: desc.width,
            Height: desc.height,
            NumArraySlices: desc.num_array_slices,
        };

        let mut frame_buffer_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateFramebuffer,
            std::ptr::from_ref(&desc),
            std::ptr::addr_of_mut!(frame_buffer_ptr)
        );

        if frame_buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<Framebuffer>::new(frame_buffer_ptr as _))
        }
    }

    pub fn create_blas(&self, desc: &BottomLevelASDesc) -> Result<Boxed<BottomLevelAS>, ()> {
        let mut blas_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateBLAS,
            std::ptr::from_ref(desc) as _,
            std::ptr::addr_of_mut!(blas_ptr)
        );

        if blas_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<BottomLevelAS>::new(blas_ptr as _))
        }
    }

    pub fn create_tlas(&self, desc: &TopLevelASDesc) -> Result<Boxed<TopLevelAS>, ()> {
        let desc = desc.into();
        let mut tlas_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateTLAS,
            std::ptr::from_ref(&desc),
            std::ptr::addr_of_mut!(tlas_ptr)
        );

        if tlas_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<TopLevelAS>::new(tlas_ptr as _))
        }
    }

    pub fn create_sbt(
        &self,
        desc: &ShaderBindingTableDesc,
    ) -> Result<Boxed<ShaderBindingTable>, ()> {
        let mut sbt_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateSBT,
            std::ptr::from_ref(&desc.0),
            std::ptr::addr_of_mut!(sbt_ptr)
        );

        if sbt_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<ShaderBindingTable>::new(sbt_ptr as _))
        }
    }

    pub fn create_pipeline_resource_signature(
        &self,
        desc: &PipelineResourceSignatureDesc,
    ) -> Result<Boxed<PipelineResourceSignature>, ()> {
        let mut prs_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreatePipelineResourceSignature,
            std::ptr::from_ref(&desc.0),
            std::ptr::addr_of_mut!(prs_ptr)
        );

        if prs_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<PipelineResourceSignature>::new(prs_ptr as _))
        }
    }

    pub fn create_device_memory(
        &self,
        create_info: &DeviceMemoryCreateInfo,
    ) -> Result<Boxed<DeviceMemory>, ()> {
        let mut compatible_resources: Vec<_> = create_info
            .compatible_resources
            .iter()
            .map(|device_object| device_object.sys_ptr())
            .collect();

        let create_info = diligent_sys::DeviceMemoryCreateInfo {
            Desc: (&create_info.desc).into(),
            InitialSize: create_info.initial_size,
            NumResources: compatible_resources.len() as u32,
            ppCompatibleResources: compatible_resources.as_mut_ptr() as _,
        };

        let mut device_memory_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateDeviceMemory,
            std::ptr::from_ref(&create_info),
            std::ptr::addr_of_mut!(device_memory_ptr)
        );

        if device_memory_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<DeviceMemory>::new(device_memory_ptr as _))
        }
    }

    pub fn create_pipeline_state_cache<T>(
        &self,
        create_info: &PipelineStateCacheCreateInfo<T>,
    ) -> Result<Boxed<PipelineStateCache>, ()> {
        let create_info = create_info.into();
        let mut pso_cache_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreatePipelineStateCache,
            std::ptr::from_ref(&create_info),
            std::ptr::addr_of_mut!(pso_cache_ptr)
        );

        if pso_cache_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<PipelineStateCache>::new(pso_cache_ptr as _))
        }
    }

    pub fn create_deferred_context(&self) -> Result<Boxed<DeferredDeviceContext>, ()> {
        let mut deferred_context_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            RenderDevice,
            CreateDeferredContext,
            std::ptr::addr_of_mut!(deferred_context_ptr)
        );

        if deferred_context_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<DeferredDeviceContext>::new(
                deferred_context_ptr as _,
            ))
        }
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
