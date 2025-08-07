use std::ffi::CString;

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert;

use crate::{
    buffer_view::{BufferView, BufferViewType},
    device_context::DeviceContext,
    device_object::DeviceObject,
    graphics_types::{BindFlags, CpuAccessFlags, ResourceState, Usage},
};

#[derive(Clone, Copy)]
pub enum BufferMode {
    Formatted,
    Structured,
    Raw,
}
const_assert!(diligent_sys::BUFFER_MODE_NUM_MODES == 4);

impl From<BufferMode> for diligent_sys::BUFFER_MODE {
    fn from(value: BufferMode) -> Self {
        (match value {
            BufferMode::Formatted => diligent_sys::BUFFER_MODE_FORMATTED,
            BufferMode::Structured => diligent_sys::BUFFER_MODE_STRUCTURED,
            BufferMode::Raw => diligent_sys::BUFFER_MODE_RAW,
        }) as _
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct MiscBufferFlags: diligent_sys::MISC_BUFFER_FLAGS {
        const None            = diligent_sys::MISC_BUFFER_FLAG_NONE as diligent_sys::MISC_BUFFER_FLAGS;
        const SparceAliasing  = diligent_sys::MISC_BUFFER_FLAG_SPARSE_ALIASING as diligent_sys::MISC_BUFFER_FLAGS;
    }
}

impl Default for MiscBufferFlags {
    fn default() -> Self {
        MiscBufferFlags::None
    }
}

#[derive(Builder)]
#[builder(derive(Clone))]
pub struct BufferDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

    size: u64,

    bind_flags: BindFlags,

    usage: Usage,

    #[builder(default)]
    cpu_access_flags: CpuAccessFlags,

    mode: Option<BufferMode>,

    #[builder(default)]
    misc_flags: MiscBufferFlags,

    #[builder(default = 0)]
    element_byte_stride: u32,

    #[builder(default = 1)]
    immediate_context_mask: u64,
}

impl From<&BufferDesc> for diligent_sys::BufferDesc {
    fn from(value: &BufferDesc) -> Self {
        diligent_sys::BufferDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            Size: value.size,
            BindFlags: value.bind_flags.bits(),
            Usage: value.usage.into(),
            CPUAccessFlags: value.cpu_access_flags.bits(),
            Mode: value
                .mode
                .map_or(diligent_sys::BUFFER_MODE_UNDEFINED as _, |bm| bm.into()),
            MiscFlags: value.misc_flags.bits(),
            ElementByteStride: value.element_byte_stride,
            ImmediateContextMask: value.immediate_context_mask,
        }
    }
}

pub struct Buffer {
    pub(crate) sys_ptr: *mut diligent_sys::IBuffer,
    virtual_functions: *mut diligent_sys::IBufferVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for Buffer {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Buffer {
    pub(crate) fn new(buffer_ptr: *mut diligent_sys::IBuffer) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IBuffer>()
        );

        let buffer = Buffer {
            device_object: DeviceObject::new(buffer_ptr as *mut diligent_sys::IDeviceObject),
            sys_ptr: buffer_ptr,
            virtual_functions: unsafe { (*buffer_ptr).pVtbl },
        };

        fn bind_flags_to_buffer_view_type(
            bind_flags: diligent_sys::BIND_FLAGS,
        ) -> diligent_sys::BUFFER_VIEW_TYPE {
            if bind_flags & diligent_sys::BIND_UNORDERED_ACCESS as diligent_sys::BIND_FLAGS != 0 {
                diligent_sys::BUFFER_VIEW_UNORDERED_ACCESS as diligent_sys::BUFFER_VIEW_TYPE
            } else if bind_flags & diligent_sys::BIND_SHADER_RESOURCE as diligent_sys::BIND_FLAGS
                != 0
            {
                diligent_sys::BUFFER_VIEW_SHADER_RESOURCE as diligent_sys::BUFFER_VIEW_TYPE
            } else {
                diligent_sys::BUFFER_VIEW_UNDEFINED as diligent_sys::BUFFER_VIEW_TYPE
            }
        }

        let buffer_desc = unsafe {
            &*((*(*buffer_ptr).pVtbl)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(buffer_ptr as *mut diligent_sys::IDeviceObject)
                as *const diligent_sys::BufferDesc)
        };

        let buffer_view_type = bind_flags_to_buffer_view_type(buffer_desc.BindFlags);

        if buffer_view_type != (diligent_sys::BUFFER_VIEW_UNDEFINED as u8) {
            let buffer_view = BufferView::new(
                unsafe {
                    (*(*buffer_ptr).pVtbl)
                        .Buffer
                        .GetDefaultView
                        .unwrap_unchecked()(buffer_ptr, buffer_view_type)
                },
                &buffer,
            );
            buffer_view.as_ref().as_ref().add_ref();
        }

        buffer
    }

    pub fn get_desc(&self) -> &diligent_sys::BufferDesc {
        // TODO
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object.sys_ptr)
                as *const diligent_sys::BufferDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn create_view(
        &self,
        view_desc: &diligent_sys::BufferViewDesc,
    ) -> Result<BufferView<'_>, ()> {
        let mut buffer_view_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .CreateView
                .unwrap_unchecked()(
                self.sys_ptr,
                view_desc,
                std::ptr::addr_of_mut!(buffer_view_ptr),
            );
        }
        if buffer_view_ptr.is_null() {
            Err(())
        } else {
            Ok(BufferView::new(buffer_view_ptr, self))
        }
    }

    pub fn get_default_view(&self, view_type: BufferViewType) -> Result<BufferView<'_>, ()> {
        let buffer_view_ptr = unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetDefaultView
                .unwrap_unchecked()(self.sys_ptr, view_type.into())
        };

        if buffer_view_ptr.is_null() {
            Err(())
        } else {
            Ok(BufferView::new(buffer_view_ptr, self))
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetNativeHandle
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn set_state(&mut self, state: ResourceState) {
        unsafe {
            (*self.virtual_functions).Buffer.SetState.unwrap_unchecked()(self.sys_ptr, state.bits())
        };
    }

    pub fn get_state(&self) -> ResourceState {
        let state =
            unsafe { (*self.virtual_functions).Buffer.GetState.unwrap_unchecked()(self.sys_ptr) };
        ResourceState::from_bits_retain(state)
    }

    pub fn get_memory_properties(&self) -> diligent_sys::MEMORY_PROPERTIES {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetMemoryProperties
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn flush_mapped_range(&mut self, start_offset: u64, size: u64) {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .FlushMappedRange
                .unwrap_unchecked()(self.sys_ptr, start_offset, size)
        }
    }

    pub fn invalidate_mapped_range(&mut self, start_offset: u64, size: u64) {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .InvalidateMappedRange
                .unwrap_unchecked()(self.sys_ptr, start_offset, size)
        }
    }

    pub fn get_sparse_properties(&self) -> diligent_sys::SparseBufferProperties {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetSparseProperties
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}

pub struct BufferMapReadToken<'a, T> {
    device_context: &'a DeviceContext,
    buffer: &'a Buffer,
    data_ptr: *const T,
}

impl<'a, T> BufferMapReadToken<'a, T> {
    pub(super) fn new(
        device_context: &'a DeviceContext,
        buffer: &'a Buffer,
        map_flags: diligent_sys::MAP_FLAGS,
    ) -> BufferMapReadToken<'a, T> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            (*device_context.virtual_functions)
                .DeviceContext
                .MapBuffer
                .unwrap_unchecked()(
                device_context.sys_ptr,
                buffer.sys_ptr,
                diligent_sys::MAP_READ as diligent_sys::MAP_TYPE,
                map_flags,
                std::ptr::addr_of_mut!(ptr),
            );
        };

        BufferMapReadToken {
            buffer,
            data_ptr: ptr as *const T,
            device_context,
        }
    }

    pub unsafe fn as_ref(&self) -> &T {
        unsafe { self.data_ptr.as_ref().unwrap_unchecked() }
    }

    pub unsafe fn as_slice(&self, len: usize, offset: isize) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data_ptr.offset(offset), len) }
    }
}

impl<T> Drop for BufferMapReadToken<'_, T> {
    fn drop(&mut self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .UnmapBuffer
                .unwrap_unchecked()(
                self.device_context.sys_ptr,
                self.buffer.sys_ptr,
                diligent_sys::MAP_READ as diligent_sys::MAP_TYPE,
            )
        }
    }
}

pub struct BufferMapWriteToken<'a, T> {
    device_context: &'a DeviceContext,
    buffer: &'a Buffer,
    data_ptr: *mut T,
}

impl<'a, T> BufferMapWriteToken<'a, T> {
    pub(super) fn new(
        device_context: &'a DeviceContext,
        buffer: &'a Buffer,
        map_flags: diligent_sys::MAP_FLAGS,
    ) -> BufferMapWriteToken<'a, T> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            (*device_context.virtual_functions)
                .DeviceContext
                .MapBuffer
                .unwrap_unchecked()(
                device_context.sys_ptr,
                buffer.sys_ptr,
                diligent_sys::MAP_WRITE as diligent_sys::MAP_TYPE,
                map_flags,
                std::ptr::addr_of_mut!(ptr),
            );
        };

        BufferMapWriteToken {
            buffer,
            data_ptr: ptr as *mut T,
            device_context,
        }
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        unsafe { self.data_ptr.as_mut().unwrap_unchecked() }
    }

    pub unsafe fn as_mut_slice(&mut self, len: usize, offset: isize) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data_ptr.offset(offset), len) }
    }
}

impl<T> Drop for BufferMapWriteToken<'_, T> {
    fn drop(&mut self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .UnmapBuffer
                .unwrap_unchecked()(
                self.device_context.sys_ptr,
                self.buffer.sys_ptr,
                diligent_sys::MAP_WRITE as diligent_sys::MAP_TYPE,
            )
        }
    }
}

pub struct BufferMapReadWriteToken<'a, T> {
    device_context: &'a DeviceContext,
    buffer: &'a Buffer,
    data_ptr: *mut T,
}

impl<'a, T> BufferMapReadWriteToken<'a, T> {
    pub(super) fn new(
        device_context: &'a DeviceContext,
        buffer: &'a Buffer,
        map_flags: diligent_sys::MAP_FLAGS,
    ) -> BufferMapReadWriteToken<'a, T> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            (*device_context.virtual_functions)
                .DeviceContext
                .MapBuffer
                .unwrap_unchecked()(
                device_context.sys_ptr,
                buffer.sys_ptr,
                diligent_sys::MAP_READ_WRITE as diligent_sys::MAP_TYPE,
                map_flags,
                std::ptr::addr_of_mut!(ptr),
            );
        };

        BufferMapReadWriteToken {
            buffer,
            data_ptr: ptr as *mut T,
            device_context,
        }
    }

    pub unsafe fn as_ref(&self) -> &T {
        unsafe { self.data_ptr.as_ref().unwrap_unchecked() }
    }

    pub unsafe fn as_slice(&self, len: usize, offset: isize) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data_ptr.offset(offset), len) }
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        unsafe { self.data_ptr.as_mut().unwrap_unchecked() }
    }

    pub unsafe fn as_mut_slice(&mut self, len: usize, offset: isize) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data_ptr.offset(offset), len) }
    }
}
impl<T> Drop for BufferMapReadWriteToken<'_, T> {
    fn drop(&mut self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .UnmapBuffer
                .unwrap_unchecked()(
                self.device_context.sys_ptr,
                self.buffer.sys_ptr,
                diligent_sys::MAP_READ_WRITE as diligent_sys::MAP_TYPE,
            )
        }
    }
}
