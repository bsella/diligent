use std::ops::Deref;

use static_assertions::{const_assert, const_assert_eq};

use crate::{buffer::Buffer, device_object::DeviceObject};

#[derive(Clone, Copy)]
pub enum BufferViewType {
    ShaderResource,
    UnorderedAccess,
}
const_assert!(diligent_sys::BUFFER_VIEW_NUM_VIEWS == 3);

impl From<BufferViewType> for diligent_sys::BUFFER_VIEW_TYPE {
    fn from(value: BufferViewType) -> Self {
        (match value {
            BufferViewType::ShaderResource => diligent_sys::BUFFER_VIEW_SHADER_RESOURCE,
            BufferViewType::UnorderedAccess => diligent_sys::BUFFER_VIEW_UNORDERED_ACCESS,
        }) as _
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
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IBufferView>()
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
