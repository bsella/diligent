use core::fmt;
use std::ops::Deref;

use static_assertions::const_assert_eq;

use super::object::Object;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDataBlobMethods>(),
    4 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DataBlob(Object);

impl Deref for DataBlob {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for DataBlob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let size = self.get_size();

            write!(
                f,
                "{}",
                String::from_raw_parts(
                    std::ptr::from_ref(self.get_data::<u8>(0)) as *mut _,
                    size,
                    size
                )
                .as_str()
            )
        }
    }
}

impl DataBlob {
    pub(crate) fn new(data_blob_ptr: *mut diligent_sys::IDataBlob) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IDeviceObject>(),
            std::mem::size_of::<diligent_sys::IDataBlob>()
        );

        Self(Object::new(data_blob_ptr as *mut diligent_sys::IObject))
    }

    pub fn resize(&mut self, new_size: usize) {
        unsafe_member_call!(self, DataBlob, Resize, new_size)
    }

    pub fn get_size(&self) -> usize {
        unsafe_member_call!(self, DataBlob, GetSize)
    }

    pub fn get_data<T>(&self, offset: usize) -> &T {
        let data_ptr = unsafe_member_call!(
            self,
            DataBlob,
            GetConstDataPtr,
            offset * std::mem::size_of::<T>()
        ) as *const T;

        unsafe { data_ptr.as_ref() }.unwrap()
    }

    pub fn get_mut_data<T>(&mut self, offset: usize) -> &mut T {
        let data_ptr = unsafe_member_call!(
            self,
            DataBlob,
            GetDataPtr,
            offset * std::mem::size_of::<T>()
        ) as *mut T;
        unsafe { data_ptr.as_mut() }.unwrap()
    }

    pub fn get_data_slice<T>(&self, size: usize, offset: usize) -> &[T] {
        let ptr = unsafe_member_call!(
            self,
            DataBlob,
            GetConstDataPtr,
            offset * std::mem::size_of::<T>()
        ) as *const T;
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    pub fn get_data_mut_slice<T>(&mut self, size: usize, offset: usize) -> &mut [T] {
        let ptr = unsafe_member_call!(
            self,
            DataBlob,
            GetDataPtr,
            offset * std::mem::size_of::<T>()
        ) as *mut T;
        unsafe { std::slice::from_raw_parts_mut(ptr, size) }
    }
}
