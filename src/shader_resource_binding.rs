use std::{ffi::CString, str::FromStr};

use crate::{
    Ported,
    device_object::{ResourceStateNoTransition, ResourceStateTransition, ResourceStateVerify},
    graphics_types::ShaderTypes,
    object::Object,
    pipeline_resource_signature::PipelineResourceSignature,
    resource_mapping::ResourceMapping,
    shader_resource_variable::{
        BindShaderResourcesFlags, ShaderResourceVariable, ShaderResourceVariableTypeFlags,
    },
};

define_ported!(
    ShaderResourceBinding,
    diligent_sys::IShaderResourceBinding,
    diligent_sys::IShaderResourceBindingMethods : 7,
    Object
);

impl ShaderResourceBinding {
    pub fn get_pipeline_resource_signature(&self) -> Option<&PipelineResourceSignature> {
        let prs_ptr =
            unsafe_member_call!(self, ShaderResourceBinding, GetPipelineResourceSignature);

        if prs_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(prs_ptr as *const PipelineResourceSignature) })
        }
    }

    pub fn bind_resources(
        &self,
        shader_stages: ShaderTypes,
        resource_mapping: &ResourceMapping,
        flags: BindShaderResourcesFlags,
    ) {
        unsafe_member_call!(
            self,
            ShaderResourceBinding,
            BindResources,
            shader_stages.bits(),
            resource_mapping.sys_ptr(),
            flags.bits()
        )
    }

    pub fn check_resources(
        &self,
        shader_stages: ShaderTypes,
        resource_mapping: &ResourceMapping,
        flags: BindShaderResourcesFlags,
    ) -> ShaderResourceVariableTypeFlags {
        let flags = unsafe_member_call!(
            self,
            ShaderResourceBinding,
            CheckResources,
            shader_stages.bits(),
            resource_mapping.sys_ptr(),
            flags.bits()
        );

        ShaderResourceVariableTypeFlags::from_bits_retain(flags)
    }

    pub fn get_variable_by_name(
        &self,
        name: impl AsRef<str>,
        shader_stages: ShaderTypes,
    ) -> Option<&ShaderResourceVariable> {
        let name = CString::from_str(name.as_ref()).unwrap();

        let variable_ptr = unsafe_member_call!(
            self,
            ShaderResourceBinding,
            GetVariableByName,
            shader_stages.bits(),
            name.as_ptr()
        );

        if variable_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(variable_ptr as *const ShaderResourceVariable) })
        }
    }

    pub fn static_resources_initialized(&self) -> bool {
        unsafe_member_call!(self, ShaderResourceBinding, StaticResourcesInitialized)
    }
}

impl ShaderResourceBinding {
    pub fn transition_state(&mut self) -> ResourceStateTransition<'_, ShaderResourceBinding> {
        ResourceStateTransition::new(self)
    }
    pub fn verify_state(&self) -> ResourceStateVerify<'_, ShaderResourceBinding> {
        ResourceStateVerify::new(self)
    }
    pub fn no_state_transition(&self) -> ResourceStateNoTransition<'_, ShaderResourceBinding> {
        ResourceStateNoTransition::new(self)
    }
}
