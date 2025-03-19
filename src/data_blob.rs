use core::fmt;

use static_assertions::const_assert;

use super::object::{AsObject, Object};

pub struct DataBlob {
    sys_ptr: *mut diligent_sys::IDataBlob,
    virtual_functions: *mut diligent_sys::IDataBlobVtbl,

    object: Object,
}

impl AsObject for DataBlob {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl fmt::Debug for DataBlob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let size = self.get_size();

            write!(
                f,
                "{}",
                String::from_raw_parts(std::ptr::from_mut(self.get_mut_data::<u8>(0)), size, size)
                    .as_str()
            )
        }
    }
}

impl DataBlob {
    pub(crate) fn new(data_blob_ptr: *mut diligent_sys::IDataBlob) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IDataBlob>()
        );

        DataBlob {
            sys_ptr: data_blob_ptr,
            virtual_functions: unsafe { (*data_blob_ptr).pVtbl },

            object: Object::new(data_blob_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        unsafe {
            (*self.virtual_functions).DataBlob.Resize.unwrap_unchecked()(self.sys_ptr, new_size)
        }
    }

    pub fn get_size(&self) -> usize {
        unsafe {
            (*self.virtual_functions)
                .DataBlob
                .GetSize
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_data<T>(&self, offset: usize) -> &T {
        unsafe {
            ((*self.virtual_functions)
                .DataBlob
                .GetConstDataPtr
                .unwrap_unchecked()(self.sys_ptr, offset) as *const T)
                .as_ref()
        }
        .unwrap()
    }

    pub fn get_mut_data<T>(&self, offset: usize) -> &mut T {
        unsafe {
            ((*self.virtual_functions)
                .DataBlob
                .GetDataPtr
                .unwrap_unchecked()(self.sys_ptr, offset) as *mut T)
                .as_mut()
        }
        .unwrap()
    }
}
