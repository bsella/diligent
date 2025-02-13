use super::graphics_types::ShaderType;
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
    shader_stages: ShaderTypes,
    sampler_or_texture_name: &'a std::ffi::CStr,
    sampler_desc: SamplerDesc<'a>,
}

impl<'a> ImmutableSamplerDesc<'a> {
    pub fn new(
        shader_stages: ShaderTypes,
        sampler_or_texture_name: &'a std::ffi::CStr,
        sampler_desc: SamplerDesc<'a>,
    ) -> Self {
        ImmutableSamplerDesc {
            shader_stages,
            sampler_or_texture_name,
            sampler_desc,
        }
    }
}

impl From<&ImmutableSamplerDesc<'_>> for bindings::ImmutableSamplerDesc {
    fn from(value: &ImmutableSamplerDesc<'_>) -> Self {
        bindings::ImmutableSamplerDesc {
            ShaderStages: value.shader_stages.bits() as bindings::SHADER_TYPE,
            SamplerOrTextureName: value.sampler_or_texture_name.as_ptr(),
            Desc: bindings::SamplerDesc::from(&value.sampler_desc),
        }
    }
}

pub struct PipelineResourceSignature {
    pub(crate) pipeline_resource_signature: *mut bindings::IPipelineResourceSignature,
    virtual_functions: *mut bindings::IPipelineResourceSignatureVtbl,

    vertex_static_variables: Vec<ShaderResourceVariable>,
    pixel_static_variables: Vec<ShaderResourceVariable>,
    geometry_static_variables: Vec<ShaderResourceVariable>,
    hull_static_variables: Vec<ShaderResourceVariable>,
    domain_static_variables: Vec<ShaderResourceVariable>,
    compute_static_variables: Vec<ShaderResourceVariable>,
    amplification_static_variables: Vec<ShaderResourceVariable>,
    mesh_static_variables: Vec<ShaderResourceVariable>,
    raygen_static_variables: Vec<ShaderResourceVariable>,
    raymiss_static_variables: Vec<ShaderResourceVariable>,
    rayclosesthit_static_variables: Vec<ShaderResourceVariable>,
    rayanyhit_static_variables: Vec<ShaderResourceVariable>,
    rayintersection_static_variables: Vec<ShaderResourceVariable>,
    callable_static_variables: Vec<ShaderResourceVariable>,
    tile_static_variables: Vec<ShaderResourceVariable>,

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
            shader_type: ShaderType,
        ) -> Vec<ShaderResourceVariable> {
            let virtual_functions =
                unsafe { (*(*pipeline_rs_ptr).pVtbl).PipelineResourceSignature };

            let shader_type = bindings::SHADER_TYPE::from(&shader_type);

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

            vertex_static_variables: Vec::new(),
            pixel_static_variables: Vec::new(),
            geometry_static_variables: Vec::new(),
            hull_static_variables: Vec::new(),
            domain_static_variables: Vec::new(),
            compute_static_variables: Vec::new(),
            amplification_static_variables: Vec::new(),
            mesh_static_variables: Vec::new(),
            raygen_static_variables: Vec::new(),
            raymiss_static_variables: Vec::new(),
            rayclosesthit_static_variables: Vec::new(),
            rayanyhit_static_variables: Vec::new(),
            rayintersection_static_variables: Vec::new(),
            callable_static_variables: Vec::new(),
            tile_static_variables: Vec::new(),
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
        shader_stages: ShaderTypes,
        resource_mapping: &ResourceMapping,
        flags: bindings::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .BindStaticResources
                .unwrap_unchecked()(
                self.pipeline_resource_signature,
                shader_stages.bits() as bindings::SHADER_TYPE,
                resource_mapping.resource_mapping,
                flags,
            );
        }
    }

    pub fn get_static_variables(&self, shader_type: ShaderType) -> &[ShaderResourceVariable] {
        match shader_type {
            ShaderType::Vertex => &self.vertex_static_variables,
            ShaderType::Pixel => &self.pixel_static_variables,
            ShaderType::Geometry => &self.geometry_static_variables,
            ShaderType::Hull => &self.hull_static_variables,
            ShaderType::Domain => &self.domain_static_variables,
            ShaderType::Compute => &self.compute_static_variables,
            ShaderType::Amplification => &self.amplification_static_variables,
            ShaderType::Mesh => &self.mesh_static_variables,
            ShaderType::RayGen => &self.raygen_static_variables,
            ShaderType::RayMiss => &self.raymiss_static_variables,
            ShaderType::RayClosestHit => &self.rayclosesthit_static_variables,
            ShaderType::RayAnyHit => &self.rayanyhit_static_variables,
            ShaderType::RayIntersection => &self.rayintersection_static_variables,
            ShaderType::Callable => &self.callable_static_variables,
            ShaderType::Tile => &self.tile_static_variables,
        }
        .as_slice()
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
