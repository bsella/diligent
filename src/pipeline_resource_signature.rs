use std::{ffi::CString, str::FromStr};

use static_assertions::const_assert;

use crate::{
    device_object::DeviceObject,
    graphics_types::{ShaderType, ShaderTypes},
    resource_mapping::ResourceMapping,
    sampler::SamplerDesc,
    shader_resource_binding::ShaderResourceBinding,
    shader_resource_variable::ShaderResourceVariable,
};

pub struct ImmutableSamplerDesc<'a> {
    shader_stages: ShaderTypes,
    sampler_or_texture_name: CString,
    sampler_desc: &'a SamplerDesc,
}

impl<'a> ImmutableSamplerDesc<'a> {
    pub fn new(
        shader_stages: ShaderTypes,
        sampler_or_texture_name: impl AsRef<str>,
        sampler_desc: &'a SamplerDesc,
    ) -> Self {
        ImmutableSamplerDesc {
            shader_stages,
            sampler_or_texture_name: CString::new(sampler_or_texture_name.as_ref()).unwrap(),
            sampler_desc,
        }
    }
}

impl From<&ImmutableSamplerDesc<'_>> for diligent_sys::ImmutableSamplerDesc {
    fn from(value: &ImmutableSamplerDesc<'_>) -> Self {
        diligent_sys::ImmutableSamplerDesc {
            ShaderStages: value.shader_stages.bits(),
            SamplerOrTextureName: value.sampler_or_texture_name.as_ptr(),
            Desc: (value.sampler_desc).into(),
        }
    }
}

pub struct PipelineResourceSignature {
    pub(crate) sys_ptr: *mut diligent_sys::IPipelineResourceSignature,
    virtual_functions: *mut diligent_sys::IPipelineResourceSignatureVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for PipelineResourceSignature {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl PipelineResourceSignature {
    #[allow(dead_code)]
    pub(crate) fn new(
        pipeline_resource_signature_ptr: *mut diligent_sys::IPipelineResourceSignature,
    ) -> Self {
        fn create_shader_resource_variables(
            pipeline_rs_ptr: *mut diligent_sys::IPipelineResourceSignature,
            shader_type: ShaderType,
        ) -> Vec<ShaderResourceVariable> {
            let virtual_functions =
                unsafe { (*(*pipeline_rs_ptr).pVtbl).PipelineResourceSignature };

            let shader_type = (&shader_type).into();

            let num_variables = unsafe {
                virtual_functions.GetStaticVariableCount.unwrap_unchecked()(
                    pipeline_rs_ptr,
                    shader_type,
                )
            } as usize;

            let static_variable_ptr = unsafe {
                virtual_functions
                    .GetStaticVariableByIndex
                    .unwrap_unchecked()(pipeline_rs_ptr, shader_type, 0)
            };

            fn create_srv_and_add_ref(
                srv_ptr: *mut diligent_sys::IShaderResourceVariable,
            ) -> ShaderResourceVariable {
                let srv = ShaderResourceVariable::new(srv_ptr);
                srv.as_ref().add_ref();
                srv
            }

            Vec::from_iter(
                std::iter::repeat(static_variable_ptr)
                    .take(num_variables)
                    .zip(0..num_variables)
                    .map(|(ptr, offset)| unsafe { ptr.add(offset) })
                    .map(|shader_rv_ptr| create_srv_and_add_ref(shader_rv_ptr)),
            )
        }

        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IPipelineResourceSignature>()
        );

        PipelineResourceSignature {
            sys_ptr: pipeline_resource_signature_ptr,
            virtual_functions: unsafe { (*pipeline_resource_signature_ptr).pVtbl },

            device_object: DeviceObject::new(
                pipeline_resource_signature_ptr as *mut diligent_sys::IDeviceObject,
            ),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::PipelineResourceSignatureDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object.sys_ptr)
                as *const diligent_sys::PipelineResourceSignatureDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn create_shader_resource_binding(
        &self,
        init_static_resources: bool,
    ) -> Result<ShaderResourceBinding, ()> {
        let mut shader_resource_binding_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .CreateShaderResourceBinding
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::addr_of_mut!(shader_resource_binding_ptr),
                init_static_resources,
            );
        }

        if shader_resource_binding_ptr.is_null() {
            Err(())
        } else {
            Ok(ShaderResourceBinding::new(shader_resource_binding_ptr))
        }
    }

    pub fn bind_static_resources(
        &self,
        shader_stages: ShaderTypes,
        resource_mapping: &ResourceMapping,
        flags: diligent_sys::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .BindStaticResources
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_stages.bits(),
                resource_mapping.sys_ptr,
                flags,
            );
        }
    }

    pub fn get_static_variable_by_name(
        &self,
        shader_type: ShaderType,
        name: impl AsRef<str>,
    ) -> Option<ShaderResourceVariable> {
        let name = CString::from_str(name.as_ref()).unwrap();

        let shader_resource_variable = unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .GetStaticVariableByName
                .unwrap_unchecked()(self.sys_ptr, (&shader_type).into(), name.as_ptr())
        };

        if shader_resource_variable.is_null() {
            None
        } else {
            let srv = ShaderResourceVariable::new(shader_resource_variable);
            srv.as_ref().add_ref();
            Some(srv)
        }
    }

    pub fn initialize_static_srb_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .InitializeStaticSRBResources
                .unwrap_unchecked()(self.sys_ptr, shader_resource_binding.sys_ptr);
        }
    }

    pub fn copy_static_resources(&self, signature: &mut PipelineResourceSignature) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .CopyStaticResources
                .unwrap_unchecked()(self.sys_ptr, signature.sys_ptr);
        }
    }

    pub fn is_compatible_with(&self, signature: &PipelineResourceSignature) -> bool {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .IsCompatibleWith
                .unwrap_unchecked()(self.sys_ptr, signature.sys_ptr)
        }
    }
}
