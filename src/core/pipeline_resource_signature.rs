use std::collections::BTreeMap;

use crate::core::bindings;
use crate::core::object::AsObject;
use crate::core::shader_resource_variable::ShaderResourceVariable;

use super::{
    device_object::{AsDeviceObject, DeviceObject},
    resource_mapping::ResourceMapping,
    shader_resource_binding::ShaderResourceBinding,
};

pub struct PipelineResourceSignature {
    m_pipeline_resource_signature: *mut bindings::IPipelineResourceSignature,
    m_virtual_functions: *mut bindings::IPipelineResourceSignatureVtbl,

    m_static_variables: BTreeMap<bindings::SHADER_TYPE, Vec<ShaderResourceVariable>>,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for PipelineResourceSignature {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
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
            m_pipeline_resource_signature: pipeline_rs_ptr,
            m_virtual_functions: unsafe { (*pipeline_rs_ptr).pVtbl },

            m_device_object: DeviceObject::new(pipeline_rs_ptr as *mut bindings::IDeviceObject),

            m_static_variables: BTreeMap::from_iter(
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

    fn get_desc(&self) -> &bindings::PipelineResourceSignatureDesc {
        unsafe {
            ((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.m_pipeline_resource_signature as *mut bindings::IDeviceObject,
            ) as *const bindings::PipelineResourceSignatureDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    fn create_shader_resource_binding(
        &mut self,
        init_static_resources: Option<bool>,
    ) -> Option<ShaderResourceBinding> {
        let mut shader_resource_binding_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .PipelineResourceSignature
                .CreateShaderResourceBinding
                .unwrap_unchecked()(
                self.m_pipeline_resource_signature,
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

    fn bind_static_resources(
        &mut self,
        shader_stages: bindings::SHADER_TYPE,
        resource_mapping: &ResourceMapping,
        flags: bindings::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineResourceSignature
                .BindStaticResources
                .unwrap_unchecked()(
                self.m_pipeline_resource_signature,
                shader_stages,
                resource_mapping.m_resource_mapping,
                flags,
            );
        }
    }

    fn get_static_variables(
        &self,
        shader_type: bindings::SHADER_TYPE,
    ) -> Option<&[ShaderResourceVariable]> {
        self.m_static_variables
            .get(&shader_type)
            .map(|v| v.as_slice())
    }

    fn initialize_static_srb_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineResourceSignature
                .InitializeStaticSRBResources
                .unwrap_unchecked()(
                self.m_pipeline_resource_signature,
                shader_resource_binding.m_shader_resource_binding,
            );
        }
    }

    fn copy_static_resources(&self, signature: &mut PipelineResourceSignature) {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineResourceSignature
                .CopyStaticResources
                .unwrap_unchecked()(
                self.m_pipeline_resource_signature,
                signature.m_pipeline_resource_signature,
            );
        }
    }

    fn is_compatible_with(&self, signature: &PipelineResourceSignature) -> bool {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineResourceSignature
                .IsCompatibleWith
                .unwrap_unchecked()(
                self.m_pipeline_resource_signature,
                signature.m_pipeline_resource_signature,
            )
        }
    }
}
