use static_assertions::const_assert_eq;

pub const API_VERSION: u32 = diligent_sys::DILIGENT_API_VERSION;

const_assert_eq!(API_VERSION, 256009);

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

#[cfg(feature = "vulkan")]
pub mod vk;

#[cfg(feature = "opengl")]
pub mod gl;

#[cfg(feature = "d3d11")]
pub mod d3d11;

#[cfg(feature = "d3d12")]
pub mod d3d12;
