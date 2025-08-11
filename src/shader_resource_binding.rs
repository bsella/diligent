use std::{ffi::CString, str::FromStr};

use static_assertions::const_assert;

use crate::{
    graphics_types::ShaderTypes,
    object::Object,
    pipeline_resource_signature::PipelineResourceSignature,
    resource_mapping::ResourceMapping,
    shader_resource_variable::{
        BindShaderResourcesFlags, ShaderResourceVariable, ShaderResourceVariableTypeFlags,
    },
};

pub struct ShaderResourceBinding {
    pub(crate) sys_ptr: *mut diligent_sys::IShaderResourceBinding,
    virtual_functions: *mut diligent_sys::IShaderResourceBindingVtbl,

    object: Object,
}

impl AsRef<Object> for ShaderResourceBinding {
    fn as_ref(&self) -> &Object {
        &self.object
    }
}

impl ShaderResourceBinding {
    pub(crate) fn new(srb_ptr: *mut diligent_sys::IShaderResourceBinding) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IShaderResourceBinding>()
        );

        ShaderResourceBinding {
            sys_ptr: srb_ptr,
            virtual_functions: unsafe { (*srb_ptr).pVtbl },
            object: Object::new(srb_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn get_pipeline_resource_signature(&self) -> Result<&PipelineResourceSignature, ()> {
        todo!()
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
            resource_mapping.sys_ptr,
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
            resource_mapping.sys_ptr,
            flags.bits()
        );

        ShaderResourceVariableTypeFlags::from_bits_retain(flags)
    }

    pub fn get_variables(
        &self,
        _shader_type: ShaderTypes,
    ) -> Result<&[ShaderResourceVariable], ()> {
        todo!()
    }

    pub fn get_variable_by_name(
        &self,
        name: impl AsRef<str>,
        shader_stages: ShaderTypes,
    ) -> Result<ShaderResourceVariable, ()> {
        let name = CString::from_str(name.as_ref()).unwrap();

        let variable = unsafe_member_call!(
            self,
            ShaderResourceBinding,
            GetVariableByName,
            shader_stages.bits(),
            name.as_ptr()
        );

        if variable.is_null() {
            Err(())
        } else {
            let srv = ShaderResourceVariable::new(variable);
            srv.as_ref().add_ref();
            Ok(srv)
        }
    }

    pub fn static_resources_initialized(&self) -> bool {
        unsafe_member_call!(self, ShaderResourceBinding, StaticResourcesInitialized,)
    }
}
