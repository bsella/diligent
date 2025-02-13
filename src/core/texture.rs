use bitflags::bitflags;
use static_assertions::const_assert;

use crate::bindings;

use super::buffer::Buffer;
use super::graphics_types::{BindFlags, CpuAccessFlags, Usage};
use super::texture_view::{TextureView, TextureViewType};

use super::device_object::{AsDeviceObject, DeviceObject};
use super::object::AsObject;

pub enum TextureDimension {
    Texture1D,
    Texture1DArray { array_size: u32 },
    Texture2D,
    Texture2DArray { array_size: u32 },
    Texture3D { depth: u32 },
    TextureCube,
    TextureCubeArray { array_size: u32 },
}
const_assert!(bindings::RESOURCE_DIM_NUM_DIMENSIONS == 9);

impl From<&TextureDimension> for bindings::RESOURCE_DIMENSION {
    fn from(value: &TextureDimension) -> Self {
        (match value {
            TextureDimension::Texture1D => bindings::RESOURCE_DIM_TEX_1D,
            TextureDimension::Texture1DArray { array_size: _ } => {
                bindings::RESOURCE_DIM_TEX_1D_ARRAY
            }
            TextureDimension::Texture2D => bindings::RESOURCE_DIM_TEX_2D,
            TextureDimension::Texture2DArray { array_size: _ } => {
                bindings::RESOURCE_DIM_TEX_2D_ARRAY
            }
            TextureDimension::Texture3D { depth: _ } => bindings::RESOURCE_DIM_TEX_3D,
            TextureDimension::TextureCube => bindings::RESOURCE_DIM_TEX_CUBE,
            TextureDimension::TextureCubeArray { array_size: _ } => {
                bindings::RESOURCE_DIM_TEX_CUBE_ARRAY
            }
        }) as bindings::RESOURCE_DIMENSION
    }
}

bitflags! {
    pub struct MiscTextureFlags: bindings::_MISC_TEXTURE_FLAGS {
        const None           = bindings::MISC_TEXTURE_FLAG_NONE;
        const GenerateMips   = bindings::MISC_TEXTURE_FLAG_GENERATE_MIPS;
        const Memoryless     = bindings::MISC_TEXTURE_FLAG_MEMORYLESS;
        const SparseAliasing = bindings::MISC_TEXTURE_FLAG_SPARSE_ALIASING;
        const Subsampled     = bindings::MISC_TEXTURE_FLAG_SUBSAMPLED;
    }
}

pub enum TextureSubResData<'a> {
    CPU(&'a [u8]),
    GPU(&'a Buffer),
}

pub struct TextureSubResource<'a> {
    source: TextureSubResData<'a>,
    source_offset: u64,
    stride: u64,
    depth_stride: u64,
}

impl<'a> TextureSubResource<'a> {
    pub fn new_cpu(data: &'a [u8], stride: u64) -> Self {
        TextureSubResource {
            source: TextureSubResData::CPU(data),
            stride,
            source_offset: 0,
            depth_stride: 0,
        }
    }

    pub fn new_gpu(data: &'a Buffer, source_offset: u64, stride: u64) -> Self {
        TextureSubResource {
            source: TextureSubResData::GPU(data),
            stride,
            source_offset,
            depth_stride: 0,
        }
    }

    pub fn depth_stride(mut self, depth_stride: u64) -> Self {
        self.depth_stride = depth_stride;
        self
    }
}

impl From<&TextureSubResource<'_>> for bindings::TextureSubResData {
    fn from(value: &TextureSubResource<'_>) -> Self {
        bindings::TextureSubResData {
            pData: if let TextureSubResData::CPU(data) = value.source {
                data.as_ptr() as *const std::ffi::c_void
            } else {
                std::ptr::null()
            },
            pSrcBuffer: if let TextureSubResData::GPU(buffer) = value.source {
                buffer.buffer
            } else {
                std::ptr::null_mut()
            },
            DepthStride: value.depth_stride,
            SrcOffset: value.source_offset,
            Stride: value.stride,
        }
    }
}

pub struct TextureDesc<'a> {
    name: &'a std::ffi::CStr,

    dimension: TextureDimension,
    width: u32,
    height: u32,
    format: bindings::_TEXTURE_FORMAT,

    mip_levels: u32,
    sample_count: u32,
    bind_flags: BindFlags,
    usage: Usage,
    cpu_access_flags: CpuAccessFlags,
    misc_flags: MiscTextureFlags,
    clear_color: [f32; 4],
    clear_depth: f32,
    clear_stencil: u8,
    immediate_context_mask: u64,
}

impl From<&TextureDesc<'_>> for bindings::TextureDesc {
    fn from(value: &TextureDesc<'_>) -> Self {
        let anon = match value.dimension {
            TextureDimension::Texture1DArray { array_size }
            | TextureDimension::Texture2DArray { array_size }
            | TextureDimension::TextureCubeArray { array_size } => {
                bindings::TextureDesc__bindgen_ty_1 {
                    ArraySize: array_size,
                }
            }
            TextureDimension::Texture3D { depth } => {
                bindings::TextureDesc__bindgen_ty_1 { Depth: depth }
            }
            _ => bindings::TextureDesc__bindgen_ty_1 { ArraySize: 1 },
        };

        bindings::TextureDesc {
            _DeviceObjectAttribs: bindings::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            Type: bindings::RESOURCE_DIMENSION::from(&value.dimension),
            Width: value.width,
            Height: value.height,
            Format: value.format as bindings::TEXTURE_FORMAT,
            MipLevels: value.mip_levels,
            SampleCount: value.sample_count,
            BindFlags: value.bind_flags.bits(),
            Usage: bindings::USAGE::from(&value.usage),
            CPUAccessFlags: value.cpu_access_flags.bits() as u8,
            MiscFlags: value.misc_flags.bits() as u8,
            ClearValue: bindings::OptimizedClearValue {
                Color: value.clear_color,
                DepthStencil: bindings::DepthStencilClearValue {
                    Depth: value.clear_depth,
                    Stencil: value.clear_stencil,
                },
                Format: value.format as bindings::TEXTURE_FORMAT,
            },
            ImmediateContextMask: value.immediate_context_mask,
            __bindgen_anon_1: anon,
        }
    }
}

impl<'a> TextureDesc<'a> {
    pub fn new(
        name: &'a std::ffi::CStr,
        dimension: TextureDimension,
        width: u32,
        height: u32,
        format: bindings::_TEXTURE_FORMAT,
    ) -> Self {
        TextureDesc {
            name,

            dimension,
            width,
            height,
            format,

            mip_levels: 1,
            sample_count: 1,
            bind_flags: BindFlags::None,
            usage: Usage::Default,
            cpu_access_flags: CpuAccessFlags::None,
            misc_flags: MiscTextureFlags::None,
            clear_color: [0.0, 0.0, 0.0, 0.0],
            clear_depth: 1.0,
            clear_stencil: 0,
            immediate_context_mask: 1,
        }
    }

    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.mip_levels = mip_levels;
        self
    }
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }
    pub fn bind_flags(mut self, bind_flags: BindFlags) -> Self {
        self.bind_flags = bind_flags;
        self
    }
    pub fn usage(mut self, usage: Usage) -> Self {
        self.usage = usage;
        self
    }
    pub fn cpu_access_flags(mut self, cpu_access_flags: CpuAccessFlags) -> Self {
        self.cpu_access_flags = cpu_access_flags;
        self
    }
    pub fn misc_flags(mut self, misc_flags: MiscTextureFlags) -> Self {
        self.misc_flags = misc_flags;
        self
    }
    pub fn clear_color(mut self, clear_color: [f32; 4]) -> Self {
        self.clear_color = clear_color;
        self
    }
    pub fn clear_depth(mut self, clear_depth: f32) -> Self {
        self.clear_depth = clear_depth;
        self
    }
    pub fn clear_stencil(mut self, clear_stencil: u8) -> Self {
        self.clear_stencil = clear_stencil;
        self
    }
    pub fn immediate_context_mask(mut self, immediate_context_mask: u64) -> Self {
        self.immediate_context_mask = immediate_context_mask;
        self
    }
}

pub struct Texture {
    pub(crate) texture: *mut bindings::ITexture,
    virtual_functions: *mut bindings::ITextureVtbl,

    device_object: DeviceObject,
}

impl AsDeviceObject for Texture {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Texture {
    pub(crate) fn new(texture_ptr: *mut bindings::ITexture) -> Self {
        Texture {
            device_object: DeviceObject::new(texture_ptr as *mut bindings::IDeviceObject),
            texture: texture_ptr,
            virtual_functions: unsafe { (*texture_ptr).pVtbl },
        }
    }

    pub fn get_desc(&self) -> &bindings::TextureDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.texture as *mut bindings::IDeviceObject)
                as *const bindings::TextureDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn create_view(
        &mut self,
        texture_view_desc: &bindings::TextureViewDesc,
    ) -> Option<TextureView> {
        let mut texture_view_ptr: *mut bindings::ITextureView = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .Texture
                .CreateView
                .unwrap_unchecked()(
                self.texture,
                std::ptr::addr_of!(texture_view_desc) as *const bindings::TextureViewDesc,
                std::ptr::addr_of_mut!(texture_view_ptr),
            );
        }

        if texture_view_ptr.is_null() {
            None
        } else {
            Some(TextureView::new(texture_view_ptr, self as *const Self))
        }
    }

    pub fn get_default_view(&self, texture_view_type: TextureViewType) -> Option<TextureView> {
        let texture_view_ptr = unsafe {
            (*self.virtual_functions)
                .Texture
                .GetDefaultView
                .unwrap_unchecked()(
                self.texture,
                bindings::TEXTURE_VIEW_TYPE::from(&texture_view_type),
            )
        };
        if texture_view_ptr.is_null() {
            None
        } else {
            let texture_view = TextureView::new(texture_view_ptr, std::ptr::addr_of!(*self));
            texture_view.as_device_object().as_object().add_ref();

            Some(texture_view)
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetNativeHandle
                .unwrap_unchecked()(self.texture)
        }
    }

    pub fn set_state(&mut self, state: bindings::RESOURCE_STATE) {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .SetState
                .unwrap_unchecked()(self.texture, state);
        }
    }

    pub fn get_state(&self) -> bindings::RESOURCE_STATE {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetState
                .unwrap_unchecked()(self.texture)
        }
    }

    pub fn get_sparse_properties(&self) -> &bindings::SparseTextureProperties {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetSparseProperties
                .unwrap_unchecked()(self.texture)
            .as_ref()
            .unwrap_unchecked()
        }
    }
}
