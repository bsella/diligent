use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::sampler::Sampler;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ISamplerVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct SamplerVk(diligent_sys::ISamplerVk);

impl Deref for SamplerVk {
    type Target = Sampler;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ISampler as *const Sampler)
        }
    }
}

impl SamplerVk {
    pub fn get_vk_sampler(&self) -> diligent_sys::VkSampler {
        unsafe_member_call!(self, SamplerVk, GetVkSampler)
    }
}
