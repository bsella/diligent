use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::device_object::{DeviceObject, DeviceObjectAttribs};

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
#[derive(Clone)]
pub struct SamplerDesc<'name>(pub(crate) diligent_sys::SamplerDesc, PhantomData<&'name ()>);

impl Deref for SamplerDesc<'_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name> SamplerDesc<'name> {
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,

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
        Self(
            diligent_sys::SamplerDesc {
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
            },
            PhantomData,
        )
    }
}

impl SamplerDesc<'_> {
    pub fn min_filter(&self) -> FilterType {
        self.0.MinFilter.into()
    }
    pub fn mag_filter(&self) -> FilterType {
        self.0.MagFilter.into()
    }
    pub fn mip_filter(&self) -> FilterType {
        self.0.MipFilter.into()
    }
    pub fn address_u(&self) -> TextureAddressMode {
        self.0.AddressU.into()
    }
    pub fn address_v(&self) -> TextureAddressMode {
        self.0.AddressV.into()
    }
    pub fn address_w(&self) -> TextureAddressMode {
        self.0.AddressW.into()
    }
    pub fn flags(&self) -> SamplerFlags {
        SamplerFlags::from_bits_retain(self.0.Flags)
    }
    pub fn unnormalized_coords(&self) -> bool {
        self.0.UnnormalizedCoords
    }
    pub fn mip_lod_bias(&self) -> f32 {
        self.0.MipLODBias
    }
    pub fn max_anisotropy(&self) -> u32 {
        self.0.MaxAnisotropy
    }
    pub fn comparison_func(&self) -> ComparisonFunction {
        self.0.ComparisonFunc.into()
    }
    pub fn border_color(&self) -> &[f32; 4usize] {
        &self.0.BorderColor
    }
    pub fn min_lod(&self) -> f32 {
        self.0.MinLOD
    }
    pub fn max_lod(&self) -> f32 {
        self.0.MaxLOD
    }
}

define_ported!(Sampler, diligent_sys::ISampler, DeviceObject);

impl Sampler {
    pub fn desc(&self) -> &SamplerDesc<'_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const SamplerDesc) }
    }
}
