use std::{ffi::CString, ops::Deref, str::FromStr};

use bitflags::bitflags;
use static_assertions::const_assert;

use crate::{
    device_object::DeviceObject,
    graphics_types::{ShaderType, ShaderTypes},
    resource_mapping::ResourceMapping,
    sampler::SamplerDesc,
    shader::ShaderResourceType,
    shader_resource_binding::ShaderResourceBinding,
    shader_resource_variable::{
        BindShaderResourcesFlags, ShaderResourceVariable, ShaderResourceVariableType,
    },
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

bitflags! {
    pub struct PipelineResourceFlags: diligent_sys::PIPELINE_RESOURCE_FLAGS {
        const None                   = diligent_sys::PIPELINE_RESOURCE_FLAG_NONE as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const NoDynamicBuffers       = diligent_sys::PIPELINE_RESOURCE_FLAG_NO_DYNAMIC_BUFFERS as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const CombinedSampler        = diligent_sys::PIPELINE_RESOURCE_FLAG_COMBINED_SAMPLER as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const FormattedBuffer        = diligent_sys::PIPELINE_RESOURCE_FLAG_FORMATTED_BUFFER as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const RuntimeArray           = diligent_sys::PIPELINE_RESOURCE_FLAG_RUNTIME_ARRAY as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const GeneralInputAttachment = diligent_sys::PIPELINE_RESOURCE_FLAG_GENERAL_INPUT_ATTACHMENT as diligent_sys::PIPELINE_RESOURCE_FLAGS;
    }
}
const_assert!(diligent_sys::PIPELINE_RESOURCE_FLAG_LAST == 16);

pub struct PipelineResourceDesc {
    name: CString,
    shader_stages: ShaderTypes,
    array_size: u32,
    resource_type: Option<ShaderResourceType>,
    var_type: ShaderResourceVariableType,
    flags: PipelineResourceFlags,
    // TODO WebGPUResourceAttribs 	       WebGPUAttribs DEFAULT_INITIALIZER({});
}

impl From<&PipelineResourceDesc> for diligent_sys::PipelineResourceDesc {
    fn from(value: &PipelineResourceDesc) -> Self {
        diligent_sys::PipelineResourceDesc {
            Name: value.name.as_ptr(),
            ArraySize: value.array_size,
            Flags: value.flags.bits(),
            ResourceType: value.resource_type.map_or(
                diligent_sys::SHADER_RESOURCE_TYPE_UNKNOWN as diligent_sys::SHADER_RESOURCE_TYPE,
                |resource_type| resource_type.into(),
            ),
            ShaderStages: value.shader_stages.bits(),
            VarType: value.var_type.into(),
            // TODO
            WebGPUAttribs: diligent_sys::WebGPUResourceAttribs {
                BindingType: diligent_sys::WEB_GPU_BINDING_TYPE_DEFAULT as _,
                TextureViewDim: diligent_sys::RESOURCE_DIM_TEX_2D as _,
                UAVTextureFormat: diligent_sys::TEX_FORMAT_UNKNOWN as _,
            },
        }
    }
}

pub struct PipelineResourceSignatureDesc<'a> {
    name: CString,
    resources: Vec<PipelineResourceDesc>,
    immutable_samplers: Vec<ImmutableSamplerDesc<'a>>,
    binding_index: u8,
    use_combined_texture_samplers: bool,
    combined_sampler_suffix: CString,
    srb_allocation_granularity: u32,
}

pub(crate) struct PipelineResourceSignatureDescWrapper {
    _resources: Vec<diligent_sys::PipelineResourceDesc>,
    _immutable_samplers: Vec<diligent_sys::ImmutableSamplerDesc>,
    desc: diligent_sys::PipelineResourceSignatureDesc,
}

impl Deref for PipelineResourceSignatureDescWrapper {
    type Target = diligent_sys::PipelineResourceSignatureDesc;
    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl From<&PipelineResourceSignatureDesc<'_>> for PipelineResourceSignatureDescWrapper {
    fn from(value: &PipelineResourceSignatureDesc<'_>) -> Self {
        let resources: Vec<_> = value
            .resources
            .iter()
            .map(|resource| resource.into())
            .collect();

        let immutable_samplers: Vec<_> = value
            .immutable_samplers
            .iter()
            .map(|sampler| sampler.into())
            .collect();

        let desc = diligent_sys::PipelineResourceSignatureDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            BindingIndex: value.binding_index,
            CombinedSamplerSuffix: value.combined_sampler_suffix.as_ptr(),
            NumImmutableSamplers: immutable_samplers.len() as u32,
            ImmutableSamplers: immutable_samplers.as_ptr(),
            SRBAllocationGranularity: value.srb_allocation_granularity,
            UseCombinedTextureSamplers: value.use_combined_texture_samplers,
            NumResources: resources.len() as u32,
            Resources: resources.as_ptr(),
        };

        PipelineResourceSignatureDescWrapper {
            _resources: resources,
            _immutable_samplers: immutable_samplers,
            desc,
        }
    }
}

impl PipelineResourceSignature {
    pub fn new(
        pipeline_resource_signature_ptr: *mut diligent_sys::IPipelineResourceSignature,
    ) -> Self {
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
        // TODO
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
        flags: BindShaderResourcesFlags,
    ) {
        unsafe {
            (*self.virtual_functions)
                .PipelineResourceSignature
                .BindStaticResources
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_stages.bits(),
                resource_mapping.sys_ptr,
                flags.bits(),
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
                .unwrap_unchecked()(self.sys_ptr, shader_type.into(), name.as_ptr())
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
