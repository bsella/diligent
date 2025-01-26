use crate::bindings;

use super::object::{AsObject, Object};

pub struct DeviceObject {
    pub(crate) device_object: *mut bindings::IDeviceObject,
    virtual_functions: *mut bindings::IDeviceObjectVtbl,
    object: Object,
}

impl AsObject for DeviceObject {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

pub trait AsDeviceObject {
    fn as_device_object(&self) -> &DeviceObject;
}

impl DeviceObject {
    pub(crate) fn new(device_object: *mut bindings::IDeviceObject) -> Self {
        DeviceObject {
            virtual_functions: unsafe { (*device_object).pVtbl },
            device_object: device_object,
            object: Object::new(device_object as *mut bindings::IObject),
        }
    }

    pub fn get_desc(&self) -> &bindings::DeviceObjectAttribs {
        unsafe {
            (*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn get_unique_id(&self) -> i32 {
        unsafe {
            (*self.virtual_functions)
                .DeviceObject
                .GetUniqueID
                .unwrap_unchecked()(self.device_object)
        }
    }
}
