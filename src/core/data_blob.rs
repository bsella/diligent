use core::fmt;

use crate::bindings;

use super::object::{AsObject, Object};

pub struct DataBlob {
    m_data_blob: *mut bindings::IDataBlob,
    m_virtual_functions: *mut bindings::IDataBlobVtbl,

    m_object: Object,
}

impl AsObject for DataBlob {
    fn as_object(&self) -> &Object {
        &self.m_object
    }
}

impl fmt::Debug for DataBlob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let size = self.get_size();

            write!(
                f,
                "{}",
                String::from_raw_parts(self.get_data_ptr::<u8>(0), size, size).as_str()
            )
        }
    }
}

impl DataBlob {
    pub(crate) fn new(data_blob_ptr: *mut bindings::IDataBlob) -> Self {
        DataBlob {
            m_data_blob: data_blob_ptr,
            m_virtual_functions: unsafe { (*data_blob_ptr).pVtbl },

            m_object: Object::new(data_blob_ptr as *mut bindings::IObject),
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        unsafe {
            (*self.m_virtual_functions)
                .DataBlob
                .Resize
                .unwrap_unchecked()(self.m_data_blob, new_size)
        }
    }

    pub fn get_size(&self) -> usize {
        unsafe {
            (*self.m_virtual_functions)
                .DataBlob
                .GetSize
                .unwrap_unchecked()(self.m_data_blob)
        }
    }

    pub fn get_data_ptr<T>(&self, offset: usize) -> *mut T {
        unsafe {
            (*self.m_virtual_functions)
                .DataBlob
                .GetDataPtr
                .unwrap_unchecked()(self.m_data_blob, offset) as *mut T
        }
    }

    pub fn get_const_data_ptr<T>(&self, offset: usize) -> *const T {
        unsafe {
            (*self.m_virtual_functions)
                .DataBlob
                .GetConstDataPtr
                .unwrap_unchecked()(self.m_data_blob, offset) as *const T
        }
    }
}
