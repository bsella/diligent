use std::ops::Deref;

use crate::sampler::Sampler;

#[repr(transparent)]
pub struct SamplerVk<'a> {
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
        SamplerVk { sampler: value }
    }
}

impl SamplerVk<'_> {
    pub fn get_vk_sampler(&self) -> diligent_sys::VkSampler {
        unsafe_member_call!(self, SamplerVk, GetVkSampler,)
    }
}
