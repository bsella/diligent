pub const API_VERSION: u32 = diligent_sys::DILIGENT_API_VERSION;

mod device_object;
mod object;

pub mod accessories;

pub mod geometry_primitives;

pub mod graphics_utilities;

pub mod platforms;

pub mod buffer;
pub mod buffer_view;
pub mod data_blob;
pub mod device_context;
pub mod engine_factory;
pub mod fence;
pub mod frame_buffer;
pub mod graphics_types;
pub mod input_layout;
pub mod pipeline_resource_signature;
pub mod pipeline_state;
pub mod render_device;
pub mod render_pass;
pub mod resource_mapping;
pub mod sampler;
pub mod shader;
pub mod shader_resource_binding;
pub mod shader_resource_variable;
pub mod swap_chain;
pub mod texture;
pub mod texture_view;

#[cfg(feature = "vulkan")]
pub mod vk;

#[cfg(feature = "opengl")]
pub mod gl;

#[cfg(feature = "d3d11")]
pub mod d3d11;

#[cfg(feature = "d3d12")]
pub mod d3d12;
