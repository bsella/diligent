use super::{
    graphics_types::ShaderTypes,
    object::{AsObject, Object},
    pipeline_resource_signature::PipelineResourceSignature,
    resource_mapping::ResourceMapping,
    shader_resource_variable::ShaderResourceVariable,
};

pub struct ShaderResourceBinding {
    pub(crate) shader_resource_binding: *mut diligent_sys::IShaderResourceBinding,
    virtual_functions: *mut diligent_sys::IShaderResourceBindingVtbl,

    object: Object,
}

impl AsObject for ShaderResourceBinding {
    fn as_object(&self) -> &Object {
        &self.object
    }
}

impl ShaderResourceBinding {
    pub(crate) fn new(srb_ptr: *mut diligent_sys::IShaderResourceBinding) -> Self {
        ShaderResourceBinding {
            shader_resource_binding: srb_ptr,
            virtual_functions: unsafe { (*srb_ptr).pVtbl },
            object: Object::new(srb_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn get_pipeline_resource_signature(&self) -> Option<&PipelineResourceSignature> {
        todo!()
    }

    pub fn bind_resources(
        &self,
        shader_stages: ShaderTypes,
        resource_mapping: &ResourceMapping,
        flags: diligent_sys::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceBinding
                .BindResources
                .unwrap_unchecked()(
                self.shader_resource_binding,
                shader_stages.bits() as diligent_sys::SHADER_TYPE,
                resource_mapping.resource_mapping,
                flags,
            )
        }
    }

    pub fn check_resources(
        &self,
        shader_stages: ShaderTypes,
        resource_mapping: &ResourceMapping,
        flags: diligent_sys::BIND_SHADER_RESOURCES_FLAGS,
    ) -> diligent_sys::SHADER_RESOURCE_VARIABLE_TYPE_FLAGS {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceBinding
                .CheckResources
                .unwrap_unchecked()(
                self.shader_resource_binding,
                shader_stages.bits() as diligent_sys::SHADER_TYPE,
                resource_mapping.resource_mapping,
                flags,
            )
        }
    }

    pub fn get_variables(&self, _shader_type: ShaderTypes) -> Option<&[ShaderResourceVariable]> {
        todo!()
    }

    pub fn get_variable_by_name(
        &self,
        name: &std::ffi::CStr,
        shader_stages: ShaderTypes,
    ) -> Option<ShaderResourceVariable> {
        let variable = unsafe {
            (*self.virtual_functions)
                .ShaderResourceBinding
                .GetVariableByName
                .unwrap_unchecked()(
                self.shader_resource_binding,
                shader_stages.bits() as diligent_sys::SHADER_TYPE,
                name.as_ptr(),
            )
        };
        if variable.is_null() {
            None
        } else {
            let srv = ShaderResourceVariable::new(variable);
            srv.as_object().add_ref();
            Some(srv)
        }
    }

    pub fn static_resources_initialized(&self) -> bool {
        unsafe {
            (*self.virtual_functions)
                .ShaderResourceBinding
                .StaticResourcesInitialized
                .unwrap_unchecked()(self.shader_resource_binding)
        }
    }
}
