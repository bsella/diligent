use std::ffi::CString;

use bitflags::bitflags;
use static_assertions::const_assert;

use super::buffer_view::BufferView;

use super::device_context::DeviceContext;
use super::graphics_types::{BindFlags, CpuAccessFlags, Usage};
use super::{
    device_object::{AsDeviceObject, DeviceObject},
    object::AsObject,
};

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
    pub fn new(name: &'a str, size: u64) -> Self {
        BufferDesc {
            size,
            name: CString::new(name).unwrap(),
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
    pub(crate) buffer: *mut diligent_sys::IBuffer,
    virtual_functions: *mut diligent_sys::IBufferVtbl,

    default_view: Option<BufferView>,

    device_object: DeviceObject,
}

impl AsDeviceObject for Buffer {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Buffer {
    pub(crate) fn new(buffer_ptr: *mut diligent_sys::IBuffer) -> Self {
        let mut buffer = Buffer {
            device_object: DeviceObject::new(buffer_ptr as *mut diligent_sys::IDeviceObject),
            buffer: buffer_ptr,
            virtual_functions: unsafe { (*buffer_ptr).pVtbl },
            default_view: None,
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
                std::ptr::addr_of!(buffer),
            );
            buffer_view.as_device_object().as_object().add_ref();
            buffer.default_view = Some(buffer_view);
        }

        buffer
    }

    pub fn get_desc(&self) -> &diligent_sys::BufferDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.buffer as *mut diligent_sys::IDeviceObject)
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
                self.buffer,
                view_desc,
                std::ptr::addr_of_mut!(buffer_view_ptr),
            );
        }
        if buffer_view_ptr.is_null() {
            None
        } else {
            Some(BufferView::new(buffer_view_ptr, self as *mut Self))
        }
    }

    pub fn get_default_view(
        &self,
        view_type: diligent_sys::BUFFER_VIEW_TYPE,
    ) -> Option<&BufferView> {
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

    pub fn set_state(&mut self, state: diligent_sys::RESOURCE_STATE) {
        unsafe { (*self.virtual_functions).Buffer.SetState.unwrap_unchecked()(self.buffer, state) }
    }

    pub fn get_state(&self) -> diligent_sys::RESOURCE_STATE {
        unsafe { (*self.virtual_functions).Buffer.GetState.unwrap_unchecked()(self.buffer) }
    }

    pub fn get_memory_properties(&self) -> diligent_sys::MEMORY_PROPERTIES {
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

    pub fn get_sparse_properties(&self) -> diligent_sys::SparseBufferProperties {
        unsafe {
            (*self.virtual_functions)
                .Buffer
                .GetSparseProperties
                .unwrap_unchecked()(self.buffer)
        }
    }
}

pub struct BufferMapReadToken<'a, T, Context: DeviceContext> {
    device_context: &'a Context,
    buffer: &'a Buffer,
    data_ptr: *const T,
}

impl<'a, T, Context> BufferMapReadToken<'a, T, Context>
where
    Context: DeviceContext,
{
    pub(super) fn new(
        device_context: &'a Context,
        buffer: &'a Buffer,
        map_flags: diligent_sys::MAP_FLAGS,
    ) -> BufferMapReadToken<'a, T, Context> {
        let mut ptr = std::ptr::null_mut() as *mut std::os::raw::c_void;
        unsafe {
            (*device_context.as_device_context().virtual_functions)
                .DeviceContext
                .MapBuffer
                .unwrap_unchecked()(
                device_context.as_device_context().device_context_ptr,
                buffer.buffer,
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
        self.data_ptr.as_ref().unwrap_unchecked()
    }

    pub unsafe fn as_slice(&self, len: usize, offset: isize) -> &[T] {
        std::slice::from_raw_parts(self.data_ptr.offset(offset), len)
    }
}

impl<T, Context> Drop for BufferMapReadToken<'_, T, Context>
where
    Context: DeviceContext,
{
    fn drop(&mut self) {
        unsafe {
            (*self.device_context.as_device_context().virtual_functions)
                .DeviceContext
                .UnmapBuffer
                .unwrap_unchecked()(
                self.device_context.as_device_context().device_context_ptr,
                self.buffer.buffer,
                diligent_sys::MAP_READ as diligent_sys::MAP_TYPE,
            )
        }
    }
}

pub struct BufferMapWriteToken<'a, T, Context: DeviceContext> {
    device_context: &'a Context,
    buffer: &'a Buffer,
    data_ptr: *mut T,
}

impl<'a, T, Context> BufferMapWriteToken<'a, T, Context>
where
    Context: DeviceContext,
{
    pub(super) fn new(
        device_context: &'a Context,
        buffer: &'a Buffer,
        map_flags: diligent_sys::MAP_FLAGS,
    ) -> BufferMapWriteToken<'a, T, Context> {
        let mut ptr = std::ptr::null_mut() as *mut std::os::raw::c_void;
        unsafe {
            (*device_context.as_device_context().virtual_functions)
                .DeviceContext
                .MapBuffer
                .unwrap_unchecked()(
                device_context.as_device_context().device_context_ptr,
                buffer.buffer,
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
        self.data_ptr.as_mut().unwrap_unchecked()
    }

    pub unsafe fn as_mut_slice(&mut self, len: usize, offset: isize) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.data_ptr.offset(offset), len)
    }
}

impl<T, Context> Drop for BufferMapWriteToken<'_, T, Context>
where
    Context: DeviceContext,
{
    fn drop(&mut self) {
        unsafe {
            (*self.device_context.as_device_context().virtual_functions)
                .DeviceContext
                .UnmapBuffer
                .unwrap_unchecked()(
                self.device_context.as_device_context().device_context_ptr,
                self.buffer.buffer,
                diligent_sys::MAP_WRITE as diligent_sys::MAP_TYPE,
            )
        }
    }
}
pub struct BufferMapReadWriteToken<'a, T, Context: DeviceContext> {
    device_context: &'a Context,
    buffer: &'a Buffer,
    data_ptr: *mut T,
}
impl<'a, T, Context> BufferMapReadWriteToken<'a, T, Context>
where
    Context: DeviceContext,
{
    pub(super) fn new(
        device_context: &'a Context,
        buffer: &'a Buffer,
        map_flags: diligent_sys::MAP_FLAGS,
    ) -> BufferMapReadWriteToken<'a, T, Context> {
        let mut ptr = std::ptr::null_mut() as *mut std::os::raw::c_void;
        unsafe {
            (*device_context.as_device_context().virtual_functions)
                .DeviceContext
                .MapBuffer
                .unwrap_unchecked()(
                device_context.as_device_context().device_context_ptr,
                buffer.buffer,
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
        self.data_ptr.as_ref().unwrap_unchecked()
    }

    pub unsafe fn as_slice(&self, len: usize, offset: isize) -> &[T] {
        std::slice::from_raw_parts(self.data_ptr.offset(offset), len)
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        self.data_ptr.as_mut().unwrap_unchecked()
    }

    pub unsafe fn as_mut_slice(&mut self, len: usize, offset: isize) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.data_ptr.offset(offset), len)
    }
}
impl<T, Context> Drop for BufferMapReadWriteToken<'_, T, Context>
where
    Context: DeviceContext,
{
    fn drop(&mut self) {
        unsafe {
            (*self.device_context.as_device_context().virtual_functions)
                .DeviceContext
                .UnmapBuffer
                .unwrap_unchecked()(
                self.device_context.as_device_context().device_context_ptr,
                self.buffer.buffer,
                diligent_sys::MAP_READ_WRITE as diligent_sys::MAP_TYPE,
            )
        }
    }
}
