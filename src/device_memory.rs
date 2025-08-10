use std::ffi::CString;

use bon::Builder;
use static_assertions::const_assert;

use crate::device_object::DeviceObject;

pub struct DeviceMemory {
    pub(crate) sys_ptr: *mut diligent_sys::IDeviceMemory,
    virtual_functions: *mut diligent_sys::IDeviceMemoryVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for DeviceMemory {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

#[derive(Clone, Copy)]
pub enum DeviceMemoryType {
    Sparce,
}

#[derive(Builder)]
pub struct DeviceMemoryDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

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
                Name: value.name.as_ptr(),
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
    pub(crate) fn new(fence_ptr: *mut diligent_sys::IDeviceMemory) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IDeviceMemory>()
        );
        DeviceMemory {
            sys_ptr: fence_ptr,
            virtual_functions: unsafe { (*fence_ptr).pVtbl },
            device_object: DeviceObject::new(fence_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn resize(&self, new_size: u64) -> bool {
        unsafe {
            (*self.virtual_functions)
                .DeviceMemory
                .Resize
                .unwrap_unchecked()(self.sys_ptr, new_size)
        }
    }

    pub fn get_capacity(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .DeviceMemory
                .GetCapacity
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn is_compatible(&self, device_objet: impl AsRef<DeviceObject>) -> bool {
        unsafe {
            (*self.virtual_functions)
                .DeviceMemory
                .IsCompatible
                .unwrap_unchecked()(self.sys_ptr, device_objet.as_ref().sys_ptr)
        }
    }
}
