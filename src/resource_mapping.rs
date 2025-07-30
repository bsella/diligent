use std::collections::BTreeMap;

use static_assertions::const_assert;

use crate::{device_object::DeviceObject, object::Object};

pub struct ResourceMapping {
    pub(crate) sys_ptr: *mut diligent_sys::IResourceMapping,
    virtual_functions: *mut diligent_sys::IResourceMappingVtbl,

    resources: BTreeMap<String, Vec<*const DeviceObject>>,

    object: Object,
}

impl AsRef<Object> for ResourceMapping {
    fn as_ref(&self) -> &Object {
        &self.object
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

        ResourceMapping {
            sys_ptr: resource_mapping_ptr,
            virtual_functions: unsafe { (*resource_mapping_ptr).pVtbl },
            object: Object::new(resource_mapping_ptr as *mut diligent_sys::IObject),

            // We're assuming that resource_mapping_ptr is a pointer to a newly created resource
            // mapping that does not contain any resources for now
            resources: BTreeMap::new(),
        }
    }

    pub fn add_resource(
        &mut self,
        name: impl AsRef<str>,
        object: &impl AsRef<DeviceObject>,
        is_unique: bool,
    ) {
        {
            let name = std::ffi::CString::new(name.as_ref()).unwrap();
            unsafe {
                (*self.virtual_functions)
                    .ResourceMapping
                    .AddResource
                    .unwrap_unchecked()(
                    self.sys_ptr,
                    name.as_ptr(),
                    object.as_ref().sys_ptr,
                    is_unique,
                );
            }
        }
        self.resources
            .entry(name.as_ref().to_owned())
            .or_default()
            .push(std::ptr::addr_of!(*object.as_ref()));
    }

    pub fn add_resource_array(
        &mut self,
        name: impl AsRef<str>,
        device_objects: &[impl AsRef<DeviceObject>],
        is_unique: bool,
    ) {
        let object_ptrs =
            Vec::from_iter(device_objects.iter().map(|object| object.as_ref().sys_ptr));

        {
            let name = std::ffi::CString::new(name.as_ref()).unwrap();

            unsafe {
                (*self.virtual_functions)
                    .ResourceMapping
                    .AddResourceArray
                    .unwrap_unchecked()(
                    self.sys_ptr,
                    name.as_ptr(),
                    0,
                    object_ptrs.as_ptr(),
                    device_objects.len() as u32,
                    is_unique,
                );
            }
        }

        self.resources
            .entry(name.as_ref().to_owned())
            .or_default()
            .extend(
                device_objects
                    .iter()
                    .map(|object| std::ptr::addr_of!(*object.as_ref())),
            );
    }

    pub fn remove_resource_by_name(&mut self, name: impl AsRef<str>, array_index: Option<u32>) {
        let array_index = array_index.unwrap_or(0);

        self.resources
            .entry(name.as_ref().to_owned())
            .and_modify(|objects| {
                objects.remove(array_index as usize);
            });

        let name = std::ffi::CString::new(name.as_ref()).unwrap();

        unsafe {
            (*self.virtual_functions)
                .ResourceMapping
                .RemoveResourceByName
                .unwrap_unchecked()(self.sys_ptr, name.as_ptr(), array_index);
        }
    }

    pub fn get_resources_by_name(&self, _name: impl AsRef<str>) -> Result<&[DeviceObject], ()> {
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
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
