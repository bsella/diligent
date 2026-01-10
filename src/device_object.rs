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
