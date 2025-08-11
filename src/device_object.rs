use static_assertions::const_assert;

use super::object::Object;

pub struct DeviceObject {
    pub(crate) sys_ptr: *mut diligent_sys::IDeviceObject,
    virtual_functions: *mut diligent_sys::IDeviceObjectVtbl,
    object: Object,
}

impl AsRef<Object> for DeviceObject {
    fn as_ref(&self) -> &Object {
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
            virtual_functions: unsafe { (*device_object_ptr).pVtbl },
            sys_ptr: device_object_ptr,
            object: Object::new(device_object_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn get_unique_id(&self) -> i32 {
        unsafe_member_call!(self, DeviceObject, GetUniqueID,)
    }
}
