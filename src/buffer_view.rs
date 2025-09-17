use std::{ffi::CString, ops::Deref};

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

#[derive(Builder)]
pub struct BufferViewDesc {
    name: Option<CString>,
    view_type: BufferViewType,
    format: Option<BufferFormat>,
    #[builder(default = 0)]
    byte_offset: u64,
    #[builder(default = 0)]
    byte_width: u64,
}

impl From<&BufferViewDesc> for diligent_sys::BufferViewDesc {
    fn from(value: &BufferViewDesc) -> Self {
        Self {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            ViewType: value.view_type.into(),
            Format: value.format.as_ref().map_or(
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
            ByteOffset: value.byte_offset,
            ByteWidth: value.byte_width,
        }
    }
}

pub struct BufferView<'a> {
    buffer: &'a Buffer,

    device_object: DeviceObject,
}

impl Deref for BufferView<'_> {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.device_object
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IBufferViewMethods>(),
    std::mem::size_of::<*const ()>()
);

impl<'a> BufferView<'a> {
    pub(crate) fn new(buffer_view_ptr: *mut diligent_sys::IBufferView, buffer: &'a Buffer) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IDeviceObject>(),
            std::mem::size_of::<diligent_sys::IBufferView>()
        );

        BufferView {
            buffer,
            device_object: DeviceObject::new(buffer_view_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    #[inline]
    pub fn get_buffer(&self) -> &Buffer {
        self.buffer
    }
}
