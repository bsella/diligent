use std::ffi::{CStr, CString};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert;

use super::{
    device_object::DeviceObject,
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
const_assert!(diligent_sys::SAMPLER_FLAG_LAST == 2);

impl Default for SamplerFlags {
    fn default() -> Self {
        SamplerFlags::None
    }
}

#[derive(Builder)]
pub struct SamplerDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: CString,

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
                Name: value.name.as_ptr(),
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

pub struct Sampler {
    pub(crate) sys_ptr: *mut diligent_sys::ISampler,
    virtual_functions: *mut diligent_sys::ISamplerVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for Sampler {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Sampler {
    pub(crate) fn new(sampler_ptr: *mut diligent_sys::ISampler) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::ISampler>()
        );

        Sampler {
            sys_ptr: sampler_ptr,
            virtual_functions: unsafe { (*sampler_ptr).pVtbl },
            device_object: DeviceObject::new(sampler_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> SamplerDesc {
        let desc = unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object.sys_ptr)
                as *const diligent_sys::SamplerDesc)
                .as_ref()
                .unwrap_unchecked()
        };

        SamplerDesc {
            name: CString::from(unsafe { CStr::from_ptr(desc._DeviceObjectAttribs.Name) }),
            address_u: desc.AddressU.into(),
            address_v: desc.AddressV.into(),
            address_w: desc.AddressW.into(),
            border_color: desc.BorderColor,
            comparison_func: desc.ComparisonFunc.into(),
            flags: SamplerFlags::from_bits_retain(desc.Flags),
            mag_filter: desc.MagFilter.into(),
            max_anisotropy: desc.MaxAnisotropy,
            max_lod: desc.MaxLOD,
            min_filter: desc.MinFilter.into(),
            min_lod: desc.MinLOD,
            mip_filter: desc.MipFilter.into(),
            mip_lod_bias: desc.MipLODBias,
            unnormalized_coords: desc.UnnormalizedCoords,
        }
    }
}
