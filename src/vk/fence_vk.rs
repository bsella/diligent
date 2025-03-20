use crate::{device_object::DeviceObject, fence::Fence};

pub struct FenceVk<'a> {
    fence_ptr: *mut diligent_sys::IFenceVk,
    virtual_functions: *mut diligent_sys::IFenceVkVtbl,

    fence: &'a Fence,
}

impl AsRef<DeviceObject> for FenceVk<'_> {
    fn as_ref(&self) -> &DeviceObject {
        self.fence.as_ref()
    }
}

impl<'a> From<&'a Fence> for FenceVk<'a> {
    fn from(value: &'a Fence) -> Self {
        FenceVk {
            fence: value,
            fence_ptr: value.sys_ptr as *mut diligent_sys::IFenceVk,
            virtual_functions: unsafe { (*(value.sys_ptr as *mut diligent_sys::IFenceVk)).pVtbl },
        }
    }
}

impl FenceVk<'_> {
    pub fn get_vk_semaphore(&self) -> diligent_sys::VkSemaphore {
        unsafe {
            (*self.virtual_functions)
                .FenceVk
                .GetVkSemaphore
                .unwrap_unchecked()(self.fence_ptr)
        }
    }
}
