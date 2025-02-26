use crate::bindings;
use static_assertions::const_assert;

use super::{
    device_object::AsDeviceObject,
    graphics_types::{SetShaderResourceFlags, ShaderTypes},
    object::{AsObject, Object},
    pipeline_state::ShaderVariableFlags,
};

pub enum ShaderResourceVariableType {
    Static,
    Mutable,
    Dynamic,
}
const_assert!(bindings::SHADER_RESOURCE_VARIABLE_TYPE_NUM_TYPES == 3);

impl From<&ShaderResourceVariableType> for bindings::SHADER_RESOURCE_VARIABLE_TYPE {
    fn from(value: &ShaderResourceVariableType) -> Self {
        (match value {
            ShaderResourceVariableType::Static => bindings::SHADER_RESOURCE_VARIABLE_TYPE_STATIC,
            ShaderResourceVariableType::Mutable => bindings::SHADER_RESOURCE_VARIABLE_TYPE_MUTABLE,
            ShaderResourceVariableType::Dynamic => bindings::SHADER_RESOURCE_VARIABLE_TYPE_DYNAMIC,
        }) as bindings::SHADER_RESOURCE_VARIABLE_TYPE
    }
}

pub struct ShaderResourceVariableDesc<'a> {
    name: &'a std::ffi::CStr,
    variable_type: ShaderResourceVariableType,
    shader_stages: ShaderTypes,
    flags: ShaderVariableFlags,
}

impl<'a> ShaderResourceVariableDesc<'a> {
    pub fn new(
        name: &'a std::ffi::CStr,
        variable_type: ShaderResourceVariableType,
        shader_stages: ShaderTypes,
    ) -> Self {
        ShaderResourceVariableDesc {
            name,
            variable_type,
            shader_stages,
            flags: ShaderVariableFlags::None,
        }
    }

    pub fn flags(mut self, flags: ShaderVariableFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl From<&ShaderResourceVariableDesc<'_>> for bindings::ShaderResourceVariableDesc {
    fn from(value: &ShaderResourceVariableDesc<'_>) -> Self {
        bindings::ShaderResourceVariableDesc {
            Name: value.name.as_ptr(),
            ShaderStages: value.shader_stages.bits() as bindings::SHADER_TYPE,
            Type: bindings::SHADER_RESOURCE_VARIABLE_TYPE::from(&value.variable_type),
            Flags: value.flags.bits() as bindings::SHADER_VARIABLE_FLAGS,
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
            shader_resource_variable,
            object: Object::new(shader_resource_variable as *mut bindings::IObject),
        }
    }

    pub fn set<DO: AsDeviceObject>(&mut self, device_object: &DO, flags: SetShaderResourceFlags) {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .Set
                .unwrap_unchecked()(
                self.shader_resource_variable,
                device_object.as_device_object().device_object,
                flags.bits(),
            )
        }
    }

    pub fn set_array<DO: AsDeviceObject>(
        &mut self,
        device_objects: &[DO],
        flags: SetShaderResourceFlags,
    ) {
        let object_ptrs = Vec::from_iter(
            device_objects
                .iter()
                .map(|object| object.as_device_object().device_object),
        );
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .SetArray
                .unwrap_unchecked()(
                self.shader_resource_variable,
                object_ptrs.as_ptr(),
                0,
                object_ptrs.len() as u32,
                flags.bits(),
            )
        }
    }

    pub fn set_buffer_range<DO: AsDeviceObject>(
        &mut self,
        device_object: &DO,
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
                device_object.as_device_object().device_object,
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
