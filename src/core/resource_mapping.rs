use std::collections::BTreeMap;

use super::{
    device_object::{AsDeviceObject, DeviceObject},
    object::{AsObject, Object},
};

pub struct ResourceMapping {
    pub(crate) resource_mapping: *mut diligent_sys::IResourceMapping,
    virtual_functions: *mut diligent_sys::IResourceMappingVtbl,

    resources: BTreeMap<String, Vec<*const DeviceObject>>,

    object: Object,
}

impl AsObject for ResourceMapping {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl ResourceMapping {
    pub(crate) fn new(resource_mapping_ptr: *mut diligent_sys::IResourceMapping) -> Self {
        ResourceMapping {
            resource_mapping: resource_mapping_ptr,
            virtual_functions: unsafe { (*resource_mapping_ptr).pVtbl },
            object: Object::new(resource_mapping_ptr as *mut diligent_sys::IObject),

            // We're assuming that resource_mapping_ptr is a pointer to a newly created resource
            // mapping that does not contain any resources for now
            resources: BTreeMap::new(),
        }
    }

    pub fn add_resource(&mut self, name: &str, object: &impl AsDeviceObject, is_unique: bool) {
        {
            let name = std::ffi::CString::new(name).unwrap();
            unsafe {
                (*self.virtual_functions)
                    .ResourceMapping
                    .AddResource
                    .unwrap_unchecked()(
                    self.resource_mapping,
                    name.as_ptr(),
                    object.as_device_object().device_object,
                    is_unique,
                );
            }
        }
        self.resources
            .entry(name.to_owned())
            .or_insert(Vec::new())
            .push(std::ptr::addr_of!(*object.as_device_object()));
    }

    pub fn add_resource_array(
        &mut self,
        name: &str,
        device_objects: &[impl AsDeviceObject],
        is_unique: bool,
    ) {
        let object_ptrs = Vec::from_iter(
            device_objects
                .iter()
                .map(|object| object.as_device_object().device_object),
        );

        {
            let name = std::ffi::CString::new(name).unwrap();

            unsafe {
                (*self.virtual_functions)
                    .ResourceMapping
                    .AddResourceArray
                    .unwrap_unchecked()(
                    self.resource_mapping,
                    name.as_ptr(),
                    0,
                    object_ptrs.as_ptr(),
                    device_objects.len() as u32,
                    is_unique,
                );
            }
        }

        self.resources
            .entry(name.to_owned())
            .or_insert(Vec::new())
            .extend(
                device_objects
                    .iter()
                    .map(|object| std::ptr::addr_of!(*object.as_device_object())),
            );
    }

    pub fn remove_resource_by_name(&mut self, name: &str, array_index: Option<u32>) {
        let array_index = array_index.unwrap_or(0);

        self.resources.entry(name.to_owned()).and_modify(|objects| {
            objects.remove(array_index as usize);
        });

        let name = std::ffi::CString::new(name).unwrap();

        unsafe {
            (*self.virtual_functions)
                .ResourceMapping
                .RemoveResourceByName
                .unwrap_unchecked()(self.resource_mapping, name.as_ptr(), array_index);
        }
    }

    pub fn get_resources_by_name(&self, _name: &str) -> Option<&[DeviceObject]> {
        //self.resources
        //    .get(name)
        //    .map(|resources| resources.as_slice())
        //    .map(|resources| resources.iter().map(|resource| *resource))
        todo!()
    }

    pub fn get_size(&self) -> usize {
        unsafe {
            (*self.virtual_functions)
                .ResourceMapping
                .GetSize
                .unwrap_unchecked()(self.resource_mapping)
        }
    }
}
