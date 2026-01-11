use crate::{
    BottomLevelAS, BottomLevelASDesc, Boxed, BoxedFromNulError, TopLevelAS, TopLevelASDesc,
    buffer::{Buffer, BufferDesc},
    fence::{Fence, FenceDesc},
    graphics_types::ResourceState,
    render_device::RenderDevice,
    texture::{Texture, TextureDesc},
};

define_ported!(
    RenderDeviceVk,
    diligent_sys::IRenderDeviceVk,
    diligent_sys::IRenderDeviceVkMethods : 11,
    RenderDevice
);

impl RenderDeviceVk {
    pub fn get_vk_device(&self) -> diligent_sys::VkDevice {
        unsafe_member_call!(self, RenderDeviceVk, GetVkDevice)
    }

    pub fn get_vk_physical_device(&self) -> diligent_sys::VkPhysicalDevice {
        unsafe_member_call!(self, RenderDeviceVk, GetVkPhysicalDevice)
    }

    pub fn get_vk_instance(&self) -> diligent_sys::VkInstance {
        unsafe_member_call!(self, RenderDeviceVk, GetVkInstance)
    }

    pub fn get_vk_version(&self) -> u32 {
        unsafe_member_call!(self, RenderDeviceVk, GetVkVersion)
    }

    /// # Safety
    /// vk_image is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn create_texture_from_vulkan_image(
        &self,
        vk_image: diligent_sys::VkImage,
        texture_desc: &TextureDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Texture>, BoxedFromNulError> {
        let mut texture_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateTextureFromVulkanImage,
            vk_image,
            &texture_desc.0,
            initial_state.bits(),
            &mut texture_ptr
        );

        Boxed::new(texture_ptr)
    }

    /// # Safety
    /// vk_buffer is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn create_buffer_from_vulkan_resource(
        &self,
        vk_buffer: diligent_sys::VkBuffer,
        buffer_desc: &BufferDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<Buffer>, BoxedFromNulError> {
        let mut buffer_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateBufferFromVulkanResource,
            vk_buffer,
            &buffer_desc.0,
            initial_state.bits(),
            &mut buffer_ptr
        );

        Boxed::new(buffer_ptr)
    }

    /// # Safety
    /// vk_blas is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn create_blas_from_vulkan_resource(
        &self,
        vk_blas: diligent_sys::VkAccelerationStructureKHR,
        blas_desc: &BottomLevelASDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<BottomLevelAS>, BoxedFromNulError> {
        let mut bottom_level_as_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateBLASFromVulkanResource,
            vk_blas,
            &blas_desc.0,
            initial_state.bits(),
            &mut bottom_level_as_ptr
        );

        Boxed::new(bottom_level_as_ptr)
    }

    /// # Safety
    /// vk_tlas is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn create_tlas_from_vulkan_resource(
        &self,
        vk_tlas: diligent_sys::VkAccelerationStructureKHR,
        tlas_desc: &TopLevelASDesc,
        initial_state: ResourceState,
    ) -> Result<Boxed<TopLevelAS>, BoxedFromNulError> {
        let mut top_level_as_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateTLASFromVulkanResource,
            vk_tlas,
            &tlas_desc.0,
            initial_state.bits(),
            &mut top_level_as_ptr
        );

        Boxed::new(top_level_as_ptr)
    }

    /// # Safety
    /// vk_timeline_semaphore is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn create_fence_from_vulkan_resource(
        &self,
        vk_timeline_semaphore: diligent_sys::VkSemaphore,
        fence_desc: &FenceDesc,
    ) -> Result<Boxed<Fence>, BoxedFromNulError> {
        let mut fence_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            RenderDeviceVk,
            CreateFenceFromVulkanResource,
            vk_timeline_semaphore,
            &fence_desc.0,
            &mut fence_ptr
        );

        Boxed::new(fence_ptr)
    }

    // TODO
    //pub fn get_device_features_vk(){}

    // TODO
    //pub fn get_dx_compiler(){}
}
