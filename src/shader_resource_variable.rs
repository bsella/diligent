use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, ops::Deref};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{device_object::DeviceObject, object::Object};

use super::{
    graphics_types::{SetShaderResourceFlags, ShaderTypes},
    pipeline_state::ShaderVariableFlags,
    shader::ShaderResourceDesc,
};

#[derive(Clone, Copy)]
pub enum ShaderResourceVariableType {
    Static,
    Mutable,
    Dynamic,
}
const_assert_eq!(diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_NUM_TYPES, 3);

impl From<ShaderResourceVariableType> for diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE {
    fn from(value: ShaderResourceVariableType) -> Self {
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
        }) as _
    }
}

impl From<diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE> for ShaderResourceVariableType {
    fn from(value: diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE) -> Self {
        match value as _ {
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

bitflags! {
    #[derive(Clone,Copy)]
    pub struct BindShaderResourcesFlags : diligent_sys::BIND_SHADER_RESOURCES_FLAGS {
        const UpdateStatic      = diligent_sys::BIND_SHADER_RESOURCES_UPDATE_STATIC as diligent_sys::BIND_SHADER_RESOURCES_FLAGS;
        const UpdateMutable     = diligent_sys::BIND_SHADER_RESOURCES_UPDATE_MUTABLE as diligent_sys::BIND_SHADER_RESOURCES_FLAGS;
        const UpdateDynamic     = diligent_sys::BIND_SHADER_RESOURCES_UPDATE_DYNAMIC as diligent_sys::BIND_SHADER_RESOURCES_FLAGS;
        const UpdateAll         = diligent_sys::BIND_SHADER_RESOURCES_UPDATE_ALL as diligent_sys::BIND_SHADER_RESOURCES_FLAGS;
        const KeepExisting      = diligent_sys::BIND_SHADER_RESOURCES_KEEP_EXISTING as diligent_sys::BIND_SHADER_RESOURCES_FLAGS;
        const VerifyAllResolved = diligent_sys::BIND_SHADER_RESOURCES_VERIFY_ALL_RESOLVED as diligent_sys::BIND_SHADER_RESOURCES_FLAGS;
        const AllowOverwrite    = diligent_sys::BIND_SHADER_RESOURCES_ALLOW_OVERWRITE as diligent_sys::BIND_SHADER_RESOURCES_FLAGS;
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct ShaderResourceVariableTypeFlags : diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS {
        const None    = diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAG_NONE as diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS;
        const Static  = diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAG_STATIC as diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS;
        const Mutable = diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAG_MUTABLE as diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS;
        const Dynamic = diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAG_DYNAMIC as diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS;
        const MutDyn  = diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAG_MUT_DYN as diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS;
        const All     = diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAG_ALL as diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS;
    }
}

#[repr(transparent)]
pub struct ShaderResourceVariableDesc<'a>(
    pub(crate) diligent_sys::ShaderResourceVariableDesc,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> ShaderResourceVariableDesc<'a> {
    #[builder]
    pub fn new(
        name: Option<&'a CStr>,

        variable_type: ShaderResourceVariableType,

        shader_stages: ShaderTypes,

        #[builder(default)] flags: ShaderVariableFlags,
    ) -> Self {
        Self(
            diligent_sys::ShaderResourceVariableDesc {
                Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
                ShaderStages: shader_stages.bits(),
                Type: variable_type.into(),
                Flags: flags.bits(),
            },
            PhantomData,
        )
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IShaderResourceVariableMethods>(),
    8 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ShaderResourceVariable(diligent_sys::IShaderResourceVariable);

impl Deref for ShaderResourceVariable {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IObject as *const Object) }
    }
}

impl ShaderResourceVariable {
    pub fn set(&self, device_object: &DeviceObject, flags: SetShaderResourceFlags) {
        unsafe_member_call!(
            self,
            ShaderResourceVariable,
            Set,
            device_object.sys_ptr(),
            flags.bits()
        )
    }

    pub fn set_array(&self, device_objects: &[&DeviceObject], flags: SetShaderResourceFlags) {
        unsafe_member_call!(
            self,
            ShaderResourceVariable,
            SetArray,
            device_objects.as_ptr() as _,
            0,
            device_objects.len() as u32,
            flags.bits()
        )
    }

    pub fn set_buffer_range(
        &self,
        device_object: &DeviceObject,
        offset: u64,
        size: u64,
        array_index: Option<u32>,
        flags: SetShaderResourceFlags,
    ) {
        unsafe_member_call!(
            self,
            ShaderResourceVariable,
            SetBufferRange,
            device_object.sys_ptr(),
            offset,
            size,
            array_index.unwrap_or(0),
            flags.bits()
        )
    }

    pub fn set_buffer_offset(&self, offset: u32, array_index: Option<u32>) {
        unsafe_member_call!(
            self,
            ShaderResourceVariable,
            SetBufferOffset,
            offset,
            array_index.unwrap_or(0)
        )
    }

    pub fn get_type(&self) -> ShaderResourceVariableType {
        unsafe_member_call!(self, ShaderResourceVariable, GetType).into()
    }

    pub fn get_resource_desc(&self) -> ShaderResourceDesc {
        let mut shader_resource_desc: MaybeUninit<diligent_sys::ShaderResourceDesc> =
            std::mem::MaybeUninit::uninit();

        unsafe_member_call!(
            self,
            ShaderResourceVariable,
            GetResourceDesc,
            shader_resource_desc.as_mut_ptr()
        );

        ShaderResourceDesc(unsafe { shader_resource_desc.assume_init() })
    }

    pub fn get_index(&self) -> u32 {
        unsafe_member_call!(self, ShaderResourceVariable, GetIndex)
    }
}
