use std::{
    ffi::CStr,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{
    Boxed, BoxedFromNulError, MapType,
    buffer_view::{BufferView, BufferViewDesc, BufferViewType},
    device_context::DeviceContext,
    device_object::DeviceObject,
    graphics_types::{BindFlags, CpuAccessFlags, MemoryProperty, ResourceState, Usage},
    resource_access_states,
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

//#[derive(Builder, Clone)]
#[repr(transparent)]
pub struct BufferDesc<'name>(pub(crate) diligent_sys::BufferDesc, PhantomData<&'name ()>);

#[bon::bon]
impl<'name> BufferDesc<'name> {
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,

        size: u64,

        bind_flags: BindFlags,

        usage: Usage,

        #[builder(default)] cpu_access_flags: CpuAccessFlags,

        mode: Option<BufferMode>,

        #[builder(default)] misc_flags: MiscBufferFlags,

        #[builder(default = 0)] element_byte_stride: u32,

        #[builder(default = 1)] immediate_context_mask: u64,
    ) -> Self {
        BufferDesc(
            diligent_sys::BufferDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
                },
                Size: size,
                BindFlags: bind_flags.bits(),
                Usage: usage.into(),
                CPUAccessFlags: cpu_access_flags.bits(),
                Mode: mode.map_or(diligent_sys::BUFFER_MODE_UNDEFINED as _, |bm| bm.into()),
                MiscFlags: misc_flags.bits(),
                ElementByteStride: element_byte_stride,
                ImmediateContextMask: immediate_context_mask,
            },
            PhantomData,
        )
    }
}

impl BufferDesc<'_> {
    pub fn size(&self) -> u64 {
        self.0.Size
    }
    pub fn bind_flags(&self) -> BindFlags {
        BindFlags::from_bits_retain(self.0.BindFlags)
    }
    pub fn usage(&self) -> Usage {
        self.0.Usage.into()
    }
    pub fn cpu_access_flags(&self) -> CpuAccessFlags {
        CpuAccessFlags::from_bits_retain(self.0.CPUAccessFlags)
    }
    pub fn mode(&self) -> Option<BufferMode> {
        match self.0.Mode as _ {
            diligent_sys::BUFFER_MODE_FORMATTED => Some(BufferMode::Formatted),
            diligent_sys::BUFFER_MODE_STRUCTURED => Some(BufferMode::Structured),
            diligent_sys::BUFFER_MODE_RAW => Some(BufferMode::Raw),
            _ => None,
        }
    }
    pub fn misc_flags(&self) -> MiscBufferFlags {
        MiscBufferFlags::from_bits_retain(self.0.MiscFlags)
    }
    pub fn element_byte_stride(&self) -> u32 {
        self.0.ElementByteStride
    }
    pub fn immediate_context_mask(&self) -> u64 {
        self.0.ImmediateContextMask
    }
}

define_ported!(
    Buffer,
    diligent_sys::IBuffer,
    diligent_sys::IBufferMethods : 9,
    DeviceObject
);

impl Buffer {
    pub fn desc(&self) -> &BufferDesc<'_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const BufferDesc) }
    }

    pub fn create_view(
        &self,
        view_desc: &BufferViewDesc,
    ) -> Result<Boxed<BufferView>, BoxedFromNulError> {
        let mut buffer_view_ptr = std::ptr::null_mut();

        unsafe_member_call!(self, Buffer, CreateView, &view_desc.0, &mut buffer_view_ptr);

        Boxed::new(buffer_view_ptr)
    }

    pub fn get_default_view(&self, view_type: BufferViewType) -> Option<&BufferView> {
        let buffer_view_ptr = unsafe_member_call!(self, Buffer, GetDefaultView, view_type.into());

        if buffer_view_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(buffer_view_ptr as *const BufferView) })
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

pub struct BufferMapToken<'context, 'buffer, T: Sized, State: MapType> {
    device_context: &'context DeviceContext,
    buffer: &'buffer Buffer,
    data: &'buffer mut [T],
    phantom: PhantomData<State>,
}

impl<'context, 'buffer, T: Sized, State: MapType> BufferMapToken<'context, 'buffer, T, State> {
    pub(super) fn new(
        device_context: &'context DeviceContext,
        buffer: &'buffer Buffer,
        map_flags: diligent_sys::MAP_FLAGS,
    ) -> BufferMapToken<'context, 'buffer, T, State> {
        let mut ptr = std::ptr::null_mut();
        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapBuffer,
            std::ptr::from_ref(&buffer.0) as *mut _,
            State::MAP_TYPE,
            map_flags,
            &mut ptr
        );

        BufferMapToken::<T, State> {
            buffer,
            data: unsafe {
                std::slice::from_raw_parts_mut(
                    ptr as *mut T,
                    buffer.desc().size() as usize / std::mem::size_of::<T>(),
                )
            },
            device_context,
            phantom: PhantomData,
        }
    }
}

impl<T: Sized, State: MapType> Drop for BufferMapToken<'_, '_, T, State> {
    fn drop(&mut self) {
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapBuffer,
            std::ptr::from_ref(&self.buffer.0) as *mut _,
            State::MAP_TYPE
        )
    }
}

impl<T> Deref for BufferMapToken<'_, '_, T, resource_access_states::Read> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

// Note : Normally you shouldn't be able to read from the write token,
//        but DerefMut cannot be implemented without Deref.
impl<T> Deref for BufferMapToken<'_, '_, T, resource_access_states::Write> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T> DerefMut for BufferMapToken<'_, '_, T, resource_access_states::Write> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<T> Deref for BufferMapToken<'_, '_, T, resource_access_states::ReadWrite> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<T> DerefMut for BufferMapToken<'_, '_, T, resource_access_states::ReadWrite> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

pub type BufferMapReadToken<'context, 'buffer, T> =
    BufferMapToken<'context, 'buffer, T, resource_access_states::Read>;
pub type BufferMapWriteToken<'context, 'buffer, T> =
    BufferMapToken<'context, 'buffer, T, resource_access_states::Write>;
pub type BufferMapReadWriteToken<'context, 'buffer, T> =
    BufferMapToken<'context, 'buffer, T, resource_access_states::ReadWrite>;
