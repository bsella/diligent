use crate::core::{
    buffer_view::BufferView,
    device_object::{AsDeviceObject, DeviceObject},
};

pub struct BufferViewVk<'a> {
    buffer_view_ptr: *mut diligent_sys::IBufferViewVk,
    virtual_functions: *mut diligent_sys::IBufferViewVkVtbl,

    buffer_view: &'a BufferView,
}

impl AsDeviceObject for BufferViewVk<'_> {
    fn as_device_object(&self) -> &DeviceObject {
        &self.buffer_view.as_device_object()
    }
}

impl<'a> From<&'a BufferView> for BufferViewVk<'a> {
    fn from(value: &'a BufferView) -> Self {
        BufferViewVk {
            buffer_view: value,
            buffer_view_ptr: value.buffer_view as *mut diligent_sys::IBufferViewVk,
            virtual_functions: unsafe {
                (*(value.buffer_view as *mut diligent_sys::IBufferViewVk)).pVtbl
            },
        }
    }
}

impl BufferViewVk<'_> {
    pub fn get_vk_buffer_view(&self) -> diligent_sys::VkBufferView {
        unsafe {
            (*self.virtual_functions)
                .BufferViewVk
                .GetVkBufferView
                .unwrap_unchecked()(self.buffer_view_ptr)
        }
    }
}
