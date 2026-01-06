use std::ffi::CStr;

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::device_object::DeviceObject;

use super::{
    graphics_types::{FilterType, TextureAddressMode},
    pipeline_state::ComparisonFunction,
};

bitflags! {
    #[derive(Clone,Copy)]
    pub struct SamplerFlags: diligent_sys::SAMPLER_FLAGS {
        const None                           = diligent_sys::SAMPLER_FLAG_NONE as diligent_sys::SAMPLER_FLAGS;
        const Subsampled                     = diligent_sys::SAMPLER_FLAG_SUBSAMPLED as diligent_sys::SAMPLER_FLAGS;
        const SubsampledCoarseReconstruction = diligent_sys::SAMPLER_FLAG_SUBSAMPLED_COARSE_RECONSTRUCTION as diligent_sys::SAMPLER_FLAGS;
    }
}
const_assert_eq!(diligent_sys::SAMPLER_FLAG_LAST, 2);

impl Default for SamplerFlags {
    fn default() -> Self {
        SamplerFlags::None
    }
}

#[repr(transparent)]
pub struct SamplerDesc(pub(crate) diligent_sys::SamplerDesc);

#[bon::bon]
impl SamplerDesc {
    #[builder]
    pub fn new(
        name: Option<&CStr>,

        #[builder(default = FilterType::Linear)] min_filter: FilterType,

        #[builder(default = FilterType::Linear)] mag_filter: FilterType,

        #[builder(default = FilterType::Linear)] mip_filter: FilterType,

        #[builder(default = TextureAddressMode::Clamp)] address_u: TextureAddressMode,

        #[builder(default = TextureAddressMode::Clamp)] address_v: TextureAddressMode,

        #[builder(default = TextureAddressMode::Clamp)] address_w: TextureAddressMode,

        #[builder(default)] flags: SamplerFlags,

        #[builder(default = false)] unnormalized_coords: bool,

        #[builder(default = 0.0)] mip_lod_bias: f32,

        #[builder(default = 0)] max_anisotropy: u32,

        #[builder(default = ComparisonFunction::Never)] comparison_func: ComparisonFunction,

        #[builder(default = [0.0, 0.0, 0.0, 0.0])] border_color: [f32; 4usize],

        #[builder(default = 0.0)] min_lod: f32,

        #[builder(default = f32::MAX)] max_lod: f32,
    ) -> Self {
        Self(diligent_sys::SamplerDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.as_ref().map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            MinFilter: min_filter.into(),
            MagFilter: mag_filter.into(),
            MipFilter: mip_filter.into(),
            AddressU: address_u.into(),
            AddressV: address_v.into(),
            AddressW: address_w.into(),
            Flags: flags.bits() as _,
            UnnormalizedCoords: unnormalized_coords,
            MipLODBias: mip_lod_bias,
            MaxAnisotropy: max_anisotropy,
            ComparisonFunc: comparison_func.into(),
            BorderColor: border_color,
            MinLOD: min_lod,
            MaxLOD: max_lod,
        })
    }
}

define_ported!(Sampler, diligent_sys::ISampler, DeviceObject);

impl Sampler {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::ISampler {
        std::ptr::from_ref(&self.0) as _
    }

    pub fn desc(&self) -> &SamplerDesc {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const SamplerDesc) }
    }
}
