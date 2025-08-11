use std::ops::Deref;

use crate::pipeline_state_cache::PipelineStateCache;

pub struct PipelineStateCacheVk<'a> {
    sys_ptr: *mut diligent_sys::IPipelineStateCacheVk,
    virtual_functions: *mut diligent_sys::IPipelineStateCacheVkVtbl,

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
        PipelineStateCacheVk {
            cache: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IPipelineStateCacheVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IPipelineStateCacheVk)).pVtbl
            },
        }
    }
}

impl PipelineStateCacheVk<'_> {
    pub fn get_vk_pipeline_cache(&self) -> diligent_sys::VkPipelineCache {
        unsafe {
            (*self.virtual_functions)
                .PipelineStateCacheVk
                .GetVkPipelineCache
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
