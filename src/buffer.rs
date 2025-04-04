use std::ffi::CString;

use bitflags::bitflags;
use static_assertions::const_assert;

use crate::device_context::DeviceContext;
use crate::graphics_types::ResourceState;

use super::buffer_view::BufferView;

use super::device_object::DeviceObject;
use super::graphics_types::{BindFlags, CpuAccessFlags, Usage};

#[derive(Clone, Copy)]
pub enum BufferMode {
    Undefined,
    Formatted,
    Structured,
    Raw,
}
const_assert!(diligent_sys::BUFFER_MODE_NUM_MODES == 4);

impl From<&BufferMode> for diligent_sys::BUFFER_MODE {
    fn from(value: &BufferMode) -> Self {
        (match value {
            BufferMode::Undefined => diligent_sys::BUFFER_MODE_UNDEFINED,
            BufferMode::Formatted => diligent_sys::BUFFER_MODE_FORMATTED,
            BufferMode::Structured => diligent_sys::BUFFER_MODE_STRUCTURED,
            BufferMode::Raw => diligent_sys::BUFFER_MODE_RAW,
        }) as diligent_sys::BUFFER_MODE
    }
}

bitflags! {
    pub struct MiscBufferFlags: diligent_sys::MISC_BUFFER_FLAGS {
        const None            = diligent_sys::MISC_BUFFER_FLAG_NONE as diligent_sys::MISC_BUFFER_FLAGS;
        const SparceAliasing  = diligent_sys::MISC_BUFFER_FLAG_SPARSE_ALIASING as diligent_sys::MISC_BUFFER_FLAGS;
    }
}

pub struct BufferDesc {
    name: CString,

    size: u64,

    bind_flags: BindFlags,
    usage: Usage,
    cpu_access_flags: CpuAccessFlags,
    mode: BufferMode,
    misc_flags: MiscBufferFlags,
    element_byte_stride: u32,
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
            Usage: diligent_sys::USAGE::from(&value.usage),
            CPUAccessFlags: value.cpu_access_flags.bits(),
            Mode: diligent_sys::BUFFER_MODE::from(&value.mode),
            MiscFlags: value.misc_flags.bits(),
            ElementByteStride: value.element_byte_stride,
            ImmediateContextMask: value.immediate_context_mask,
        }
    }
}

impl<'a> BufferDesc {
    pub fn new(name: impl AsRef<str>, size: u64) -> Self {
        BufferDesc {
            size,
            name: CString::new(name.as_ref()).unwrap(),
            bind_flags: BindFlags::None,
            usage: Usage::Default,
            cpu_access_flags: CpuAccessFlags::None,
            mode: BufferMode::Undefined,
            misc_flags: MiscBufferFlags::None,

            element_byte_stride: 0,
            immediate_context_mask: 1,
        }
    }

    pub fn bind_flags(mut self, bind_flags: BindFlags) -> Self {
        self.bind_flags = bind_flags;
        self
    }
    pub fn usage(mut self, usage: Usage) -> Self {
        self.usage = usage;
        self
    }
    pub fn cpu_access_flags(mut self, cpu_access_flags: CpuAccessFlags) -> Self {
        self.cpu_access_flags = cpu_access_flags;
        self
    }
    pub fn mode(mut self, mode: BufferMode) -> Self {
        self.mode = mode;
        self
    }
    pub fn misc_flags(mut self, misc_flags: MiscBufferFlags) -> Self {
        self.misc_flags = misc_flags;
        self
    }
    pub fn element_byte_stride(mut self, element_byte_stride: u32) -> Self {
        self.element_byte_stride = element_byte_stride;
        self
    }
    pub fn immediate_context_mask(mut self, immediate_context_mask: u64) -> Self {
        self.immediate_context_mask = immediate_context_mask;
        self
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

    pub fn create_view(&mut self, view_desc: &diligent_sys::BufferViewDesc) -> Option<BufferView> {
        let mut buffer_view_ptr: *mut diligent_sys::IBufferView = std::ptr::null_mut();
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
            None
        } else {
            Some(BufferView::new(buffer_view_ptr, self))
        }
    }

    pub fn get_default_view(
        &self,
        view_type: diligent_sys::BUFFER_VIEW_TYPE,
    ) -> Option<BufferView> {
        let buffer_view_ptr = unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetDefaultView
                .unwrap_unchecked()(self.sys_ptr, view_type)
        };

        if buffer_view_ptr.is_null() {
            None
        } else {
            Some(BufferView::new(buffer_view_ptr, self))
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
