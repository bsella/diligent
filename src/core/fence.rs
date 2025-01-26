use crate::bindings;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct Fence {
    pub(crate) fence: *mut bindings::IFence,
    virtual_functions: *mut bindings::IFenceVtbl,

    device_object: DeviceObject,
}

impl AsDeviceObject for Fence {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Fence {
    pub(crate) fn new(fence_ptr: *mut bindings::IFence) -> Self {
        Fence {
            fence: fence_ptr,
            virtual_functions: unsafe { (*fence_ptr).pVtbl },
            device_object: DeviceObject::new(fence_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> &bindings::FenceDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.fence as *mut bindings::IDeviceObject)
                as *const bindings::FenceDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }
    fn get_completed_value(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .Fence
                .GetCompletedValue
                .unwrap_unchecked()(self.fence)
        }
    }
    fn signal(&self, value: u64) {
        unsafe { (*self.virtual_functions).Fence.Signal.unwrap_unchecked()(self.fence, value) }
    }
    fn wait(&self, value: u64) {
        unsafe { (*self.virtual_functions).Fence.Wait.unwrap_unchecked()(self.fence, value) }
    }
}
