use std::ops::Deref;

use static_assertions::{const_assert, const_assert_eq};

use crate::{device_object::DeviceObject, object::Object};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IResourceMappingMethods>(),
    5 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ResourceMapping(Object);

impl Deref for ResourceMapping {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ResourceMapping {
    pub(crate) fn new(resource_mapping_ptr: *mut diligent_sys::IResourceMapping) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IResourceMapping>()
        );

        Self(Object::new(
            resource_mapping_ptr as *mut diligent_sys::IObject,
        ))
    }

    pub fn add_resource(&mut self, name: impl AsRef<str>, object: &DeviceObject, is_unique: bool) {
        let name = std::ffi::CString::new(name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ResourceMapping,
            AddResource,
            name.as_ptr(),
            object.sys_ptr as _,
            is_unique
        );
    }

    pub fn add_resource_array(
        &mut self,
        name: impl AsRef<str>,
        device_objects: &[&DeviceObject],
        is_unique: bool,
    ) {
        let object_ptrs = Vec::from_iter(device_objects.iter().map(|object| object.sys_ptr));

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
    ) -> Option<DeviceObject> {
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
            let object = DeviceObject::new(resource_ptr);
            object.add_ref();
            Some(object)
        }
    }

    pub fn get_size(&self) -> usize {
        unsafe_member_call!(self, ResourceMapping, GetSize)
    }
}
