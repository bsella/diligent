define_ported!(
    Object,
    diligent_sys::IObject,
    diligent_sys::IObjectMethods : 4
);

impl Object {
    pub(crate) fn add_ref(&self) {
        unsafe_member_call!(self, Object, AddRef);
    }
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IObject {
        std::ptr::from_ref(&self.0) as _
    }
}
