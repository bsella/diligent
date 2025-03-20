use std::ffi::CString;

use static_assertions::const_assert;

use super::device_object::DeviceObject;

pub struct Fence {
    pub(crate) sys_ptr: *mut diligent_sys::IFence,
    virtual_functions: *mut diligent_sys::IFenceVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for Fence {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

pub enum FenceType {
    CpuWaitOnly,
    General,
}

const_assert!(diligent_sys::FENCE_TYPE_LAST == 1);

pub struct FenceDesc {
    name: CString,
    fence_type: FenceType,
}

impl From<&FenceDesc> for diligent_sys::FenceDesc {
    fn from(value: &FenceDesc) -> Self {
        diligent_sys::FenceDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            Type: match value.fence_type {
                FenceType::CpuWaitOnly => diligent_sys::FENCE_TYPE_CPU_WAIT_ONLY,
                FenceType::General => diligent_sys::FENCE_TYPE_GENERAL,
            } as diligent_sys::FENCE_TYPE,
        }
    }
}

impl FenceDesc {
    pub fn fence_desc(mut self, fence_type: FenceType) -> Self {
        self.fence_type = fence_type;
        self
    }
}

impl Fence {
    pub(crate) fn new(fence_ptr: *mut diligent_sys::IFence) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IFence>()
        );
        Fence {
            sys_ptr: fence_ptr,
            virtual_functions: unsafe { (*fence_ptr).pVtbl },
            device_object: DeviceObject::new(fence_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::FenceDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.sys_ptr as *mut diligent_sys::IDeviceObject)
                as *const diligent_sys::FenceDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn get_completed_value(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .Fence
                .GetCompletedValue
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn signal(&self, value: u64) {
        unsafe { (*self.virtual_functions).Fence.Signal.unwrap_unchecked()(self.sys_ptr, value) }
    }

    pub fn wait(&self, value: u64) {
        unsafe { (*self.virtual_functions).Fence.Wait.unwrap_unchecked()(self.sys_ptr, value) }
    }
}
