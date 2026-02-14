use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ops::Deref,
    str::FromStr,
};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{
    Boxed, BoxedFromNulError, Ported,
    device_object::{DeviceObject, DeviceObjectAttribs},
    graphics_types::{ShaderType, ShaderTypes},
    resource_mapping::ResourceMapping,
    sampler::SamplerDesc,
    shader::ShaderResourceType,
    shader_resource_binding::ShaderResourceBinding,
    shader_resource_variable::{
        BindShaderResourcesFlags, ShaderResourceVariable, ShaderResourceVariableType,
    },
};

#[repr(transparent)]
pub struct ImmutableSamplerDesc<'sampler_or_texture_name, 'sampler_name, 'sampler_desc>(
    pub(crate) diligent_sys::ImmutableSamplerDesc,
    PhantomData<(
        &'sampler_or_texture_name (),
        &'sampler_name (),
        &'sampler_desc (),
    )>,
);

#[bon::bon]
impl<'sampler_or_texture_name, 'sampler_name, 'sampler_desc>
    ImmutableSamplerDesc<'sampler_or_texture_name, 'sampler_name, 'sampler_desc>
{
    #[builder]
    pub fn new(
        shader_stages: ShaderTypes,
        sampler_or_texture_name: &'sampler_or_texture_name CStr,
        sampler_desc: &'sampler_desc SamplerDesc<'sampler_name>,
    ) -> Self {
        ImmutableSamplerDesc(
            diligent_sys::ImmutableSamplerDesc {
                ShaderStages: shader_stages.bits(),
                SamplerOrTextureName: sampler_or_texture_name.as_ptr(),
                Desc: sampler_desc.0,
            },
            PhantomData,
        )
    }
}

define_ported!(
    PipelineResourceSignature,
    diligent_sys::IPipelineResourceSignature,
    diligent_sys::IPipelineResourceSignatureMethods : 8,
    DeviceObject
);

bitflags! {
    #[derive(Clone,Copy)]
    pub struct PipelineResourceFlags: diligent_sys::PIPELINE_RESOURCE_FLAGS {
        const None                   = diligent_sys::PIPELINE_RESOURCE_FLAG_NONE as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const NoDynamicBuffers       = diligent_sys::PIPELINE_RESOURCE_FLAG_NO_DYNAMIC_BUFFERS as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const InlineConstants        = diligent_sys::PIPELINE_RESOURCE_FLAG_INLINE_CONSTANTS as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const CombinedSampler        = diligent_sys::PIPELINE_RESOURCE_FLAG_COMBINED_SAMPLER as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const FormattedBuffer        = diligent_sys::PIPELINE_RESOURCE_FLAG_FORMATTED_BUFFER as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const RuntimeArray           = diligent_sys::PIPELINE_RESOURCE_FLAG_RUNTIME_ARRAY as diligent_sys::PIPELINE_RESOURCE_FLAGS;
        const GeneralInputAttachment = diligent_sys::PIPELINE_RESOURCE_FLAG_GENERAL_INPUT_ATTACHMENT as diligent_sys::PIPELINE_RESOURCE_FLAGS;
    }
}
const_assert_eq!(diligent_sys::PIPELINE_RESOURCE_FLAG_LAST, 32);

#[repr(transparent)]
pub struct PipelineResourceDesc<'name>(diligent_sys::PipelineResourceDesc, PhantomData<&'name ()>);

#[bon::bon]
impl<'name> PipelineResourceDesc<'name> {
    #[builder]
    pub fn new(
        name: &'name CStr,
        shader_stages: ShaderTypes,
        array_size: u32,
        resource_type: Option<ShaderResourceType>,
        var_type: ShaderResourceVariableType,
        flags: PipelineResourceFlags,
        // TODO WebGPUResourceAttribs 	       WebGPUAttribs DEFAULT_INITIALIZER({});
    ) -> Self {
        PipelineResourceDesc(
            diligent_sys::PipelineResourceDesc {
                Name: name.as_ptr(),
                ArraySize: array_size,
                Flags: flags.bits(),
                ResourceType: resource_type.map_or(
                    diligent_sys::SHADER_RESOURCE_TYPE_UNKNOWN
                        as diligent_sys::SHADER_RESOURCE_TYPE,
                    |resource_type| resource_type.into(),
                ),
                ShaderStages: shader_stages.bits(),
                VarType: var_type.into(),
                // TODO
                WebGPUAttribs: diligent_sys::WebGPUResourceAttribs {
                    BindingType: diligent_sys::WEB_GPU_BINDING_TYPE_DEFAULT as _,
                    TextureViewDim: diligent_sys::RESOURCE_DIM_TEX_2D as _,
                    UAVTextureFormat: diligent_sys::TEX_FORMAT_UNKNOWN as _,
                },
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[allow(clippy::type_complexity)]
pub struct PipelineResourceSignatureDesc<
    'name,
    'resources,
    'resource_name,
    'sampler_suffix,
    'sampler_or_texture_name,
    'sampler_name,
    'sampler_desc,
    'immutable_samplers,
>(
    pub(crate) diligent_sys::PipelineResourceSignatureDesc,
    PhantomData<(
        &'name (),
        &'resources (),
        &'resource_name (),
        &'sampler_suffix (),
        &'sampler_or_texture_name (),
        &'sampler_name (),
        &'sampler_desc (),
        &'immutable_samplers (),
    )>,
);

impl Deref for PipelineResourceSignatureDesc<'_, '_, '_, '_, '_, '_, '_, '_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<
    'name,
    'resources,
    'resource_name,
    'sampler_suffix,
    'sampler_or_texture_name,
    'sampler_name,
    'sampler_desc,
    'immutable_samplers,
>
    PipelineResourceSignatureDesc<
        'name,
        'resources,
        'resource_name,
        'sampler_suffix,
        'sampler_or_texture_name,
        'sampler_name,
        'sampler_desc,
        'immutable_samplers,
    >
{
    #[builder]
    pub fn new(
        name: &'name CStr,
        resources: &'resources [PipelineResourceDesc<'resource_name>],
        immutable_samplers: &'immutable_samplers [ImmutableSamplerDesc<
            'sampler_or_texture_name,
            'sampler_name,
            'sampler_desc,
        >],
        binding_index: u8,
        use_combined_texture_samplers: bool,
        combined_sampler_suffix: &'sampler_suffix CStr,
        srb_allocation_granularity: u32,
    ) -> Self {
        PipelineResourceSignatureDesc(
            diligent_sys::PipelineResourceSignatureDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.as_ptr(),
                },
                BindingIndex: binding_index,
                CombinedSamplerSuffix: combined_sampler_suffix.as_ptr(),
                NumImmutableSamplers: immutable_samplers.len() as u32,
                ImmutableSamplers: if immutable_samplers.is_empty() {
                    std::ptr::null()
                } else {
                    immutable_samplers.as_ptr() as _
                },
                SRBAllocationGranularity: srb_allocation_granularity,
                UseCombinedTextureSamplers: use_combined_texture_samplers,
                NumResources: resources.len() as u32,
                Resources: if resources.is_empty() {
                    std::ptr::null()
                } else {
                    resources.as_ptr() as _
                },
            },
            PhantomData,
        )
    }
}

impl PipelineResourceSignatureDesc<'_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn resources(&self) -> &[PipelineResourceDesc<'_>] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.Resources as *const PipelineResourceDesc,
                self.0.NumResources as usize,
            )
        }
    }
    pub fn immutable_samplers(&self) -> &[ImmutableSamplerDesc<'_, '_, '_>] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.ImmutableSamplers as *const ImmutableSamplerDesc,
                self.0.NumImmutableSamplers as usize,
            )
        }
    }
    pub fn binding_index(&self) -> u8 {
        self.0.BindingIndex
    }
    pub fn use_combined_texture_samplers(&self) -> bool {
        self.0.UseCombinedTextureSamplers
    }
    pub fn combined_sampler_suffix(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.CombinedSamplerSuffix) }
    }
    pub fn srb_allocation_granularity(&self) -> u32 {
        self.0.SRBAllocationGranularity
    }
}

impl PipelineResourceSignature {
    pub fn desc(&self) -> &PipelineResourceSignatureDesc<'_, '_, '_, '_, '_, '_, '_, '_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const PipelineResourceSignatureDesc) }
    }

    pub fn create_shader_resource_binding(
        &self,
        init_static_resources: bool,
    ) -> Result<Boxed<ShaderResourceBinding>, BoxedFromNulError> {
        let mut shader_resource_binding_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            PipelineResourceSignature,
            CreateShaderResourceBinding,
            &mut shader_resource_binding_ptr,
            init_static_resources
        );

        Boxed::new(shader_resource_binding_ptr)
    }

    pub fn bind_static_resources(
        &self,
        shader_stages: ShaderTypes,
        resource_mapping: &ResourceMapping,
        flags: BindShaderResourcesFlags,
    ) {
        unsafe_member_call!(
            self,
            PipelineResourceSignature,
            BindStaticResources,
            shader_stages.bits(),
            resource_mapping.sys_ptr(),
            flags.bits()
        )
    }

    pub fn get_static_variable_by_name(
        &self,
        shader_type: ShaderType,
        name: impl AsRef<str>,
    ) -> Option<&ShaderResourceVariable> {
        let name = CString::from_str(name.as_ref()).unwrap();

        let shader_resource_variable = unsafe_member_call!(
            self,
            PipelineResourceSignature,
            GetStaticVariableByName,
            shader_type.into(),
            name.as_ptr()
        );

        if shader_resource_variable.is_null() {
            None
        } else {
            Some(unsafe { &*(shader_resource_variable as *const ShaderResourceVariable) })
        }
    }

    pub fn initialize_static_srb_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe_member_call!(
            self,
            PipelineResourceSignature,
            InitializeStaticSRBResources,
            shader_resource_binding.sys_ptr()
        )
    }

    pub fn copy_static_resources(&self, signature: &mut PipelineResourceSignature) {
        unsafe_member_call!(
            self,
            PipelineResourceSignature,
            CopyStaticResources,
            signature.sys_ptr()
        )
    }

    pub fn is_compatible_with(&self, signature: &PipelineResourceSignature) -> bool {
        unsafe_member_call!(
            self,
            PipelineResourceSignature,
            IsCompatibleWith,
            signature.sys_ptr()
        )
    }
}
