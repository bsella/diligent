use super::graphics_types::RenderDeviceType;

// https://en.wikipedia.org/wiki/SRGB
pub fn linear_to_gamma(x: f32) -> f32 {
    return if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * f32::powf(x, 1.0 / 2.4) - 0.055
    };
}

pub fn linear_to_srgba(rgba: [f32; 4]) -> [f32; 4] {
    return [
        linear_to_gamma(rgba[0]),
        linear_to_gamma(rgba[1]),
        linear_to_gamma(rgba[2]),
        rgba[3],
    ];
}

pub fn get_render_device_type_string(
    device_type: &RenderDeviceType,
    get_enum_string: bool,
) -> &'static str {
    if get_enum_string {
        match device_type {
            #[cfg(feature = "d3d11")]
            RenderDeviceType::D3D11 => "RENDER_DEVICE_TYPE_D3D11",
            #[cfg(feature = "d3d12")]
            RenderDeviceType::D3D12 => "RENDER_DEVICE_TYPE_D3D12",
            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => "RENDER_DEVICE_TYPE_GL",
            //RenderDeviceType::GLES => "RENDER_DEVICE_TYPE_GLES",
            #[cfg(feature = "vulkan")]
            RenderDeviceType::VULKAN => "RENDER_DEVICE_TYPE_VULKAN",
            #[cfg(feature = "metal")]
            RenderDeviceType::METAL => "RENDER_DEVICE_TYPE_METAL",
            #[cfg(feature = "webgpu")]
            RenderDeviceType::WEBGPU => "RENDER_DEVICE_TYPE_WEBGPU",
        }
    } else {
        match device_type {
            #[cfg(feature = "d3d11")]
            RenderDeviceType::D3D11 => "Direct3D11",
            #[cfg(feature = "d3d12")]
            RenderDeviceType::D3D12 => "Direct3D12",
            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => "OpenGL",
            //RenderDeviceType::GLES => "OpenGLES",
            #[cfg(feature = "vulkan")]
            RenderDeviceType::VULKAN => "Vulkan",
            #[cfg(feature = "metal")]
            RenderDeviceType::METAL => "Metal",
            #[cfg(feature = "webgpu")]
            RenderDeviceType::WEBGPU => "WebGPU",
        }
    }
}
