use crate::bindings;
use static_assertions::const_assert;

use super::device_object::DeviceObject;
use super::graphics_types::ShaderTypes;
use super::object::{AsObject, Object};
use super::pipeline_state::ShaderVariableFlags;

pub enum ShaderResourceVariableType {
    Static,
    Mutable,
    Dynamic,
}
const_assert!(bindings::SHADER_RESOURCE_VARIABLE_TYPE_NUM_TYPES == 3);

impl Into<bindings::SHADER_RESOURCE_VARIABLE_TYPE> for ShaderResourceVariableType {
    fn into(self) -> bindings::SHADER_RESOURCE_VARIABLE_TYPE {
        (match self {
            ShaderResourceVariableType::Static => bindings::SHADER_RESOURCE_VARIABLE_TYPE_STATIC,
            ShaderResourceVariableType::Mutable => bindings::SHADER_RESOURCE_VARIABLE_TYPE_MUTABLE,
            ShaderResourceVariableType::Dynamic => bindings::SHADER_RESOURCE_VARIABLE_TYPE_DYNAMIC,
        }) as bindings::SHADER_RESOURCE_VARIABLE_TYPE
    }
}

pub struct ShaderResourceVariableDesc<'a> {
    pub name: &'a std::ffi::CStr,
    pub variable_type: ShaderResourceVariableType,
    pub shader_stages: ShaderTypes,
    pub flags: ShaderVariableFlags,
}

impl<'a> Into<bindings::ShaderResourceVariableDesc> for ShaderResourceVariableDesc<'a> {
    fn into(self) -> bindings::ShaderResourceVariableDesc {
        bindings::ShaderResourceVariableDesc {
            Name: self.name.as_ptr(),
            ShaderStages: self.shader_stages.bits() as bindings::SHADER_TYPE,
            Type: self.variable_type.into(),
            Flags: self.flags.bits() as bindings::SHADER_VARIABLE_FLAGS,
        }
    }
}

pub struct ShaderResourceVariable {
    pub(crate) shader_resource_variable: *mut bindings::IShaderResourceVariable,
    virtual_functions: *mut bindings::IShaderResourceVariableVtbl,
    object: Object,
}

impl AsObject for ShaderResourceVariable {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl ShaderResourceVariable {
    pub(crate) fn new(shader_resource_variable: *mut bindings::IShaderResourceVariable) -> Self {
        ShaderResourceVariable {
            virtual_functions: unsafe { (*shader_resource_variable).pVtbl },
            shader_resource_variable: shader_resource_variable,
            object: Object::new(shader_resource_variable as *mut bindings::IObject),
        }
    }

    pub fn set(
        &mut self,
        device_object: &DeviceObject,
        flags: Option<bindings::SET_SHADER_RESOURCE_FLAGS>,
    ) {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .Set
                .unwrap_unchecked()(
                self.shader_resource_variable,
                device_object.device_object,
                flags.unwrap_or(bindings::SET_SHADER_RESOURCE_FLAG_NONE),
            )
        }
    }

    pub fn set_array(
        &mut self,
        device_objects: &[DeviceObject],
        flags: Option<bindings::SET_SHADER_RESOURCE_FLAGS>,
    ) {
        let object_ptrs = Vec::from_iter(device_objects.iter().map(|object| object.device_object));
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .SetArray
                .unwrap_unchecked()(
                self.shader_resource_variable,
                object_ptrs.as_ptr(),
                0,
                object_ptrs.len() as u32,
                flags.unwrap_or(bindings::SET_SHADER_RESOURCE_FLAG_NONE),
            )
        }
    }

    pub fn set_buffer_range(
        &mut self,
        device_object: &DeviceObject,
        offset: u64,
        size: u64,
        array_index: Option<u32>,
        flags: Option<bindings::SET_SHADER_RESOURCE_FLAGS>,
    ) {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .SetBufferRange
                .unwrap_unchecked()(
                self.shader_resource_variable,
                device_object.device_object,
                offset,
                size,
                array_index.unwrap_or(0),
                flags.unwrap_or(bindings::SET_SHADER_RESOURCE_FLAG_NONE),
            )
        }
    }

    pub fn set_buffer_offset(&mut self, offset: u32, array_index: Option<u32>) {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .SetBufferOffset
                .unwrap_unchecked()(
                self.shader_resource_variable,
                offset,
                array_index.unwrap_or(0),
            )
        }
    }

    pub fn get_type(&self) -> bindings::SHADER_RESOURCE_VARIABLE_TYPE {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .GetType
                .unwrap_unchecked()(self.shader_resource_variable)
        }
    }

    pub fn get_resource_desc(&self) -> bindings::ShaderResourceDesc {
        let mut shader_resource_desc = bindings::ShaderResourceDesc::default();
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .GetResourceDesc
                .unwrap_unchecked()(
                self.shader_resource_variable,
                std::ptr::addr_of_mut!(shader_resource_desc),
            );
        }
        shader_resource_desc
    }

    pub fn get_index(&self) -> u32 {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .GetIndex
                .unwrap_unchecked()(self.shader_resource_variable)
        }
    }
}
