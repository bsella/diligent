use crate::bindings;

use super::buffer_view::BufferView;

use super::{
    device_object::{AsDeviceObject, DeviceObject},
    object::AsObject,
};

pub struct Buffer {
    pub(crate) buffer: *mut bindings::IBuffer,
    virtual_functions: *mut bindings::IBufferVtbl,

    default_view: Option<BufferView>,

    device_object: DeviceObject,
}

impl AsDeviceObject for Buffer {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Buffer {
    pub(crate) fn new(
        buffer_ptr: *mut bindings::IBuffer,
        buffer_desc: &bindings::BufferDesc,
    ) -> Self {
        let mut buffer = Buffer {
            device_object: DeviceObject::new(buffer_ptr as *mut bindings::IDeviceObject),
            buffer: buffer_ptr,
            virtual_functions: unsafe { (*buffer_ptr).pVtbl },
            default_view: None,
        };

        fn bind_flags_to_buffer_view_type(
            bind_flag: bindings::BIND_FLAGS,
        ) -> bindings::BUFFER_VIEW_TYPE {
            if bind_flag & bindings::BIND_UNORDERED_ACCESS != 0 {
                bindings::BUFFER_VIEW_UNORDERED_ACCESS as u8
            } else if bind_flag & bindings::BIND_SHADER_RESOURCE != 0 {
                bindings::BUFFER_VIEW_SHADER_RESOURCE as u8
            } else {
                bindings::BUFFER_VIEW_UNDEFINED as u8
            }
        }

        let buffer_view_type = bind_flags_to_buffer_view_type(buffer_desc.BindFlags);

        if buffer_view_type != (bindings::BUFFER_VIEW_UNDEFINED as u8) {
            let buffer_view = BufferView::new(
                unsafe {
                    (*(*buffer_ptr).pVtbl)
                        .Buffer
                        .GetDefaultView
                        .unwrap_unchecked()(buffer_ptr, buffer_view_type)
                },
                std::ptr::addr_of!(buffer),
            );
            buffer_view.as_device_object().as_object().add_ref();
            buffer.default_view = Some(buffer_view);
        }

        buffer
    }

    pub fn get_desc(&self) -> &bindings::BufferDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.buffer as *mut bindings::IDeviceObject)
                as *const bindings::BufferDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn create_view(&mut self, view_desc: bindings::BufferViewDesc) -> Option<BufferView> {
        let mut buffer_view_ptr: *mut bindings::IBufferView = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .CreateView
                .unwrap_unchecked()(
                self.buffer,
                &view_desc,
                std::ptr::addr_of_mut!(buffer_view_ptr),
            );
        }
        if buffer_view_ptr.is_null() {
            None
        } else {
            Some(BufferView::new(buffer_view_ptr, self as *mut Self))
        }
    }

    pub fn get_default_view(&self, view_type: bindings::BUFFER_VIEW_TYPE) -> Option<&BufferView> {
        if unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetDefaultView
                .unwrap_unchecked()(self.buffer, view_type)
        }
        .is_null()
        {
            None
        } else {
            self.default_view.as_ref()
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetNativeHandle
                .unwrap_unchecked()(self.buffer)
        }
    }

    pub fn set_state(&mut self, state: bindings::RESOURCE_STATE) {
        unsafe { (*self.virtual_functions).Buffer.SetState.unwrap_unchecked()(self.buffer, state) }
    }

    pub fn get_state(&self) -> bindings::RESOURCE_STATE {
        unsafe { (*self.virtual_functions).Buffer.GetState.unwrap_unchecked()(self.buffer) }
    }

    pub fn get_memory_properties(&self) -> bindings::MEMORY_PROPERTIES {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetMemoryProperties
                .unwrap_unchecked()(self.buffer)
        }
    }

    pub fn flush_mapped_range(&mut self, start_offset: u64, size: u64) {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .FlushMappedRange
                .unwrap_unchecked()(self.buffer, start_offset, size)
        }
    }

    pub fn invalidate_mapped_range(&mut self, start_offset: u64, size: u64) {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .InvalidateMappedRange
                .unwrap_unchecked()(self.buffer, start_offset, size)
        }
    }

    pub fn get_sparse_properties(&self) -> bindings::SparseBufferProperties {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetSparseProperties
                .unwrap_unchecked()(self.buffer)
        }
    }
}
