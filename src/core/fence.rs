use super::device_object::{AsDeviceObject, DeviceObject};

pub struct Fence {
    pub(crate) fence: *mut diligent_sys::IFence,
    virtual_functions: *mut diligent_sys::IFenceVtbl,

    device_object: DeviceObject,
}

impl AsDeviceObject for Fence {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Fence {
    pub(crate) fn new(fence_ptr: *mut diligent_sys::IFence) -> Self {
        Fence {
            fence: fence_ptr,
            virtual_functions: unsafe { (*fence_ptr).pVtbl },
            device_object: DeviceObject::new(fence_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::FenceDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.fence as *mut diligent_sys::IDeviceObject)
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
                .unwrap_unchecked()(self.fence)
        }
    }

    pub fn signal(&self, value: u64) {
        unsafe { (*self.virtual_functions).Fence.Signal.unwrap_unchecked()(self.fence, value) }
    }

    pub fn wait(&self, value: u64) {
        unsafe { (*self.virtual_functions).Fence.Wait.unwrap_unchecked()(self.fence, value) }
    }
}
