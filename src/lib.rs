use static_assertions::const_assert_eq;

pub const API_VERSION: u32 = diligent_sys::DILIGENT_API_VERSION;

const_assert_eq!(API_VERSION, 256011);

macro_rules! unsafe_member_call {
    ($instance:expr, $type_name: ident, $func_name:ident $(, $arg:expr) *) => (
        unsafe {
            (*(*($instance.sys_ptr as *mut paste::paste! {diligent_sys::[<I $type_name>]})).pVtbl)
                .$type_name
                .$func_name
                .unwrap_unchecked()(
                $instance.sys_ptr as _,
                $($arg), *
            )
        }
    );
}

mod device_object;
mod object;

pub mod geometry_primitives;

pub mod graphics_utilities;

pub mod platforms;

pub mod blas;
pub mod buffer;
pub mod buffer_view;
pub mod command_queue;
pub mod data_blob;
pub mod device_context;
pub mod device_memory;
pub mod engine_factory;
pub mod fence;
pub mod frame_buffer;
pub mod graphics_types;
pub mod input_layout;
pub mod pipeline_resource_signature;
pub mod pipeline_state;
pub mod pipeline_state_cache;
pub mod query;
pub mod render_device;
pub mod render_pass;
pub mod resource_mapping;
pub mod sampler;
pub mod shader;
pub mod shader_binding_table;
pub mod shader_resource_binding;
pub mod shader_resource_variable;
pub mod swap_chain;
pub mod texture;
pub mod texture_view;
pub mod tlas;

pub mod memory_allocator;

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
