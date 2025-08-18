use std::{ffi::CString, ops::Deref, str::FromStr};

use static_assertions::const_assert_eq;

use crate::{
    graphics_types::ShaderTypes,
    object::Object,
    pipeline_resource_signature::PipelineResourceSignature,
    resource_mapping::ResourceMapping,
    shader_resource_variable::{
        BindShaderResourcesFlags, ShaderResourceVariable, ShaderResourceVariableTypeFlags,
    },
};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IShaderResourceBindingMethods>(),
    7 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ShaderResourceBinding(Object);

impl Deref for ShaderResourceBinding {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ShaderResourceBinding {
    pub(crate) fn new(srb_ptr: *mut diligent_sys::IShaderResourceBinding) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IObject>(),
            std::mem::size_of::<diligent_sys::IShaderResourceBinding>()
        );

        Self(Object::new(srb_ptr as *mut diligent_sys::IObject))
    }

    pub fn get_pipeline_resource_signature(&self) -> Option<PipelineResourceSignature> {
        let prs_ptr =
            unsafe_member_call!(self, ShaderResourceBinding, GetPipelineResourceSignature);

        if prs_ptr.is_null() {
            None
        } else {
            let prs = PipelineResourceSignature::new(prs_ptr);
            prs.add_ref();
            Some(prs)
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
            resource_mapping.sys_ptr as _,
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
            resource_mapping.sys_ptr as _,
            flags.bits()
        );

        ShaderResourceVariableTypeFlags::from_bits_retain(flags)
    }

    pub fn get_variable_by_name(
        &self,
        name: impl AsRef<str>,
        shader_stages: ShaderTypes,
    ) -> Option<ShaderResourceVariable> {
        let name = CString::from_str(name.as_ref()).unwrap();

        let variable = unsafe_member_call!(
            self,
            ShaderResourceBinding,
            GetVariableByName,
            shader_stages.bits(),
            name.as_ptr()
        );

        if variable.is_null() {
            None
        } else {
            let srv = ShaderResourceVariable::new(variable);
            srv.add_ref();
            Some(srv)
        }
    }

    pub fn static_resources_initialized(&self) -> bool {
        unsafe_member_call!(self, ShaderResourceBinding, StaticResourcesInitialized)
    }
}
