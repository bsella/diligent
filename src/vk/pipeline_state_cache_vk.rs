use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::pipeline_state_cache::PipelineStateCache;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IPipelineStateCacheVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct PipelineStateCacheVk<'a>(&'a PipelineStateCache);

impl Deref for PipelineStateCacheVk<'_> {
    type Target = PipelineStateCache;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a PipelineStateCache> for PipelineStateCacheVk<'a> {
    fn from(value: &'a PipelineStateCache) -> Self {
        PipelineStateCacheVk(value)
    }
}

impl PipelineStateCacheVk<'_> {
    pub fn get_vk_pipeline_cache(&self) -> diligent_sys::VkPipelineCache {
        unsafe_member_call!(self, PipelineStateCacheVk, GetVkPipelineCache)
    }
}
