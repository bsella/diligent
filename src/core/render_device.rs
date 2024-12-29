use crate::bindings;

use std::option::Option;

use crate::buffer::Buffer;
use crate::data_blob::DataBlob;
use crate::sampler::Sampler;
use crate::shader::Shader;
use crate::texture::Texture;

use super::fence::Fence;
use super::object::{AsObject, Object};
use super::pipeline_state::PipelineState;
use super::resource_mapping::ResourceMapping;

pub struct RenderDevice {
    m_render_device: *mut bindings::IRenderDevice,
    m_virtual_functions: *mut bindings::IRenderDeviceVtbl,

    m_object: Object,
}

impl AsObject for RenderDevice {
    fn as_object(&self) -> &Object {
        &self.m_object
    }
}

impl RenderDevice {
    pub(crate) fn new(render_device_ptr: *mut bindings::IRenderDevice) -> Self {
        RenderDevice {
            m_render_device: render_device_ptr,
            m_virtual_functions: unsafe { (*render_device_ptr).pVtbl },
            m_object: Object::new(render_device_ptr as *mut bindings::IObject),
        }
    }

    fn create_buffer(
        &mut self,
        buffer_desc: &bindings::BufferDesc,
        buffer_data: Option<&bindings::BufferData>,
    ) -> Option<Buffer> {
        let mut buffer_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateBuffer
                .unwrap_unchecked()(
                self.m_render_device,
                buffer_desc,
                match buffer_data {
                    Some(data) => std::ptr::addr_of!(data) as *const bindings::BufferData,
                    None => std::ptr::null(),
                },
                std::ptr::addr_of_mut!(buffer_ptr),
            )
        }
        if buffer_ptr.is_null() {
            None
        } else {
            Some(Buffer::new(buffer_ptr, buffer_desc))
        }
    }

    fn create_shader(
        &mut self,
        shader_ci: &bindings::ShaderCreateInfo,
    ) -> Result<Shader, DataBlob> {
        let mut shader_ptr: *mut bindings::IShader = std::ptr::null_mut();
        let mut data_blob_ptr: *mut bindings::IDataBlob = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateShader
                .unwrap_unchecked()(
                self.m_render_device,
                std::ptr::addr_of!(shader_ci) as *const bindings::ShaderCreateInfo,
                std::ptr::addr_of_mut!(shader_ptr),
                std::ptr::addr_of_mut!(data_blob_ptr),
            );
        }

        let data_blob = DataBlob::new(data_blob_ptr);

        if shader_ptr.is_null() {
            Err(data_blob)
        } else {
            Ok(Shader::new(shader_ptr))
        }
    }

    fn create_texture(
        &mut self,
        texture_desc: &bindings::TextureDesc,
        texture_data: Option<&bindings::TextureData>,
    ) -> Option<Texture> {
        let mut texture_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateTexture
                .unwrap_unchecked()(
                self.m_render_device,
                texture_desc,
                match texture_data {
                    Some(data) => std::ptr::addr_of!(data) as *const bindings::TextureData,
                    None => std::ptr::null(),
                },
                std::ptr::addr_of_mut!(texture_ptr),
            )
        };

        if texture_ptr.is_null() {
            None
        } else {
            Some(Texture::new(texture_ptr, texture_desc))
        }
    }

    fn create_sampler(&mut self, sampler_desc: &bindings::SamplerDesc) -> Option<Sampler> {
        let mut sampler_ptr: *mut bindings::ISampler = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateSampler
                .unwrap_unchecked()(
                self.m_render_device,
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

    fn create_resource_mapping(
        &mut self,
        resource_mapping_ci: &bindings::ResourceMappingCreateInfo,
    ) -> Option<ResourceMapping> {
        let mut resource_mapping_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateResourceMapping
                .unwrap_unchecked()(
                self.m_render_device,
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

    fn create_graphics_pipeline_state(
        &mut self,
        pipeline_ci: &bindings::GraphicsPipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateGraphicsPipelineState
                .unwrap_unchecked()(
                self.m_render_device,
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

    fn create_compute_pipeline_state(
        &mut self,
        pipeline_ci: &bindings::ComputePipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateComputePipelineState
                .unwrap_unchecked()(
                self.m_render_device,
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

    fn create_ray_tracing_pipeline_state(
        &mut self,
        pipeline_ci: &bindings::RayTracingPipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateRayTracingPipelineState
                .unwrap_unchecked()(
                self.m_render_device,
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

    fn create_tile_pipeline_state(
        &mut self,
        pipeline_ci: &bindings::TilePipelineStateCreateInfo,
    ) -> Option<PipelineState> {
        let mut pipeline_state_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateTilePipelineState
                .unwrap_unchecked()(
                self.m_render_device,
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

    fn create_fence(&mut self, fence_desc: &bindings::FenceDesc) -> Option<Fence> {
        let mut fence_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .CreateFence
                .unwrap_unchecked()(
                self.m_render_device,
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

    //fn create_query();
    //fn create_render_pass();
    //fn create_framebuffer();
    //fn create_blas();
    //fn create_tlas();
    //fn create_sbt();
    //fn create_pipeline_resource_signature();
    //fn create_device_memory();

    fn get_adapter_info(&self) -> &bindings::GraphicsAdapterInfo {
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .GetAdapterInfo
                .unwrap_unchecked()(self.m_render_device)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_device_info(&self) -> &bindings::RenderDeviceInfo {
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .GetDeviceInfo
                .unwrap_unchecked()(self.m_render_device)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_texture_format_info(
        &self,
        format: bindings::TEXTURE_FORMAT,
    ) -> &bindings::TextureFormatInfo {
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .GetTextureFormatInfo
                .unwrap_unchecked()(self.m_render_device, format)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_texture_format_info_ext(
        &self,
        format: bindings::TEXTURE_FORMAT,
    ) -> &bindings::TextureFormatInfoExt {
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .GetTextureFormatInfoExt
                .unwrap_unchecked()(self.m_render_device, format)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_sparse_texture_format_info(
        &self,
        format: bindings::TEXTURE_FORMAT,
        dimension: bindings::RESOURCE_DIMENSION,
        sample_count: u32,
    ) -> bindings::SparseTextureFormatInfo {
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .GetSparseTextureFormatInfo
                .unwrap_unchecked()(
                self.m_render_device, format, dimension, sample_count
            )
        }
    }

    fn release_stale_resources(&mut self, force_release: bool) {
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .ReleaseStaleResources
                .unwrap_unchecked()(self.m_render_device, force_release)
        }
    }

    fn idle_gpu(&mut self) {
        unsafe {
            (*self.m_virtual_functions)
                .RenderDevice
                .IdleGPU
                .unwrap_unchecked()(self.m_render_device)
        }
    }

    //fn get_engine_factory();
    //fn get_shader_compilation_thread_pool();
}
