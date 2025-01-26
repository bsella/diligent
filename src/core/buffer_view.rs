use crate::bindings;

use super::buffer::Buffer;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct BufferView {
    buffer_view: *mut bindings::IBufferView,
    virtual_functions: *mut bindings::IBufferViewVtbl,
    buffer: *const Buffer,

    device_object: DeviceObject,
}

impl AsDeviceObject for BufferView {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl BufferView {
    pub(crate) fn new(buffer_view: *mut bindings::IBufferView, buffer: *const Buffer) -> Self {
        BufferView {
            virtual_functions: unsafe { (*buffer_view).pVtbl },
            buffer_view: buffer_view,
            buffer: buffer,
            device_object: DeviceObject::new(buffer_view as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> &bindings::BufferViewDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.buffer_view as *mut bindings::IDeviceObject)
                as *const bindings::BufferViewDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    #[inline]
    fn get_buffer(&self) -> &Buffer {
        unsafe { self.buffer.as_ref().unwrap_unchecked() }
    }
}
