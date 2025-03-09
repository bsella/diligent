use super::buffer::Buffer;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct BufferView {
    pub(crate) buffer_view: *mut diligent_sys::IBufferView,
    virtual_functions: *mut diligent_sys::IBufferViewVtbl,
    buffer: *const Buffer,

    device_object: DeviceObject,
}

impl AsDeviceObject for BufferView {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl BufferView {
    pub(crate) fn new(buffer_view: *mut diligent_sys::IBufferView, buffer: *const Buffer) -> Self {
        BufferView {
            virtual_functions: unsafe { (*buffer_view).pVtbl },
            buffer_view: buffer_view,
            buffer: buffer,
            device_object: DeviceObject::new(buffer_view as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::BufferViewDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.buffer_view as *mut diligent_sys::IDeviceObject
            ) as *const diligent_sys::BufferViewDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    #[inline]
    pub fn get_buffer(&self) -> &Buffer {
        unsafe { self.buffer.as_ref().unwrap_unchecked() }
    }
}
