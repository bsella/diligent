use crate::bindings;

use bitflags::bitflags;
use static_assertions::const_assert;

use super::{
    device_object::{AsDeviceObject, DeviceObject},
    graphics_types::{FilterType, TextureAddressMode},
    pipeline_state::ComparisonFunction,
};

bitflags! {
    pub struct SamplerFlags: bindings::_SAMPLER_FLAGS {
        const None                           = bindings::SAMPLER_FLAG_NONE;
        const Subsampled                     = bindings::SAMPLER_FLAG_SUBSAMPLED;
        const SubsampledCoarseReconstruction = bindings::SAMPLER_FLAG_SUBSAMPLED_COARSE_RECONSTRUCTION;
    }
}
const_assert!(bindings::SAMPLER_FLAG_SUBSAMPLED_COARSE_RECONSTRUCTION == 2);

pub struct SamplerDesc<'a> {
    pub name: &'a std::ffi::CStr,
    pub min_filter: FilterType,
    pub mag_filter: FilterType,
    pub mip_filter: FilterType,
    pub address_u: TextureAddressMode,
    pub address_v: TextureAddressMode,
    pub address_w: TextureAddressMode,
    pub flags: SamplerFlags,
    pub unnormalized_coords: bool,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: ComparisonFunction,
    pub border_color: [f32; 4usize],
    pub min_lod: f32,
    pub max_lod: f32,
}

impl<'a> Into<bindings::SamplerDesc> for SamplerDesc<'a> {
    fn into(self) -> bindings::SamplerDesc {
        bindings::SamplerDesc {
            _DeviceObjectAttribs: bindings::DeviceObjectAttribs {
                Name: self.name.as_ptr(),
            },
            MinFilter: self.min_filter.into(),
            MagFilter: self.mag_filter.into(),
            MipFilter: self.mip_filter.into(),
            AddressU: self.address_u.into(),
            AddressV: self.address_v.into(),
            AddressW: self.address_w.into(),
            Flags: self.flags.bits() as bindings::SAMPLER_FLAGS,
            UnnormalizedCoords: self.unnormalized_coords,
            MipLODBias: self.mip_lod_bias,
            MaxAnisotropy: self.max_anisotropy,
            ComparisonFunc: self.comparison_func.into(),
            BorderColor: self.border_color,
            MinLOD: self.min_lod,
            MaxLOD: self.max_lod,
        }
    }
}

pub struct Sampler {
    pub(crate) sampler: *mut bindings::ISampler,
    virtual_functions: *mut bindings::ISamplerVtbl,

    device_object: DeviceObject,
}

impl AsDeviceObject for Sampler {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Sampler {
    pub(crate) fn new(sampler_ptr: *mut bindings::ISampler) -> Self {
        Sampler {
            sampler: sampler_ptr,
            virtual_functions: unsafe { (*sampler_ptr).pVtbl },
            device_object: DeviceObject::new(sampler_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> &bindings::SamplerDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.sampler as *mut bindings::IDeviceObject)
                as *const bindings::SamplerDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}
