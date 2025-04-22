use std::ffi::CString;

use bitflags::bitflags;
use static_assertions::const_assert;

use crate::graphics_types::ResourceState;

use super::buffer::Buffer;
use super::graphics_types::{BindFlags, CpuAccessFlags, TextureFormat, Usage};
use super::texture_view::{TextureView, TextureViewType};

use super::device_object::DeviceObject;

pub enum TextureDimension {
    Texture1D,
    Texture1DArray { array_size: u32 },
    Texture2D,
    Texture2DArray { array_size: u32 },
    Texture3D { depth: u32 },
    TextureCube,
    TextureCubeArray { array_size: u32 },
}
const_assert!(diligent_sys::RESOURCE_DIM_NUM_DIMENSIONS == 9);

impl From<&TextureDimension> for diligent_sys::RESOURCE_DIMENSION {
    fn from(value: &TextureDimension) -> Self {
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
        }) as diligent_sys::RESOURCE_DIMENSION
    }
}

bitflags! {
    pub struct MiscTextureFlags: diligent_sys::MISC_TEXTURE_FLAGS {
        const None           = diligent_sys::MISC_TEXTURE_FLAG_NONE as diligent_sys::MISC_TEXTURE_FLAGS;
        const GenerateMips   = diligent_sys::MISC_TEXTURE_FLAG_GENERATE_MIPS as diligent_sys::MISC_TEXTURE_FLAGS;
        const Memoryless     = diligent_sys::MISC_TEXTURE_FLAG_MEMORYLESS as diligent_sys::MISC_TEXTURE_FLAGS;
        const SparseAliasing = diligent_sys::MISC_TEXTURE_FLAG_SPARSE_ALIASING as diligent_sys::MISC_TEXTURE_FLAGS;
        const Subsampled     = diligent_sys::MISC_TEXTURE_FLAG_SUBSAMPLED as diligent_sys::MISC_TEXTURE_FLAGS;
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

impl From<&TextureSubResource<'_>> for diligent_sys::TextureSubResData {
    fn from(value: &TextureSubResource<'_>) -> Self {
        diligent_sys::TextureSubResData {
            pData: if let TextureSubResData::CPU(data) = value.source {
                data.as_ptr() as *const std::ffi::c_void
            } else {
                std::ptr::null()
            },
            pSrcBuffer: if let TextureSubResData::GPU(buffer) = value.source {
                buffer.sys_ptr
            } else {
                std::ptr::null_mut()
            },
            DepthStride: value.depth_stride,
            SrcOffset: value.source_offset,
            Stride: value.stride,
        }
    }
}

pub struct TextureDesc {
    name: CString,

    dimension: TextureDimension,
    width: u32,
    height: u32,
    format: TextureFormat,

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

impl From<&TextureDesc> for diligent_sys::TextureDesc {
    fn from(value: &TextureDesc) -> Self {
        let anon = match value.dimension {
            TextureDimension::Texture1DArray { array_size }
            | TextureDimension::Texture2DArray { array_size }
            | TextureDimension::TextureCubeArray { array_size } => {
                diligent_sys::TextureDesc__bindgen_ty_1 {
                    ArraySize: array_size,
                }
            }
            TextureDimension::Texture3D { depth } => {
                diligent_sys::TextureDesc__bindgen_ty_1 { Depth: depth }
            }
            _ => diligent_sys::TextureDesc__bindgen_ty_1 { ArraySize: 1 },
        };

        diligent_sys::TextureDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            Type: diligent_sys::RESOURCE_DIMENSION::from(&value.dimension),
            Width: value.width,
            Height: value.height,
            Format: diligent_sys::TEXTURE_FORMAT::from(&value.format),
            MipLevels: value.mip_levels,
            SampleCount: value.sample_count,
            BindFlags: value.bind_flags.bits(),
            Usage: diligent_sys::USAGE::from(&value.usage),
            CPUAccessFlags: value.cpu_access_flags.bits(),
            MiscFlags: value.misc_flags.bits(),
            ClearValue: diligent_sys::OptimizedClearValue {
                Color: value.clear_color,
                DepthStencil: diligent_sys::DepthStencilClearValue {
                    Depth: value.clear_depth,
                    Stencil: value.clear_stencil,
                },
                Format: diligent_sys::TEXTURE_FORMAT::from(&value.format),
            },
            ImmediateContextMask: value.immediate_context_mask,
            __bindgen_anon_1: anon,
        }
    }
}

impl TextureDesc {
    pub fn new(
        name: impl AsRef<str>,
        dimension: TextureDimension,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Self {
        TextureDesc {
            name: CString::new(name.as_ref()).unwrap(),

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
    pub(crate) sys_ptr: *mut diligent_sys::ITexture,
    virtual_functions: *mut diligent_sys::ITextureVtbl,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for Texture {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Texture {
    pub(crate) fn new(texture_ptr: *mut diligent_sys::ITexture) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::ITexture>()
        );

        Texture {
            sys_ptr: texture_ptr,
            virtual_functions: unsafe { (*texture_ptr).pVtbl },
            device_object: DeviceObject::new(texture_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::TextureDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.device_object.sys_ptr)
                as *const diligent_sys::TextureDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn create_view(
        &mut self,
        texture_view_desc: &diligent_sys::TextureViewDesc,
    ) -> Result<TextureView, ()> {
        let mut texture_view_ptr: *mut diligent_sys::ITextureView = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .Texture
                .CreateView
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::addr_of!(texture_view_desc) as *const diligent_sys::TextureViewDesc,
                std::ptr::addr_of_mut!(texture_view_ptr),
            );
        }

        if texture_view_ptr.is_null() {
            Err(())
        } else {
            Ok(TextureView::new(texture_view_ptr, self as *const Self))
        }
    }

    pub fn get_default_view(&self, texture_view_type: TextureViewType) -> Result<TextureView, ()> {
        let texture_view_ptr = unsafe {
            (*self.virtual_functions)
                .Texture
                .GetDefaultView
                .unwrap_unchecked()(
                self.sys_ptr,
                diligent_sys::TEXTURE_VIEW_TYPE::from(&texture_view_type),
            )
        };
        if texture_view_ptr.is_null() {
            Err(())
        } else {
            let texture_view = TextureView::new(texture_view_ptr, std::ptr::addr_of!(*self));
            texture_view.as_ref().as_ref().add_ref();

            Ok(texture_view)
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetNativeHandle
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn set_state(&mut self, state: ResourceState) {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .SetState
                .unwrap_unchecked()(self.sys_ptr, state.bits());
        }
    }

    pub fn get_state(&self) -> ResourceState {
        let state = unsafe {
            (*self.virtual_functions)
                .Texture
                .GetState
                .unwrap_unchecked()(self.sys_ptr)
        };
        ResourceState::from_bits_retain(state)
    }

    pub fn get_sparse_properties(&self) -> &diligent_sys::SparseTextureProperties {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetSparseProperties
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }
}
