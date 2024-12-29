mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/diligent_bindings.rs"));
}

mod device_object;
mod object;

pub mod buffer;
pub mod buffer_view;
pub mod data_blob;
pub mod defaults;
pub mod engine_factory;
pub mod fence;
pub mod pipeline_resource_signature;
pub mod pipeline_state;
pub mod render_device;
pub mod resource_mapping;
pub mod sampler;
pub mod shader;
pub mod shader_resource_binding;
pub mod shader_resource_variable;
pub mod texture;
pub mod texture_view;

#[cfg(feature = "VULKAN_SUPPORTED")]
mod engine_factory_vk;

pub mod tools;
