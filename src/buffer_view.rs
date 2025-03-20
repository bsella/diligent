use static_assertions::const_assert;

use super::buffer::Buffer;

use super::device_object::DeviceObject;

pub struct BufferView<'a> {
    // sys_ptr is used in the interop feature but we get a warning when the interop feature is not exabled
    #[allow(dead_code)]
    pub(crate) sys_ptr: *mut diligent_sys::IBufferView,
    virtual_functions: *mut diligent_sys::IBufferViewVtbl,
    buffer: &'a Buffer,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for BufferView<'_> {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl<'a> BufferView<'a> {
    pub(crate) fn new(buffer_view_ptr: *mut diligent_sys::IBufferView, buffer: &'a Buffer) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IBufferView>()
        );

        BufferView {
            sys_ptr: buffer_view_ptr,
            virtual_functions: unsafe { (*buffer_view_ptr).pVtbl },
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
        self.buffer
    }
}
