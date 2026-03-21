use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use crate::{
    Ported,
    device_object::{DeviceObject, DeviceObjectAttribs},
};

define_ported!(
    DeviceMemory,
    diligent_sys::IDeviceMemory,
    diligent_sys::IDeviceMemoryMethods : 3,
    DeviceObject
);

#[derive(Clone, Copy)]
pub enum DeviceMemoryType {
    Sparce,
}

impl DeviceMemoryType {
    pub fn from_sys(value: diligent_sys::DEVICE_MEMORY_TYPE) -> Option<DeviceMemoryType> {
        match value as _ {
            diligent_sys::DEVICE_MEMORY_TYPE_UNDEFINED => None,
            diligent_sys::DEVICE_MEMORY_TYPE_SPARSE => Some(DeviceMemoryType::Sparce),
            _ => panic!("Unknown DEVICE_MEMORY_TYPE value"),
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DeviceMemoryDesc<'name>(diligent_sys::DeviceMemoryDesc, PhantomData<&'name ()>);

impl Deref for DeviceMemoryDesc<'_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name> DeviceMemoryDesc<'name> {
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,

        device_memory_type: Option<DeviceMemoryType>,

        page_size: u64,

        #[builder(default = 1)] immediate_context_mask: u64,
    ) -> Self {
        DeviceMemoryDesc(
            diligent_sys::DeviceMemoryDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.as_ref().map_or(std::ptr::null(), |name| name.as_ptr()),
                },
                Type: if let Some(memory_type) = device_memory_type {
                    match memory_type {
                        DeviceMemoryType::Sparce => diligent_sys::DEVICE_MEMORY_TYPE_SPARSE,
                    }
                } else {
                    diligent_sys::DEVICE_MEMORY_TYPE_UNDEFINED
                } as diligent_sys::DEVICE_MEMORY_TYPE,

                PageSize: page_size,

                ImmediateContextMask: immediate_context_mask,
            },
            PhantomData,
        )
    }
}

impl DeviceMemoryDesc<'_> {
    pub fn device_memory_type(&self) -> Option<DeviceMemoryType> {
        DeviceMemoryType::from_sys(self.0.Type)
    }
    pub fn page_size(&self) -> u64 {
        self.0.PageSize
    }
    pub fn immediate_context_mask(&self) -> u64 {
        self.0.ImmediateContextMask
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DeviceMemoryCreateInfo<'name, 'resources, 'objects>(
    pub(crate) diligent_sys::DeviceMemoryCreateInfo,
    PhantomData<(&'name (), &'resources (), &'objects ())>,
);

#[bon::bon]
impl<'name, 'resources, 'objects> DeviceMemoryCreateInfo<'name, 'resources, 'objects> {
    #[builder(derive(Clone))]
    pub fn new(
        desc: DeviceMemoryDesc<'name>,
        initial_size: u64,
        compatible_resources: &'resources [&'objects DeviceObject],
    ) -> Self {
        DeviceMemoryCreateInfo(
            diligent_sys::DeviceMemoryCreateInfo {
                Desc: desc.0,
                InitialSize: initial_size,
                ppCompatibleResources: compatible_resources
                    .first()
                    .map_or(std::ptr::null_mut(), |resource| {
                        std::ptr::from_ref(resource) as _
                    }),
                NumResources: compatible_resources.len() as u32,
            },
            PhantomData,
        )
    }
}

impl DeviceMemory {
    pub fn desc(&self) -> &DeviceMemoryDesc<'_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const DeviceMemoryDesc) }
    }

    pub fn resize(&mut self, new_size: u64) -> bool {
        unsafe_member_call!(self, DeviceMemory, Resize, new_size)
    }

    pub fn get_capacity(&self) -> u64 {
        unsafe_member_call!(self, DeviceMemory, GetCapacity)
    }

    pub fn is_compatible(&self, device_objet: &DeviceObject) -> bool {
        unsafe_member_call!(self, DeviceMemory, IsCompatible, device_objet.sys_ptr())
    }
}

// # Safety : Access to DeviceMemory can be thread safe
unsafe impl Sync for DeviceMemory {}
