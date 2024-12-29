use crate::bindings;

impl Default for bindings::ShaderResourceDesc {
    fn default() -> Self {
        bindings::ShaderResourceDesc {
            Name: std::ptr::null(),
            Type: bindings::SHADER_RESOURCE_TYPE_UNKNOWN as u8,
            ArraySize: 0,
        }
    }
}
