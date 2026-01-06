use crate::sampler::Sampler;

define_ported!(
    SamplerVk,
    diligent_sys::ISamplerVk,
    diligent_sys::ISamplerVkMethods : 1,
    Sampler
);

impl SamplerVk {
    pub fn get_vk_sampler(&self) -> diligent_sys::VkSampler {
        unsafe_member_call!(self, SamplerVk, GetVkSampler)
    }
}
