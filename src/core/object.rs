use crate::bindings;

pub(crate) struct Object {
    m_object: *mut bindings::IObject,
    m_virtual_functions: *mut bindings::IObjectVtbl,
}

impl Object {
    pub(crate) fn new(object: *mut bindings::IObject) -> Self {
        Object {
            m_virtual_functions: unsafe { (*object).pVtbl },
            m_object: object,
        }
    }

    pub(crate) fn add_ref(&self) {
        unsafe {
            (*self.m_virtual_functions).Object.AddRef.unwrap_unchecked()(self.m_object);
        }
    }
}

pub trait AsObject {
    fn as_object(&self) -> &Object;
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            (*self.m_virtual_functions)
                .Object
                .Release
                .unwrap_unchecked()(self.m_object);
        }
    }
}
