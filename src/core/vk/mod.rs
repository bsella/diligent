pub mod engine_factory_vk;

#[cfg(feature = "vulkan_interop")]
#[path = ""]
mod vulkan_interop_modules {
    pub mod buffer_view_vk;
    pub mod buffer_vk;
    pub mod device_context_vk;
    pub mod fence_vk;
    pub mod pipeline_state_vk;
    pub mod render_device_vk;
    pub mod sampler_vk;
    pub mod swap_chain_vk;
    pub mod texture_view_vk;
    pub mod texture_vk;
    // TODO
    // BottomLevelASVk
    // CommandQueueVk
    // DeviceMemoryVk
    // FramebufferVk
    // PipelineStateCacheVk
    // QueryVk
    // RenderPassVk
    // ShaderBindingTableVk
    // TopLevelASVk
}

#[cfg(feature = "vulkan_interop")]
pub use vulkan_interop_modules::*;
