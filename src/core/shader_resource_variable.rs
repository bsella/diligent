use crate::core::bindings;

use super::object::{AsObject, Object};
use super::device_object::DeviceObject;

pub struct ShaderResourceVariable {
    pub(crate) m_shader_resource_variable: *mut bindings::IShaderResourceVariable,
    m_virtual_functions: *mut bindings::IShaderResourceVariableVtbl,
    m_object: Object,
}

impl AsObject for ShaderResourceVariable {
    fn as_object(&self) -> &Object {
        &self.m_object
    }
}

impl ShaderResourceVariable {
    pub(crate) fn create(shader_resource_variable: *mut bindings::IShaderResourceVariable) -> Self {
        ShaderResourceVariable {
            m_virtual_functions: unsafe { (*shader_resource_variable).pVtbl },
            m_shader_resource_variable: shader_resource_variable,
            m_object: Object::create(
                shader_resource_variable as *mut bindings::IObject,
            ),
        }
    }

    fn set(
        &mut self,
        device_object: &DeviceObject,
        flags: Option<bindings::SET_SHADER_RESOURCE_FLAGS>,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .ShaderResourceVariable
                .Set
                .unwrap_unchecked()(
                self.m_shader_resource_variable,
                device_object.m_device_object,
                flags.unwrap_or(bindings::SET_SHADER_RESOURCE_FLAG_NONE),
            )
        }
    }

    fn set_array(
        &mut self,
        device_objects: &[DeviceObject],
        flags: Option<bindings::SET_SHADER_RESOURCE_FLAGS>,
    ) {
        let object_ptrs =
            Vec::from_iter(device_objects.iter().map(|object| object.m_device_object));
        unsafe {
            (*self.m_virtual_functions)
                .ShaderResourceVariable
                .SetArray
                .unwrap_unchecked()(
                self.m_shader_resource_variable,
                object_ptrs.as_ptr(),
                0,
                object_ptrs.len() as u32,
                flags.unwrap_or(bindings::SET_SHADER_RESOURCE_FLAG_NONE),
            )
        }
    }

    fn set_buffer_range(
        &mut self,
        device_object: &DeviceObject,
        offset: u64,
        size: u64,
        array_index: Option<u32>,
        flags: Option<bindings::SET_SHADER_RESOURCE_FLAGS>,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .ShaderResourceVariable
                .SetBufferRange
                .unwrap_unchecked()(
                self.m_shader_resource_variable,
                device_object.m_device_object,
                offset,
                size,
                array_index.unwrap_or(0),
                flags.unwrap_or(bindings::SET_SHADER_RESOURCE_FLAG_NONE),
            )
        }
    }

    fn set_buffer_offset(&mut self, offset: u32, array_index: Option<u32>) {
        unsafe {
            (*self.m_virtual_functions)
                .ShaderResourceVariable
                .SetBufferOffset
                .unwrap_unchecked()(
                self.m_shader_resource_variable,
                offset,
                array_index.unwrap_or(0),
            )
        }
    }
    fn get_type(&self) -> bindings::SHADER_RESOURCE_VARIABLE_TYPE {
        unsafe {
            (*self.m_virtual_functions)
                .ShaderResourceVariable
                .GetType
                .unwrap_unchecked()(self.m_shader_resource_variable)
        }
    }
    fn get_resource_desc(&self) -> bindings::ShaderResourceDesc {
        let mut shader_resource_desc = bindings::ShaderResourceDesc::default();
        unsafe {
            (*self.m_virtual_functions)
                .ShaderResourceVariable
                .GetResourceDesc
                .unwrap_unchecked()(
                self.m_shader_resource_variable,
                std::ptr::addr_of_mut!(shader_resource_desc),
            );
        }
        shader_resource_desc
    }
    fn get_index(&self) -> u32 {
        unsafe {
            (*self.m_virtual_functions)
                .ShaderResourceVariable
                .GetIndex
                .unwrap_unchecked()(self.m_shader_resource_variable)
        }
    }
}
