use crate::bindings;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct Fence {
    pub(crate) m_fence: *mut bindings::IFence,
    m_virtual_functions: *mut bindings::IFenceVtbl,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for Fence {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl Fence {
    pub(crate) fn new(fence_ptr: *mut bindings::IFence) -> Self {
        Fence {
            m_fence: fence_ptr,
            m_virtual_functions: unsafe { (*fence_ptr).pVtbl },
            m_device_object: DeviceObject::new(fence_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> &bindings::FenceDesc {
        unsafe {
            ((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.m_fence as *mut bindings::IDeviceObject)
                as *const bindings::FenceDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }
    fn get_completed_value(&self) -> u64 {
        unsafe {
            (*self.m_virtual_functions)
                .Fence
                .GetCompletedValue
                .unwrap_unchecked()(self.m_fence)
        }
    }
    fn signal(&self, value: u64) {
        unsafe { (*self.m_virtual_functions).Fence.Signal.unwrap_unchecked()(self.m_fence, value) }
    }
    fn wait(&self, value: u64) {
        unsafe { (*self.m_virtual_functions).Fence.Wait.unwrap_unchecked()(self.m_fence, value) }
    }
}
