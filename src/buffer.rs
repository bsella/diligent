use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert_eq;

use crate::{
    buffer_view::{BufferView, BufferViewDesc, BufferViewType},
    device_context::DeviceContext,
    device_object::DeviceObject,
    graphics_types::{BindFlags, CpuAccessFlags, MemoryProperty, ResourceState, Usage},
};

#[derive(Clone, Copy)]
pub enum BufferMode {
    Formatted,
    Structured,
    Raw,
}
const_assert_eq!(diligent_sys::BUFFER_MODE_NUM_MODES, 4);

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

#[derive(Builder, Clone)]
#[builder(derive(Clone))]
pub struct BufferDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: Option<CString>,

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
                Name: value
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
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

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferMethods>(),
    9 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct Buffer(DeviceObject);

impl Deref for Buffer {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Buffer {
    pub(crate) fn new(buffer_ptr: *mut diligent_sys::IBuffer) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IDeviceObject>(),
            std::mem::size_of::<diligent_sys::IBuffer>()
        );

        Buffer(DeviceObject::new(
            buffer_ptr as *mut diligent_sys::IDeviceObject,
        ))
    }

    pub fn create_view(&self, view_desc: &BufferViewDesc) -> Result<BufferView<'_>, ()> {
        let mut buffer_view_ptr = std::ptr::null_mut();
        let view_desc = view_desc.into();
        unsafe_member_call!(
            self,
            Buffer,
            CreateView,
            std::ptr::from_ref(&view_desc),
            std::ptr::addr_of_mut!(buffer_view_ptr)
        );

        if buffer_view_ptr.is_null() {
            Err(())
        } else {
            Ok(BufferView::new(buffer_view_ptr, self))
        }
    }

    pub fn get_default_view(&self, view_type: BufferViewType) -> Option<BufferView<'_>> {
        let buffer_view_ptr = unsafe_member_call!(self, Buffer, GetDefaultView, view_type.into());

        if buffer_view_ptr.is_null() {
            None
        } else {
            let view = BufferView::new(buffer_view_ptr, self);
            view.add_ref();
            Some(view)
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe_member_call!(self, Buffer, GetNativeHandle)
    }

    pub fn set_state(&mut self, state: ResourceState) {
        unsafe_member_call!(self, Buffer, SetState, state.bits())
    }

    pub fn get_state(&self) -> ResourceState {
        let state = unsafe_member_call!(self, Buffer, GetState);
        ResourceState::from_bits_retain(state)
    }

    pub fn get_memory_properties(&self) -> Option<MemoryProperty> {
        let prop = unsafe_member_call!(self, Buffer, GetMemoryProperties);
        match prop as _ {
            diligent_sys::MEMORY_PROPERTY_UNKNOWN => None,
            diligent_sys::MEMORY_PROPERTY_HOST_COHERENT => Some(MemoryProperty::HostCoherent),
            _ => panic!("Unknown MEMORY_PROPERTY value"),
        }
    }

    pub fn flush_mapped_range(&mut self, start_offset: u64, size: u64) {
        unsafe_member_call!(self, Buffer, FlushMappedRange, start_offset, size)
    }

    pub fn invalidate_mapped_range(&mut self, start_offset: u64, size: u64) {
        unsafe_member_call!(self, Buffer, InvalidateMappedRange, start_offset, size)
    }

    pub fn get_sparse_properties(&self) -> diligent_sys::SparseBufferProperties {
        unsafe_member_call!(self, Buffer, GetSparseProperties)
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
        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapBuffer,
            buffer.sys_ptr as _,
            diligent_sys::MAP_READ as diligent_sys::MAP_TYPE,
            map_flags,
            std::ptr::addr_of_mut!(ptr)
        );

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
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapBuffer,
            self.buffer.sys_ptr as _,
            diligent_sys::MAP_READ as diligent_sys::MAP_TYPE
        )
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
        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapBuffer,
            buffer.sys_ptr as _,
            diligent_sys::MAP_WRITE as diligent_sys::MAP_TYPE,
            map_flags,
            std::ptr::addr_of_mut!(ptr)
        );

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
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapBuffer,
            self.buffer.sys_ptr as _,
            diligent_sys::MAP_WRITE as diligent_sys::MAP_TYPE
        )
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
        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapBuffer,
            buffer.sys_ptr as _,
            diligent_sys::MAP_READ_WRITE as diligent_sys::MAP_TYPE,
            map_flags,
            std::ptr::addr_of_mut!(ptr)
        );

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
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapBuffer,
            self.buffer.sys_ptr as _,
            diligent_sys::MAP_READ_WRITE as diligent_sys::MAP_TYPE
        )
    }
}
