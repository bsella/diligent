#[repr(transparent)]
pub struct Object {
    pub(crate) sys_ptr: *mut diligent_sys::IObject,
}

impl Object {
    pub(crate) fn new(sys_ptr: *mut diligent_sys::IObject) -> Self {
        Object { sys_ptr }
    }

    pub(crate) fn add_ref(&self) {
        unsafe_member_call!(self, Object, AddRef,);
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe_member_call!(self, Object, Release,);
    }
}
