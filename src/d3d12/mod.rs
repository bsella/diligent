pub mod engine_factory_d3d12;

#[cfg(feature = "d3d12_interop")]
#[path = ""]
mod d3d12_interop_modules {
    //pub mod buffer_view_vk;
    //pub mod buffer_vk;
    //pub mod device_context_vk;
    //pub mod fence_vk;
    //pub mod pipeline_state_vk;
    //pub mod render_device_vk;
    //pub mod sampler_vk;
    //pub mod swap_chain_vk;
    //pub mod texture_view_vk;
    //pub mod texture_vk;
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

#[cfg(feature = "d3d12_interop")]
pub use d3d12_interop_modules::*;
