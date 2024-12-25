mod bindings{
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/diligent_bindings.rs"));
}

mod defaults;

mod object;

mod device_object;

mod buffer_view;

mod buffer;

mod texture_view;

mod texture;

mod sampler;

mod shader;

mod shader_resource_variable;
mod shader_resource_binding;

mod data_blob;

mod resource_mapping;

mod fence;

mod pipeline_state;

mod pipeline_resource_signature;

mod render_device;

mod engine_factory;
mod engine_factory_vk;