use std::ops::Deref;

use crate::sampler::Sampler;

pub struct SamplerVk<'a> {
    sys_ptr: *mut diligent_sys::ISamplerVk,
    virtual_functions: *mut diligent_sys::ISamplerVkVtbl,

    sampler: &'a Sampler,
}

impl Deref for SamplerVk<'_> {
    type Target = Sampler;
    fn deref(&self) -> &Self::Target {
        self.sampler
    }
}

impl<'a> From<&'a Sampler> for SamplerVk<'a> {
    fn from(value: &'a Sampler) -> Self {
        SamplerVk {
            sampler: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::ISamplerVk,
            virtual_functions: unsafe { (*(value.sys_ptr as *mut diligent_sys::ISamplerVk)).pVtbl },
        }
    }
}

impl SamplerVk<'_> {
    pub fn get_vk_sampler(&self) -> diligent_sys::VkSampler {
        unsafe {
            (*self.virtual_functions)
                .SamplerVk
                .GetVkSampler
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
