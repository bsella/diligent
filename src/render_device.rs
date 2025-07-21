use std::{ffi::CString, os::raw::c_void, str::FromStr};

use static_assertions::const_assert;

use crate::{
    blas::{BottomLevelAS, BottomLevelASDesc, BottomLevelASDescWrapper},
    buffer::{Buffer, BufferDesc},
    data_blob::DataBlob,
    device_context::{DeferredDeviceContext, DeviceContext},
    device_memory::{DeviceMemory, DeviceMemoryCreateInfo},
    fence::{Fence, FenceDesc},
    frame_buffer::{Framebuffer, FramebufferDesc},
    graphics_types::{
        DeviceFeatures, GraphicsAdapterInfo, RenderDeviceType, TextureFormat, Version,
    },
    object::Object,
    pipeline_resource_signature::{
        PipelineResourceSignature, PipelineResourceSignatureDesc,
        PipelineResourceSignatureDescWrapper,
    },
    pipeline_state::{
        GraphicsPipelineStateCreateInfo, GraphicsPipelineStateCreateInfoWrapper, PipelineState,
        RayTracingPipelineStateCreateInfo, RayTracingPipelineStateCreateInfoWrapper,
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
    texture::{Texture, TextureDesc, TextureSubResource},
    tlas::{TopLevelAS, TopLevelASDesc},
};

pub struct RenderDeviceInfo {
    device_type: RenderDeviceType,
    api_version: Version,

    features: DeviceFeatures,
    // TODO
    //NDCAttribs NDC DEFAULT_INITIALIZER({});
    //RenderDeviceShaderVersionInfo MaxShaderVersion DEFAULT_INITIALIZER({});
}

impl RenderDeviceInfo {
    pub fn device_type(&self) -> &RenderDeviceType {
        &self.device_type
    }

    pub fn api_version(&self) -> &Version {
        &self.api_version
    }

    pub fn features(&self) -> &DeviceFeatures {
        &self.features
    }
}

pub struct RenderDevice {
    pub(crate) sys_ptr: *mut diligent_sys::IRenderDevice,
    virtual_functions: *mut diligent_sys::IRenderDeviceVtbl,

    object: Object,
}

impl AsRef<Object> for RenderDevice {
    fn as_ref(&self) -> &Object {
        &self.object
    }
}

impl RenderDevice {
    pub(crate) fn new(render_device_ptr: *mut diligent_sys::IRenderDevice) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IRenderDevice>()
        );

        RenderDevice {
            sys_ptr: render_device_ptr,
            virtual_functions: unsafe { (*render_device_ptr).pVtbl },
            object: Object::new(render_device_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn create_buffer(&self, buffer_desc: &BufferDesc) -> Result<Buffer, ()> {
        let mut buffer_ptr = std::ptr::null_mut();

        let buffer_desc = buffer_desc.into();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateBuffer
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::addr_of!(buffer_desc),
                std::ptr::null(),
                std::ptr::addr_of_mut!(buffer_ptr),
            )
        }

        if buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Buffer::new(buffer_ptr))
        }
    }

    pub fn create_buffer_with_data<T: ?Sized>(
        &self,
        buffer_desc: &BufferDesc,
        buffer_data: &T,
        device_context: Option<&DeviceContext>,
    ) -> Result<Buffer, ()> {
        let mut buffer_ptr = std::ptr::null_mut();

        let buffer_data = diligent_sys::BufferData {
            pData: std::ptr::from_ref(buffer_data) as *const c_void,
            DataSize: std::mem::size_of_val(buffer_data) as u64,
            pContext: device_context.map_or(std::ptr::null_mut(), |context| context.sys_ptr),
        };

        let buffer_desc = buffer_desc.into();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateBuffer
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&buffer_desc),
                std::ptr::from_ref(&buffer_data),
                std::ptr::addr_of_mut!(buffer_ptr),
            )
        }

        if buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Buffer::new(buffer_ptr))
        }
    }

    pub fn create_shader(&self, shader_ci: &ShaderCreateInfo) -> Result<Shader, Option<DataBlob>> {
        let mut shader_ptr = std::ptr::null_mut();
        let mut data_blob_ptr = std::ptr::null_mut();

        let shader_ci_wrapper = ShaderCreateInfoWrapper::from(shader_ci);

        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateShader
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&shader_ci_wrapper),
                std::ptr::addr_of_mut!(shader_ptr),
                std::ptr::addr_of_mut!(data_blob_ptr),
            );
        }

        if shader_ptr.is_null() {
            if data_blob_ptr.is_null() {
                Err(None)
            } else {
                Err(Some(DataBlob::new(data_blob_ptr)))
            }
        } else {
            Ok(Shader::new(shader_ptr))
        }
    }

    pub fn create_texture(
        &self,
        texture_desc: &TextureDesc,
        subresources: &[&TextureSubResource],
        device_context: Option<&DeviceContext>,
    ) -> Result<Texture, ()> {
        let mut texture_ptr = std::ptr::null_mut();
        let texture_desc = texture_desc.into();

        let mut subresources: Vec<_> = subresources.iter().map(|&subres| subres.into()).collect();

        let texture_data = diligent_sys::TextureData {
            NumSubresources: subresources.len() as u32,
            pSubResources: subresources.as_mut_ptr(),
            pContext: device_context.map_or(std::ptr::null_mut(), |c| c.sys_ptr),
        };

        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateTexture
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::addr_of!(texture_desc),
                if device_context.is_none() && subresources.is_empty() {
                    std::ptr::null()
                } else {
                    std::ptr::addr_of!(texture_data)
                },
                std::ptr::addr_of_mut!(texture_ptr),
            )
        };

        if texture_ptr.is_null() {
            Err(())
        } else {
            Ok(Texture::new(texture_ptr))
        }
    }

    pub fn create_sampler(&self, sampler_desc: &SamplerDesc) -> Result<Sampler, ()> {
        let sampler_desc = sampler_desc.into();

        let mut sampler_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateSampler
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::addr_of!(sampler_desc),
                std::ptr::addr_of_mut!(sampler_ptr),
            );
        }

        if sampler_ptr.is_null() {
            Err(())
        } else {
            Ok(Sampler::new(sampler_ptr))
        }
    }

    pub fn create_resource_mapping(
        &self,
        resource_mapping_ci: &diligent_sys::ResourceMappingCreateInfo,
    ) -> Result<ResourceMapping, ()> {
        let mut resource_mapping_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateResourceMapping
                .unwrap_unchecked()(
                self.sys_ptr,
                resource_mapping_ci,
                std::ptr::addr_of_mut!(resource_mapping_ptr),
            );
        }

        if resource_mapping_ptr.is_null() {
            Err(())
        } else {
            Ok(ResourceMapping::new(resource_mapping_ptr))
        }
    }

    pub fn create_graphics_pipeline_state(
        &self,
        pipeline_ci: &GraphicsPipelineStateCreateInfo,
    ) -> Result<PipelineState, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        let pipeline_ci_wrapper = GraphicsPipelineStateCreateInfoWrapper::from(pipeline_ci);

        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateGraphicsPipelineState
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&pipeline_ci_wrapper),
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }
        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_compute_pipeline_state(
        &self,
        pipeline_ci: &diligent_sys::ComputePipelineStateCreateInfo,
    ) -> Result<PipelineState, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateComputePipelineState
                .unwrap_unchecked()(
                self.sys_ptr,
                pipeline_ci,
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }

        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_ray_tracing_pipeline_state(
        &self,
        pipeline_ci: &RayTracingPipelineStateCreateInfo,
    ) -> Result<PipelineState, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();

        let pipeline_ci = RayTracingPipelineStateCreateInfoWrapper::from(pipeline_ci);

        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateRayTracingPipelineState
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&pipeline_ci),
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }
        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_tile_pipeline_state(
        &self,
        pipeline_ci: &diligent_sys::TilePipelineStateCreateInfo,
    ) -> Result<PipelineState, ()> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateTilePipelineState
                .unwrap_unchecked()(
                self.sys_ptr,
                pipeline_ci,
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }
        if pipeline_state_ptr.is_null() {
            Err(())
        } else {
            Ok(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_fence(&self, fence_desc: &FenceDesc) -> Result<Fence, ()> {
        let fence_desc = fence_desc.into();

        let mut fence_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateFence
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&fence_desc),
                std::ptr::addr_of_mut!(fence_ptr),
            );
        }
        if fence_ptr.is_null() {
            Err(())
        } else {
            Ok(Fence::new(fence_ptr))
        }
    }

    fn create_query<QueryDataType: GetSysQueryType + Default>(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Query<QueryDataType>, ()> {
        let name = CString::from_str(name.as_ref()).unwrap();
        let query_desc = diligent_sys::QueryDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.as_ptr(),
            },
            Type: QueryDataType::QUERY_TYPE,
        };

        let mut query_ptr = std::ptr::null_mut();

        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateQuery
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&query_desc),
                std::ptr::addr_of_mut!(query_ptr),
            );
        }

        if query_ptr.is_null() {
            Err(())
        } else {
            Ok(Query::<QueryDataType>::new(query_ptr))
        }
    }

    pub fn create_query_occlusion(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Query<QueryDataOcclusion>, ()> {
        self.create_query(name)
    }

    pub fn create_query_binary_occlusion(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Query<QueryDataBinaryOcclusion>, ()> {
        self.create_query(name)
    }

    pub fn create_query_timestamp(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Query<QueryDataTimestamp>, ()> {
        self.create_query(name)
    }

    pub fn create_query_pipeline_statistics(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Query<QueryDataPipelineStatistics>, ()> {
        self.create_query(name)
    }

    pub fn create_query_duration(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Query<QueryDataDuration>, ()> {
        self.create_query(name)
    }

    pub fn create_render_pass(&self, desc: &RenderPassDesc) -> Result<RenderPass, ()> {
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
                Name: desc.name.as_ptr(),
            },
            AttachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            SubpassCount: subpasses.len() as u32,
            pSubpasses: subpasses.as_ptr(),
            DependencyCount: dependencies.len() as u32,
            pDependencies: dependencies.as_ptr(),
        };

        let mut render_pass_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateRenderPass
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&desc),
                std::ptr::addr_of_mut!(render_pass_ptr),
            );
        }
        if render_pass_ptr.is_null() {
            Err(())
        } else {
            Ok(RenderPass::new(render_pass_ptr))
        }
    }

    pub fn create_framebuffer(&self, desc: &FramebufferDesc) -> Result<Framebuffer, ()> {
        let texture_views = desc
            .attachments
            .iter()
            .map(|view| view.sys_ptr)
            .collect::<Vec<_>>();

        let desc = diligent_sys::FramebufferDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: desc.name.as_ptr(),
            },
            pRenderPass: desc.render_pass.sys_ptr,
            AttachmentCount: texture_views.len() as u32,
            ppAttachments: texture_views.as_ptr(),
            Width: desc.width,
            Height: desc.height,
            NumArraySlices: desc.num_array_slices,
        };

        let mut frame_buffer_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateFramebuffer
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&desc),
                std::ptr::addr_of_mut!(frame_buffer_ptr),
            );
        }
        if frame_buffer_ptr.is_null() {
            Err(())
        } else {
            Ok(Framebuffer::new(frame_buffer_ptr))
        }
    }

    pub fn create_blas(&self, desc: &BottomLevelASDesc) -> Result<BottomLevelAS, ()> {
        let desc = BottomLevelASDescWrapper::from(desc);
        let desc = *desc;
        let mut blas_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateBLAS
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&desc),
                std::ptr::addr_of_mut!(blas_ptr),
            );
        }
        if blas_ptr.is_null() {
            Err(())
        } else {
            Ok(BottomLevelAS::new(blas_ptr))
        }
    }

    pub fn create_tlas(&self, desc: &TopLevelASDesc) -> Result<TopLevelAS, ()> {
        let desc = desc.into();
        let mut tlas_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateTLAS
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&desc),
                std::ptr::addr_of_mut!(tlas_ptr),
            );
        }
        if tlas_ptr.is_null() {
            Err(())
        } else {
            Ok(TopLevelAS::new(tlas_ptr))
        }
    }

    pub fn create_sbt(&self, desc: &ShaderBindingTableDesc) -> Result<ShaderBindingTable, ()> {
        let desc = desc.into();
        let mut sbt_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateSBT
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&desc),
                std::ptr::addr_of_mut!(sbt_ptr),
            );
        }
        if sbt_ptr.is_null() {
            Err(())
        } else {
            Ok(ShaderBindingTable::new(sbt_ptr))
        }
    }

    pub fn create_pipeline_resource_signature(
        &self,
        desc: &PipelineResourceSignatureDesc,
    ) -> Result<PipelineResourceSignature, ()> {
        let desc = PipelineResourceSignatureDescWrapper::from(desc);

        let mut prs_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreatePipelineResourceSignature
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&desc),
                std::ptr::addr_of_mut!(prs_ptr),
            );
        }
        if prs_ptr.is_null() {
            Err(())
        } else {
            Ok(PipelineResourceSignature::new(prs_ptr))
        }
    }

    pub fn create_device_memory(
        &self,
        create_info: &DeviceMemoryCreateInfo,
    ) -> Result<DeviceMemory, ()> {
        let mut compatible_resources: Vec<_> = create_info
            .compatible_resources
            .iter()
            .map(|device_object| device_object.sys_ptr)
            .collect();

        let create_info = diligent_sys::DeviceMemoryCreateInfo {
            Desc: (&create_info.desc).into(),
            InitialSize: create_info.initial_size,
            NumResources: compatible_resources.len() as u32,
            ppCompatibleResources: compatible_resources.as_mut_ptr(),
        };

        let mut device_memory_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateDeviceMemory
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&create_info),
                std::ptr::addr_of_mut!(device_memory_ptr),
            )
        }
        if device_memory_ptr.is_null() {
            Err(())
        } else {
            Ok(DeviceMemory::new(device_memory_ptr))
        }
    }

    pub fn create_pipeline_state_cache<T>(
        &self,
        create_info: &PipelineStateCacheCreateInfo<T>,
    ) -> Result<PipelineStateCache, ()> {
        let create_info = create_info.into();
        let mut pso_cache_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreatePipelineStateCache
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&create_info),
                std::ptr::addr_of_mut!(pso_cache_ptr),
            )
        }
        if pso_cache_ptr.is_null() {
            Err(())
        } else {
            Ok(PipelineStateCache::new(pso_cache_ptr))
        }
    }

    pub fn create_deferred_context(&self) -> Result<DeferredDeviceContext, ()> {
        let mut deferred_context_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateDeferredContext
                .unwrap_unchecked()(
                self.sys_ptr, std::ptr::addr_of_mut!(deferred_context_ptr)
            )
        }
        if deferred_context_ptr.is_null() {
            Err(())
        } else {
            Ok(DeferredDeviceContext::new(deferred_context_ptr))
        }
    }

    pub fn get_adapter_info(&self) -> GraphicsAdapterInfo {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetAdapterInfo
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
        .into()
    }

    pub fn get_device_info(&self) -> RenderDeviceInfo {
        let render_device_info = unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetDeviceInfo
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        };

        RenderDeviceInfo {
            device_type: match render_device_info.Type {
                #[cfg(feature = "d3d11")]
                diligent_sys::RENDER_DEVICE_TYPE_D3D11 => RenderDeviceType::D3D11,
                #[cfg(feature = "d3d12")]
                diligent_sys::RENDER_DEVICE_TYPE_D3D12 => RenderDeviceType::D3D12,
                #[cfg(feature = "opengl")]
                diligent_sys::RENDER_DEVICE_TYPE_GL => RenderDeviceType::GL,
                //diligent_sys::RENDER_DEVICE_TYPE_GLES => RenderDeviceType::GLES,
                #[cfg(feature = "vulkan")]
                diligent_sys::RENDER_DEVICE_TYPE_VULKAN => RenderDeviceType::VULKAN,
                #[cfg(feature = "metal")]
                diligent_sys::RENDER_DEVICE_TYPE_METAL => RenderDeviceType::METAL,
                #[cfg(feature = "webgpu")]
                diligent_sys::RENDER_DEVICE_TYPE_WEBGPU => RenderDeviceType::WEBGPU,
                _ => panic!(),
            },
            api_version: Version {
                major: render_device_info.APIVersion.Major,
                minor: render_device_info.APIVersion.Minor,
            },
            features: (&render_device_info.Features).into(),
        }
    }

    pub fn get_texture_format_info(
        &self,
        format: TextureFormat,
    ) -> &diligent_sys::TextureFormatInfo {
        // TODO
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetTextureFormatInfo
                .unwrap_unchecked()(self.sys_ptr, format.into())
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_texture_format_info_ext(
        &self,
        format: TextureFormat,
    ) -> &diligent_sys::TextureFormatInfoExt {
        // TODO
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetTextureFormatInfoExt
                .unwrap_unchecked()(self.sys_ptr, format.into())
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_sparse_texture_format_info(
        &self,
        format: TextureFormat,
        dimension: diligent_sys::RESOURCE_DIMENSION,
        sample_count: u32,
    ) -> diligent_sys::SparseTextureFormatInfo {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetSparseTextureFormatInfo
                .unwrap_unchecked()(self.sys_ptr, format.into(), dimension, sample_count)
        }
    }

    pub fn release_stale_resources(&self, force_release: bool) {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .ReleaseStaleResources
                .unwrap_unchecked()(self.sys_ptr, force_release)
        }
    }

    pub fn idle_gpu(&self) {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .IdleGPU
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    //TODO pub fn get_engine_factory(&self) -> &EngineFactory {}
    //TODO pub fn get_shader_compilation_thread_pool();
}
