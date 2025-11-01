use std::ops::Deref;
use std::ops::DerefMut;

use static_assertions::const_assert_eq;

pub const API_VERSION: u32 = diligent_sys::DILIGENT_API_VERSION;

const_assert_eq!(API_VERSION, 256012);

macro_rules! unsafe_member_call {
    ($instance:expr, $type_name: ident, $func_name:ident $(, $arg:expr) *) => (
        unsafe {
            (*$instance.0.pVtbl)
                .$type_name
                .$func_name
                .unwrap_unchecked()(
                std::ptr::addr_of!($instance.0) as _,
                $($arg), *
            )
        }
    );
}

pub mod geometry_primitives;

pub mod graphics_utilities;

pub mod platforms;

mod device_object;
mod object;

mod blas;
mod buffer;
mod buffer_view;
mod command_queue;
mod data_blob;
mod device_context;
mod device_memory;
mod engine_factory;
mod fence;
mod frame_buffer;
mod graphics_types;
mod input_layout;
mod memory_allocator;
mod pipeline_resource_signature;
mod pipeline_state;
mod pipeline_state_cache;
mod query;
mod render_device;
mod render_pass;
mod resource_mapping;
mod sampler;
mod shader;
mod shader_binding_table;
mod shader_resource_binding;
mod shader_resource_variable;
mod swap_chain;
mod texture;
mod texture_view;
mod tlas;

use crate::object::Object;

pub use self::blas::*;
pub use self::buffer::*;
pub use self::buffer_view::*;
pub use self::command_queue::*;
pub use self::data_blob::*;
pub use self::device_context::*;
pub use self::device_memory::*;
pub use self::engine_factory::*;
pub use self::fence::*;
pub use self::frame_buffer::*;
pub use self::graphics_types::*;
pub use self::input_layout::*;
pub use self::memory_allocator::*;
pub use self::pipeline_resource_signature::*;
pub use self::pipeline_state::*;
pub use self::pipeline_state_cache::*;
pub use self::query::*;
pub use self::render_device::*;
pub use self::render_pass::*;
pub use self::resource_mapping::*;
pub use self::sampler::*;
pub use self::shader::*;
pub use self::shader_binding_table::*;
pub use self::shader_resource_binding::*;
pub use self::shader_resource_variable::*;
pub use self::swap_chain::*;
pub use self::texture::*;
pub use self::texture_view::*;
pub use self::tlas::*;

#[cfg(feature = "vulkan")]
pub mod vk;

#[cfg(feature = "opengl")]
pub mod gl;

#[cfg(feature = "d3d11")]
pub mod d3d11;

#[cfg(feature = "d3d12")]
pub mod d3d12;

#[repr(transparent)]
pub struct APIInfo(diligent_sys::APIInfo);

impl APIInfo {
    pub fn struct_size(&self) -> usize {
        self.0.StructSize
    }
    pub fn api_version(&self) -> u32 {
        self.0.APIVersion as u32
    }
    pub fn render_target_blend_desc_size(&self) -> usize {
        self.0.RenderTargetBlendDescSize
    }
    pub fn blend_state_desc_size(&self) -> usize {
        self.0.BlendStateDescSize
    }
    pub fn buffer_desc_size(&self) -> usize {
        self.0.BufferDescSize
    }
    pub fn buffer_data_size(&self) -> usize {
        self.0.BufferDataSize
    }
    pub fn buffer_format_size(&self) -> usize {
        self.0.BufferFormatSize
    }
    pub fn buffer_view_desc_size(&self) -> usize {
        self.0.BufferViewDescSize
    }
    pub fn stencil_op_desc_size(&self) -> usize {
        self.0.StencilOpDescSize
    }
    pub fn depth_stencil_state_desc_size(&self) -> usize {
        self.0.DepthStencilStateDescSize
    }
    pub fn sampler_properties_size(&self) -> usize {
        self.0.SamplerPropertiesSize
    }
    pub fn texture_properties_size(&self) -> usize {
        self.0.TexturePropertiesSize
    }
    pub fn render_device_info_size(&self) -> usize {
        self.0.RenderDeviceInfoSize
    }
    pub fn draw_attribs_size(&self) -> usize {
        self.0.DrawAttribsSize
    }
    pub fn dispatch_compute_attribs_size(&self) -> usize {
        self.0.DispatchComputeAttribsSize
    }
    pub fn viewport_size(&self) -> usize {
        self.0.ViewportSize
    }
    pub fn rect_size(&self) -> usize {
        self.0.RectSize
    }
    pub fn copy_texture_attribs_size(&self) -> usize {
        self.0.CopyTextureAttribsSize
    }
    pub fn device_object_attribs_size(&self) -> usize {
        self.0.DeviceObjectAttribsSize
    }
    pub fn graphics_adapter_info_size(&self) -> usize {
        self.0.GraphicsAdapterInfoSize
    }
    pub fn display_mode_attribs_size(&self) -> usize {
        self.0.DisplayModeAttribsSize
    }
    pub fn swap_chain_desc_size(&self) -> usize {
        self.0.SwapChainDescSize
    }
    pub fn full_screen_mode_desc_size(&self) -> usize {
        self.0.FullScreenModeDescSize
    }
    pub fn open_xr_attribs_size(&self) -> usize {
        self.0.OpenXRAttribsSize
    }
    pub fn engine_create_info_size(&self) -> usize {
        self.0.EngineCreateInfoSize
    }
    pub fn engine_gl_create_info_size(&self) -> usize {
        self.0.EngineGLCreateInfoSize
    }
    pub fn engine_d3d11_create_info_size(&self) -> usize {
        self.0.EngineD3D11CreateInfoSize
    }
    pub fn engine_d3d12_create_info_size(&self) -> usize {
        self.0.EngineD3D12CreateInfoSize
    }
    pub fn engine_vk_create_info_size(&self) -> usize {
        self.0.EngineVkCreateInfoSize
    }
    pub fn engine_mtl_create_info_size(&self) -> usize {
        self.0.EngineMtlCreateInfoSize
    }
    pub fn box_size(&self) -> usize {
        self.0.BoxSize
    }
    pub fn texture_format_attribs_size(&self) -> usize {
        self.0.TextureFormatAttribsSize
    }
    pub fn texture_format_info_size(&self) -> usize {
        self.0.TextureFormatInfoSize
    }
    pub fn texture_format_info_ext_size(&self) -> usize {
        self.0.TextureFormatInfoExtSize
    }
    pub fn state_transition_desc_size(&self) -> usize {
        self.0.StateTransitionDescSize
    }
    pub fn layout_element_size(&self) -> usize {
        self.0.LayoutElementSize
    }
    pub fn input_layout_desc_size(&self) -> usize {
        self.0.InputLayoutDescSize
    }
    pub fn sample_desc_size(&self) -> usize {
        self.0.SampleDescSize
    }
    pub fn shader_resource_variable_desc_size(&self) -> usize {
        self.0.ShaderResourceVariableDescSize
    }
    pub fn immutable_sampler_desc_size(&self) -> usize {
        self.0.ImmutableSamplerDescSize
    }
    pub fn pipeline_resource_layout_desc_size(&self) -> usize {
        self.0.PipelineResourceLayoutDescSize
    }
    pub fn pipeline_state_desc_size(&self) -> usize {
        self.0.PipelineStateDescSize
    }
    pub fn graphics_pipeline_desc_size(&self) -> usize {
        self.0.GraphicsPipelineDescSize
    }
    pub fn graphics_pipeline_state_create_info_size(&self) -> usize {
        self.0.GraphicsPipelineStateCreateInfoSize
    }
    pub fn compute_pipeline_state_create_info_size(&self) -> usize {
        self.0.ComputePipelineStateCreateInfoSize
    }
    pub fn ray_tracing_pipeline_desc_size(&self) -> usize {
        self.0.RayTracingPipelineDescSize
    }
    pub fn ray_tracing_pipeline_state_create_info_size(&self) -> usize {
        self.0.RayTracingPipelineStateCreateInfoSize
    }
    pub fn rasterizer_state_desc_size(&self) -> usize {
        self.0.RasterizerStateDescSize
    }
    pub fn resource_mapping_entry_size(&self) -> usize {
        self.0.ResourceMappingEntrySize
    }
    pub fn resource_mapping_create_info_size(&self) -> usize {
        self.0.ResourceMappingCreateInfoSize
    }
    pub fn sampler_desc_size(&self) -> usize {
        self.0.SamplerDescSize
    }
    pub fn shader_desc_size(&self) -> usize {
        self.0.ShaderDescSize
    }
    pub fn shader_macro_size(&self) -> usize {
        self.0.ShaderMacroSize
    }
    pub fn shader_macro_array_size(&self) -> usize {
        self.0.ShaderMacroArraySize
    }
    pub fn shader_create_info_size(&self) -> usize {
        self.0.ShaderCreateInfoSize
    }
    pub fn shader_resource_desc_size(&self) -> usize {
        self.0.ShaderResourceDescSize
    }
    pub fn depth_stencil_clear_value_size(&self) -> usize {
        self.0.DepthStencilClearValueSize
    }
    pub fn optimized_clear_value_size(&self) -> usize {
        self.0.OptimizedClearValueSize
    }
    pub fn texture_desc_size(&self) -> usize {
        self.0.TextureDescSize
    }
    pub fn texture_sub_res_data_size(&self) -> usize {
        self.0.TextureSubResDataSize
    }
    pub fn texture_data_size(&self) -> usize {
        self.0.TextureDataSize
    }
    pub fn mapped_texture_subresource_size(&self) -> usize {
        self.0.MappedTextureSubresourceSize
    }
    pub fn texture_view_desc_size(&self) -> usize {
        self.0.TextureViewDescSize
    }
}

pub struct Boxed<T> {
    ptr: *mut T,
}

impl<T> Boxed<T> {
    pub(crate) fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub fn from_ref(object: &Object) -> Self {
        object.add_ref();
        Self {
            ptr: object.sys_ptr() as *mut T,
        }
    }
}

impl<T> Deref for Boxed<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Boxed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for Boxed<T> {
    fn drop(&mut self) {
        let object_ptr = self.ptr as *mut diligent_sys::IObject;
        unsafe {
            (*(*object_ptr).pVtbl).Object.Release.unwrap_unchecked()(object_ptr);
        }
    }
}
