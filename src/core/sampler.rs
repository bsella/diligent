use crate::core::bindings;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct Sampler {
    pub(crate) m_sampler: *mut bindings::ISampler,
    m_virtual_functions: *mut bindings::ISamplerVtbl,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for Sampler {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl Sampler {
    pub(crate) fn create(sampler_ptr: *mut bindings::ISampler) -> Self {
        Sampler {
            m_sampler: sampler_ptr,
            m_virtual_functions: unsafe { (*sampler_ptr).pVtbl },
            m_device_object: DeviceObject::create(sampler_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> &bindings::SamplerDesc {
        unsafe {
            ((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.m_sampler as *mut bindings::IDeviceObject)
                as *const bindings::SamplerDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}
