use static_assertions::const_assert;

use super::buffer::Buffer;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct BufferView {
    _sys_ptr: *mut diligent_sys::IBufferView,
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
    pub(crate) fn new(
        buffer_view_ptr: *mut diligent_sys::IBufferView,
        buffer: *const Buffer,
    ) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IBufferView>()
        );

        BufferView {
            virtual_functions: unsafe { (*buffer_view_ptr).pVtbl },
            _sys_ptr: buffer_view_ptr,
            buffer,
            device_object: DeviceObject::new(buffer_view_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::BufferViewDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object.sys_ptr)
                as *const diligent_sys::BufferViewDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    #[inline]
    pub fn get_buffer(&self) -> &Buffer {
        unsafe { self.buffer.as_ref().unwrap_unchecked() }
    }
}
