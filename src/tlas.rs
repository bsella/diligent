use std::ffi::CString;

use bon::Builder;
use static_assertions::const_assert;

use crate::{
    blas::RayTracingBuildAsFlags, device_object::DeviceObject, graphics_types::ResourceState,
};

#[derive(Builder)]
pub struct TopLevelASDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

    #[builder(default = 0)]
    max_instance_count: u32,

    #[builder(default)]
    flags: RayTracingBuildAsFlags,

    #[builder(default = 0)]
    compacted_size: u64,

    #[builder(default = 1)]
    immediate_context_mask: u64,
}

impl From<&TopLevelASDesc> for diligent_sys::TopLevelASDesc {
    fn from(value: &TopLevelASDesc) -> Self {
        Self {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            MaxInstanceCount: value.max_instance_count,
            Flags: value.flags.bits(),
            CompactedSize: value.compacted_size,
            ImmediateContextMask: value.immediate_context_mask,
        }
    }
}

pub struct TopLevelAS {
    pub(crate) sys_ptr: *mut diligent_sys::ITopLevelAS,
    virtual_functions: *mut diligent_sys::ITopLevelASVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for TopLevelAS {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl TopLevelAS {
    pub(crate) fn new(sys_ptr: *mut diligent_sys::ITopLevelAS) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::ITopLevelAS>()
        );

        Self {
            sys_ptr,
            virtual_functions: unsafe { (*sys_ptr).pVtbl },
            device_object: DeviceObject::new(sys_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    // TODO pub fn get_desc() -> TopLevelASDesc;
    // TODO pub fn get_instance_desc(&self, name: impl AsRef<str>) -> TLASInstanceDesc {}
    // TODO pub fn get_build_info(&self) -> TLASBuildInfo {}
    // TODO pub fn get_scratch_buffer_sizes(&self) -> ScratchBufferSizes {}

    pub fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .TopLevelAS
                .GetNativeHandle
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn set_state(&self, state: ResourceState) {
        unsafe {
            (*self.virtual_functions)
                .TopLevelAS
                .SetState
                .unwrap_unchecked()(self.sys_ptr, state.bits())
        }
    }

    pub fn get_state(&self) -> ResourceState {
        ResourceState::from_bits_retain(unsafe {
            (*self.virtual_functions)
                .TopLevelAS
                .GetState
                .unwrap_unchecked()(self.sys_ptr)
        })
    }
}
