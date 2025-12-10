use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::object::Object;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceObjectMethods>(),
    4 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceObject(diligent_sys::IDeviceObject);

impl Deref for DeviceObject {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IObject as *const Object) }
    }
}

impl DeviceObject {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IDeviceObject {
        std::ptr::from_ref(&self.0) as _
    }

    pub fn get_unique_id(&self) -> i32 {
        unsafe_member_call!(self, DeviceObject, GetUniqueID)
    }
}
