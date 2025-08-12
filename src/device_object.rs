use std::ops::Deref;

use static_assertions::{const_assert, const_assert_eq};

use super::object::Object;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceObjectMethods>(),
    4 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceObject {
    object: Object,
}

impl Deref for DeviceObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl DeviceObject {
    pub(crate) fn new(device_object_ptr: *mut diligent_sys::IDeviceObject) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IDeviceObject>()
        );
        DeviceObject {
            object: Object::new(device_object_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn get_unique_id(&self) -> i32 {
        unsafe_member_call!(self, DeviceObject, GetUniqueID,)
    }
}
