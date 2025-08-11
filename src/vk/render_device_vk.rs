use std::ops::Deref;

use crate::{
    buffer::{Buffer, BufferDesc},
    fence::{Fence, FenceDesc},
    graphics_types::ResourceState,
    render_device::RenderDevice,
    texture::{Texture, TextureDesc},
};

pub struct RenderDeviceVk<'a> {
    sys_ptr: *mut diligent_sys::IRenderDeviceVk,
    virtual_functions: *mut diligent_sys::IRenderDeviceVkVtbl,

    render_device: &'a RenderDevice,
}

impl Deref for RenderDeviceVk<'_> {
    type Target = RenderDevice;
    fn deref(&self) -> &Self::Target {
        self.render_device
    }
}

impl<'a> From<&'a RenderDevice> for RenderDeviceVk<'a> {
    fn from(value: &'a RenderDevice) -> Self {
        RenderDeviceVk {
            render_device: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IRenderDeviceVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IRenderDeviceVk)).pVtbl
            },
        }
    }
}

impl RenderDeviceVk<'_> {
    pub fn get_vk_device(&self) -> diligent_sys::VkDevice {
        unsafe_member_call!(self, RenderDeviceVk, GetVkDevice,)
    }

    pub fn get_vk_physical_device(&self) -> diligent_sys::VkPhysicalDevice {
        unsafe_member_call!(self, RenderDeviceVk, GetVkPhysicalDevice,)
    }

    pub fn get_vk_instance(&self) -> diligent_sys::VkInstance {
        unsafe_member_call!(self, RenderDeviceVk, GetVkInstance,)
    }

    pub fn get_vk_version(&self) -> u32 {
        unsafe_member_call!(self, RenderDeviceVk, GetVkVersion,)
    }

    pub fn create_texture_from_vulkan_image(
        &self,
        vk_image: diligent_sys::VkImage,
        texture_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Option<Texture> {
        let texture_desc = diligent_sys::TextureDesc::from(texture_desc);

        let mut texture_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateTextureFromVulkanImage,
            vk_image,
            std::ptr::from_ref(&texture_desc),
            initial_state.bits(),
            std::ptr::addr_of_mut!(texture_ptr)
        );

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

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateBufferFromVulkanResource,
            vk_buffer,
            std::ptr::from_ref(&buffer_desc),
            initial_state.bits(),
            std::ptr::addr_of_mut!(buffer_ptr)
        );

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

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateFenceFromVulkanResource,
            vk_timeline_semaphore,
            std::ptr::from_ref(&fence_desc),
            std::ptr::addr_of_mut!(fence_ptr)
        );

        if fence_ptr.is_null() {
            None
        } else {
            Some(Fence::new(fence_ptr))
        }
    }
}
