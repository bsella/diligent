use crate::core::{
    buffer::{Buffer, BufferDesc},
    fence::{Fence, FenceDesc},
    graphics_types::ResourceState,
    object::AsObject,
    render_device::RenderDevice,
    texture::{Texture, TextureDesc},
};

pub struct RenderDeviceVk<'a> {
    render_device_ptr: *mut diligent_sys::IRenderDeviceVk,
    virtual_functions: *mut diligent_sys::IRenderDeviceVkVtbl,

    render_device: &'a RenderDevice,
}

impl AsObject for RenderDeviceVk<'_> {
    fn as_object(&self) -> &crate::core::object::Object {
        self.render_device.as_object()
    }
}

impl<'a> From<&'a RenderDevice> for RenderDeviceVk<'a> {
    fn from(value: &'a RenderDevice) -> Self {
        RenderDeviceVk {
            render_device: value,
            render_device_ptr: value.render_device as *mut diligent_sys::IRenderDeviceVk,
            virtual_functions: unsafe {
                (*(value.render_device as *mut diligent_sys::IRenderDeviceVk)).pVtbl
            },
        }
    }
}

impl RenderDeviceVk<'_> {
    pub fn get_vk_device(&self) -> diligent_sys::VkDevice {
        unsafe {
            (*self.virtual_functions)
                .RenderDeviceVk
                .GetVkDevice
                .unwrap_unchecked()(self.render_device_ptr)
        }
    }

    pub fn get_vk_physical_device(&self) -> diligent_sys::VkPhysicalDevice {
        unsafe {
            (*self.virtual_functions)
                .RenderDeviceVk
                .GetVkPhysicalDevice
                .unwrap_unchecked()(self.render_device_ptr)
        }
    }

    pub fn get_vk_instance(&self) -> diligent_sys::VkInstance {
        unsafe {
            (*self.virtual_functions)
                .RenderDeviceVk
                .GetVkInstance
                .unwrap_unchecked()(self.render_device_ptr)
        }
    }

    pub fn get_vk_version(&self) -> u32 {
        unsafe {
            (*self.virtual_functions)
                .RenderDeviceVk
                .GetVkVersion
                .unwrap_unchecked()(self.render_device_ptr)
        }
    }

    pub fn create_texture_from_vulkan_image(
        &self,
        vk_image: diligent_sys::VkImage,
        texture_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Option<Texture> {
        let texture_desc = diligent_sys::TextureDesc::from(texture_desc);

        let mut texture_ptr = std::ptr::null_mut();

        unsafe {
            (*self.virtual_functions)
                .RenderDeviceVk
                .CreateTextureFromVulkanImage
                .unwrap_unchecked()(
                self.render_device_ptr,
                vk_image,
                std::ptr::from_ref(&texture_desc),
                initial_state.bits(),
                std::ptr::addr_of_mut!(texture_ptr),
            )
        };

        if texture_ptr.is_null() {
            None
        } else {
            Some(Texture::new(texture_ptr))
        }
    }

    pub fn create_buffer_from_vulkan_resource(
        &self,
        vk_buffer: diligent_sys::VkBuffer,
        buffer_desc: &BufferDesc,
        initial_state: ResourceState,
    ) -> Option<Buffer> {
        let buffer_desc = diligent_sys::BufferDesc::from(buffer_desc);

        let mut buffer_ptr = std::ptr::null_mut();

        unsafe {
            (*self.virtual_functions)
                .RenderDeviceVk
                .CreateBufferFromVulkanResource
                .unwrap_unchecked()(
                self.render_device_ptr,
                vk_buffer,
                std::ptr::from_ref(&buffer_desc),
                initial_state.bits(),
                std::ptr::addr_of_mut!(buffer_ptr),
            )
        };

        if buffer_ptr.is_null() {
            None
        } else {
            Some(Buffer::new(buffer_ptr))
        }
    }

    //pub fn create_blas_from_vulkan_resource(&self,   vkBLAS: VkAccelerationStructureKHR, blas_desc: &BottomLevelASDesc , RESOURCE_STATE              InitialState) -> Option<BottomLevelAS>{}
    //pub fn create_tlas_from_vulkan_resource(&self,   vkTLAS: VkAccelerationStructureKHR, tlas_desc: &TopLevelASDesc    , RESOURCE_STATE             InitialState)  -> Option<TopLevelAS>{}

    pub fn create_fence_from_vulkan_resource(
        &self,
        vk_timeline_semaphore: diligent_sys::VkSemaphore,
        fence_desc: &FenceDesc,
    ) -> Option<Fence> {
        let fence_desc = diligent_sys::FenceDesc::from(fence_desc);

        let mut fence_ptr = std::ptr::null_mut();

        unsafe {
            (*self.virtual_functions)
                .RenderDeviceVk
                .CreateFenceFromVulkanResource
                .unwrap_unchecked()(
                self.render_device_ptr,
                vk_timeline_semaphore,
                std::ptr::from_ref(&fence_desc),
                std::ptr::addr_of_mut!(fence_ptr),
            )
        };

        if fence_ptr.is_null() {
            None
        } else {
            Some(Fence::new(fence_ptr))
        }
    }
}
