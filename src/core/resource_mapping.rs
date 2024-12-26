use std::collections::BTreeMap;

use crate::core::bindings;

use super::{
    device_object::DeviceObject,
    object::{AsObject, Object},
};

pub struct ResourceMapping {
    pub(crate) m_resource_mapping: *mut bindings::IResourceMapping,
    m_virtual_functions: *mut bindings::IResourceMappingVtbl,

    m_resources: BTreeMap<String, Vec<*const DeviceObject>>,

    m_object: Object,
}

impl AsObject for ResourceMapping {
    fn as_object(&self) -> &Object {
        &self.m_object
    }
}

impl ResourceMapping {
    pub(crate) fn new(resource_mapping_ptr: *mut bindings::IResourceMapping) -> Self {
        ResourceMapping {
            m_resource_mapping: resource_mapping_ptr,
            m_virtual_functions: unsafe { (*resource_mapping_ptr).pVtbl },
            m_object: Object::new(resource_mapping_ptr as *mut bindings::IObject),

            // We're assuming that resource_mapping_ptr is a pointer to a newly created resource
            // mapping that does not contain any resources for now
            m_resources: BTreeMap::new(),
        }
    }

    fn add_resource(&mut self, name: &str, object: &DeviceObject, is_unique: bool) {
        unsafe {
            (*self.m_virtual_functions)
                .ResourceMapping
                .AddResource
                .unwrap_unchecked()(
                self.m_resource_mapping,
                name.as_bytes().as_ptr() as *const i8,
                object.m_device_object,
                is_unique,
            );
        }
        self.m_resources
            .entry(name.to_string())
            .or_insert(Vec::new())
            .push(std::ptr::addr_of!(*object));
    }

    fn add_resource_array(&mut self, name: &str, objects: &[DeviceObject], is_unique: bool) {
        let object_ptrs = Vec::from_iter(objects.iter().map(|object| object.m_device_object));

        unsafe {
            (*self.m_virtual_functions)
                .ResourceMapping
                .AddResourceArray
                .unwrap_unchecked()(
                self.m_resource_mapping,
                name.as_bytes().as_ptr() as *const i8,
                0,
                object_ptrs.as_ptr(),
                objects.len() as u32,
                is_unique,
            );
        }

        self.m_resources
            .entry(name.to_string())
            .or_insert(Vec::new())
            .extend(objects.iter().map(|object| std::ptr::addr_of!(*object)));
    }

    fn remove_resource_by_name(&mut self, name: &str, array_index: Option<u32>) {
        let array_index = array_index.unwrap_or(0);

        self.m_resources
            .entry(name.to_string())
            .and_modify(|objects| {
                objects.remove(array_index as usize);
            });

        unsafe {
            (*self.m_virtual_functions)
                .ResourceMapping
                .RemoveResourceByName
                .unwrap_unchecked()(
                self.m_resource_mapping,
                name.as_bytes().as_ptr() as *const i8,
                array_index,
            );
        }
    }

    // TODO
    //fn get_resources_by_name(&self, name: &str) -> Option<&[DeviceObject]> {
    //    self.m_resources
    //        .get(name)
    //        .map(|resources| resources.as_slice())
    //        .map(|resources| resources.iter().map(|resource| *resource))
    //}

    fn get_size(&self) -> usize {
        unsafe {
            (*self.m_virtual_functions)
                .ResourceMapping
                .GetSize
                .unwrap_unchecked()(self.m_resource_mapping)
        }
    }
}
