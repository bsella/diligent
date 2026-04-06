pub mod sample_base;
pub mod textured_cube;
pub mod window;

pub trait GetDeviceType {
    fn is_gl(&self) -> bool;
    fn is_vulkan(&self) -> bool;
    fn is_d3d11(&self) -> bool;
    fn is_d3d12(&self) -> bool;
    fn is_metal(&self) -> bool;
    fn is_webgpu(&self) -> bool;
}

impl GetDeviceType for diligent::RenderDeviceType {
    fn is_gl(&self) -> bool {
        #[cfg(not(feature = "opengl"))]
        {
            false
        }
        #[cfg(feature = "opengl")]
        {
            matches!(self, Self::GL)
        }
    }
    fn is_vulkan(&self) -> bool {
        #[cfg(not(feature = "vulkan"))]
        {
            false
        }
        #[cfg(feature = "vulkan")]
        {
            matches!(self, Self::VULKAN)
        }
    }
    fn is_d3d11(&self) -> bool {
        #[cfg(not(feature = "d3d11"))]
        {
            false
        }
        #[cfg(feature = "d3d11")]
        {
            matches!(self, Self::D3D11)
        }
    }
    fn is_d3d12(&self) -> bool {
        #[cfg(not(feature = "d3d12"))]
        {
            false
        }
        #[cfg(feature = "d3d12")]
        {
            matches!(self, Self::D3D12)
        }
    }
    fn is_metal(&self) -> bool {
        #[cfg(not(feature = "metal"))]
        {
            false
        }
        #[cfg(feature = "metal")]
        {
            matches!(self, Self::METAL)
        }
    }
    fn is_webgpu(&self) -> bool {
        #[cfg(not(feature = "webgpu"))]
        {
            false
        }
        #[cfg(feature = "webgpu")]
        {
            matches!(self, Self::WEBGPU)
        }
    }
}
