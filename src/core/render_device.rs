use crate::bindings;

use std::option::Option;

use super::buffer::{Buffer, BufferDesc};
use super::data_blob::DataBlob;
use super::graphics_types::RenderDeviceType;
use super::sampler::Sampler;
use super::shader::{Shader, ShaderCreateInfo};
use super::texture::{Texture, TextureDesc};

use super::fence::Fence;
use super::object::{AsObject, Object};
use super::pipeline_state::{GraphicsPipelineStateCreateInfo, PipelineState};
use super::resource_mapping::ResourceMapping;

pub struct RenderDeviceInfo {
    device_type: RenderDeviceType,
    //TODO
    //api_version: Version,
    //DeviceFeatures Features;
    //NDCAttribs NDC DEFAULT_INITIALIZER({});
    //RenderDeviceShaderVersionInfo MaxShaderVersion DEFAULT_INITIALIZER({});
}

impl RenderDeviceInfo {
    pub fn device_type(&self) -> &RenderDeviceType {
        &self.device_type
    }
}

pub struct RenderDevice {
    pub(crate) render_device: *mut bindings::IRenderDevice,
    virtual_functions: *mut bindings::IRenderDeviceVtbl,

    object: Object,
}

impl AsObject for RenderDevice {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl RenderDevice {
    pub(crate) fn new(render_device_ptr: *mut bindings::IRenderDevice) -> Self {
        RenderDevice {
            render_device: render_device_ptr,
            virtual_functions: unsafe { (*render_device_ptr).pVtbl },
            object: Object::new(render_device_ptr as *mut bindings::IObject),
        }
    }

    pub fn create_buffer(
        &self,
        buffer_desc: BufferDesc,
        buffer_data: Option<&bindings::BufferData>,
    ) -> Option<Buffer> {
        let mut buffer_ptr = std::ptr::null_mut();

        let buffer_desc = buffer_desc.into();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateBuffer
                .unwrap_unchecked()(
                self.render_device,
                std::ptr::addr_of!(buffer_desc),
                match buffer_data {
                    Some(&data) => std::ptr::addr_of!(data),
                    None => std::ptr::null(),
                },
                std::ptr::addr_of_mut!(buffer_ptr),
            )
        }

        if buffer_ptr.is_null() {
            None
        } else {
            Some(Buffer::new(buffer_ptr))
        }
    }

    pub fn create_shader(&self, shader_ci: ShaderCreateInfo) -> Result<Shader, DataBlob> {
        let mut shader_ptr: *mut bindings::IShader = std::ptr::null_mut();
        let mut data_blob_ptr: *mut bindings::IDataBlob = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateShader
                .unwrap_unchecked()(
                self.render_device,
                std::ptr::from_ref(&shader_ci.into()),
                std::ptr::addr_of_mut!(shader_ptr),
                std::ptr::addr_of_mut!(data_blob_ptr),
            );
        }

        if shader_ptr.is_null() {
            Err(DataBlob::new(data_blob_ptr))
        } else {
            Ok(Shader::new(shader_ptr))
        }
    }

    pub fn create_texture(
        &self,
        texture_desc: TextureDesc,
        texture_data: Option<&bindings::TextureData>,
    ) -> Option<Texture> {
        let mut texture_ptr = std::ptr::null_mut();
        let texture_desc = texture_desc.into();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateTexture
                .unwrap_unchecked()(
                self.render_device,
                std::ptr::addr_of!(texture_desc),
                match texture_data {
                    Some(&data) => std::ptr::addr_of!(data),
                    None => std::ptr::null(),
                },
                std::ptr::addr_of_mut!(texture_ptr),
            )
        };

        if texture_ptr.is_null() {
            None
        } else {
            Some(Texture::new(texture_ptr))
        }
    }

    pub fn create_sampler(&self, sampler_desc: &bindings::SamplerDesc) -> Option<Sampler> {
        let mut sampler_ptr: *mut bindings::ISampler = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateSampler
                .unwrap_unchecked()(
                self.render_device,
                std::ptr::addr_of!(sampler_desc) as *const bindings::SamplerDesc,
                std::ptr::addr_of_mut!(sampler_ptr),
            );
        }

        if sampler_ptr.is_null() {
            None
        } else {
            Some(Sampler::new(sampler_ptr))
        }
    }

    pub fn create_resource_mapping(
        &self,
        resource_mapping_ci: &bindings::ResourceMappingCreateInfo,
    ) -> Option<ResourceMapping> {
        let mut resource_mapping_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateResourceMapping
                .unwrap_unchecked()(
                self.render_device,
                resource_mapping_ci,
                std::ptr::addr_of_mut!(resource_mapping_ptr),
            );
        }

        if resource_mapping_ptr.is_null() {
            None
        } else {
            Some(ResourceMapping::new(resource_mapping_ptr))
        }
    }

    pub fn create_graphics_pipeline_state(
        &self,
        pipeline_ci: GraphicsPipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateGraphicsPipelineState
                .unwrap_unchecked()(
                self.render_device,
                std::ptr::from_ref(&pipeline_ci.into()),
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }
        if pipeline_state_ptr.is_null() {
            None
        } else {
            Some(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_compute_pipeline_state(
        &self,
        pipeline_ci: &bindings::ComputePipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateComputePipelineState
                .unwrap_unchecked()(
                self.render_device,
                pipeline_ci,
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }

        if pipeline_state_ptr.is_null() {
            None
        } else {
            Some(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_ray_tracing_pipeline_state(
        &self,
        pipeline_ci: &bindings::RayTracingPipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateRayTracingPipelineState
                .unwrap_unchecked()(
                self.render_device,
                pipeline_ci,
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }
        if pipeline_state_ptr.is_null() {
            None
        } else {
            Some(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_tile_pipeline_state(
        &self,
        pipeline_ci: &bindings::TilePipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateTilePipelineState
                .unwrap_unchecked()(
                self.render_device,
                pipeline_ci,
                std::ptr::addr_of_mut!(pipeline_state_ptr),
            );
        }
        if pipeline_state_ptr.is_null() {
            None
        } else {
            Some(PipelineState::new(pipeline_state_ptr))
        }
    }

    pub fn create_fence(&self, fence_desc: &bindings::FenceDesc) -> Option<Fence> {
        let mut fence_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .CreateFence
                .unwrap_unchecked()(
                self.render_device,
                fence_desc,
                std::ptr::addr_of_mut!(fence_ptr),
            );
        }
        if fence_ptr.is_null() {
            None
        } else {
            Some(Fence::new(fence_ptr))
        }
    }

    // pub fn create_query();
    // pub fn create_render_pass();
    // pub fn create_framebuffer();
    // pub fn create_blas();
    // pub fn create_tlas();
    // pub fn create_sbt();
    // pub fn create_pipeline_resource_signature();
    // pub fn create_device_memory();

    pub fn get_adapter_info(&self) -> &bindings::GraphicsAdapterInfo {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetAdapterInfo
                .unwrap_unchecked()(self.render_device)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_device_info(&self) -> RenderDeviceInfo {
        let render_device_info = unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetDeviceInfo
                .unwrap_unchecked()(self.render_device)
            .as_ref()
            .unwrap_unchecked()
        };

        RenderDeviceInfo {
            device_type: match render_device_info.Type {
                bindings::RENDER_DEVICE_TYPE_D3D11 => RenderDeviceType::D3D11,
                bindings::RENDER_DEVICE_TYPE_D3D12 => RenderDeviceType::D3D12,
                bindings::RENDER_DEVICE_TYPE_GL => RenderDeviceType::GL,
                bindings::RENDER_DEVICE_TYPE_GLES => RenderDeviceType::GLES,
                bindings::RENDER_DEVICE_TYPE_VULKAN => RenderDeviceType::VULKAN,
                bindings::RENDER_DEVICE_TYPE_METAL => RenderDeviceType::METAL,
                bindings::RENDER_DEVICE_TYPE_WEBGPU => RenderDeviceType::WEBGPU,
                _ => panic!(),
            },
        }
    }

    pub fn get_texture_format_info(
        &self,
        format: bindings::TEXTURE_FORMAT,
    ) -> &bindings::TextureFormatInfo {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetTextureFormatInfo
                .unwrap_unchecked()(self.render_device, format)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_texture_format_info_ext(
        &self,
        format: bindings::TEXTURE_FORMAT,
    ) -> &bindings::TextureFormatInfoExt {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetTextureFormatInfoExt
                .unwrap_unchecked()(self.render_device, format)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_sparse_texture_format_info(
        &self,
        format: bindings::TEXTURE_FORMAT,
        dimension: bindings::RESOURCE_DIMENSION,
        sample_count: u32,
    ) -> bindings::SparseTextureFormatInfo {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .GetSparseTextureFormatInfo
                .unwrap_unchecked()(self.render_device, format, dimension, sample_count)
        }
    }

    pub fn release_stale_resources(&self, force_release: bool) {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .ReleaseStaleResources
                .unwrap_unchecked()(self.render_device, force_release)
        }
    }

    pub fn idle_gpu(&self) {
        unsafe {
            (*self.virtual_functions)
                .RenderDevice
                .IdleGPU
                .unwrap_unchecked()(self.render_device)
        }
    }

    //pub fn get_engine_factory();
    //pub fn get_shader_compilation_thread_pool();
}
