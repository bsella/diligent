use crate::core::bindings;

use super::object::{AsObject, Object};

pub struct DeviceObject {
    pub(crate) m_device_object: *mut bindings::IDeviceObject,
    m_virtual_functions: *mut bindings::IDeviceObjectVtbl,
    m_object: Object,
}

impl DeviceObject {
    pub(crate) fn new(device_object: *mut bindings::IDeviceObject) -> Self {
        DeviceObject {
            m_virtual_functions: unsafe { (*device_object).pVtbl },
            m_device_object: device_object,
            m_object: Object::new(device_object as *mut bindings::IObject),
        }
    }
}

impl AsObject for DeviceObject {
    fn as_object(&self) -> &Object {
        &self.m_object
    }
}

pub trait AsDeviceObject {
    fn as_device_object(&self) -> &DeviceObject;
}

impl DeviceObject {
    fn get_desc(&self) -> &bindings::DeviceObjectAttribs {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.m_device_object)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_unique_id(&self) -> i32 {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceObject
                .GetUniqueID
                .unwrap_unchecked()(self.m_device_object)
        }
    }
}
