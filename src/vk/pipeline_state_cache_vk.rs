use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::pipeline_state_cache::PipelineStateCache;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IPipelineStateCacheVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct PipelineStateCacheVk(diligent_sys::IPipelineStateCacheVk);

impl Deref for PipelineStateCacheVk {
    type Target = PipelineStateCache;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IPipelineStateCache
                as *const PipelineStateCache)
        }
    }
}

impl PipelineStateCacheVk {
    pub fn get_vk_pipeline_cache(&self) -> diligent_sys::VkPipelineCache {
        unsafe_member_call!(self, PipelineStateCacheVk, GetVkPipelineCache)
    }
}
