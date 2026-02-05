use std::ffi::CStr;

use crate::object::Object;

define_ported!(
    DeviceObject,
    diligent_sys::IDeviceObject,
    diligent_sys::IDeviceObjectMethods : 4,
    Object
);

impl DeviceObject {
    pub fn get_unique_id(&self) -> i32 {
        unsafe_member_call!(self, DeviceObject, GetUniqueID)
    }
}

#[repr(transparent)]
pub struct DeviceObjectAttribs(diligent_sys::DeviceObjectAttribs);

impl DeviceObjectAttribs {
    pub fn name(&self) -> Option<&CStr> {
        if self.0.Name.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(self.0.Name)) }
        }
    }
}
