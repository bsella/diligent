use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
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

#[derive(Builder)]
pub struct SamplerDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: Option<CString>,

    #[builder(default = FilterType::Linear)]
    min_filter: FilterType,

    #[builder(default = FilterType::Linear)]
    mag_filter: FilterType,

    #[builder(default = FilterType::Linear)]
    mip_filter: FilterType,

    #[builder(default = TextureAddressMode::Clamp)]
    address_u: TextureAddressMode,

    #[builder(default = TextureAddressMode::Clamp)]
    address_v: TextureAddressMode,

    #[builder(default = TextureAddressMode::Clamp)]
    address_w: TextureAddressMode,

    #[builder(default)]
    flags: SamplerFlags,

    #[builder(default = false)]
    unnormalized_coords: bool,

    #[builder(default = 0.0)]
    mip_lod_bias: f32,

    #[builder(default = 0)]
    max_anisotropy: u32,

    #[builder(default = ComparisonFunction::Never)]
    comparison_func: ComparisonFunction,

    #[builder(default = [0.0, 0.0, 0.0, 0.0])]
    border_color: [f32; 4usize],

    #[builder(default = 0.0)]
    min_lod: f32,

    #[builder(default = f32::MAX)]
    max_lod: f32,
}

impl From<&SamplerDesc> for diligent_sys::SamplerDesc {
    fn from(value: &SamplerDesc) -> Self {
        diligent_sys::SamplerDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            MinFilter: value.min_filter.into(),
            MagFilter: value.mag_filter.into(),
            MipFilter: value.mip_filter.into(),
            AddressU: value.address_u.into(),
            AddressV: value.address_v.into(),
            AddressW: value.address_w.into(),
            Flags: value.flags.bits() as _,
            UnnormalizedCoords: value.unnormalized_coords,
            MipLODBias: value.mip_lod_bias,
            MaxAnisotropy: value.max_anisotropy,
            ComparisonFunc: value.comparison_func.into(),
            BorderColor: value.border_color,
            MinLOD: value.min_lod,
            MaxLOD: value.max_lod,
        }
    }
}

#[repr(transparent)]
pub struct Sampler(diligent_sys::ISampler);

impl Deref for Sampler {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IDeviceObject
                as *const DeviceObject)
        }
    }
}

impl Sampler {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::ISampler {
        std::ptr::from_ref(&self.0) as _
    }
}
