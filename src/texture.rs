use std::{
    ffi::CStr,
    marker::PhantomData,
    num::NonZero,
    ops::{Deref, DerefMut},
};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert_eq;

use crate::{
    Boxed, MapType,
    buffer::Buffer,
    device_context::DeviceContext,
    device_object::DeviceObject,
    graphics_types::{BindFlags, CpuAccessFlags, ResourceState, TextureFormat, Usage},
    resource_access_states,
    texture_view::{TextureView, TextureViewDesc, TextureViewType},
};

#[derive(Clone, Copy)]
pub enum TextureDimension {
    Texture1D,
    Texture1DArray { array_size: NonZero<usize> },
    Texture2D,
    Texture2DArray { array_size: NonZero<usize> },
    Texture3D { depth: NonZero<usize> },
    TextureCube,
    TextureCubeArray { array_size: NonZero<usize> },
}
const_assert_eq!(diligent_sys::RESOURCE_DIM_NUM_DIMENSIONS, 9);

impl From<TextureDimension> for diligent_sys::RESOURCE_DIMENSION {
    fn from(value: TextureDimension) -> Self {
        (match value {
            TextureDimension::Texture1D => diligent_sys::RESOURCE_DIM_TEX_1D,
            TextureDimension::Texture1DArray { array_size: _ } => {
                diligent_sys::RESOURCE_DIM_TEX_1D_ARRAY
            }
            TextureDimension::Texture2D => diligent_sys::RESOURCE_DIM_TEX_2D,
            TextureDimension::Texture2DArray { array_size: _ } => {
                diligent_sys::RESOURCE_DIM_TEX_2D_ARRAY
            }
            TextureDimension::Texture3D { depth: _ } => diligent_sys::RESOURCE_DIM_TEX_3D,
            TextureDimension::TextureCube => diligent_sys::RESOURCE_DIM_TEX_CUBE,
            TextureDimension::TextureCubeArray { array_size: _ } => {
                diligent_sys::RESOURCE_DIM_TEX_CUBE_ARRAY
            }
        }) as _
    }
}

impl TextureDimension {
    pub fn is_array(&self) -> bool {
        matches!(
            &self,
            TextureDimension::Texture1DArray { array_size: _ }
                | TextureDimension::Texture2DArray { array_size: _ }
                | TextureDimension::TextureCubeArray { array_size: _ }
        )
    }

    #[allow(non_snake_case)]
    pub fn is_1D(&self) -> bool {
        matches!(
            &self,
            TextureDimension::Texture1D | TextureDimension::Texture1DArray { array_size: _ }
        )
    }

    #[allow(non_snake_case)]
    pub fn is_2D(&self) -> bool {
        matches!(
            &self,
            TextureDimension::Texture2D
                | TextureDimension::Texture2DArray { array_size: _ }
                | TextureDimension::TextureCube
                | TextureDimension::TextureCubeArray { array_size: _ }
        )
    }

    #[allow(non_snake_case)]
    pub fn is_3D(&self) -> bool {
        matches!(&self, TextureDimension::Texture3D { depth: _ })
    }

    pub fn is_cube(&self) -> bool {
        matches!(
            &self,
            TextureDimension::TextureCube | TextureDimension::TextureCubeArray { array_size: _ }
        )
    }

    pub fn array_size(&self) -> usize {
        match self {
            TextureDimension::Texture1DArray { array_size } => array_size.get(),
            TextureDimension::Texture2DArray { array_size } => array_size.get(),
            TextureDimension::TextureCubeArray { array_size } => array_size.get(),
            _ => 1,
        }
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct MiscTextureFlags: diligent_sys::MISC_TEXTURE_FLAGS {
        const None           = diligent_sys::MISC_TEXTURE_FLAG_NONE as diligent_sys::MISC_TEXTURE_FLAGS;
        const GenerateMips   = diligent_sys::MISC_TEXTURE_FLAG_GENERATE_MIPS as diligent_sys::MISC_TEXTURE_FLAGS;
        const Memoryless     = diligent_sys::MISC_TEXTURE_FLAG_MEMORYLESS as diligent_sys::MISC_TEXTURE_FLAGS;
        const SparseAliasing = diligent_sys::MISC_TEXTURE_FLAG_SPARSE_ALIASING as diligent_sys::MISC_TEXTURE_FLAGS;
        const Subsampled     = diligent_sys::MISC_TEXTURE_FLAG_SUBSAMPLED as diligent_sys::MISC_TEXTURE_FLAGS;
    }
}

impl Default for MiscTextureFlags {
    fn default() -> Self {
        MiscTextureFlags::None
    }
}

pub enum TextureSubResData<'a> {
    CPU(&'a [u8]),
    GPU(&'a Buffer),
}

#[derive(Builder)]
pub struct TextureSubResource<'a> {
    #[builder(setters(vis = ""))]
    source: TextureSubResData<'a>,

    #[builder(setters(vis = ""))]
    source_offset: u64,

    #[builder(setters(vis = ""))]
    stride: u64,

    #[builder(default = 0)]
    depth_stride: u64,
}

use texture_sub_resource_builder::{IsUnset, SetSource, SetSourceOffset, SetStride, State};
impl<'a, S: State> TextureSubResourceBuilder<'a, S> {
    pub fn from_host(
        self,
        data: &'a [u8],
        stride: u64,
    ) -> TextureSubResourceBuilder<'a, SetSource<SetSourceOffset<SetStride<S>>>>
    where
        S::Source: IsUnset,
        S::SourceOffset: IsUnset,
        S::Stride: IsUnset,
    {
        self.stride(stride)
            .source_offset(0)
            .source(TextureSubResData::CPU(data))
    }
}

impl<'a, S: State> TextureSubResourceBuilder<'a, S> {
    pub fn from_device(
        self,
        data: &'a Buffer,
        source_offset: u64,
        stride: u64,
    ) -> TextureSubResourceBuilder<'a, SetSource<SetSourceOffset<SetStride<S>>>>
    where
        S::Source: IsUnset,
        S::SourceOffset: IsUnset,
        S::Stride: IsUnset,
    {
        self.stride(stride)
            .source_offset(source_offset)
            .source(TextureSubResData::GPU(data))
    }
}

impl From<&TextureSubResource<'_>> for diligent_sys::TextureSubResData {
    fn from(value: &TextureSubResource<'_>) -> Self {
        diligent_sys::TextureSubResData {
            pData: if let TextureSubResData::CPU(data) = value.source {
                data.as_ptr() as *const std::ffi::c_void
            } else {
                std::ptr::null()
            },
            pSrcBuffer: if let TextureSubResData::GPU(buffer) = value.source {
                buffer.sys_ptr() as _
            } else {
                std::ptr::null_mut()
            },
            DepthStride: value.depth_stride,
            SrcOffset: value.source_offset,
            Stride: value.stride,
        }
    }
}

#[repr(transparent)]
pub struct TextureDesc(pub(crate) diligent_sys::TextureDesc);

#[bon::bon]
impl TextureDesc {
    #[builder]
    pub fn new(
        name: Option<&CStr>,

        dimension: TextureDimension,

        width: u32,

        height: u32,

        format: TextureFormat,

        #[builder(default = 1)] mip_levels: u32,

        #[builder(default = 1)] sample_count: u32,

        #[builder(default)] bind_flags: BindFlags,

        #[builder(default)] usage: Usage,

        #[builder(default)] cpu_access_flags: CpuAccessFlags,

        #[builder(default)] misc_flags: MiscTextureFlags,

        #[builder(default = [0.0, 0.0, 0.0, 0.0])] clear_color: [f32; 4],

        #[builder(default = 1.0)] clear_depth: f32,

        #[builder(default = 0)] clear_stencil: u8,

        #[builder(default = 1)] immediate_context_mask: u64,
    ) -> Self {
        Self(diligent_sys::TextureDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.as_ref().map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            Type: dimension.into(),
            Width: width,
            Height: height,
            Format: format.into(),
            MipLevels: mip_levels,
            SampleCount: sample_count,
            BindFlags: bind_flags.bits(),
            Usage: usage.into(),
            CPUAccessFlags: cpu_access_flags.bits(),
            MiscFlags: misc_flags.bits(),
            ClearValue: diligent_sys::OptimizedClearValue {
                Color: clear_color,
                DepthStencil: diligent_sys::DepthStencilClearValue {
                    Depth: clear_depth,
                    Stencil: clear_stencil,
                },
                Format: format.into(),
            },
            ImmediateContextMask: immediate_context_mask,
            __bindgen_anon_1: match dimension {
                TextureDimension::Texture1DArray { array_size }
                | TextureDimension::Texture2DArray { array_size }
                | TextureDimension::TextureCubeArray { array_size } => {
                    diligent_sys::TextureDesc__bindgen_ty_1 {
                        ArraySize: array_size.get() as u32,
                    }
                }
                TextureDimension::Texture3D { depth } => diligent_sys::TextureDesc__bindgen_ty_1 {
                    Depth: depth.get() as u32,
                },
                _ => diligent_sys::TextureDesc__bindgen_ty_1 { ArraySize: 1 },
            },
        })
    }
}

impl TextureDesc {
    pub fn dimension(&self) -> TextureDimension {
        let array_size =
            || NonZero::new(unsafe { self.0.__bindgen_anon_1.ArraySize as usize }).unwrap();

        match self.0.Type as _ {
            diligent_sys::RESOURCE_DIM_TEX_1D => TextureDimension::Texture1D,
            diligent_sys::RESOURCE_DIM_TEX_1D_ARRAY => TextureDimension::Texture1DArray {
                array_size: array_size(),
            },
            diligent_sys::RESOURCE_DIM_TEX_2D => TextureDimension::Texture2D,
            diligent_sys::RESOURCE_DIM_TEX_2D_ARRAY => TextureDimension::Texture2DArray {
                array_size: array_size(),
            },
            diligent_sys::RESOURCE_DIM_TEX_3D => TextureDimension::Texture3D {
                depth: NonZero::new(unsafe { self.0.__bindgen_anon_1.Depth as usize }).unwrap(),
            },
            diligent_sys::RESOURCE_DIM_TEX_CUBE => TextureDimension::TextureCube,
            diligent_sys::RESOURCE_DIM_TEX_CUBE_ARRAY => TextureDimension::TextureCubeArray {
                array_size: array_size(),
            },
            _ => panic!("Unknown RESOURCE_DIM value"),
        }
    }
    pub fn width(&self) -> u32 {
        self.0.Width
    }
    pub fn height(&self) -> u32 {
        self.0.Height
    }
    pub fn depth(&self) -> u32 {
        unsafe { self.0.__bindgen_anon_1.Depth }
    }
    pub fn format(&self) -> TextureFormat {
        self.0.Format.into()
    }
    pub fn mip_levels(&self) -> u32 {
        self.0.MipLevels
    }
    pub fn sample_count(&self) -> u32 {
        self.0.SampleCount
    }
    pub fn bind_flags(&self) -> BindFlags {
        BindFlags::from_bits_retain(self.0.BindFlags)
    }
    pub fn usage(&self) -> Usage {
        self.0.Usage.into()
    }
    pub fn cpu_access_flags(&self) -> CpuAccessFlags {
        CpuAccessFlags::from_bits_retain(self.0.CPUAccessFlags)
    }
    pub fn misc_flags(&self) -> MiscTextureFlags {
        MiscTextureFlags::from_bits_retain(self.0.MiscFlags)
    }
    pub fn clear_color(&self) -> &[f32; 4] {
        &self.0.ClearValue.Color
    }
    pub fn clear_depth(&self) -> f32 {
        self.0.ClearValue.DepthStencil.Depth
    }
    pub fn clear_stencil(&self) -> u8 {
        self.0.ClearValue.DepthStencil.Stencil
    }
    pub fn immediate_context_mask(&self) -> u64 {
        self.0.ImmediateContextMask
    }
}

impl TextureDesc {
    pub fn mip_level_logical_width(&self, mip_level: u32) -> u32 {
        u32::max(self.width() >> mip_level, 1)
    }
    pub fn mip_level_logical_height(&self, mip_level: u32) -> u32 {
        u32::max(self.height() >> mip_level, 1)
    }
    pub fn mip_level_depth(&self, mip_level: u32) -> u32 {
        u32::max(self.depth() >> mip_level, 1)
    }

    pub fn mip_level_storage_width(&self, mip_level: u32) -> usize {
        if let Some(crate::ComponentType::Compressed) = self.format().component_type() {
            todo!()
        } else {
            self.mip_level_logical_width(mip_level) as usize
        }
    }

    pub fn mip_level_storage_height(&self, mip_level: u32) -> usize {
        if let Some(crate::ComponentType::Compressed) = self.format().component_type() {
            todo!()
        } else {
            self.mip_level_logical_height(mip_level) as usize
        }
    }

    pub fn mip_level_row_size(&self, mip_level: u32) -> usize {
        if let Some(crate::ComponentType::Compressed) = self.format().component_type() {
            todo!()
        } else {
            self.mip_level_storage_width(mip_level)
                * self.format().component_size() as usize
                * self.format().num_components() as usize
        }
    }

    pub fn mip_level_depth_slice_size(&self, mip_level: u32) -> usize {
        if let Some(crate::ComponentType::Compressed) = self.format().component_type() {
            todo!()
        } else {
            self.mip_level_row_size(mip_level) * self.mip_level_storage_height(mip_level)
        }
    }

    pub fn mip_level_mip_size(&self, mip_level: u32) -> usize {
        self.mip_level_depth_slice_size(mip_level) * self.mip_level_depth(mip_level) as usize
    }

    pub fn get_standard_sparse_texture_properties(&self) -> SparseTextureProperties {
        todo!()
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct SparseTextureFlags: diligent_sys::SPARSE_TEXTURE_FLAGS {
        const None                 = diligent_sys::SPARSE_TEXTURE_FLAG_NONE as diligent_sys::SPARSE_TEXTURE_FLAGS;
        const SingleMiptail        = diligent_sys::SPARSE_TEXTURE_FLAG_SINGLE_MIPTAIL as diligent_sys::SPARSE_TEXTURE_FLAGS;
        const AlignedMipSize       = diligent_sys::SPARSE_TEXTURE_FLAG_ALIGNED_MIP_SIZE as diligent_sys::SPARSE_TEXTURE_FLAGS;
        const NonStandardBlockSize = diligent_sys::SPARSE_TEXTURE_FLAG_NONSTANDARD_BLOCK_SIZE as diligent_sys::SPARSE_TEXTURE_FLAGS;
    }
}
const_assert_eq!(diligent_sys::SPARSE_TEXTURE_FLAG_LAST, 4);

#[repr(transparent)]
pub struct SparseTextureProperties(diligent_sys::SparseTextureProperties);

#[bon::bon]
impl SparseTextureProperties {
    #[builder]
    pub fn new(
        #[builder(default = 0)] address_space_size: u64,
        #[builder(default = 0)] mip_tail_offset: u64,
        #[builder(default = 0)] mip_tail_stride: u64,
        #[builder(default = 0)] mip_tail_size: u64,
        #[builder(default = !0)] first_mip_in_tail: u32,
        #[builder(default = [0;_])] tile_size: [u32; 3usize],
        #[builder(default = 0)] block_size: u32,
        #[builder(default = SparseTextureFlags::None)] flags: SparseTextureFlags,
    ) -> Self {
        Self(diligent_sys::SparseTextureProperties {
            AddressSpaceSize: address_space_size,
            MipTailOffset: mip_tail_offset,
            MipTailStride: mip_tail_stride,
            MipTailSize: mip_tail_size,
            FirstMipInTail: first_mip_in_tail,
            TileSize: tile_size,
            BlockSize: block_size,
            Flags: flags.bits(),
        })
    }
}

define_ported!(
    Texture,
    diligent_sys::ITexture,
    diligent_sys::ITextureMethods : 6,
    DeviceObject
);

impl Texture {
    pub fn desc(&self) -> &TextureDesc {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const TextureDesc) }
    }

    pub fn create_view(
        &self,
        texture_view_desc: &TextureViewDesc,
    ) -> Result<Boxed<TextureView>, ()> {
        let mut texture_view_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            Texture,
            CreateView,
            &texture_view_desc.0,
            &mut texture_view_ptr
        );

        if texture_view_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::new(texture_view_ptr))
        }
    }

    pub fn get_default_view(&self, texture_view_type: TextureViewType) -> Option<&TextureView> {
        let texture_view_ptr =
            unsafe_member_call!(self, Texture, GetDefaultView, texture_view_type.into());
        if texture_view_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(texture_view_ptr as *const TextureView) })
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe_member_call!(self, Texture, GetNativeHandle)
    }

    pub fn set_state(&mut self, state: ResourceState) {
        unsafe_member_call!(self, Texture, SetState, state.bits());
    }

    pub fn get_state(&self) -> ResourceState {
        let state = unsafe_member_call!(self, Texture, GetState);
        ResourceState::from_bits_retain(state)
    }

    pub fn get_sparse_properties(&self) -> &SparseTextureProperties {
        let properties_ptr = unsafe_member_call!(self, Texture, GetSparseProperties);
        unsafe { &*(properties_ptr as *const &SparseTextureProperties) }
    }
}

pub struct TextureMapToken<'a, T: Sized, State: MapType> {
    device_context: &'a DeviceContext,
    texture: &'a Texture,
    data: &'a mut [T],
    stride: u64,
    depth_stride: u64,
    mip_level: u32,
    array_slice: u32,
    phantom: PhantomData<State>,
}

impl<'a, T: Sized, State: MapType> TextureMapToken<'a, T, State> {
    #[cfg(any(feature = "d3d11", feature = "d3d12", feature = "vulkan"))]
    pub(super) fn new(
        device_context: &'a DeviceContext,
        texture: &'a Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: crate::MapFlags,
        map_region: Option<crate::Box>,
    ) -> TextureMapToken<'a, T, State> {
        let mut mapped_resource = std::mem::MaybeUninit::uninit();

        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapTextureSubresource,
            texture.sys_ptr(),
            mip_level,
            array_slice,
            diligent_sys::MAP_READ as diligent_sys::MAP_TYPE,
            map_flags.bits(),
            map_region.map_or(std::ptr::null(), |bx| { &bx.0 }),
            mapped_resource.as_mut_ptr()
        );

        let mapped_resource = unsafe { mapped_resource.assume_init() };

        TextureMapToken::<T, State> {
            device_context,
            texture,
            mip_level,
            array_slice,
            data: unsafe {
                std::slice::from_raw_parts_mut(
                    mapped_resource.pData as *mut T,
                    texture.desc().mip_level_mip_size(mip_level) / std::mem::size_of::<T>(),
                )
            },
            stride: mapped_resource.Stride,
            depth_stride: mapped_resource.DepthStride,
            phantom: PhantomData,
        }
    }

    pub fn stride(&self) -> u64 {
        self.stride
    }

    pub fn depth_stride(&self) -> u64 {
        self.depth_stride
    }
}

impl<'a, T: Sized, State: MapType> Drop for TextureMapToken<'a, T, State> {
    fn drop(&mut self) {
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapTextureSubresource,
            self.texture.sys_ptr(),
            self.mip_level,
            self.array_slice
        )
    }
}

impl<T> Deref for TextureMapToken<'_, T, resource_access_states::Read> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

// Note : Normally you shouldn't be able to read from the write token,
//        but DerefMut cannot be implemented without Deref.
impl<'a, T> Deref for TextureMapToken<'a, T, resource_access_states::Write> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> DerefMut for TextureMapToken<'a, T, resource_access_states::Write> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, T> Deref for TextureMapToken<'a, T, resource_access_states::ReadWrite> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> DerefMut for TextureMapToken<'a, T, resource_access_states::ReadWrite> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

pub type TextureMapReadToken<'a, T> = TextureMapToken<'a, T, resource_access_states::Read>;
pub type TextureMapWriteToken<'a, T> = TextureMapToken<'a, T, resource_access_states::Write>;
pub type TextureMapReadWriteToken<'a, T> =
    TextureMapToken<'a, T, resource_access_states::ReadWrite>;
