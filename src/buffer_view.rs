use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use static_assertions::const_assert_eq;

use crate::{
    buffer::Buffer,
    device_object::{DeviceObject, DeviceObjectAttribs},
    graphics_types::ValueType,
};

#[derive(Clone, Copy)]
pub enum BufferViewType {
    ShaderResource,
    UnorderedAccess,
}
const_assert_eq!(diligent_sys::BUFFER_VIEW_NUM_VIEWS, 3);

impl From<BufferViewType> for diligent_sys::BUFFER_VIEW_TYPE {
    fn from(value: BufferViewType) -> Self {
        (match value {
            BufferViewType::ShaderResource => diligent_sys::BUFFER_VIEW_SHADER_RESOURCE,
            BufferViewType::UnorderedAccess => diligent_sys::BUFFER_VIEW_UNORDERED_ACCESS,
        }) as _
    }
}

impl From<diligent_sys::BUFFER_VIEW_TYPE> for BufferViewType {
    fn from(value: diligent_sys::BUFFER_VIEW_TYPE) -> Self {
        match value as _ {
            diligent_sys::BUFFER_VIEW_SHADER_RESOURCE => BufferViewType::ShaderResource,
            diligent_sys::BUFFER_VIEW_UNORDERED_ACCESS => BufferViewType::UnorderedAccess,
            _ => panic!("Unknown BUFFER_VIEW_TYPE value"),
        }
    }
}

#[derive(Clone)]
pub struct BufferFormat(diligent_sys::BufferFormat);

#[bon::bon]
impl BufferFormat {
    #[builder(derive(Clone))]
    pub fn new(
        value_type: Option<ValueType>,
        #[builder(default = 1)] num_components: u8,
        #[builder(default = false)] is_normalized: bool,
    ) -> Self {
        BufferFormat(diligent_sys::BufferFormat {
            ValueType: value_type.map_or(diligent_sys::VT_UNDEFINED as _, |vt| vt.into()),
            NumComponents: num_components,
            IsNormalized: is_normalized,
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct BufferViewDesc<'name>(
    pub(crate) diligent_sys::BufferViewDesc,
    PhantomData<&'name ()>,
);

impl Deref for BufferViewDesc<'_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name> BufferViewDesc<'name> {
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,
        view_type: BufferViewType,
        #[builder(default = BufferFormat::builder().build())] format: BufferFormat,
        #[builder(default = 0)] byte_offset: u64,
        #[builder(default = 0)] byte_width: u64,
    ) -> Self {
        Self(
            diligent_sys::BufferViewDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
                },
                ViewType: view_type.into(),
                Format: format.0,
                ByteOffset: byte_offset,
                ByteWidth: byte_width,
            },
            PhantomData,
        )
    }
}

impl BufferViewDesc<'_> {
    pub fn view_type(&self) -> BufferViewType {
        self.0.ViewType.into()
    }
    pub fn format(&self) -> BufferFormat {
        BufferFormat(self.0.Format)
    }
    pub fn byte_offset(&self) -> u64 {
        self.0.ByteOffset
    }
    pub fn byte_width(&self) -> u64 {
        self.0.ByteWidth
    }
}

define_ported!(
    BufferView,
    diligent_sys::IBufferView,
    diligent_sys::IBufferViewMethods : 1,
    DeviceObject
);

impl BufferView {
    pub fn desc(&self) -> &BufferViewDesc<'_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const BufferViewDesc) }
    }

    pub fn get_buffer(&self) -> &Buffer {
        let buffer_ptr = unsafe_member_call!(self, BufferView, GetBuffer);

        unsafe { &*(buffer_ptr as *const Buffer) }
    }

    pub fn get_buffer_mut(&mut self) -> &mut Buffer {
        let buffer_ptr = unsafe_member_call!(self, BufferView, GetBuffer);

        unsafe { &mut *(buffer_ptr as *mut Buffer) }
    }
}

// # Safety : Access to BufferView can be thread safe
unsafe impl Sync for BufferView {}
