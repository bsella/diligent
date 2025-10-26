use std::{ffi::CString, num::NonZero, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert_eq;

use crate::{
    buffer::Buffer,
    device_context::DeviceContext,
    device_object::DeviceObject,
    graphics_types::{BindFlags, CpuAccessFlags, MapFlags, ResourceState, TextureFormat, Usage},
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
                buffer.sys_ptr as _
            } else {
                std::ptr::null_mut()
            },
            DepthStride: value.depth_stride,
            SrcOffset: value.source_offset,
            Stride: value.stride,
        }
    }
}

#[derive(Builder)]
pub struct TextureDesc {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: Option<CString>,

    dimension: TextureDimension,

    width: u32,

    height: u32,

    format: TextureFormat,

    #[builder(default = 1)]
    mip_levels: u32,

    #[builder(default = 1)]
    sample_count: u32,

    #[builder(default)]
    bind_flags: BindFlags,

    #[builder(default)]
    usage: Usage,

    #[builder(default)]
    cpu_access_flags: CpuAccessFlags,

    #[builder(default)]
    misc_flags: MiscTextureFlags,

    #[builder(default = [0.0, 0.0, 0.0, 0.0])]
    clear_color: [f32; 4],

    #[builder(default = 1.0)]
    clear_depth: f32,

    #[builder(default = 0)]
    clear_stencil: u8,

    #[builder(default = 1)]
    immediate_context_mask: u64,
}

impl From<&TextureDesc> for diligent_sys::TextureDesc {
    fn from(value: &TextureDesc) -> Self {
        let anon = match value.dimension {
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
        };

        diligent_sys::TextureDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            Type: value.dimension.into(),
            Width: value.width,
            Height: value.height,
            Format: value.format.into(),
            MipLevels: value.mip_levels,
            SampleCount: value.sample_count,
            BindFlags: value.bind_flags.bits(),
            Usage: value.usage.into(),
            CPUAccessFlags: value.cpu_access_flags.bits(),
            MiscFlags: value.misc_flags.bits(),
            ClearValue: diligent_sys::OptimizedClearValue {
                Color: value.clear_color,
                DepthStencil: diligent_sys::DepthStencilClearValue {
                    Depth: value.clear_depth,
                    Stencil: value.clear_stencil,
                },
                Format: value.format.into(),
            },
            ImmediateContextMask: value.immediate_context_mask,
            __bindgen_anon_1: anon,
        }
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

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureMethods>(),
    6 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct Texture(DeviceObject);

impl Deref for Texture {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Texture {
    pub(crate) fn new(texture_ptr: *mut diligent_sys::ITexture) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IDeviceObject>(),
            std::mem::size_of::<diligent_sys::ITexture>()
        );

        Self(DeviceObject::new(
            texture_ptr as *mut diligent_sys::IDeviceObject,
        ))
    }

    pub fn create_view(&self, texture_view_desc: &TextureViewDesc) -> Result<TextureView, ()> {
        let mut texture_view_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            Texture,
            CreateView,
            std::ptr::from_ref(&texture_view_desc) as _,
            std::ptr::addr_of_mut!(texture_view_ptr)
        );

        if texture_view_ptr.is_null() {
            Err(())
        } else {
            Ok(TextureView::new(texture_view_ptr))
        }
    }

    pub fn get_default_view(&self, texture_view_type: TextureViewType) -> Option<TextureView> {
        let texture_view_ptr =
            unsafe_member_call!(self, Texture, GetDefaultView, texture_view_type.into());
        if texture_view_ptr.is_null() {
            None
        } else {
            let texture_view = TextureView::new(texture_view_ptr);
            texture_view.add_ref();

            Some(texture_view)
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

pub struct TextureSubresourceReadMapToken<'a, T> {
    device_context: &'a DeviceContext,
    texture: &'a Texture,
    mip_level: u32,
    array_slice: u32,
    data_ptr: *const T,
}

impl<'a, T> TextureSubresourceReadMapToken<'a, T> {
    pub(super) fn new(
        device_context: &'a DeviceContext,
        texture: &'a Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<diligent_sys::Box>,
    ) -> TextureSubresourceReadMapToken<'a, T> {
        let ptr = std::ptr::null_mut();
        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapTextureSubresource,
            texture.sys_ptr as _,
            mip_level,
            array_slice,
            diligent_sys::MAP_READ as diligent_sys::MAP_TYPE,
            map_flags.bits(),
            map_region
                .as_ref()
                .map_or(std::ptr::null(), std::ptr::from_ref),
            ptr
        );

        TextureSubresourceReadMapToken {
            device_context,
            texture,
            mip_level,
            array_slice,
            data_ptr: ptr as *mut T,
        }
    }

    pub unsafe fn as_ref(&self) -> &T {
        unsafe { self.data_ptr.as_ref().unwrap_unchecked() }
    }

    pub unsafe fn as_slice(&self, len: usize, offset: isize) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data_ptr.offset(offset), len) }
    }
}

impl<T> Drop for TextureSubresourceReadMapToken<'_, T> {
    fn drop(&mut self) {
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapTextureSubresource,
            self.texture.sys_ptr as _,
            self.mip_level,
            self.array_slice
        )
    }
}

pub struct TextureSubresourceWriteMapToken<'a, T> {
    device_context: &'a DeviceContext,
    texture: &'a Texture,
    mip_level: u32,
    array_slice: u32,
    data_ptr: *mut T,
}

impl<'a, T> TextureSubresourceWriteMapToken<'a, T> {
    pub(super) fn new(
        device_context: &'a DeviceContext,
        texture: &'a Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<diligent_sys::Box>,
    ) -> TextureSubresourceWriteMapToken<'a, T> {
        let ptr = std::ptr::null_mut();
        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapTextureSubresource,
            texture.sys_ptr as _,
            mip_level,
            array_slice,
            diligent_sys::MAP_WRITE as diligent_sys::MAP_TYPE,
            map_flags.bits(),
            map_region
                .as_ref()
                .map_or(std::ptr::null(), std::ptr::from_ref),
            ptr
        );

        TextureSubresourceWriteMapToken {
            device_context,
            texture,
            mip_level,
            array_slice,
            data_ptr: ptr as *mut T,
        }
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        unsafe { self.data_ptr.as_mut().unwrap_unchecked() }
    }

    pub unsafe fn as_mut_slice(&mut self, len: usize, offset: isize) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data_ptr.offset(offset), len) }
    }
}

impl<T> Drop for TextureSubresourceWriteMapToken<'_, T> {
    fn drop(&mut self) {
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapTextureSubresource,
            self.texture.sys_ptr as _,
            self.mip_level,
            self.array_slice
        )
    }
}

pub struct TextureSubresourceReadWriteMapToken<'a, T> {
    device_context: &'a DeviceContext,
    texture: &'a Texture,
    mip_level: u32,
    array_slice: u32,
    data_ptr: *mut T,
}

impl<'a, T> TextureSubresourceReadWriteMapToken<'a, T> {
    pub(super) fn new(
        device_context: &'a DeviceContext,
        texture: &'a Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<diligent_sys::Box>,
    ) -> TextureSubresourceReadWriteMapToken<'a, T> {
        let ptr = std::ptr::null_mut();
        unsafe_member_call!(
            device_context,
            DeviceContext,
            MapTextureSubresource,
            texture.sys_ptr as _,
            mip_level,
            array_slice,
            diligent_sys::MAP_READ_WRITE as diligent_sys::MAP_TYPE,
            map_flags.bits(),
            map_region
                .as_ref()
                .map_or(std::ptr::null(), std::ptr::from_ref),
            ptr
        );

        TextureSubresourceReadWriteMapToken {
            device_context,
            texture,
            mip_level,
            array_slice,
            data_ptr: ptr as *mut T,
        }
    }

    pub unsafe fn as_ref(&self) -> &T {
        unsafe { self.data_ptr.as_ref().unwrap_unchecked() }
    }

    pub unsafe fn as_slice(&self, len: usize, offset: isize) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data_ptr.offset(offset), len) }
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        unsafe { self.data_ptr.as_mut().unwrap_unchecked() }
    }

    pub unsafe fn as_mut_slice(&mut self, len: usize, offset: isize) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data_ptr.offset(offset), len) }
    }
}

impl<T> Drop for TextureSubresourceReadWriteMapToken<'_, T> {
    fn drop(&mut self) {
        unsafe_member_call!(
            self.device_context,
            DeviceContext,
            UnmapTextureSubresource,
            self.texture.sys_ptr as _,
            self.mip_level,
            self.array_slice
        )
    }
}
