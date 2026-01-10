use std::ffi::CString;

use bon::Builder;

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

#[derive(Builder)]
pub struct DeviceMemoryDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: Option<CString>,

    device_memory_type: Option<DeviceMemoryType>,

    page_size: u64,

    #[builder(default = 1)]
    immediate_context_mask: u64,
}

#[derive(Builder)]
pub struct DeviceMemoryCreateInfo<'a> {
    pub(crate) desc: DeviceMemoryDesc,

    pub(crate) initial_size: u64,

    pub(crate) compatible_resources: Vec<&'a DeviceObject>,
}

impl From<&DeviceMemoryDesc> for diligent_sys::DeviceMemoryDesc {
    fn from(value: &DeviceMemoryDesc) -> Self {
        diligent_sys::DeviceMemoryDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            Type: if let Some(memory_type) = value.device_memory_type {
                match memory_type {
                    DeviceMemoryType::Sparce => diligent_sys::DEVICE_MEMORY_TYPE_SPARSE,
                }
            } else {
                diligent_sys::DEVICE_MEMORY_TYPE_UNDEFINED
            } as diligent_sys::DEVICE_MEMORY_TYPE,

            PageSize: value.page_size,

            ImmediateContextMask: value.immediate_context_mask,
        }
    }
}

impl DeviceMemory {
    pub fn desc(&self) -> &DeviceMemoryDesc {
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
