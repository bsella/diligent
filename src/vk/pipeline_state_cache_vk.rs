use std::ops::Deref;

use crate::pipeline_state_cache::PipelineStateCache;

#[repr(transparent)]
pub struct PipelineStateCacheVk<'a> {
    cache: &'a PipelineStateCache,
}

impl Deref for PipelineStateCacheVk<'_> {
    type Target = PipelineStateCache;
    fn deref(&self) -> &Self::Target {
        self.cache
    }
}

impl<'a> From<&'a PipelineStateCache> for PipelineStateCacheVk<'a> {
    fn from(value: &'a PipelineStateCache) -> Self {
        PipelineStateCacheVk { cache: value }
    }
}

impl PipelineStateCacheVk<'_> {
    pub fn get_vk_pipeline_cache(&self) -> diligent_sys::VkPipelineCache {
        unsafe_member_call!(self, PipelineStateCacheVk, GetVkPipelineCache,)
    }
}
