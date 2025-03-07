use std::mem::MaybeUninit;

use static_assertions::const_assert;

use super::{
    device_object::AsDeviceObject,
    graphics_types::{SetShaderResourceFlags, ShaderTypes},
    object::{AsObject, Object},
    pipeline_state::ShaderVariableFlags,
    shader::ShaderResourceDesc,
};

pub enum ShaderResourceVariableType {
    Static,
    Mutable,
    Dynamic,
}
const_assert!(diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_NUM_TYPES == 3);

impl From<&ShaderResourceVariableType> for diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE {
    fn from(value: &ShaderResourceVariableType) -> Self {
        (match value {
            ShaderResourceVariableType::Static => {
                diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_STATIC
            }
            ShaderResourceVariableType::Mutable => {
                diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_MUTABLE
            }
            ShaderResourceVariableType::Dynamic => {
                diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_DYNAMIC
            }
        }) as diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE
    }
}

impl Into<ShaderResourceVariableType> for diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE {
    fn into(self) -> ShaderResourceVariableType {
        match self as diligent_sys::_SHADER_RESOURCE_VARIABLE_TYPE {
            diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_STATIC => {
                ShaderResourceVariableType::Static
            }
            diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_MUTABLE => {
                ShaderResourceVariableType::Mutable
            }
            diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_DYNAMIC => {
                ShaderResourceVariableType::Dynamic
            }
            _ => panic!(),
        }
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

impl From<&ShaderResourceVariableDesc<'_>> for diligent_sys::ShaderResourceVariableDesc {
    fn from(value: &ShaderResourceVariableDesc<'_>) -> Self {
        diligent_sys::ShaderResourceVariableDesc {
            Name: value.name.as_ptr(),
            ShaderStages: value.shader_stages.bits() as diligent_sys::SHADER_TYPE,
            Type: diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE::from(&value.variable_type),
            Flags: value.flags.bits() as diligent_sys::SHADER_VARIABLE_FLAGS,
        }
    }
}

pub struct ShaderResourceVariable {
    pub(crate) shader_resource_variable: *mut diligent_sys::IShaderResourceVariable,
    virtual_functions: *mut diligent_sys::IShaderResourceVariableVtbl,
    object: Object,
}

impl AsObject for ShaderResourceVariable {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl ShaderResourceVariable {
    pub(crate) fn new(
        shader_resource_variable: *mut diligent_sys::IShaderResourceVariable,
    ) -> Self {
        ShaderResourceVariable {
            virtual_functions: unsafe { (*shader_resource_variable).pVtbl },
            shader_resource_variable,
            object: Object::new(shader_resource_variable as *mut diligent_sys::IObject),
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
        flags: Option<diligent_sys::SET_SHADER_RESOURCE_FLAGS>,
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
                flags.unwrap_or(diligent_sys::SET_SHADER_RESOURCE_FLAG_NONE),
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

    pub fn get_type(&self) -> ShaderResourceVariableType {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceVariable
                .GetType
                .unwrap_unchecked()(self.shader_resource_variable)
        }
        .into()
    }

    pub fn get_resource_desc(&self) -> ShaderResourceDesc {
        let shader_resource_desc = unsafe {
            let mut shader_resource_desc: MaybeUninit<diligent_sys::ShaderResourceDesc> =
                std::mem::MaybeUninit::uninit();

            (*self.virtual_functions)
                .ShaderResourceVariable
                .GetResourceDesc
                .unwrap_unchecked()(
                self.shader_resource_variable,
                shader_resource_desc.as_mut_ptr(),
            );
            shader_resource_desc.assume_init()
        };

        ShaderResourceDesc {
            name: unsafe { std::ffi::CStr::from_ptr(shader_resource_desc.Name) },
            array_size: shader_resource_desc.ArraySize as usize,
            resource_type: shader_resource_desc.Type.into(),
        }
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
