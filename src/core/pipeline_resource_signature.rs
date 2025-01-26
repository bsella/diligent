use std::collections::BTreeMap;

use super::sampler::SamplerDesc;
use super::shader_resource_variable::ShaderResourceVariable;
use super::{graphics_types::ShaderTypes, object::AsObject};
use crate::bindings;

use super::{
    device_object::{AsDeviceObject, DeviceObject},
    resource_mapping::ResourceMapping,
    shader_resource_binding::ShaderResourceBinding,
};

pub struct ImmutableSamplerDesc<'a> {
    pub shader_stages: ShaderTypes,
    pub sampler_or_texture_name: &'a std::ffi::CStr,
    pub sampler_desc: SamplerDesc<'a>,
}

impl<'a> Into<bindings::ImmutableSamplerDesc> for ImmutableSamplerDesc<'a> {
    fn into(self) -> bindings::ImmutableSamplerDesc {
        bindings::ImmutableSamplerDesc {
            ShaderStages: self.shader_stages.bits() as bindings::SHADER_TYPE,
            SamplerOrTextureName: self.sampler_or_texture_name.as_ptr(),
            Desc: self.sampler_desc.into(),
        }
    }
}

pub struct PipelineResourceSignature {
    pub(crate) pipeline_resource_signature: *mut bindings::IPipelineResourceSignature,
    virtual_functions: *mut bindings::IPipelineResourceSignatureVtbl,

    static_variables: BTreeMap<bindings::SHADER_TYPE, Vec<ShaderResourceVariable>>,

    device_object: DeviceObject,
}

impl AsDeviceObject for PipelineResourceSignature {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl PipelineResourceSignature {
    pub(crate) fn new(pipeline_rs_ptr: *mut bindings::IPipelineResourceSignature) -> Self {
        fn create_shader_resource_variables(
            pipeline_rs_ptr: *mut bindings::IPipelineResourceSignature,
            shader_type: bindings::SHADER_TYPE,
        ) -> Vec<ShaderResourceVariable> {
            let virtual_functions =
                unsafe { (*(*pipeline_rs_ptr).pVtbl).PipelineResourceSignature };

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
                srv_ptr: *mut bindings::IShaderResourceVariable,
            ) -> ShaderResourceVariable {
                let srv = ShaderResourceVariable::new(srv_ptr);
                srv.as_object().add_ref();
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

        PipelineResourceSignature {
            pipeline_resource_signature: pipeline_rs_ptr,
            virtual_functions: unsafe { (*pipeline_rs_ptr).pVtbl },

            device_object: DeviceObject::new(pipeline_rs_ptr as *mut bindings::IDeviceObject),

            static_variables: BTreeMap::from_iter(
                [
                    bindings::SHADER_TYPE_VERTEX,
                    bindings::SHADER_TYPE_PIXEL,
                    bindings::SHADER_TYPE_GEOMETRY,
                    bindings::SHADER_TYPE_HULL,
                    bindings::SHADER_TYPE_DOMAIN,
                    bindings::SHADER_TYPE_COMPUTE,
                    bindings::SHADER_TYPE_AMPLIFICATION,
                    bindings::SHADER_TYPE_MESH,
                    bindings::SHADER_TYPE_RAY_GEN,
                    bindings::SHADER_TYPE_RAY_MISS,
                    bindings::SHADER_TYPE_RAY_CLOSEST_HIT,
                    bindings::SHADER_TYPE_RAY_ANY_HIT,
                    bindings::SHADER_TYPE_RAY_INTERSECTION,
                    bindings::SHADER_TYPE_CALLABLE,
                    bindings::SHADER_TYPE_TILE,
                ]
                .iter()
                .map(|shader_type| {
                    (
                        *shader_type,
                        create_shader_resource_variables(pipeline_rs_ptr, *shader_type),
                    )
                }),
            ),
        }
    }

    pub fn get_desc(&self) -> &bindings::PipelineResourceSignatureDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.pipeline_resource_signature as *mut bindings::IDeviceObject,
            ) as *const bindings::PipelineResourceSignatureDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn create_shader_resource_binding(
        &self,
        init_static_resources: Option<bool>,
    ) -> Option<ShaderResourceBinding> {
        let mut shader_resource_binding_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .CreateShaderResourceBinding
                .unwrap_unchecked()(
                self.pipeline_resource_signature,
                std::ptr::addr_of_mut!(shader_resource_binding_ptr),
                init_static_resources.unwrap_or(false),
            );
        }

        if shader_resource_binding_ptr.is_null() {
            None
        } else {
            Some(ShaderResourceBinding::new(shader_resource_binding_ptr))
        }
    }

    pub fn bind_static_resources(
        &self,
        shader_stages: bindings::SHADER_TYPE,
        resource_mapping: &ResourceMapping,
        flags: bindings::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .BindStaticResources
                .unwrap_unchecked()(
                self.pipeline_resource_signature,
                shader_stages,
                resource_mapping.resource_mapping,
                flags,
            );
        }
    }

    pub fn get_static_variables(
        &self,
        shader_type: bindings::SHADER_TYPE,
    ) -> Option<&[ShaderResourceVariable]> {
        self.static_variables
            .get(&shader_type)
            .map(|v| v.as_slice())
    }

    pub fn initialize_static_srb_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .InitializeStaticSRBResources
                .unwrap_unchecked()(
                self.pipeline_resource_signature,
                shader_resource_binding.shader_resource_binding,
            );
        }
    }

    pub fn copy_static_resources(&self, signature: &mut PipelineResourceSignature) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .CopyStaticResources
                .unwrap_unchecked()(
                self.pipeline_resource_signature,
                signature.pipeline_resource_signature,
            );
        }
    }

    pub fn is_compatible_with(&self, signature: &PipelineResourceSignature) -> bool {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .IsCompatibleWith
                .unwrap_unchecked()(
                self.pipeline_resource_signature,
                signature.pipeline_resource_signature,
            )
        }
    }
}
