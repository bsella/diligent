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
    name: &'a std::ffi::CStr,
    min_filter: FilterType,
    mag_filter: FilterType,
    mip_filter: FilterType,
    address_u: TextureAddressMode,
    address_v: TextureAddressMode,
    address_w: TextureAddressMode,
    flags: SamplerFlags,
    unnormalized_coords: bool,
    mip_lod_bias: f32,
    max_anisotropy: u32,
    comparison_func: ComparisonFunction,
    border_color: [f32; 4usize],
    min_lod: f32,
    max_lod: f32,
}

impl<'a> SamplerDesc<'a> {
    pub fn new(name: &'a std::ffi::CStr) -> Self {
        SamplerDesc {
            name,
            min_filter: FilterType::Linear,
            mag_filter: FilterType::Linear,
            mip_filter: FilterType::Linear,

            address_u: TextureAddressMode::Clamp,
            address_v: TextureAddressMode::Clamp,
            address_w: TextureAddressMode::Clamp,
            flags: SamplerFlags::None,
            unnormalized_coords: false,
            mip_lod_bias: 0.0,
            max_anisotropy: 0,
            comparison_func: ComparisonFunction::Never,
            border_color: [0.0, 0.0, 0.0, 0.0],
            min_lod: 0.0,
            max_lod: f32::MAX,
        }
    }

    pub fn min_filter(mut self, min_filter: FilterType) -> Self {
        self.min_filter = min_filter;
        self
    }
    pub fn mag_filter(mut self, mag_filter: FilterType) -> Self {
        self.mag_filter = mag_filter;
        self
    }
    pub fn mip_filter(mut self, mip_filter: FilterType) -> Self {
        self.mip_filter = mip_filter;
        self
    }
    pub fn address_u(mut self, address_u: TextureAddressMode) -> Self {
        self.address_u = address_u;
        self
    }
    pub fn address_v(mut self, address_v: TextureAddressMode) -> Self {
        self.address_v = address_v;
        self
    }
    pub fn address_w(mut self, address_w: TextureAddressMode) -> Self {
        self.address_w = address_w;
        self
    }
    pub fn flags(mut self, flags: SamplerFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn unnormalized_coords(mut self, unnormalized_coords: bool) -> Self {
        self.unnormalized_coords = unnormalized_coords;
        self
    }
    pub fn mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.mip_lod_bias = mip_lod_bias;
        self
    }
    pub fn max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.max_anisotropy = max_anisotropy;
        self
    }
    pub fn comparison_func(mut self, comparison_func: ComparisonFunction) -> Self {
        self.comparison_func = comparison_func;
        self
    }
    pub fn border_color(mut self, border_color: [f32; 4usize]) -> Self {
        self.border_color = border_color;
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.min_lod = min_lod;
        self
    }
    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.max_lod = max_lod;
        self
    }
}

impl From<&SamplerDesc<'_>> for bindings::SamplerDesc {
    fn from(value: &SamplerDesc<'_>) -> Self {
        bindings::SamplerDesc {
            _DeviceObjectAttribs: bindings::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            MinFilter: bindings::FILTER_TYPE::from(&value.min_filter),
            MagFilter: bindings::FILTER_TYPE::from(&value.mag_filter),
            MipFilter: bindings::FILTER_TYPE::from(&value.mip_filter),
            AddressU: bindings::TEXTURE_ADDRESS_MODE::from(&value.address_u),
            AddressV: bindings::TEXTURE_ADDRESS_MODE::from(&value.address_v),
            AddressW: bindings::TEXTURE_ADDRESS_MODE::from(&value.address_w),
            Flags: value.flags.bits() as bindings::SAMPLER_FLAGS,
            UnnormalizedCoords: value.unnormalized_coords,
            MipLODBias: value.mip_lod_bias,
            MaxAnisotropy: value.max_anisotropy,
            ComparisonFunc: bindings::COMPARISON_FUNCTION::from(&value.comparison_func),
            BorderColor: value.border_color,
            MinLOD: value.min_lod,
            MaxLOD: value.max_lod,
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

    pub fn get_desc(&self) -> &bindings::SamplerDesc {
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
