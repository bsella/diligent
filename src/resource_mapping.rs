use std::ffi::CStr;

use crate::{device_object::DeviceObject, object::Object};

#[repr(transparent)]
pub struct ResourceMappingEntry(pub(crate) diligent_sys::ResourceMappingEntry);

#[bon::bon]
impl ResourceMappingEntry {
    #[builder]
    pub fn new(name: Option<&CStr>, object: &DeviceObject, array_index: Option<u32>) -> Self {
        ResourceMappingEntry(diligent_sys::ResourceMappingEntry {
            Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
            pObject: object.sys_ptr(),
            ArrayIndex: array_index.unwrap_or(0),
        })
    }
}

#[repr(transparent)]
pub struct ResourceMappingCreateInfo(pub(crate) diligent_sys::ResourceMappingCreateInfo);

#[bon::bon]
impl ResourceMappingCreateInfo {
    #[builder]
    pub fn new(entries: &[ResourceMappingEntry]) -> Self {
        ResourceMappingCreateInfo(diligent_sys::ResourceMappingCreateInfo {
            pEntries: entries.first().map_or(std::ptr::null(), |entry| &entry.0),
            NumEntries: entries.len() as u32,
        })
    }
}

define_ported!(
    ResourceMapping,
    diligent_sys::IResourceMapping,
    diligent_sys::IResourceMappingMethods : 5,
    Object
);

impl ResourceMapping {
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
        let name = std::ffi::CString::new(name.as_ref()).unwrap();

        unsafe_member_call!(
            self,
            ResourceMapping,
            AddResourceArray,
            name.as_ptr(),
            0,
            device_objects.as_ptr() as _,
            device_objects.len() as u32,
            is_unique
        );
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
