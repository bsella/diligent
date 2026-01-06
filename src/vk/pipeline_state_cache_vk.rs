use crate::pipeline_state_cache::PipelineStateCache;

define_ported!(
    PipelineStateCacheVk,
    diligent_sys::IPipelineStateCacheVk,
    diligent_sys::IPipelineStateCacheVkMethods : 1,
    PipelineStateCache
);

impl PipelineStateCacheVk {
    pub fn get_vk_pipeline_cache(&self) -> diligent_sys::VkPipelineCache {
        unsafe_member_call!(self, PipelineStateCacheVk, GetVkPipelineCache)
    }
}
