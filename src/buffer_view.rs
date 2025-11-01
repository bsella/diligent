use std::{ffi::CString, marker::PhantomData, ops::Deref};

use bon::Builder;
use static_assertions::const_assert_eq;

use crate::{buffer::Buffer, device_object::DeviceObject, graphics_types::ValueType};

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

#[derive(Builder)]
pub struct BufferFormat {
    value_type: Option<ValueType>,
    num_components: u8,
    #[builder(default = false)]
    is_normalized: bool,
}

#[repr(transparent)]
pub struct BufferViewDesc(diligent_sys::BufferViewDesc);

#[bon::bon]
impl BufferViewDesc {
    #[builder]
    pub fn new(
        #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
        name: Option<CString>,
        view_type: BufferViewType,
        format: Option<BufferFormat>,
        #[builder(default = 0)] byte_offset: u64,
        #[builder(default = 0)] byte_width: u64,
    ) -> Self {
        Self(diligent_sys::BufferViewDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            ViewType: view_type.into(),
            Format: format.as_ref().map_or(
                diligent_sys::BufferFormat {
                    ValueType: diligent_sys::VT_UNDEFINED as _,
                    NumComponents: 1,
                    IsNormalized: false,
                },
                |format| diligent_sys::BufferFormat {
                    ValueType: format
                        .value_type
                        .map_or(diligent_sys::VT_UNDEFINED as _, |vt| vt.into()),
                    NumComponents: format.num_components,
                    IsNormalized: format.is_normalized,
                },
            ),
            ByteOffset: byte_offset,
            ByteWidth: byte_width,
        })
    }
}

#[repr(transparent)]
pub struct BufferView<'buffer>(diligent_sys::IBufferView, PhantomData<&'buffer Buffer>);

impl Deref for BufferView<'_> {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IDeviceObject
                as *const DeviceObject)
        }
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferViewMethods>(),
    std::mem::size_of::<*const ()>()
);

impl BufferView<'_> {
    pub fn get_buffer(&self) -> &Buffer {
        let buffer_ptr = unsafe_member_call!(self, BufferView, GetBuffer);

        unsafe { &*(buffer_ptr as *const Buffer) }
    }
}
