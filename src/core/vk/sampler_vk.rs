use crate::core::{
    device_object::{AsDeviceObject, DeviceObject},
    sampler::Sampler,
};

pub struct SamplerVk<'a> {
    sampler_ptr: *mut diligent_sys::ISamplerVk,
    virtual_functions: *mut diligent_sys::ISamplerVkVtbl,

    sampler: &'a Sampler,
}

impl AsDeviceObject for SamplerVk<'_> {
    fn as_device_object(&self) -> &DeviceObject {
        &self.sampler.as_device_object()
    }
}

impl<'a> From<&'a Sampler> for SamplerVk<'a> {
    fn from(value: &'a Sampler) -> Self {
        SamplerVk {
            sampler: value,
            sampler_ptr: value.sampler as *mut diligent_sys::ISamplerVk,
            virtual_functions: unsafe { (*(value.sampler as *mut diligent_sys::ISamplerVk)).pVtbl },
        }
    }
}

impl SamplerVk<'_> {
    pub fn get_vk_sampler(&self) -> diligent_sys::VkSampler {
        unsafe {
            (*self.virtual_functions)
                .SamplerVk
                .GetVkSampler
                .unwrap_unchecked()(self.sampler_ptr)
        }
    }
}
