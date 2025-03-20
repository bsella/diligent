pub(crate) struct Object {
    pub(crate) object: *mut diligent_sys::IObject,
    virtual_functions: *mut diligent_sys::IObjectVtbl,
}

impl Object {
    pub(crate) fn new(object: *mut diligent_sys::IObject) -> Self {
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

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            (*self.virtual_functions).Object.Release.unwrap_unchecked()(self.object);
        }
    }
}
