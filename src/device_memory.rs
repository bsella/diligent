use std::{ffi::CStr, marker::PhantomData};

use crate::device_object::DeviceObject;

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

#[repr(transparent)]
pub struct DeviceMemoryDesc<'name>(diligent_sys::DeviceMemoryDesc, PhantomData<&'name ()>);

#[bon::bon]
impl<'name> DeviceMemoryDesc<'name> {
    #[builder]
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

#[repr(transparent)]
pub struct DeviceMemoryCreateInfo<'name, 'resources, 'objects>(
    pub(crate) diligent_sys::DeviceMemoryCreateInfo,
    PhantomData<(&'name (), &'resources (), &'objects ())>,
);

#[bon::bon]
impl<'name, 'resources, 'objects> DeviceMemoryCreateInfo<'name, 'resources, 'objects> {
    #[builder]
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

    pub fn resize(&self, new_size: u64) -> bool {
        unsafe_member_call!(self, DeviceMemory, Resize, new_size)
    }

    pub fn get_capacity(&self) -> u64 {
        unsafe_member_call!(self, DeviceMemory, GetCapacity)
    }

    pub fn is_compatible(&self, device_objet: &DeviceObject) -> bool {
        unsafe_member_call!(self, DeviceMemory, IsCompatible, device_objet.sys_ptr())
    }
}
