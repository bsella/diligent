use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{device_object::DeviceObject, object::Object};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IResourceMappingMethods>(),
    5 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ResourceMapping(diligent_sys::IResourceMapping);

impl Deref for ResourceMapping {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IObject as *const Object) }
    }
}

impl ResourceMapping {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IResourceMapping {
        std::ptr::addr_of!(self.0) as _
    }

    pub fn add_resource(&mut self, name: impl AsRef<str>, object: &DeviceObject, is_unique: bool) {
        let name = std::ffi::CString::new(name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ResourceMapping,
            AddResource,
            name.as_ptr(),
            object.sys_ptr() as _,
            is_unique
        );
    }

    pub fn add_resource_array(
        &mut self,
        name: impl AsRef<str>,
        device_objects: &[&DeviceObject],
        is_unique: bool,
    ) {
        let object_ptrs = Vec::from_iter(device_objects.iter().map(|object| object.sys_ptr()));

        {
            let name = std::ffi::CString::new(name.as_ref()).unwrap();

            unsafe_member_call!(
                self,
                ResourceMapping,
                AddResourceArray,
                name.as_ptr(),
                0,
                object_ptrs.as_ptr() as _,
                device_objects.len() as u32,
                is_unique
            );
        }
    }

    pub fn remove_resource_by_name(&mut self, name: impl AsRef<str>, array_index: Option<u32>) {
        let array_index = array_index.unwrap_or(0);

        let name = std::ffi::CString::new(name.as_ref()).unwrap();

        unsafe_member_call!(
            self,
            ResourceMapping,
            RemoveResourceByName,
            name.as_ptr(),
            array_index
        );
    }

    pub fn get_resource_by_name(
        &self,
        name: impl AsRef<str>,
        array_index: Option<u32>,
    ) -> Option<&DeviceObject> {
        let array_index = array_index.unwrap_or(0);

        let name = std::ffi::CString::new(name.as_ref()).unwrap();

        let resource_ptr = unsafe_member_call!(
            self,
            ResourceMapping,
            GetResource,
            name.as_ptr(),
            array_index
        );

        if resource_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(resource_ptr as *const DeviceObject) })
        }
    }

    pub fn get_size(&self) -> usize {
        unsafe_member_call!(self, ResourceMapping, GetSize)
    }
}
