use static_assertions::const_assert_eq;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IObjectMethods>(),
    4 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct Object(diligent_sys::IObject);

impl Object {
    pub(crate) fn add_ref(&self) {
        unsafe_member_call!(self, Object, AddRef);
    }
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IObject {
        std::ptr::from_ref(&self.0) as _
    }
}
