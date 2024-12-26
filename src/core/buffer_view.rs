use crate::core::bindings;

use crate::core::buffer::Buffer;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct BufferView {
    m_buffer_view: *mut bindings::IBufferView,
    m_virtual_functions: *mut bindings::IBufferViewVtbl,
    m_buffer: *const Buffer,

    m_device_object: DeviceObject,
}

impl BufferView {
    pub(crate) fn new(buffer_view: *mut bindings::IBufferView, buffer: *const Buffer) -> Self {
        BufferView {
            m_virtual_functions: unsafe { (*buffer_view).pVtbl },
            m_buffer_view: buffer_view,
            m_buffer: buffer,
            m_device_object: DeviceObject::new(buffer_view as *mut bindings::IDeviceObject),
        }
    }
}

impl AsDeviceObject for BufferView {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl BufferView {
    fn get_desc(&self) -> &bindings::BufferViewDesc {
        unsafe {
            ((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.m_buffer_view as *mut bindings::IDeviceObject)
                as *const bindings::BufferViewDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    #[inline]
    fn get_buffer(&self) -> &Buffer {
        unsafe { self.m_buffer.as_ref().unwrap_unchecked() }
    }
}
