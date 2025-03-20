use crate::{device_object::DeviceObject, sampler::Sampler};

pub struct SamplerVk<'a> {
    sampler_ptr: *mut diligent_sys::ISamplerVk,
    virtual_functions: *mut diligent_sys::ISamplerVkVtbl,

    sampler: &'a Sampler,
}

impl AsRef<DeviceObject> for SamplerVk<'_> {
    fn as_ref(&self) -> &DeviceObject {
        self.sampler.as_ref()
    }
}

impl<'a> From<&'a Sampler> for SamplerVk<'a> {
    fn from(value: &'a Sampler) -> Self {
        SamplerVk {
            sampler: value,
            sampler_ptr: value.sys_ptr as *mut diligent_sys::ISamplerVk,
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
                .unwrap_unchecked()(self.sampler_ptr)
        }
    }
}
