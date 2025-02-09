use bitflags::bitflags;
use static_assertions::const_assert;

use crate::bindings;

use super::graphics_types::{BindFlags, CpuAccessFlags, Usage};
use super::texture_view::TextureView;

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

impl Into<bindings::RESOURCE_DIMENSION> for TextureDimension {
    fn into(self) -> bindings::RESOURCE_DIMENSION {
        (match self {
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

impl<'a> Into<bindings::TextureDesc> for TextureDesc<'a> {
    fn into(self) -> bindings::TextureDesc {
        let anon = match self.dimension {
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
            _ => bindings::TextureDesc__bindgen_ty_1 { ArraySize: 0 },
        };

        bindings::TextureDesc {
            _DeviceObjectAttribs: bindings::DeviceObjectAttribs {
                Name: self.name.as_ptr(),
            },
            Type: self.dimension.into(),
            Width: self.width,
            Height: self.height,
            Format: self.format as bindings::TEXTURE_FORMAT,
            MipLevels: self.mip_levels,
            SampleCount: self.sample_count,
            BindFlags: self.bind_flags.bits(),
            Usage: self.usage.into(),
            CPUAccessFlags: self.cpu_access_flags.bits() as u8,
            MiscFlags: self.misc_flags.bits() as u8,
            ClearValue: bindings::OptimizedClearValue {
                Color: self.clear_color,
                DepthStencil: bindings::DepthStencilClearValue {
                    Depth: self.clear_depth,
                    Stencil: self.clear_stencil,
                },
                Format: self.format as bindings::TEXTURE_FORMAT,
            },
            ImmediateContextMask: self.immediate_context_mask,
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

    default_view: Option<TextureView>,

    device_object: DeviceObject,
}

impl AsDeviceObject for Texture {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Texture {
    pub(crate) fn new(texture_ptr: *mut bindings::ITexture) -> Self {
        let mut texture = Texture {
            device_object: DeviceObject::new(texture_ptr as *mut bindings::IDeviceObject),
            texture: texture_ptr,
            virtual_functions: unsafe { (*texture_ptr).pVtbl },
            default_view: None,
        };

        fn bind_flags_to_texture_view_type(
            bind_flag: bindings::BIND_FLAGS,
        ) -> bindings::_TEXTURE_VIEW_TYPE {
            if bind_flag & bindings::BIND_SHADER_RESOURCE != 0 {
                bindings::TEXTURE_VIEW_SHADER_RESOURCE
            } else if bind_flag & bindings::BIND_RENDER_TARGET != 0 {
                bindings::BUFFER_VIEW_SHADER_RESOURCE
            } else if bind_flag & bindings::BIND_DEPTH_STENCIL != 0 {
                bindings::TEXTURE_VIEW_DEPTH_STENCIL
            } else if bind_flag & bindings::BIND_UNORDERED_ACCESS != 0 {
                bindings::TEXTURE_VIEW_UNORDERED_ACCESS
            } else if bind_flag & bindings::BIND_SHADING_RATE != 0 {
                bindings::TEXTURE_VIEW_SHADING_RATE
            } else {
                bindings::TEXTURE_VIEW_UNDEFINED
            }
        }

        let texture_desc = unsafe {
            &*((*(*texture_ptr).pVtbl)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(texture_ptr as *mut bindings::IDeviceObject)
                as *const bindings::TextureDesc)
        };

        let texture_view_type = bind_flags_to_texture_view_type(texture_desc.BindFlags);

        if texture_view_type != bindings::BUFFER_VIEW_UNDEFINED {
            let texture_view = TextureView::new(
                unsafe {
                    (*(*texture_ptr).pVtbl)
                        .Texture
                        .GetDefaultView
                        .unwrap_unchecked()(texture_ptr, texture_view_type as u8)
                },
                std::ptr::addr_of!(texture),
            );
            texture_view.as_device_object().as_object().add_ref();
            texture.default_view = Some(texture_view);
        }

        texture
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

    pub fn get_default_view(
        &self,
        texture_view_type: bindings::TEXTURE_VIEW_TYPE,
    ) -> Option<&TextureView> {
        if unsafe {
            (*self.virtual_functions)
                .Texture
                .GetDefaultView
                .unwrap_unchecked()(self.texture, texture_view_type)
        }
        .is_null()
        {
            None
        } else {
            self.default_view.as_ref()
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
