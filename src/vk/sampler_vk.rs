use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::sampler::Sampler;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ISamplerVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct SamplerVk<'a>(&'a Sampler);

impl Deref for SamplerVk<'_> {
    type Target = Sampler;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a Sampler> for SamplerVk<'a> {
    fn from(value: &'a Sampler) -> Self {
        SamplerVk(value)
    }
}

impl SamplerVk<'_> {
    pub fn get_vk_sampler(&self) -> diligent_sys::VkSampler {
        unsafe_member_call!(self, SamplerVk, GetVkSampler)
    }
}
