use crate::bindings;

pub(crate) struct Object {
    pub(crate) object: *mut bindings::IObject,
    virtual_functions: *mut bindings::IObjectVtbl,
}

impl Object {
    pub(crate) fn new(object: *mut bindings::IObject) -> Self {
        Object {
            virtual_functions: unsafe { (*object).pVtbl },
            object: object,
        }
    }

    pub(crate) fn add_ref(&self) {
        unsafe {
            (*self.virtual_functions).Object.AddRef.unwrap_unchecked()(self.object);
        }
    }
}

pub(crate) trait AsObject {
    fn as_object(&self) -> &Object;
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            (*self.virtual_functions).Object.Release.unwrap_unchecked()(self.object);
        }
    }
}
