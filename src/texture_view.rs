use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{
    Ported,
    device_object::{
        DeviceObject, DeviceObjectAttribs, ResourceStateNoTransition, ResourceStateTransition,
        ResourceStateVerify,
    },
    graphics_types::TextureFormat,
    sampler::Sampler,
    texture::{Texture, TextureDimension},
};

#[derive(Clone, Copy)]
pub enum TextureViewType {
    ShaderResource,
    RenderTarget,
    DepthStencil,
    ReadOnlyDepthStencil,
    UnorderedAccess,
    ShadingRate,
}

impl From<TextureViewType> for diligent_sys::TEXTURE_VIEW_TYPE {
    fn from(value: TextureViewType) -> Self {
        (match value {
            TextureViewType::ShaderResource => diligent_sys::TEXTURE_VIEW_SHADER_RESOURCE,
            TextureViewType::RenderTarget => diligent_sys::TEXTURE_VIEW_RENDER_TARGET,
            TextureViewType::DepthStencil => diligent_sys::TEXTURE_VIEW_DEPTH_STENCIL,
            TextureViewType::ReadOnlyDepthStencil => {
                diligent_sys::TEXTURE_VIEW_READ_ONLY_DEPTH_STENCIL
            }
            TextureViewType::UnorderedAccess => diligent_sys::TEXTURE_VIEW_UNORDERED_ACCESS,
            TextureViewType::ShadingRate => diligent_sys::TEXTURE_VIEW_SHADING_RATE,
        }) as _
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct UavAccessFlags: diligent_sys::UAV_ACCESS_FLAG {
        const Unspecified = diligent_sys::UAV_ACCESS_UNSPECIFIED as diligent_sys::UAV_ACCESS_FLAG;
        const Read        = diligent_sys::UAV_ACCESS_FLAG_READ as diligent_sys::UAV_ACCESS_FLAG;
        const Write       = diligent_sys::UAV_ACCESS_FLAG_WRITE as diligent_sys::UAV_ACCESS_FLAG;
        const ReadWrite   = diligent_sys::UAV_ACCESS_FLAG_READ_WRITE as diligent_sys::UAV_ACCESS_FLAG;
    }
}

const_assert_eq!(diligent_sys::UAV_ACCESS_FLAG_LAST, 3);

bitflags! {
    #[derive(Clone,Copy)]
    pub struct TextureViewFlags: diligent_sys::TEXTURE_VIEW_FLAGS {
        const None                  = diligent_sys::TEXTURE_VIEW_FLAG_NONE as diligent_sys::TEXTURE_VIEW_FLAGS;
        const AllowMipMapGeneration = diligent_sys::TEXTURE_VIEW_FLAG_ALLOW_MIP_MAP_GENERATION as diligent_sys::TEXTURE_VIEW_FLAGS;
    }
}

const_assert_eq!(diligent_sys::TEXTURE_VIEW_FLAG_LAST, 1);

#[derive(Clone, Copy)]
pub enum TextureComponentSwizzle {
    Identity,
    Zero,
    One,
    R,
    G,
    B,
    A,
}
const_assert_eq!(diligent_sys::TEXTURE_COMPONENT_SWIZZLE_COUNT, 7);

impl From<TextureComponentSwizzle> for diligent_sys::TEXTURE_COMPONENT_SWIZZLE {
    fn from(value: TextureComponentSwizzle) -> Self {
        (match value {
            TextureComponentSwizzle::Identity => diligent_sys::TEXTURE_COMPONENT_SWIZZLE_IDENTITY,
            TextureComponentSwizzle::Zero => diligent_sys::TEXTURE_COMPONENT_SWIZZLE_ZERO,
            TextureComponentSwizzle::One => diligent_sys::TEXTURE_COMPONENT_SWIZZLE_ONE,
            TextureComponentSwizzle::R => diligent_sys::TEXTURE_COMPONENT_SWIZZLE_R,
            TextureComponentSwizzle::G => diligent_sys::TEXTURE_COMPONENT_SWIZZLE_G,
            TextureComponentSwizzle::B => diligent_sys::TEXTURE_COMPONENT_SWIZZLE_B,
            TextureComponentSwizzle::A => diligent_sys::TEXTURE_COMPONENT_SWIZZLE_A,
        }) as _
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct TextureComponentMapping(diligent_sys::TextureComponentMapping);

#[bon::bon]
impl TextureComponentMapping {
    #[builder(derive(Clone))]
    pub fn new(
        #[builder(default = TextureComponentSwizzle::Identity)] r: TextureComponentSwizzle,
        #[builder(default = TextureComponentSwizzle::Identity)] g: TextureComponentSwizzle,
        #[builder(default = TextureComponentSwizzle::Identity)] b: TextureComponentSwizzle,
        #[builder(default = TextureComponentSwizzle::Identity)] a: TextureComponentSwizzle,
    ) -> Self {
        Self(diligent_sys::TextureComponentMapping {
            R: r as diligent_sys::TEXTURE_COMPONENT_SWIZZLE,
            G: g as diligent_sys::TEXTURE_COMPONENT_SWIZZLE,
            B: b as diligent_sys::TEXTURE_COMPONENT_SWIZZLE,
            A: a as diligent_sys::TEXTURE_COMPONENT_SWIZZLE,
        })
    }
}

impl TextureComponentMapping {
    #[rustfmt::skip]
    pub fn is_identity(&self) -> bool {
            ( self.0.R == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_IDENTITY as diligent_sys::TEXTURE_COMPONENT_SWIZZLE
        ||    self.0.R == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_R        as diligent_sys::TEXTURE_COMPONENT_SWIZZLE)
        &&  ( self.0.G == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_IDENTITY as diligent_sys::TEXTURE_COMPONENT_SWIZZLE
        ||    self.0.G == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_G        as diligent_sys::TEXTURE_COMPONENT_SWIZZLE)
        &&  ( self.0.B == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_IDENTITY as diligent_sys::TEXTURE_COMPONENT_SWIZZLE
        ||    self.0.B == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_B        as diligent_sys::TEXTURE_COMPONENT_SWIZZLE)
        &&  ( self.0.A == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_IDENTITY as diligent_sys::TEXTURE_COMPONENT_SWIZZLE
        ||    self.0.A == diligent_sys::TEXTURE_COMPONENT_SWIZZLE_A        as diligent_sys::TEXTURE_COMPONENT_SWIZZLE)
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct TextureViewDesc<'name>(
    pub(crate) diligent_sys::TextureViewDesc,
    PhantomData<&'name ()>,
);

impl Deref for TextureViewDesc<'_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name> TextureViewDesc<'name> {
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,
        view_type: TextureViewType,
        dimension: Option<TextureDimension>,
        num_array_or_depth_slices: usize,
        first_array_or_depth_slice: usize,
        format: Option<TextureFormat>,
        #[builder(default = 0)] most_detailed_mip: usize,
        #[builder(default = 0)] num_mip_levels: usize,
        #[builder(default = UavAccessFlags::Unspecified)] access_flags: UavAccessFlags,
        #[builder(default = TextureViewFlags::None)] flags: TextureViewFlags,
        #[builder(default = TextureComponentMapping::builder().build())]
        swizzle: TextureComponentMapping,
    ) -> Self {
        Self(
            diligent_sys::TextureViewDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.as_ref().map_or(std::ptr::null(), |name| name.as_ptr()),
                },
                ViewType: view_type.into(),
                TextureDim: dimension
                    .map_or(diligent_sys::RESOURCE_DIM_UNDEFINED as _, |dim| dim.into()),
                Format: format.map_or(diligent_sys::TEX_FORMAT_UNKNOWN as _, |dim| dim.into()),
                MostDetailedMip: most_detailed_mip as u32,
                NumMipLevels: num_mip_levels as u32,
                __bindgen_anon_1: diligent_sys::TextureViewDesc__bindgen_ty_1 {
                    FirstArraySlice: first_array_or_depth_slice as u32,
                },
                __bindgen_anon_2: diligent_sys::TextureViewDesc__bindgen_ty_2 {
                    NumArraySlices: num_array_or_depth_slices as u32,
                },
                AccessFlags: access_flags.bits(),
                Flags: flags.bits(),
                Swizzle: swizzle.0,
            },
            PhantomData,
        )
    }
}

define_ported!(
    TextureView,
    diligent_sys::ITextureView,
    diligent_sys::ITextureViewMethods : 3,
    DeviceObject
);

impl TextureView {
    pub fn desc(&self) -> &TextureViewDesc<'_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const TextureViewDesc) }
    }

    pub fn set_sampler(&mut self, sampler: &Sampler) {
        unsafe_member_call!(self, TextureView, SetSampler, sampler.sys_ptr());
    }

    pub fn get_sampler(&self) -> Option<&Sampler> {
        let sampler_ptr = unsafe_member_call!(self, TextureView, GetSampler);

        if sampler_ptr.is_null() {
            None
        } else {
            unsafe { Some(&*(sampler_ptr as *const Sampler)) }
        }
    }

    pub fn get_texture(&self) -> &Texture {
        let texture_ptr = unsafe_member_call!(self, TextureView, GetTexture);
        unsafe { &*(texture_ptr as *const Texture) }
    }

    pub fn get_texture_mut(&mut self) -> &mut Texture {
        let texture_ptr = unsafe_member_call!(self, TextureView, GetTexture);
        unsafe { &mut *(texture_ptr as *mut Texture) }
    }
}

impl TextureView {
    pub fn transition_state(&mut self) -> ResourceStateTransition<'_, TextureView> {
        ResourceStateTransition::new(self)
    }
    pub fn verify_state(&self) -> ResourceStateVerify<'_, TextureView> {
        ResourceStateVerify::new(self)
    }
    pub fn no_state_transition(&self) -> ResourceStateNoTransition<'_, TextureView> {
        ResourceStateNoTransition::new(self)
    }
}
