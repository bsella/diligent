use std::{
    ffi::{CStr, CString},
    fmt::Display,
};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::{const_assert, const_assert_eq};

use crate::SparseTextureFlags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct ShaderTypes: diligent_sys::SHADER_TYPE {
        const Vertex          = diligent_sys::SHADER_TYPE_VERTEX as diligent_sys::SHADER_TYPE;
        const Pixel           = diligent_sys::SHADER_TYPE_PIXEL as diligent_sys::SHADER_TYPE;
        const Geometry        = diligent_sys::SHADER_TYPE_GEOMETRY as diligent_sys::SHADER_TYPE;
        const Hull            = diligent_sys::SHADER_TYPE_HULL as diligent_sys::SHADER_TYPE;
        const Domain          = diligent_sys::SHADER_TYPE_DOMAIN as diligent_sys::SHADER_TYPE;
        const Compute         = diligent_sys::SHADER_TYPE_COMPUTE as diligent_sys::SHADER_TYPE;
        const Amplification   = diligent_sys::SHADER_TYPE_AMPLIFICATION as diligent_sys::SHADER_TYPE;
        const Mesh            = diligent_sys::SHADER_TYPE_MESH as diligent_sys::SHADER_TYPE;
        const RayGen          = diligent_sys::SHADER_TYPE_RAY_GEN as diligent_sys::SHADER_TYPE;
        const RayMiss         = diligent_sys::SHADER_TYPE_RAY_MISS as diligent_sys::SHADER_TYPE;
        const RayClosestHit   = diligent_sys::SHADER_TYPE_RAY_CLOSEST_HIT as diligent_sys::SHADER_TYPE;
        const RayAnyHit       = diligent_sys::SHADER_TYPE_RAY_ANY_HIT as diligent_sys::SHADER_TYPE;
        const RayIntersection = diligent_sys::SHADER_TYPE_RAY_INTERSECTION as diligent_sys::SHADER_TYPE;
        const Callable        = diligent_sys::SHADER_TYPE_CALLABLE as diligent_sys::SHADER_TYPE;
        const Tile            = diligent_sys::SHADER_TYPE_TILE as diligent_sys::SHADER_TYPE;

        const VertexPixel   = diligent_sys::SHADER_TYPE_VS_PS as diligent_sys::SHADER_TYPE;
        const AllGraphics   = diligent_sys::SHADER_TYPE_ALL_GRAPHICS as diligent_sys::SHADER_TYPE;
        const AllMesh       = diligent_sys::SHADER_TYPE_ALL_MESH as diligent_sys::SHADER_TYPE;
        const AllRayTracing = diligent_sys::SHADER_TYPE_ALL_RAY_TRACING as diligent_sys::SHADER_TYPE;
        const All           = diligent_sys::SHADER_TYPE_ALL as diligent_sys::SHADER_TYPE;
    }
}
const_assert_eq!(diligent_sys::SHADER_TYPE_LAST, 16384);

#[derive(Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Pixel,
    Geometry,
    Hull,
    Domain,
    Compute,
    Amplification,
    Mesh,
    RayGen,
    RayMiss,
    RayClosestHit,
    RayAnyHit,
    RayIntersection,
    Callable,
    Tile,
}

impl From<ShaderType> for diligent_sys::SHADER_TYPE {
    fn from(value: ShaderType) -> Self {
        (match value {
            ShaderType::Vertex => diligent_sys::SHADER_TYPE_VERTEX,
            ShaderType::Pixel => diligent_sys::SHADER_TYPE_PIXEL,
            ShaderType::Geometry => diligent_sys::SHADER_TYPE_GEOMETRY,
            ShaderType::Hull => diligent_sys::SHADER_TYPE_HULL,
            ShaderType::Domain => diligent_sys::SHADER_TYPE_DOMAIN,
            ShaderType::Compute => diligent_sys::SHADER_TYPE_COMPUTE,
            ShaderType::Amplification => diligent_sys::SHADER_TYPE_AMPLIFICATION,
            ShaderType::Mesh => diligent_sys::SHADER_TYPE_MESH,
            ShaderType::RayGen => diligent_sys::SHADER_TYPE_RAY_GEN,
            ShaderType::RayMiss => diligent_sys::SHADER_TYPE_RAY_MISS,
            ShaderType::RayClosestHit => diligent_sys::SHADER_TYPE_RAY_CLOSEST_HIT,
            ShaderType::RayAnyHit => diligent_sys::SHADER_TYPE_RAY_ANY_HIT,
            ShaderType::RayIntersection => diligent_sys::SHADER_TYPE_RAY_INTERSECTION,
            ShaderType::Callable => diligent_sys::SHADER_TYPE_CALLABLE,
            ShaderType::Tile => diligent_sys::SHADER_TYPE_TILE,
        }) as _
    }
}

#[derive(Clone, Copy)]
pub enum FilterType {
    Point,
    Linear,
    Anisotropic,
    ComparisonPoint,
    ComparisonLinear,
    ComparisonAnisotropic,
    MinimumPoint,
    MinimumLinear,
    MinimumAnisotropic,
    MaximumPoint,
    MaximumLinear,
    MaximumAnisotropic,
}
const_assert_eq!(diligent_sys::FILTER_TYPE_NUM_FILTERS, 13);

impl From<FilterType> for diligent_sys::FILTER_TYPE {
    fn from(value: FilterType) -> Self {
        (match value {
            FilterType::Point => diligent_sys::FILTER_TYPE_POINT,
            FilterType::Linear => diligent_sys::FILTER_TYPE_LINEAR,
            FilterType::Anisotropic => diligent_sys::FILTER_TYPE_ANISOTROPIC,
            FilterType::ComparisonPoint => diligent_sys::FILTER_TYPE_COMPARISON_POINT,
            FilterType::ComparisonLinear => diligent_sys::FILTER_TYPE_COMPARISON_LINEAR,
            FilterType::ComparisonAnisotropic => diligent_sys::FILTER_TYPE_COMPARISON_ANISOTROPIC,
            FilterType::MinimumPoint => diligent_sys::FILTER_TYPE_MINIMUM_POINT,
            FilterType::MinimumLinear => diligent_sys::FILTER_TYPE_MINIMUM_LINEAR,
            FilterType::MinimumAnisotropic => diligent_sys::FILTER_TYPE_MINIMUM_ANISOTROPIC,
            FilterType::MaximumPoint => diligent_sys::FILTER_TYPE_MAXIMUM_POINT,
            FilterType::MaximumLinear => diligent_sys::FILTER_TYPE_MAXIMUM_LINEAR,
            FilterType::MaximumAnisotropic => diligent_sys::FILTER_TYPE_MAXIMUM_ANISOTROPIC,
        }) as _
    }
}

impl From<diligent_sys::FILTER_TYPE> for FilterType {
    fn from(value: diligent_sys::FILTER_TYPE) -> Self {
        match value as _ {
            diligent_sys::FILTER_TYPE_POINT => FilterType::Point,
            diligent_sys::FILTER_TYPE_LINEAR => FilterType::Linear,
            diligent_sys::FILTER_TYPE_ANISOTROPIC => FilterType::Anisotropic,
            diligent_sys::FILTER_TYPE_COMPARISON_POINT => FilterType::ComparisonPoint,
            diligent_sys::FILTER_TYPE_COMPARISON_LINEAR => FilterType::ComparisonLinear,
            diligent_sys::FILTER_TYPE_COMPARISON_ANISOTROPIC => FilterType::ComparisonAnisotropic,
            diligent_sys::FILTER_TYPE_MINIMUM_POINT => FilterType::MinimumPoint,
            diligent_sys::FILTER_TYPE_MINIMUM_LINEAR => FilterType::MinimumLinear,
            diligent_sys::FILTER_TYPE_MINIMUM_ANISOTROPIC => FilterType::MinimumAnisotropic,
            diligent_sys::FILTER_TYPE_MAXIMUM_POINT => FilterType::MaximumPoint,
            diligent_sys::FILTER_TYPE_MAXIMUM_LINEAR => FilterType::MaximumLinear,
            diligent_sys::FILTER_TYPE_MAXIMUM_ANISOTROPIC => FilterType::MaximumAnisotropic,
            _ => panic!("Unknown filter type"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TextureAddressMode {
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}
const_assert_eq!(diligent_sys::TEXTURE_ADDRESS_NUM_MODES, 6);

impl From<TextureAddressMode> for diligent_sys::TEXTURE_ADDRESS_MODE {
    fn from(value: TextureAddressMode) -> Self {
        (match value {
            TextureAddressMode::Wrap => diligent_sys::TEXTURE_ADDRESS_WRAP,
            TextureAddressMode::Mirror => diligent_sys::TEXTURE_ADDRESS_MIRROR,
            TextureAddressMode::Clamp => diligent_sys::TEXTURE_ADDRESS_CLAMP,
            TextureAddressMode::Border => diligent_sys::TEXTURE_ADDRESS_BORDER,
            TextureAddressMode::MirrorOnce => diligent_sys::TEXTURE_ADDRESS_MIRROR_ONCE,
        }) as _
    }
}

impl From<diligent_sys::TEXTURE_ADDRESS_MODE> for TextureAddressMode {
    fn from(value: diligent_sys::TEXTURE_ADDRESS_MODE) -> Self {
        match value as _ {
            diligent_sys::TEXTURE_ADDRESS_WRAP => TextureAddressMode::Wrap,
            diligent_sys::TEXTURE_ADDRESS_MIRROR => TextureAddressMode::Mirror,
            diligent_sys::TEXTURE_ADDRESS_CLAMP => TextureAddressMode::Clamp,
            diligent_sys::TEXTURE_ADDRESS_BORDER => TextureAddressMode::Border,
            diligent_sys::TEXTURE_ADDRESS_MIRROR_ONCE => TextureAddressMode::MirrorOnce,
            _ => panic!("Unknown texture address mode"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum PrimitiveTopology {
    TriangleList,
    TriangleStrip,
    PointList,
    LineList,
    LineStrip,
    TriangleListAdj,
    TriangleStripAdj,
    LineListAdj,
    LineStripAdj,
    ControlPointPatchList1,
    ControlPointPatchList2,
    ControlPointPatchList3,
    ControlPointPatchList4,
    ControlPointPatchList5,
    ControlPointPatchList6,
    ControlPointPatchList7,
    ControlPointPatchList8,
    ControlPointPatchList9,
    ControlPointPatchList10,
    ControlPointPatchList11,
    ControlPointPatchList12,
    ControlPointPatchList13,
    ControlPointPatchList14,
    ControlPointPatchList15,
    ControlPointPatchList16,
    ControlPointPatchList17,
    ControlPointPatchList18,
    ControlPointPatchList19,
    ControlPointPatchList20,
    ControlPointPatchList21,
    ControlPointPatchList22,
    ControlPointPatchList23,
    ControlPointPatchList24,
    ControlPointPatchList25,
    ControlPointPatchList26,
    ControlPointPatchList27,
    ControlPointPatchList28,
    ControlPointPatchList29,
    ControlPointPatchList30,
    ControlPointPatchList31,
    ControlPointPatchList32,
}
const_assert_eq!(diligent_sys::PRIMITIVE_TOPOLOGY_NUM_TOPOLOGIES, 42);

impl From<PrimitiveTopology> for diligent_sys::PRIMITIVE_TOPOLOGY {
    fn from(value: PrimitiveTopology) -> Self {
        (match value {
            PrimitiveTopology::TriangleList => diligent_sys::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            PrimitiveTopology::TriangleStrip => diligent_sys::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            PrimitiveTopology::PointList => diligent_sys::PRIMITIVE_TOPOLOGY_POINT_LIST,
            PrimitiveTopology::LineList => diligent_sys::PRIMITIVE_TOPOLOGY_LINE_LIST,
            PrimitiveTopology::LineStrip => diligent_sys::PRIMITIVE_TOPOLOGY_LINE_STRIP,
            PrimitiveTopology::TriangleListAdj => {
                diligent_sys::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_ADJ
            }
            PrimitiveTopology::TriangleStripAdj => {
                diligent_sys::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_ADJ
            }
            PrimitiveTopology::LineListAdj => diligent_sys::PRIMITIVE_TOPOLOGY_LINE_LIST_ADJ,
            PrimitiveTopology::LineStripAdj => diligent_sys::PRIMITIVE_TOPOLOGY_LINE_STRIP_ADJ,
            PrimitiveTopology::ControlPointPatchList1 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList2 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList3 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList4 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList5 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList6 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList7 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList8 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList9 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList10 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList11 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList12 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList13 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList14 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList15 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList16 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList17 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList18 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList19 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList20 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList21 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList22 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList23 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList24 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList25 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList26 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList27 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList28 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList29 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList30 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList31 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList32 => {
                diligent_sys::PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST
            }
        }) as _
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct BindFlags: diligent_sys::BIND_FLAGS {
        const None             = diligent_sys::BIND_NONE as diligent_sys::BIND_FLAGS;
        const VertexBuffer     = diligent_sys::BIND_VERTEX_BUFFER as diligent_sys::BIND_FLAGS;
        const IndexBuffer      = diligent_sys::BIND_INDEX_BUFFER as diligent_sys::BIND_FLAGS;
        const UniformBuffer    = diligent_sys::BIND_UNIFORM_BUFFER as diligent_sys::BIND_FLAGS;
        const ShaderResource   = diligent_sys::BIND_SHADER_RESOURCE as diligent_sys::BIND_FLAGS;
        const StreamOutput     = diligent_sys::BIND_STREAM_OUTPUT as diligent_sys::BIND_FLAGS;
        const RenderTarget     = diligent_sys::BIND_RENDER_TARGET as diligent_sys::BIND_FLAGS;
        const DepthStencil     = diligent_sys::BIND_DEPTH_STENCIL as diligent_sys::BIND_FLAGS;
        const UnorderedAccess  = diligent_sys::BIND_UNORDERED_ACCESS as diligent_sys::BIND_FLAGS;
        const IndirectDrawArgs = diligent_sys::BIND_INDIRECT_DRAW_ARGS as diligent_sys::BIND_FLAGS;
        const InputAttachement = diligent_sys::BIND_INPUT_ATTACHMENT as diligent_sys::BIND_FLAGS;
        const RayTracing       = diligent_sys::BIND_RAY_TRACING as diligent_sys::BIND_FLAGS;
        const ShadingRate      = diligent_sys::BIND_SHADING_RATE as diligent_sys::BIND_FLAGS;
    }
}
const_assert_eq!(diligent_sys::BIND_FLAG_LAST, 2048);

impl Default for BindFlags {
    fn default() -> Self {
        BindFlags::None
    }
}

#[derive(Clone, Copy, Default)]
pub enum Usage {
    #[default]
    Default,
    Immutable,
    Dynamic,
    Staging,
    Unified,
    Sparse,
}
const_assert_eq!(diligent_sys::USAGE_NUM_USAGES, 6);

impl From<Usage> for diligent_sys::USAGE {
    fn from(value: Usage) -> Self {
        (match value {
            Usage::Immutable => diligent_sys::USAGE_IMMUTABLE,
            Usage::Default => diligent_sys::USAGE_DEFAULT,
            Usage::Dynamic => diligent_sys::USAGE_DYNAMIC,
            Usage::Staging => diligent_sys::USAGE_STAGING,
            Usage::Unified => diligent_sys::USAGE_UNIFIED,
            Usage::Sparse => diligent_sys::USAGE_SPARSE,
        }) as _
    }
}

impl From<diligent_sys::USAGE> for Usage {
    fn from(value: diligent_sys::USAGE) -> Self {
        match value as _ {
            diligent_sys::USAGE_IMMUTABLE => Usage::Immutable,
            diligent_sys::USAGE_DEFAULT => Usage::Default,
            diligent_sys::USAGE_DYNAMIC => Usage::Dynamic,
            diligent_sys::USAGE_STAGING => Usage::Staging,
            diligent_sys::USAGE_UNIFIED => Usage::Unified,
            diligent_sys::USAGE_SPARSE => Usage::Sparse,
            _ => panic!("Unknown USAGE value"),
        }
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct CpuAccessFlags: diligent_sys::CPU_ACCESS_FLAGS {
        const None  = diligent_sys::CPU_ACCESS_NONE as diligent_sys::CPU_ACCESS_FLAGS;
        const Read  = diligent_sys::CPU_ACCESS_READ as diligent_sys::CPU_ACCESS_FLAGS;
        const Write = diligent_sys::CPU_ACCESS_WRITE as diligent_sys::CPU_ACCESS_FLAGS;
    }
}
const_assert_eq!(diligent_sys::CPU_ACCESS_FLAG_LAST, 2);

impl Default for CpuAccessFlags {
    fn default() -> Self {
        CpuAccessFlags::None
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct SetShaderResourceFlags: diligent_sys::SET_SHADER_RESOURCE_FLAGS {
        const None          = diligent_sys::SET_SHADER_RESOURCE_FLAG_NONE as diligent_sys::SET_SHADER_RESOURCE_FLAGS;
        const AllowOverrite = diligent_sys::SET_SHADER_RESOURCE_FLAG_ALLOW_OVERWRITE as diligent_sys::SET_SHADER_RESOURCE_FLAGS;
    }
}

impl Default for SetShaderResourceFlags {
    fn default() -> Self {
        SetShaderResourceFlags::None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RenderDeviceType {
    #[cfg(feature = "d3d11")]
    D3D11,
    #[cfg(feature = "d3d12")]
    D3D12,
    #[cfg(feature = "opengl")]
    GL,
    #[cfg(feature = "vulkan")]
    VULKAN,
    #[cfg(feature = "metal")]
    METAL,
    #[cfg(feature = "webgpu")]
    WEBGPU,
}
const_assert_eq!(diligent_sys::RENDER_DEVICE_TYPE_COUNT, 8);

impl Display for RenderDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            #[cfg(feature = "d3d11")]
            RenderDeviceType::D3D11 => "Direct3D11",
            #[cfg(feature = "d3d12")]
            RenderDeviceType::D3D12 => "Direct3D12",
            #[cfg(feature = "opengl")]
            RenderDeviceType::GL => "OpenGL",
            //RenderDeviceType::GLES => "OpenGLES",
            #[cfg(feature = "vulkan")]
            RenderDeviceType::VULKAN => "Vulkan",
            #[cfg(feature = "metal")]
            RenderDeviceType::METAL => "Metal",
            #[cfg(feature = "webgpu")]
            RenderDeviceType::WEBGPU => "WebGPU",
        };
        f.write_str(string)
    }
}

pub const fn get_prefered_device_type() -> RenderDeviceType {
    // Prefer metal, if it's supported. In other words, prefer Metal if you're on Mac.
    #[cfg(feature = "metal")]
    return RenderDeviceType::METAL;

    // If you're on windows, prefer Direct3D12, then Direct3D11.
    #[allow(unreachable_code)]
    #[cfg(feature = "d3d12")]
    return RenderDeviceType::D3D12;

    #[allow(unreachable_code)]
    #[cfg(feature = "d3d11")]
    return RenderDeviceType::D3D11;

    #[allow(unreachable_code)]
    #[cfg(feature = "vulkan")]
    return RenderDeviceType::VULKAN;

    #[allow(unreachable_code)]
    #[cfg(feature = "opengl")]
    return RenderDeviceType::GL;

    #[allow(unreachable_code)]
    #[cfg(feature = "webgpu")]
    return RenderDeviceType::WEBGPU;
}

#[derive(Clone, Copy)]
pub enum ValueType {
    Int8,
    Int16,
    Int32,
    Uint8,
    Uint16,
    Uint32,
    Float16,
    Float32,
    Float64,
}
const_assert_eq!(diligent_sys::VT_NUM_TYPES, 10);

impl From<ValueType> for diligent_sys::VALUE_TYPE {
    fn from(value: ValueType) -> Self {
        (match value {
            ValueType::Int8 => diligent_sys::VT_INT8,
            ValueType::Int16 => diligent_sys::VT_INT16,
            ValueType::Int32 => diligent_sys::VT_INT32,
            ValueType::Uint8 => diligent_sys::VT_UINT8,
            ValueType::Uint16 => diligent_sys::VT_UINT16,
            ValueType::Uint32 => diligent_sys::VT_UINT32,
            ValueType::Float16 => diligent_sys::VT_FLOAT16,
            ValueType::Float32 => diligent_sys::VT_FLOAT32,
            ValueType::Float64 => diligent_sys::VT_FLOAT64,
        }) as _
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct MapFlags: diligent_sys::MAP_FLAGS {
        const None        = diligent_sys::MAP_FLAG_NONE as diligent_sys::MAP_FLAGS;
        const DoNotWait   = diligent_sys::MAP_FLAG_DO_NOT_WAIT as diligent_sys::MAP_FLAGS;
        const Discard     = diligent_sys::MAP_FLAG_DISCARD as diligent_sys::MAP_FLAGS;
        const NoOverwrite = diligent_sys::MAP_FLAG_NO_OVERWRITE as diligent_sys::MAP_FLAGS;
    }
}

#[derive(Clone, Copy)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Version { major, minor }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AdapterType {
    Unknown,
    Software,
    Integrated,
    Discrete,
}

impl AdapterType {
    const fn priority(&self) -> u8 {
        match self {
            AdapterType::Unknown => 0,
            AdapterType::Software => 1,
            AdapterType::Integrated => 2,
            AdapterType::Discrete => 3,
        }
    }
}

// Prefer Discrete over Integrated over Software
const_assert!(
    AdapterType::Discrete.priority() > AdapterType::Integrated.priority()
        && AdapterType::Integrated.priority() > AdapterType::Software.priority()
        && AdapterType::Software.priority() > AdapterType::Unknown.priority()
);

impl Ord for AdapterType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialOrd for AdapterType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<AdapterType> for diligent_sys::ADAPTER_TYPE {
    fn from(value: AdapterType) -> Self {
        match value {
            AdapterType::Unknown => {
                diligent_sys::ADAPTER_TYPE_UNKNOWN as diligent_sys::ADAPTER_TYPE
            }
            AdapterType::Software => {
                diligent_sys::ADAPTER_TYPE_SOFTWARE as diligent_sys::ADAPTER_TYPE
            }
            AdapterType::Integrated => {
                diligent_sys::ADAPTER_TYPE_INTEGRATED as diligent_sys::ADAPTER_TYPE
            }
            AdapterType::Discrete => {
                diligent_sys::ADAPTER_TYPE_DISCRETE as diligent_sys::ADAPTER_TYPE
            }
        }
    }
}

const_assert_eq!(diligent_sys::ADAPTER_TYPE_COUNT, 4);

#[derive(Clone, Copy)]
pub enum AdapterVendor {
    Unknown,
    Nvidia,
    AMD,
    Intel,
    ARM,
    Qualcomm,
    Imgtech,
    Msft,
    Apple,
    Mesa,
    Broadcom,
}
const_assert_eq!(diligent_sys::ADAPTER_VENDOR_LAST, 10);

#[repr(transparent)]
pub struct AdapterMemoryInfo(diligent_sys::AdapterMemoryInfo);

impl AdapterMemoryInfo {
    pub fn local_memory(&self) -> u64 {
        self.0.LocalMemory
    }
    pub fn host_visible_memory(&self) -> u64 {
        self.0.HostVisibleMemory
    }
    pub fn unified_memory(&self) -> u64 {
        self.0.UnifiedMemory
    }
    pub fn max_memory_allocation(&self) -> u64 {
        self.0.MaxMemoryAllocation
    }
    pub fn unified_memory_cpu_access(&self) -> CpuAccessFlags {
        CpuAccessFlags::from_bits_retain(self.0.UnifiedMemoryCPUAccess)
    }
    pub fn memoryless_texture_bind_flags(&self) -> BindFlags {
        BindFlags::from_bits_retain(self.0.MemorylessTextureBindFlags)
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct RaytracingCapFlags : diligent_sys::RAY_TRACING_CAP_FLAGS {
        const None               = diligent_sys::RAY_TRACING_CAP_FLAG_NONE as diligent_sys::RAY_TRACING_CAP_FLAGS;
        const StandaloneShaders  = diligent_sys::RAY_TRACING_CAP_FLAG_STANDALONE_SHADERS as diligent_sys::RAY_TRACING_CAP_FLAGS;
        const InlineRayTracing   = diligent_sys::RAY_TRACING_CAP_FLAG_INLINE_RAY_TRACING as diligent_sys::RAY_TRACING_CAP_FLAGS;
        const IndirectRayTracing = diligent_sys::RAY_TRACING_CAP_FLAG_INDIRECT_RAY_TRACING as diligent_sys::RAY_TRACING_CAP_FLAGS;
    }
}

#[repr(transparent)]
pub struct RayTracingProperties(diligent_sys::RayTracingProperties);
impl RayTracingProperties {
    pub fn max_recursion_depth(&self) -> u32 {
        self.0.MaxRecursionDepth
    }
    pub fn shader_group_handle_size(&self) -> u32 {
        self.0.ShaderGroupHandleSize
    }
    pub fn max_shader_record_stride(&self) -> u32 {
        self.0.MaxShaderRecordStride
    }
    pub fn shader_group_base_alignment(&self) -> u32 {
        self.0.ShaderGroupBaseAlignment
    }
    pub fn max_ray_gen_threads(&self) -> u32 {
        self.0.MaxRayGenThreads
    }
    pub fn max_instances_per_tlas(&self) -> u32 {
        self.0.MaxInstancesPerTLAS
    }
    pub fn max_primitives_per_blas(&self) -> u32 {
        self.0.MaxPrimitivesPerBLAS
    }
    pub fn max_geometries_per_blas(&self) -> u32 {
        self.0.MaxGeometriesPerBLAS
    }
    pub fn vertex_buffer_alignment(&self) -> u32 {
        self.0.VertexBufferAlignment
    }
    pub fn index_buffer_alignment(&self) -> u32 {
        self.0.IndexBufferAlignment
    }
    pub fn transform_buffer_alignment(&self) -> u32 {
        self.0.TransformBufferAlignment
    }
    pub fn box_buffer_alignment(&self) -> u32 {
        self.0.BoxBufferAlignment
    }
    pub fn scratch_buffer_alignment(&self) -> u32 {
        self.0.ScratchBufferAlignment
    }
    pub fn instance_buffer_alignment(&self) -> u32 {
        self.0.IndexBufferAlignment
    }
    pub fn cap_flags(&self) -> RaytracingCapFlags {
        RaytracingCapFlags::from_bits_retain(self.0.CapFlags)
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct WaveFeature : diligent_sys::WAVE_FEATURE {
        const Unknown         = diligent_sys::WAVE_FEATURE_UNKNOWN as diligent_sys::WAVE_FEATURE;
        const Basic           = diligent_sys::WAVE_FEATURE_BASIC as diligent_sys::WAVE_FEATURE;
        const Vote            = diligent_sys::WAVE_FEATURE_VOTE as diligent_sys::WAVE_FEATURE;
        const Arithmetic      = diligent_sys::WAVE_FEATURE_ARITHMETIC as diligent_sys::WAVE_FEATURE;
        const Ballout         = diligent_sys::WAVE_FEATURE_BALLOUT as diligent_sys::WAVE_FEATURE;
        const Shuffle         = diligent_sys::WAVE_FEATURE_SHUFFLE as diligent_sys::WAVE_FEATURE;
        const ShuffleRelative = diligent_sys::WAVE_FEATURE_SHUFFLE_RELATIVE as diligent_sys::WAVE_FEATURE;
        const Clustered       = diligent_sys::WAVE_FEATURE_CLUSTERED as diligent_sys::WAVE_FEATURE;
        const Quad            = diligent_sys::WAVE_FEATURE_QUAD as diligent_sys::WAVE_FEATURE;
    }
}
const_assert_eq!(diligent_sys::WAVE_FEATURE_LAST, 128);

#[repr(transparent)]
pub struct WaveOpProperties(diligent_sys::WaveOpProperties);
impl WaveOpProperties {
    pub fn min_size(&self) -> u32 {
        self.0.MinSize
    }
    pub fn max_size(&self) -> u32 {
        self.0.MaxSize
    }
    pub fn supported_stages(&self) -> ShaderTypes {
        ShaderTypes::from_bits_retain(self.0.SupportedStages)
    }
    pub fn features(&self) -> WaveFeature {
        WaveFeature::from_bits_retain(self.0.MinSize)
    }
}

#[repr(transparent)]
pub struct BufferProperties(diligent_sys::BufferProperties);
impl BufferProperties {
    pub fn constant_buffer_offset_alignment(&self) -> u32 {
        self.0.ConstantBufferOffsetAlignment
    }
    pub fn structured_buffer_offset_alignment(&self) -> u32 {
        self.0.StructuredBufferOffsetAlignment
    }
}

#[repr(transparent)]
pub struct TextureProperties(diligent_sys::TextureProperties);
impl TextureProperties {
    pub fn max_texture1d_dimension(&self) -> u32 {
        self.0.MaxTexture1DDimension
    }
    pub fn max_texture1d_array_slices(&self) -> u32 {
        self.0.MaxTexture1DArraySlices
    }
    pub fn max_texture2d_dimension(&self) -> u32 {
        self.0.MaxTexture2DDimension
    }
    pub fn max_texture2d_array_slices(&self) -> u32 {
        self.0.MaxTexture2DArraySlices
    }
    pub fn max_texture3d_dimension(&self) -> u32 {
        self.0.MaxTexture3DDimension
    }
    pub fn max_texture_cube_dimension(&self) -> u32 {
        self.0.MaxTextureCubeDimension
    }
    pub fn texture2d_ms_supported(&self) -> bool {
        self.0.Texture2DMSSupported
    }
    pub fn texture2d_ms_array_supported(&self) -> bool {
        self.0.Texture2DMSArraySupported
    }
    pub fn texture_view_supported(&self) -> bool {
        self.0.TextureViewSupported
    }
    pub fn cubemap_arrays_supported(&self) -> bool {
        self.0.CubemapArraysSupported
    }
    pub fn texture_view_2d_on_3d_supported(&self) -> bool {
        self.0.TextureView2DOn3DSupported
    }
}

#[repr(transparent)]
pub struct SamplerProperties(diligent_sys::SamplerProperties);
impl SamplerProperties {
    pub fn border_sampling_mode_supported(&self) -> bool {
        self.0.BorderSamplingModeSupported
    }
    pub fn max_anisotropy(&self) -> u8 {
        self.0.MaxAnisotropy
    }
    pub fn lod_bias_supported(&self) -> bool {
        self.0.LODBiasSupported
    }
}

#[repr(transparent)]
pub struct MeshShaderProperties(diligent_sys::MeshShaderProperties);
impl MeshShaderProperties {
    pub fn max_thread_group_count_x(&self) -> u32 {
        self.0.MaxThreadGroupCountX
    }
    pub fn max_thread_group_count_y(&self) -> u32 {
        self.0.MaxThreadGroupCountY
    }
    pub fn max_thread_group_count_z(&self) -> u32 {
        self.0.MaxThreadGroupCountZ
    }
    pub fn max_thread_group_total_count(&self) -> u32 {
        self.0.MaxThreadGroupTotalCount
    }
}

#[derive(Clone, Copy)]
pub enum ShadingRate {
    _1X1,
    _1X2,
    _1X4,
    _2X1,
    _2X2,
    _2X4,
    _4X1,
    _4X2,
    _4X4,
}
const_assert_eq!(diligent_sys::SHADING_RATE_MAX, 10);

impl From<ShadingRate> for diligent_sys::SHADING_RATE {
    fn from(value: ShadingRate) -> Self {
        (match value {
            ShadingRate::_1X1 => diligent_sys::SHADING_RATE_1X1,
            ShadingRate::_1X2 => diligent_sys::SHADING_RATE_1X2,
            ShadingRate::_1X4 => diligent_sys::SHADING_RATE_1X4,
            ShadingRate::_2X1 => diligent_sys::SHADING_RATE_2X1,
            ShadingRate::_2X2 => diligent_sys::SHADING_RATE_2X2,
            ShadingRate::_2X4 => diligent_sys::SHADING_RATE_2X4,
            ShadingRate::_4X1 => diligent_sys::SHADING_RATE_4X1,
            ShadingRate::_4X2 => diligent_sys::SHADING_RATE_4X2,
            ShadingRate::_4X4 => diligent_sys::SHADING_RATE_4X4,
        }) as _
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct SampleCount : diligent_sys::SAMPLE_COUNT {
        const None = diligent_sys::SAMPLE_COUNT_NONE as diligent_sys::SAMPLE_COUNT;
        const _1   = diligent_sys::SAMPLE_COUNT_1 as diligent_sys::SAMPLE_COUNT;
        const _2   = diligent_sys::SAMPLE_COUNT_2 as diligent_sys::SAMPLE_COUNT;
        const _4   = diligent_sys::SAMPLE_COUNT_4 as diligent_sys::SAMPLE_COUNT;
        const _8   = diligent_sys::SAMPLE_COUNT_8 as diligent_sys::SAMPLE_COUNT;
        const _16  = diligent_sys::SAMPLE_COUNT_16 as diligent_sys::SAMPLE_COUNT;
        const _32  = diligent_sys::SAMPLE_COUNT_32 as diligent_sys::SAMPLE_COUNT;
        const _64  = diligent_sys::SAMPLE_COUNT_64 as diligent_sys::SAMPLE_COUNT;
    }
}
const_assert_eq!(diligent_sys::SAMPLE_COUNT_MAX, 64);

#[repr(transparent)]
pub struct ShadingRateMode(diligent_sys::ShadingRateMode);
impl ShadingRateMode {
    pub fn rate(&self) -> ShadingRate {
        match self.0.Rate as _ {
            diligent_sys::SHADING_RATE_1X1 => ShadingRate::_1X1,
            diligent_sys::SHADING_RATE_1X2 => ShadingRate::_1X2,
            diligent_sys::SHADING_RATE_1X4 => ShadingRate::_1X4,
            diligent_sys::SHADING_RATE_2X1 => ShadingRate::_2X1,
            diligent_sys::SHADING_RATE_2X2 => ShadingRate::_2X2,
            diligent_sys::SHADING_RATE_2X4 => ShadingRate::_2X4,
            diligent_sys::SHADING_RATE_4X1 => ShadingRate::_4X1,
            diligent_sys::SHADING_RATE_4X2 => ShadingRate::_4X2,
            diligent_sys::SHADING_RATE_4X4 => ShadingRate::_4X4,
            _ => panic!(),
        }
    }
    pub fn sample_bits(&self) -> SampleCount {
        SampleCount::from_bits_retain(self.0.SampleBits)
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct ShadingRateCapFlags : diligent_sys::SHADING_RATE_CAP_FLAGS {
        const None                              = diligent_sys::SHADING_RATE_CAP_FLAG_NONE as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const PerDraw                           = diligent_sys::SHADING_RATE_CAP_FLAG_PER_DRAW as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const PerPrimitive                      = diligent_sys::SHADING_RATE_CAP_FLAG_PER_PRIMITIVE as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const TextureBased                      = diligent_sys::SHADING_RATE_CAP_FLAG_TEXTURE_BASED as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const SampleMask                        = diligent_sys::SHADING_RATE_CAP_FLAG_SAMPLE_MASK as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const ShaderSampleMask                  = diligent_sys::SHADING_RATE_CAP_FLAG_SHADER_SAMPLE_MASK as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const ShaderDepthStencilWrite           = diligent_sys::SHADING_RATE_CAP_FLAG_SHADER_DEPTH_STENCIL_WRITE as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const PerPrimitiveWithMultipleViewports = diligent_sys::SHADING_RATE_CAP_FLAG_PER_PRIMITIVE_WITH_MULTIPLE_VIEWPORTS as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const SameTextureForWholeRenderpass     = diligent_sys::SHADING_RATE_CAP_FLAG_SAME_TEXTURE_FOR_WHOLE_RENDERPASS as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const TextureArray                      = diligent_sys::SHADING_RATE_CAP_FLAG_TEXTURE_ARRAY as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const ShadingRateShaderInput            = diligent_sys::SHADING_RATE_CAP_FLAG_SHADING_RATE_SHADER_INPUT as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const AdditionalInvocations             = diligent_sys::SHADING_RATE_CAP_FLAG_ADDITIONAL_INVOCATIONS as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const Non_subsampledRenderTarget        = diligent_sys::SHADING_RATE_CAP_FLAG_NON_SUBSAMPLED_RENDER_TARGET as diligent_sys::SHADING_RATE_CAP_FLAGS;
        const Subsampled_renderTarget           = diligent_sys::SHADING_RATE_CAP_FLAG_SUBSAMPLED_RENDER_TARGET as diligent_sys::SHADING_RATE_CAP_FLAGS;
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct ShadingRateCombiner : diligent_sys::SHADING_RATE_COMBINER {
        const Passthrough = diligent_sys::SHADING_RATE_COMBINER_PASSTHROUGH as diligent_sys::SHADING_RATE_COMBINER;
        const Override    = diligent_sys::SHADING_RATE_COMBINER_OVERRIDE as diligent_sys::SHADING_RATE_COMBINER;
        const Min         = diligent_sys::SHADING_RATE_COMBINER_MIN as diligent_sys::SHADING_RATE_COMBINER;
        const Max         = diligent_sys::SHADING_RATE_COMBINER_MAX as diligent_sys::SHADING_RATE_COMBINER;
        const Sum         = diligent_sys::SHADING_RATE_COMBINER_SUM as diligent_sys::SHADING_RATE_COMBINER;
        const Mul         = diligent_sys::SHADING_RATE_COMBINER_MUL as diligent_sys::SHADING_RATE_COMBINER;
    }
}
const_assert_eq!(diligent_sys::SHADING_RATE_COMBINER_LAST, 32);

bitflags! {
    #[derive(Clone, Copy)]
    pub struct ShadingRateFormat : diligent_sys::SHADING_RATE_FORMAT {
        const Unknown    = diligent_sys::SHADING_RATE_FORMAT_UNKNOWN as diligent_sys::SHADING_RATE_FORMAT;
        const Palette    = diligent_sys::SHADING_RATE_FORMAT_PALETTE as diligent_sys::SHADING_RATE_FORMAT;
        const Unorm8     = diligent_sys::SHADING_RATE_FORMAT_UNORM8 as diligent_sys::SHADING_RATE_FORMAT;
        const ColRowFp32 = diligent_sys::SHADING_RATE_FORMAT_COL_ROW_FP32 as diligent_sys::SHADING_RATE_FORMAT;
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct ShadingRateTextureAccess : diligent_sys::SHADING_RATE_TEXTURE_ACCESS {
        const Unknown  = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_UNKNOWN as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
        const OnGpu    = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_ON_GPU as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
        const OnSubmit = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_ON_SUBMIT as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
        const OnSetRtv = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_ON_SET_RTV as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
    }
}

#[repr(transparent)]
pub struct ShadingRateProperties(diligent_sys::ShadingRateProperties);
impl ShadingRateProperties {
    pub fn shading_rates(&self) -> &[ShadingRateMode] {
        unsafe {
            std::slice::from_raw_parts(
                std::ptr::from_ref(&self.0.ShadingRates[0]) as *const ShadingRateMode,
                self.0.NumShadingRates as usize,
            )
        }
    }
    pub fn cap_flags(&self) -> ShadingRateCapFlags {
        ShadingRateCapFlags::from_bits_retain(self.0.CapFlags)
    }
    pub fn combiners(&self) -> ShadingRateCombiner {
        ShadingRateCombiner::from_bits_retain(self.0.Combiners)
    }
    pub fn format(&self) -> ShadingRateFormat {
        ShadingRateFormat::from_bits_retain(self.0.Format)
    }
    pub fn shading_rate_texture_access(&self) -> ShadingRateTextureAccess {
        ShadingRateTextureAccess::from_bits_retain(self.0.ShadingRateTextureAccess)
    }
    pub fn bind_flags(&self) -> BindFlags {
        BindFlags::from_bits_retain(self.0.BindFlags)
    }
    pub fn min_tile_size(&self) -> (u32, u32) {
        (self.0.MinTileSize[0], self.0.MinTileSize[1])
    }
    pub fn max_tile_size(&self) -> (u32, u32) {
        (self.0.MaxTileSize[0], self.0.MaxTileSize[1])
    }
    pub fn max_subsampled_array_slices(&self) -> u32 {
        self.0.MaxSabsampledArraySlices
    }
}

#[repr(transparent)]
pub struct ComputeShaderProperties(diligent_sys::ComputeShaderProperties);
impl ComputeShaderProperties {
    pub fn shared_memory_size(&self) -> u32 {
        self.0.SharedMemorySize
    }
    pub fn max_thread_group_invocations(&self) -> u32 {
        self.0.MaxThreadGroupInvocations
    }
    pub fn max_thread_group_size_x(&self) -> u32 {
        self.0.MaxThreadGroupSizeX
    }
    pub fn max_thread_group_size_y(&self) -> u32 {
        self.0.MaxThreadGroupSizeY
    }
    pub fn max_thread_group_size_z(&self) -> u32 {
        self.0.MaxThreadGroupSizeZ
    }
    pub fn max_thread_group_count_x(&self) -> u32 {
        self.0.MaxThreadGroupCountX
    }
    pub fn max_thread_group_count_y(&self) -> u32 {
        self.0.MaxThreadGroupCountY
    }
    pub fn max_thread_group_count_z(&self) -> u32 {
        self.0.MaxThreadGroupCountZ
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct DrawCommandCapFlags : diligent_sys::DRAW_COMMAND_CAP_FLAGS {
        const None                      = diligent_sys::DRAW_COMMAND_CAP_FLAG_NONE as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const BaseVertex                = diligent_sys::DRAW_COMMAND_CAP_FLAG_BASE_VERTEX as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const DrawIndirect              = diligent_sys::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const DrawIndirectFirstInstance = diligent_sys::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT_FIRST_INSTANCE as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const NativeMultiDrawIndirect   = diligent_sys::DRAW_COMMAND_CAP_FLAG_NATIVE_MULTI_DRAW_INDIRECT as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const DrawIndirectCounterBuffer = diligent_sys::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT_COUNTER_BUFFER as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
    }
}

#[repr(transparent)]
pub struct DrawCommandProperties(diligent_sys::DrawCommandProperties);
impl DrawCommandProperties {
    pub fn cap_flags(&self) -> DrawCommandCapFlags {
        DrawCommandCapFlags::from_bits_retain(self.0.CapFlags)
    }
    pub fn max_index_value(&self) -> u32 {
        self.0.MaxIndexValue
    }
    pub fn max_draw_indirect_count(&self) -> u32 {
        self.0.MaxDrawIndirectCount
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct SparseResourceCapFlags : diligent_sys::SPARSE_RESOURCE_CAP_FLAGS {
        const None                     = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_NONE as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const ShaderResourceResidency  = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_SHADER_RESOURCE_RESIDENCY as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Buffer                   = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_BUFFER as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Texture2D                = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_2D as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Texture3D                = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_3D as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Texture2Samples          = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_2_SAMPLES as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Texture4Samples          = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_4_SAMPLES as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Texture8Samples          = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_8_SAMPLES as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Texture16Samples         = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_16_SAMPLES as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Aliased                  = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_ALIASED as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Standard2DTileShape      = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_STANDARD_2D_TILE_SHAPE as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Standard2DMSTileShape    = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_STANDARD_2DMS_TILE_SHAPE as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Standard3DTileShape      = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_STANDARD_3D_TILE_SHAPE as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const AlignedMipSize           = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_ALIGNED_MIP_SIZE as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const NonResidentStrict        = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_NON_RESIDENT_STRICT as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const Texture2dArrayMipTail    = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_2D_ARRAY_MIP_TAIL as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const BufferStandardBlock      = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_BUFFER_STANDARD_BLOCK as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const NonResidentSafe          = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_NON_RESIDENT_SAFE as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
        const MixedResourceTypeSupport = diligent_sys::SPARSE_RESOURCE_CAP_FLAG_MIXED_RESOURCE_TYPE_SUPPORT as diligent_sys::SPARSE_RESOURCE_CAP_FLAGS;
    }
}

#[repr(transparent)]
pub struct SparseResourceProperties(diligent_sys::SparseResourceProperties);
impl SparseResourceProperties {
    pub fn address_space_size(&self) -> u64 {
        self.0.AddressSpaceSize
    }
    pub fn resource_space_size(&self) -> u64 {
        self.0.ResourceSpaceSize
    }
    pub fn cap_flags(&self) -> SparseResourceCapFlags {
        SparseResourceCapFlags::from_bits_retain(self.0.CapFlags)
    }
    pub fn standard_block_size(&self) -> u32 {
        self.0.StandardBlockSize
    }
    pub fn buffer_bind_flags(&self) -> BindFlags {
        BindFlags::from_bits_retain(self.0.BufferBindFlags)
    }
}

#[derive(Clone, Copy)]
pub enum DeviceFeatureState {
    Disabled,
    Enabled,
    Optional,
}

impl From<diligent_sys::DEVICE_FEATURE_STATE> for DeviceFeatureState {
    fn from(value: diligent_sys::DEVICE_FEATURE_STATE) -> Self {
        match value as _ {
            diligent_sys::DEVICE_FEATURE_STATE_DISABLED => DeviceFeatureState::Disabled,
            diligent_sys::DEVICE_FEATURE_STATE_ENABLED => DeviceFeatureState::Enabled,
            diligent_sys::DEVICE_FEATURE_STATE_OPTIONAL => DeviceFeatureState::Optional,
            _ => panic!(),
        }
    }
}

impl From<DeviceFeatureState> for diligent_sys::DEVICE_FEATURE_STATE {
    fn from(value: DeviceFeatureState) -> Self {
        (match value {
            DeviceFeatureState::Disabled => diligent_sys::DEVICE_FEATURE_STATE_DISABLED,
            DeviceFeatureState::Enabled => diligent_sys::DEVICE_FEATURE_STATE_ENABLED,
            DeviceFeatureState::Optional => diligent_sys::DEVICE_FEATURE_STATE_OPTIONAL,
        }) as _
    }
}

#[repr(transparent)]
pub struct DeviceFeatures(pub(crate) diligent_sys::DeviceFeatures);
impl DeviceFeatures {
    pub fn separable_programs(&self) -> DeviceFeatureState {
        self.0.SeparablePrograms.into()
    }
    pub fn shader_resource_queries(&self) -> DeviceFeatureState {
        self.0.ShaderResourceQueries.into()
    }
    pub fn wireframe_fill(&self) -> DeviceFeatureState {
        self.0.WireframeFill.into()
    }
    pub fn multithreaded_resource_creation(&self) -> DeviceFeatureState {
        self.0.MultithreadedResourceCreation.into()
    }
    pub fn compute_shaders(&self) -> DeviceFeatureState {
        self.0.ComputeShaders.into()
    }
    pub fn geometry_shaders(&self) -> DeviceFeatureState {
        self.0.GeometryShaders.into()
    }
    pub fn tessellation(&self) -> DeviceFeatureState {
        self.0.Tessellation.into()
    }
    pub fn mesh_shaders(&self) -> DeviceFeatureState {
        self.0.MeshShaders.into()
    }
    pub fn ray_tracing(&self) -> DeviceFeatureState {
        self.0.RayTracing.into()
    }
    pub fn bindless_resources(&self) -> DeviceFeatureState {
        self.0.BindlessResources.into()
    }
    pub fn occlusion_queries(&self) -> DeviceFeatureState {
        self.0.OcclusionQueries.into()
    }
    pub fn binary_occlusion_queries(&self) -> DeviceFeatureState {
        self.0.BinaryOcclusionQueries.into()
    }
    pub fn timestamp_queries(&self) -> DeviceFeatureState {
        self.0.TimestampQueries.into()
    }
    pub fn pipeline_statistics_queries(&self) -> DeviceFeatureState {
        self.0.PipelineStatisticsQueries.into()
    }
    pub fn duration_queries(&self) -> DeviceFeatureState {
        self.0.DurationQueries.into()
    }
    pub fn depth_bias_clamp(&self) -> DeviceFeatureState {
        self.0.DepthBiasClamp.into()
    }
    pub fn depth_clamp(&self) -> DeviceFeatureState {
        self.0.DepthClamp.into()
    }
    pub fn independent_blend(&self) -> DeviceFeatureState {
        self.0.IndependentBlend.into()
    }
    pub fn dual_source_blend(&self) -> DeviceFeatureState {
        self.0.DualSourceBlend.into()
    }
    pub fn multi_viewport(&self) -> DeviceFeatureState {
        self.0.MultiViewport.into()
    }
    pub fn texture_compression_bc(&self) -> DeviceFeatureState {
        self.0.TextureCompressionBC.into()
    }
    pub fn texture_compression_etc2(&self) -> DeviceFeatureState {
        self.0.TextureCompressionETC2.into()
    }
    pub fn vertex_pipeline_uav_writes_and_atomics(&self) -> DeviceFeatureState {
        self.0.VertexPipelineUAVWritesAndAtomics.into()
    }
    pub fn pixel_uav_writes_and_atomics(&self) -> DeviceFeatureState {
        self.0.PixelUAVWritesAndAtomics.into()
    }
    pub fn texture_uav_extended_formats(&self) -> DeviceFeatureState {
        self.0.TextureUAVExtendedFormats.into()
    }
    pub fn shader_float16(&self) -> DeviceFeatureState {
        self.0.ShaderFloat16.into()
    }
    pub fn resource_buffer16_bit_access(&self) -> DeviceFeatureState {
        self.0.ResourceBuffer16BitAccess.into()
    }
    pub fn uniform_buffer16_bit_access(&self) -> DeviceFeatureState {
        self.0.UniformBuffer16BitAccess.into()
    }
    pub fn shader_input_output16(&self) -> DeviceFeatureState {
        self.0.ShaderInputOutput16.into()
    }
    pub fn shader_int8(&self) -> DeviceFeatureState {
        self.0.ShaderInt8.into()
    }
    pub fn resource_buffer8_bit_access(&self) -> DeviceFeatureState {
        self.0.ResourceBuffer8BitAccess.into()
    }
    pub fn uniform_buffer8_bit_access(&self) -> DeviceFeatureState {
        self.0.UniformBuffer8BitAccess.into()
    }
    pub fn shader_resource_static_arrays(&self) -> DeviceFeatureState {
        self.0.ShaderResourceStaticArrays.into()
    }
    pub fn shader_resource_runtime_arrays(&self) -> DeviceFeatureState {
        self.0.ShaderResourceRuntimeArrays.into()
    }
    pub fn wave_op(&self) -> DeviceFeatureState {
        self.0.WaveOp.into()
    }
    pub fn instance_data_step_rate(&self) -> DeviceFeatureState {
        self.0.InstanceDataStepRate.into()
    }
    pub fn native_fence(&self) -> DeviceFeatureState {
        self.0.NativeFence.into()
    }
    pub fn tile_shaders(&self) -> DeviceFeatureState {
        self.0.TileShaders.into()
    }
    pub fn transfer_queue_timestamp_queries(&self) -> DeviceFeatureState {
        self.0.TransferQueueTimestampQueries.into()
    }
    pub fn variable_rate_shading(&self) -> DeviceFeatureState {
        self.0.VariableRateShading.into()
    }
    pub fn sparse_resources(&self) -> DeviceFeatureState {
        self.0.SparseResources.into()
    }
    pub fn subpass_framebuffer_fetch(&self) -> DeviceFeatureState {
        self.0.SubpassFramebufferFetch.into()
    }
    pub fn texture_component_swizzle(&self) -> DeviceFeatureState {
        self.0.TextureComponentSwizzle.into()
    }
    pub fn texture_subresource_views(&self) -> DeviceFeatureState {
        self.0.TextureSubresourceViews.into()
    }
    pub fn native_multi_draw(&self) -> DeviceFeatureState {
        self.0.NativeMultiDraw.into()
    }
    pub fn async_shader_compilation(&self) -> DeviceFeatureState {
        self.0.AsyncShaderCompilation.into()
    }
    pub fn formatted_buffers(&self) -> DeviceFeatureState {
        self.0.FormattedBuffers.into()
    }

    pub fn set_separable_programs(&mut self, state: DeviceFeatureState) {
        self.0.SeparablePrograms = state.into()
    }
    pub fn set_shader_resource_queries(&mut self, state: DeviceFeatureState) {
        self.0.ShaderResourceQueries = state.into()
    }
    pub fn set_wireframe_fill(&mut self, state: DeviceFeatureState) {
        self.0.WireframeFill = state.into()
    }
    pub fn set_multithreaded_resource_creation(&mut self, state: DeviceFeatureState) {
        self.0.MultithreadedResourceCreation = state.into()
    }
    pub fn set_compute_shaders(&mut self, state: DeviceFeatureState) {
        self.0.ComputeShaders = state.into()
    }
    pub fn set_geometry_shaders(&mut self, state: DeviceFeatureState) {
        self.0.GeometryShaders = state.into()
    }
    pub fn set_tessellation(&mut self, state: DeviceFeatureState) {
        self.0.Tessellation = state.into()
    }
    pub fn set_mesh_shaders(&mut self, state: DeviceFeatureState) {
        self.0.MeshShaders = state.into()
    }
    pub fn set_ray_tracing(&mut self, state: DeviceFeatureState) {
        self.0.RayTracing = state.into()
    }
    pub fn set_bindless_resources(&mut self, state: DeviceFeatureState) {
        self.0.BindlessResources = state.into()
    }
    pub fn set_occlusion_queries(&mut self, state: DeviceFeatureState) {
        self.0.OcclusionQueries = state.into()
    }
    pub fn set_binary_occlusion_queries(&mut self, state: DeviceFeatureState) {
        self.0.BinaryOcclusionQueries = state.into()
    }
    pub fn set_timestamp_queries(&mut self, state: DeviceFeatureState) {
        self.0.TimestampQueries = state.into()
    }
    pub fn set_pipeline_statistics_queries(&mut self, state: DeviceFeatureState) {
        self.0.PipelineStatisticsQueries = state.into()
    }
    pub fn set_duration_queries(&mut self, state: DeviceFeatureState) {
        self.0.DurationQueries = state.into()
    }
    pub fn set_depth_bias_clamp(&mut self, state: DeviceFeatureState) {
        self.0.DepthBiasClamp = state.into()
    }
    pub fn set_depth_clamp(&mut self, state: DeviceFeatureState) {
        self.0.DepthClamp = state.into()
    }
    pub fn set_independent_blend(&mut self, state: DeviceFeatureState) {
        self.0.IndependentBlend = state.into()
    }
    pub fn set_dual_source_blend(&mut self, state: DeviceFeatureState) {
        self.0.DualSourceBlend = state.into()
    }
    pub fn set_multi_viewport(&mut self, state: DeviceFeatureState) {
        self.0.MultiViewport = state.into()
    }
    pub fn set_texture_compression_bc(&mut self, state: DeviceFeatureState) {
        self.0.TextureCompressionBC = state.into()
    }
    pub fn set_texture_compression_etc2(&mut self, state: DeviceFeatureState) {
        self.0.TextureCompressionETC2 = state.into()
    }
    pub fn set_vertex_pipeline_uav_writes_and_atomics(&mut self, state: DeviceFeatureState) {
        self.0.VertexPipelineUAVWritesAndAtomics = state.into()
    }
    pub fn set_pixel_uav_writes_and_atomics(&mut self, state: DeviceFeatureState) {
        self.0.PixelUAVWritesAndAtomics = state.into()
    }
    pub fn set_texture_uav_extended_formats(&mut self, state: DeviceFeatureState) {
        self.0.TextureUAVExtendedFormats = state.into()
    }
    pub fn set_shader_float16(&mut self, state: DeviceFeatureState) {
        self.0.ShaderFloat16 = state.into()
    }
    pub fn set_resource_buffer16_bit_access(&mut self, state: DeviceFeatureState) {
        self.0.ResourceBuffer16BitAccess = state.into()
    }
    pub fn set_uniform_buffer16_bit_access(&mut self, state: DeviceFeatureState) {
        self.0.UniformBuffer16BitAccess = state.into()
    }
    pub fn set_shader_input_output16(&mut self, state: DeviceFeatureState) {
        self.0.ShaderInputOutput16 = state.into()
    }
    pub fn set_shader_int8(&mut self, state: DeviceFeatureState) {
        self.0.ShaderInt8 = state.into()
    }
    pub fn set_resource_buffer8_bit_access(&mut self, state: DeviceFeatureState) {
        self.0.ResourceBuffer8BitAccess = state.into()
    }
    pub fn set_uniform_buffer8_bit_access(&mut self, state: DeviceFeatureState) {
        self.0.UniformBuffer8BitAccess = state.into()
    }
    pub fn set_shader_resource_static_arrays(&mut self, state: DeviceFeatureState) {
        self.0.ShaderResourceStaticArrays = state.into()
    }
    pub fn set_shader_resource_runtime_arrays(&mut self, state: DeviceFeatureState) {
        self.0.ShaderResourceRuntimeArrays = state.into()
    }
    pub fn set_wave_op(&mut self, state: DeviceFeatureState) {
        self.0.WaveOp = state.into()
    }
    pub fn set_instance_data_step_rate(&mut self, state: DeviceFeatureState) {
        self.0.InstanceDataStepRate = state.into()
    }
    pub fn set_native_fence(&mut self, state: DeviceFeatureState) {
        self.0.NativeFence = state.into()
    }
    pub fn set_tile_shaders(&mut self, state: DeviceFeatureState) {
        self.0.TileShaders = state.into()
    }
    pub fn set_transfer_queue_timestamp_queries(&mut self, state: DeviceFeatureState) {
        self.0.TransferQueueTimestampQueries = state.into()
    }
    pub fn set_variable_rate_shading(&mut self, state: DeviceFeatureState) {
        self.0.VariableRateShading = state.into()
    }
    pub fn set_sparse_resources(&mut self, state: DeviceFeatureState) {
        self.0.SparseResources = state.into()
    }
    pub fn set_subpass_framebuffer_fetch(&mut self, state: DeviceFeatureState) {
        self.0.SubpassFramebufferFetch = state.into()
    }
    pub fn set_texture_component_swizzle(&mut self, state: DeviceFeatureState) {
        self.0.TextureComponentSwizzle = state.into()
    }
    pub fn set_texture_subresource_views(&mut self, state: DeviceFeatureState) {
        self.0.TextureSubresourceViews = state.into()
    }
    pub fn set_native_multi_draw(&mut self, state: DeviceFeatureState) {
        self.0.NativeMultiDraw = state.into()
    }
    pub fn set_async_shader_compilation(&mut self, state: DeviceFeatureState) {
        self.0.AsyncShaderCompilation = state.into()
    }
    pub fn set_formatted_buffers(&mut self, state: DeviceFeatureState) {
        self.0.FormattedBuffers = state.into()
    }
}

impl DeviceFeatures {
    pub fn set_all(&mut self, state: DeviceFeatureState) {
        let state = state.into();
        self.0.SeparablePrograms = state;
        self.0.ShaderResourceQueries = state;
        self.0.WireframeFill = state;
        self.0.MultithreadedResourceCreation = state;
        self.0.ComputeShaders = state;
        self.0.GeometryShaders = state;
        self.0.Tessellation = state;
        self.0.MeshShaders = state;
        self.0.RayTracing = state;
        self.0.BindlessResources = state;
        self.0.OcclusionQueries = state;
        self.0.BinaryOcclusionQueries = state;
        self.0.TimestampQueries = state;
        self.0.PipelineStatisticsQueries = state;
        self.0.DurationQueries = state;
        self.0.DepthBiasClamp = state;
        self.0.DepthClamp = state;
        self.0.IndependentBlend = state;
        self.0.DualSourceBlend = state;
        self.0.MultiViewport = state;
        self.0.TextureCompressionBC = state;
        self.0.TextureCompressionETC2 = state;
        self.0.VertexPipelineUAVWritesAndAtomics = state;
        self.0.PixelUAVWritesAndAtomics = state;
        self.0.TextureUAVExtendedFormats = state;
        self.0.ShaderFloat16 = state;
        self.0.ResourceBuffer16BitAccess = state;
        self.0.UniformBuffer16BitAccess = state;
        self.0.ShaderInputOutput16 = state;
        self.0.ShaderInt8 = state;
        self.0.ResourceBuffer8BitAccess = state;
        self.0.UniformBuffer8BitAccess = state;
        self.0.ShaderResourceStaticArrays = state;
        self.0.ShaderResourceRuntimeArrays = state;
        self.0.WaveOp = state;
        self.0.InstanceDataStepRate = state;
        self.0.NativeFence = state;
        self.0.TileShaders = state;
        self.0.TransferQueueTimestampQueries = state;
        self.0.VariableRateShading = state;
        self.0.SparseResources = state;
        self.0.SubpassFramebufferFetch = state;
        self.0.TextureComponentSwizzle = state;
        self.0.TextureSubresourceViews = state;
        self.0.NativeMultiDraw = state;
        self.0.AsyncShaderCompilation = state;
        self.0.FormattedBuffers = state;
    }
}

impl Default for DeviceFeatures {
    fn default() -> Self {
        Self(diligent_sys::DeviceFeatures {
            SeparablePrograms: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ShaderResourceQueries: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            WireframeFill: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            MultithreadedResourceCreation: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ComputeShaders: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            GeometryShaders: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            Tessellation: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            MeshShaders: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            RayTracing: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            BindlessResources: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            OcclusionQueries: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            BinaryOcclusionQueries: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TimestampQueries: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            PipelineStatisticsQueries: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            DurationQueries: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            DepthBiasClamp: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            DepthClamp: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            IndependentBlend: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            DualSourceBlend: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            MultiViewport: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TextureCompressionBC: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TextureCompressionETC2: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            VertexPipelineUAVWritesAndAtomics: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            PixelUAVWritesAndAtomics: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TextureUAVExtendedFormats: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ShaderFloat16: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ResourceBuffer16BitAccess: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            UniformBuffer16BitAccess: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ShaderInputOutput16: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ShaderInt8: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ResourceBuffer8BitAccess: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            UniformBuffer8BitAccess: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ShaderResourceStaticArrays: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            ShaderResourceRuntimeArrays: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            WaveOp: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            InstanceDataStepRate: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            NativeFence: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TileShaders: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TransferQueueTimestampQueries: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            VariableRateShading: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            SparseResources: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            SubpassFramebufferFetch: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TextureComponentSwizzle: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            TextureSubresourceViews: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            NativeMultiDraw: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            AsyncShaderCompilation: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
            FormattedBuffers: diligent_sys::DEVICE_FEATURE_STATE_DISABLED as _,
        })
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct CommandQueueType : diligent_sys::COMMAND_QUEUE_TYPE {
        const Unknown       = diligent_sys::COMMAND_QUEUE_TYPE_UNKNOWN as diligent_sys::COMMAND_QUEUE_TYPE;
        const Transfer      = diligent_sys::COMMAND_QUEUE_TYPE_TRANSFER as diligent_sys::COMMAND_QUEUE_TYPE;
        const Compute       = diligent_sys::COMMAND_QUEUE_TYPE_COMPUTE as diligent_sys::COMMAND_QUEUE_TYPE;
        const Graphics      = diligent_sys::COMMAND_QUEUE_TYPE_GRAPHICS as diligent_sys::COMMAND_QUEUE_TYPE;
        const PrimaryMask   = diligent_sys::COMMAND_QUEUE_TYPE_PRIMARY_MASK as diligent_sys::COMMAND_QUEUE_TYPE;
        const SparseBinding = diligent_sys::COMMAND_QUEUE_TYPE_SPARSE_BINDING as diligent_sys::COMMAND_QUEUE_TYPE;
    }
}
const_assert_eq!(diligent_sys::COMMAND_QUEUE_TYPE_MAX_BIT, 7);

#[repr(transparent)]
pub struct CommandQueueInfo(diligent_sys::CommandQueueInfo);

impl CommandQueueInfo {
    pub fn queue_type(&self) -> CommandQueueType {
        CommandQueueType::from_bits_retain(self.0.QueueType)
    }
    pub fn max_device_contexts(&self) -> u32 {
        self.0.MaxDeviceContexts
    }
    pub fn texture_copy_granularity(&self) -> [u32; 3usize] {
        self.0.TextureCopyGranularity
    }
}

#[repr(transparent)]
pub struct GraphicsAdapterInfo(diligent_sys::GraphicsAdapterInfo);
impl GraphicsAdapterInfo {
    pub fn description(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.Description.as_ptr()) }
    }
    pub fn adapter_type(&self) -> AdapterType {
        match self.0.Type as _ {
            diligent_sys::ADAPTER_TYPE_UNKNOWN => AdapterType::Unknown,
            diligent_sys::ADAPTER_TYPE_SOFTWARE => AdapterType::Software,
            diligent_sys::ADAPTER_TYPE_INTEGRATED => AdapterType::Integrated,
            diligent_sys::ADAPTER_TYPE_DISCRETE => AdapterType::Discrete,
            _ => panic!(),
        }
    }
    pub fn vendor(&self) -> AdapterVendor {
        match self.0.Vendor as _ {
            diligent_sys::ADAPTER_VENDOR_UNKNOWN => AdapterVendor::Unknown,
            diligent_sys::ADAPTER_VENDOR_NVIDIA => AdapterVendor::Nvidia,
            diligent_sys::ADAPTER_VENDOR_AMD => AdapterVendor::AMD,
            diligent_sys::ADAPTER_VENDOR_INTEL => AdapterVendor::Intel,
            diligent_sys::ADAPTER_VENDOR_ARM => AdapterVendor::ARM,
            diligent_sys::ADAPTER_VENDOR_QUALCOMM => AdapterVendor::Qualcomm,
            diligent_sys::ADAPTER_VENDOR_IMGTECH => AdapterVendor::Imgtech,
            diligent_sys::ADAPTER_VENDOR_MSFT => AdapterVendor::Msft,
            diligent_sys::ADAPTER_VENDOR_APPLE => AdapterVendor::Apple,
            diligent_sys::ADAPTER_VENDOR_MESA => AdapterVendor::Mesa,
            diligent_sys::ADAPTER_VENDOR_BROADCOM => AdapterVendor::Broadcom,
            _ => panic!(),
        }
    }
    pub fn vendor_id(&self) -> u32 {
        self.0.VendorId
    }
    pub fn device_id(&self) -> u32 {
        self.0.DeviceId
    }
    pub fn num_outputs(&self) -> u32 {
        self.0.NumOutputs
    }
    pub fn memory(&self) -> &AdapterMemoryInfo {
        unsafe { std::mem::transmute(&self.0.Memory) }
    }
    pub fn ray_tracing(&self) -> &RayTracingProperties {
        unsafe { std::mem::transmute(&self.0.RayTracing) }
    }
    pub fn wave_op(&self) -> &WaveOpProperties {
        unsafe { std::mem::transmute(&self.0.WaveOp) }
    }
    pub fn buffer(&self) -> &BufferProperties {
        unsafe { std::mem::transmute(&self.0.Buffer) }
    }
    pub fn texture(&self) -> &TextureProperties {
        unsafe { std::mem::transmute(&self.0.Texture) }
    }
    pub fn sampler(&self) -> &SamplerProperties {
        unsafe { std::mem::transmute(&self.0.Sampler) }
    }
    pub fn mesh_shader(&self) -> &MeshShaderProperties {
        unsafe { std::mem::transmute(&self.0.MeshShader) }
    }
    pub fn shading_rate(&self) -> &ShadingRateProperties {
        unsafe { std::mem::transmute(&self.0.ShadingRate) }
    }
    pub fn compute_shader(&self) -> &ComputeShaderProperties {
        unsafe { std::mem::transmute(&self.0.ComputeShader) }
    }
    pub fn draw_command(&self) -> &DrawCommandProperties {
        unsafe { std::mem::transmute(&self.0.DrawCommand) }
    }
    pub fn sparse_resources(&self) -> &SparseResourceProperties {
        unsafe { std::mem::transmute(&self.0.SparseResources) }
    }
    pub fn features(&self) -> &DeviceFeatures {
        unsafe { std::mem::transmute(&self.0.Features) }
    }
    pub fn queues(&self) -> &[CommandQueueInfo] {
        unsafe {
            std::slice::from_raw_parts(
                std::ptr::from_ref(&self.0.Queues[0]) as *const CommandQueueInfo,
                self.0.NumQueues as usize,
            )
        }
    }
}

#[derive(Clone, Copy)]
pub enum SurfaceTransform {
    Optimal,
    Identity,
    Rotate90,
    Rotate180,
    Rotate270,
    HorizontalMirror,
    HorizontalMirrorRotate90,
    HorizontalMirrorRotate180,
    HorizontalMirrorRotate270,
}

impl From<SurfaceTransform> for diligent_sys::SURFACE_TRANSFORM {
    fn from(value: SurfaceTransform) -> Self {
        (match value {
            SurfaceTransform::Optimal => diligent_sys::SURFACE_TRANSFORM_OPTIMAL,
            SurfaceTransform::Identity => diligent_sys::SURFACE_TRANSFORM_IDENTITY,
            SurfaceTransform::Rotate90 => diligent_sys::SURFACE_TRANSFORM_ROTATE_90,
            SurfaceTransform::Rotate180 => diligent_sys::SURFACE_TRANSFORM_ROTATE_180,
            SurfaceTransform::Rotate270 => diligent_sys::SURFACE_TRANSFORM_ROTATE_270,
            SurfaceTransform::HorizontalMirror => diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR,
            SurfaceTransform::HorizontalMirrorRotate90 => {
                diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90
            }
            SurfaceTransform::HorizontalMirrorRotate180 => {
                diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180
            }
            SurfaceTransform::HorizontalMirrorRotate270 => {
                diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270
            }
        }) as _
    }
}

impl From<diligent_sys::SURFACE_TRANSFORM> for SurfaceTransform {
    fn from(value: diligent_sys::SURFACE_TRANSFORM) -> Self {
        match value as _ {
            diligent_sys::SURFACE_TRANSFORM_OPTIMAL => SurfaceTransform::Optimal,
            diligent_sys::SURFACE_TRANSFORM_IDENTITY => SurfaceTransform::Identity,
            diligent_sys::SURFACE_TRANSFORM_ROTATE_90 => SurfaceTransform::Rotate90,
            diligent_sys::SURFACE_TRANSFORM_ROTATE_180 => SurfaceTransform::Rotate180,
            diligent_sys::SURFACE_TRANSFORM_ROTATE_270 => SurfaceTransform::Rotate270,
            diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR => SurfaceTransform::HorizontalMirror,
            diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90 => {
                SurfaceTransform::HorizontalMirrorRotate90
            }
            diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180 => {
                SurfaceTransform::HorizontalMirrorRotate180
            }
            diligent_sys::SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270 => {
                SurfaceTransform::HorizontalMirrorRotate270
            }
            _ => panic!(),
        }
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct ResourceState: diligent_sys::RESOURCE_STATE {
        const Undefined         = diligent_sys::RESOURCE_STATE_UNDEFINED as diligent_sys::RESOURCE_STATE;
        const VertexBuffer      = diligent_sys::RESOURCE_STATE_VERTEX_BUFFER as diligent_sys::RESOURCE_STATE;
        const ConstantBuffer    = diligent_sys::RESOURCE_STATE_CONSTANT_BUFFER as diligent_sys::RESOURCE_STATE;
        const IndexBuffer       = diligent_sys::RESOURCE_STATE_INDEX_BUFFER as diligent_sys::RESOURCE_STATE;
        const RenderTarget      = diligent_sys::RESOURCE_STATE_RENDER_TARGET as diligent_sys::RESOURCE_STATE;
        const UnorderedAccess   = diligent_sys::RESOURCE_STATE_UNORDERED_ACCESS as diligent_sys::RESOURCE_STATE;
        const DepthWrite        = diligent_sys::RESOURCE_STATE_DEPTH_WRITE as diligent_sys::RESOURCE_STATE;
        const DepthRead         = diligent_sys::RESOURCE_STATE_DEPTH_READ as diligent_sys::RESOURCE_STATE;
        const ShaderResource    = diligent_sys::RESOURCE_STATE_SHADER_RESOURCE as diligent_sys::RESOURCE_STATE;
        const StreamOut         = diligent_sys::RESOURCE_STATE_STREAM_OUT as diligent_sys::RESOURCE_STATE;
        const IndirectArgument  = diligent_sys::RESOURCE_STATE_INDIRECT_ARGUMENT as diligent_sys::RESOURCE_STATE;
        const CopyDest          = diligent_sys::RESOURCE_STATE_COPY_DEST as diligent_sys::RESOURCE_STATE;
        const CopySource        = diligent_sys::RESOURCE_STATE_COPY_SOURCE as diligent_sys::RESOURCE_STATE;
        const ResolveDest       = diligent_sys::RESOURCE_STATE_RESOLVE_DEST as diligent_sys::RESOURCE_STATE;
        const ResolveSource     = diligent_sys::RESOURCE_STATE_RESOLVE_SOURCE as diligent_sys::RESOURCE_STATE;
        const InputAttachment   = diligent_sys::RESOURCE_STATE_INPUT_ATTACHMENT as diligent_sys::RESOURCE_STATE;
        const Present           = diligent_sys::RESOURCE_STATE_PRESENT as diligent_sys::RESOURCE_STATE;
        const BuildAsRead       = diligent_sys::RESOURCE_STATE_BUILD_AS_READ as diligent_sys::RESOURCE_STATE;
        const BuildSsWrite      = diligent_sys::RESOURCE_STATE_BUILD_AS_WRITE as diligent_sys::RESOURCE_STATE;
        const RayTracing        = diligent_sys::RESOURCE_STATE_RAY_TRACING as diligent_sys::RESOURCE_STATE;
        const Common            = diligent_sys::RESOURCE_STATE_COMMON as diligent_sys::RESOURCE_STATE;
        const ShadingRate       = diligent_sys::RESOURCE_STATE_SHADING_RATE as diligent_sys::RESOURCE_STATE;
        const GenericRead       = diligent_sys::RESOURCE_STATE_GENERIC_READ as diligent_sys::RESOURCE_STATE;
    }
}

const_assert_eq!(diligent_sys::RESOURCE_STATE_MAX_BIT, 2097152);

#[derive(Clone, Copy)]
pub enum QueuePriority {
    Low,
    Medium,
    High,
    RealTime,
}
const_assert_eq!(diligent_sys::QUEUE_PRIORITY_LAST, 4);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    Undefined,
    Float,
    Snorm,
    Unorm,
    UnormSRGB,
    Sint,
    Uint,
    Depth,
    DepthStencil,
    Compound,
    Compressed,
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TextureFormat {
    RGBA32_TYPELESS,
    RGBA32_FLOAT,
    RGBA32_UINT,
    RGBA32_SINT,
    RGB32_TYPELESS,
    RGB32_FLOAT,
    RGB32_UINT,
    RGB32_SINT,
    RGBA16_TYPELESS,
    RGBA16_FLOAT,
    RGBA16_UNORM,
    RGBA16_UINT,
    RGBA16_SNORM,
    RGBA16_SINT,
    RG32_TYPELESS,
    RG32_FLOAT,
    RG32_UINT,
    RG32_SINT,
    R32G8X24_TYPELESS,
    D32_FLOAT_S8X24_UINT,
    R32_FLOAT_X8X24_TYPELESS,
    X32_TYPELESS_G8X24_UINT,
    RGB10A2_TYPELESS,
    RGB10A2_UNORM,
    RGB10A2_UINT,
    R11G11B10_FLOAT,
    RGBA8_TYPELESS,
    RGBA8_UNORM,
    RGBA8_UNORM_SRGB,
    RGBA8_UINT,
    RGBA8_SNORM,
    RGBA8_SINT,
    RG16_TYPELESS,
    RG16_FLOAT,
    RG16_UNORM,
    RG16_UINT,
    RG16_SNORM,
    RG16_SINT,
    R32_TYPELESS,
    D32_FLOAT,
    R32_FLOAT,
    R32_UINT,
    R32_SINT,
    R24G8_TYPELESS,
    D24_UNORM_S8_UINT,
    R24_UNORM_X8_TYPELESS,
    X24_TYPELESS_G8_UINT,
    RG8_TYPELESS,
    RG8_UNORM,
    RG8_UINT,
    RG8_SNORM,
    RG8_SINT,
    R16_TYPELESS,
    R16_FLOAT,
    D16_UNORM,
    R16_UNORM,
    R16_UINT,
    R16_SNORM,
    R16_SINT,
    R8_TYPELESS,
    R8_UNORM,
    R8_UINT,
    R8_SNORM,
    R8_SINT,
    A8_UNORM,
    R1_UNORM,
    RGB9E5_SHAREDEXP,
    RG8_B8G8_UNORM,
    G8R8_G8B8_UNORM,
    BC1_TYPELESS,
    BC1_UNORM,
    BC1_UNORM_SRGB,
    BC2_TYPELESS,
    BC2_UNORM,
    BC2_UNORM_SRGB,
    BC3_TYPELESS,
    BC3_UNORM,
    BC3_UNORM_SRGB,
    BC4_TYPELESS,
    BC4_UNORM,
    BC4_SNORM,
    BC5_TYPELESS,
    BC5_UNORM,
    BC5_SNORM,
    B5G6R5_UNORM,
    B5G5R5A1_UNORM,
    BGRA8_UNORM,
    BGRX8_UNORM,
    R10G10B10_XR_BIAS_A2_UNORM,
    BGRA8_TYPELESS,
    BGRA8_UNORM_SRGB,
    BGRX8_TYPELESS,
    BGRX8_UNORM_SRGB,
    BC6H_TYPELESS,
    BC6H_UF16,
    BC6H_SF16,
    BC7_TYPELESS,
    BC7_UNORM,
    BC7_UNORM_SRGB,
    ETC2_RGB8_UNORM,
    ETC2_RGB8_UNORM_SRGB,
    ETC2_RGB8A1_UNORM,
    ETC2_RGB8A1_UNORM_SRGB,
    ETC2_RGBA8_UNORM,
    ETC2_RGBA8_UNORM_SRGB,
}
const_assert_eq!(diligent_sys::TEX_FORMAT_NUM_FORMATS, 106);

impl From<TextureFormat> for diligent_sys::TEXTURE_FORMAT {
    fn from(value: TextureFormat) -> Self {
        (match value {
            TextureFormat::RGBA32_TYPELESS => diligent_sys::TEX_FORMAT_RGBA32_TYPELESS,
            TextureFormat::RGBA32_FLOAT => diligent_sys::TEX_FORMAT_RGBA32_FLOAT,
            TextureFormat::RGBA32_UINT => diligent_sys::TEX_FORMAT_RGBA32_UINT,
            TextureFormat::RGBA32_SINT => diligent_sys::TEX_FORMAT_RGBA32_SINT,
            TextureFormat::RGB32_TYPELESS => diligent_sys::TEX_FORMAT_RGB32_TYPELESS,
            TextureFormat::RGB32_FLOAT => diligent_sys::TEX_FORMAT_RGB32_FLOAT,
            TextureFormat::RGB32_UINT => diligent_sys::TEX_FORMAT_RGB32_UINT,
            TextureFormat::RGB32_SINT => diligent_sys::TEX_FORMAT_RGB32_SINT,
            TextureFormat::RGBA16_TYPELESS => diligent_sys::TEX_FORMAT_RGBA16_TYPELESS,
            TextureFormat::RGBA16_FLOAT => diligent_sys::TEX_FORMAT_RGBA16_FLOAT,
            TextureFormat::RGBA16_UNORM => diligent_sys::TEX_FORMAT_RGBA16_UNORM,
            TextureFormat::RGBA16_UINT => diligent_sys::TEX_FORMAT_RGBA16_UINT,
            TextureFormat::RGBA16_SNORM => diligent_sys::TEX_FORMAT_RGBA16_SNORM,
            TextureFormat::RGBA16_SINT => diligent_sys::TEX_FORMAT_RGBA16_SINT,
            TextureFormat::RG32_TYPELESS => diligent_sys::TEX_FORMAT_RG32_TYPELESS,
            TextureFormat::RG32_FLOAT => diligent_sys::TEX_FORMAT_RG32_FLOAT,
            TextureFormat::RG32_UINT => diligent_sys::TEX_FORMAT_RG32_UINT,
            TextureFormat::RG32_SINT => diligent_sys::TEX_FORMAT_RG32_SINT,
            TextureFormat::R32G8X24_TYPELESS => diligent_sys::TEX_FORMAT_R32G8X24_TYPELESS,
            TextureFormat::D32_FLOAT_S8X24_UINT => diligent_sys::TEX_FORMAT_D32_FLOAT_S8X24_UINT,
            TextureFormat::R32_FLOAT_X8X24_TYPELESS => {
                diligent_sys::TEX_FORMAT_R32_FLOAT_X8X24_TYPELESS
            }
            TextureFormat::X32_TYPELESS_G8X24_UINT => {
                diligent_sys::TEX_FORMAT_X32_TYPELESS_G8X24_UINT
            }
            TextureFormat::RGB10A2_TYPELESS => diligent_sys::TEX_FORMAT_RGB10A2_TYPELESS,
            TextureFormat::RGB10A2_UNORM => diligent_sys::TEX_FORMAT_RGB10A2_UNORM,
            TextureFormat::RGB10A2_UINT => diligent_sys::TEX_FORMAT_RGB10A2_UINT,
            TextureFormat::R11G11B10_FLOAT => diligent_sys::TEX_FORMAT_R11G11B10_FLOAT,
            TextureFormat::RGBA8_TYPELESS => diligent_sys::TEX_FORMAT_RGBA8_TYPELESS,
            TextureFormat::RGBA8_UNORM => diligent_sys::TEX_FORMAT_RGBA8_UNORM,
            TextureFormat::RGBA8_UNORM_SRGB => diligent_sys::TEX_FORMAT_RGBA8_UNORM_SRGB,
            TextureFormat::RGBA8_UINT => diligent_sys::TEX_FORMAT_RGBA8_UINT,
            TextureFormat::RGBA8_SNORM => diligent_sys::TEX_FORMAT_RGBA8_SNORM,
            TextureFormat::RGBA8_SINT => diligent_sys::TEX_FORMAT_RGBA8_SINT,
            TextureFormat::RG16_TYPELESS => diligent_sys::TEX_FORMAT_RG16_TYPELESS,
            TextureFormat::RG16_FLOAT => diligent_sys::TEX_FORMAT_RG16_FLOAT,
            TextureFormat::RG16_UNORM => diligent_sys::TEX_FORMAT_RG16_UNORM,
            TextureFormat::RG16_UINT => diligent_sys::TEX_FORMAT_RG16_UINT,
            TextureFormat::RG16_SNORM => diligent_sys::TEX_FORMAT_RG16_SNORM,
            TextureFormat::RG16_SINT => diligent_sys::TEX_FORMAT_RG16_SINT,
            TextureFormat::R32_TYPELESS => diligent_sys::TEX_FORMAT_R32_TYPELESS,
            TextureFormat::D32_FLOAT => diligent_sys::TEX_FORMAT_D32_FLOAT,
            TextureFormat::R32_FLOAT => diligent_sys::TEX_FORMAT_R32_FLOAT,
            TextureFormat::R32_UINT => diligent_sys::TEX_FORMAT_R32_UINT,
            TextureFormat::R32_SINT => diligent_sys::TEX_FORMAT_R32_SINT,
            TextureFormat::R24G8_TYPELESS => diligent_sys::TEX_FORMAT_R24G8_TYPELESS,
            TextureFormat::D24_UNORM_S8_UINT => diligent_sys::TEX_FORMAT_D24_UNORM_S8_UINT,
            TextureFormat::R24_UNORM_X8_TYPELESS => diligent_sys::TEX_FORMAT_R24_UNORM_X8_TYPELESS,
            TextureFormat::X24_TYPELESS_G8_UINT => diligent_sys::TEX_FORMAT_X24_TYPELESS_G8_UINT,
            TextureFormat::RG8_TYPELESS => diligent_sys::TEX_FORMAT_RG8_TYPELESS,
            TextureFormat::RG8_UNORM => diligent_sys::TEX_FORMAT_RG8_UNORM,
            TextureFormat::RG8_UINT => diligent_sys::TEX_FORMAT_RG8_UINT,
            TextureFormat::RG8_SNORM => diligent_sys::TEX_FORMAT_RG8_SNORM,
            TextureFormat::RG8_SINT => diligent_sys::TEX_FORMAT_RG8_SINT,
            TextureFormat::R16_TYPELESS => diligent_sys::TEX_FORMAT_R16_TYPELESS,
            TextureFormat::R16_FLOAT => diligent_sys::TEX_FORMAT_R16_FLOAT,
            TextureFormat::D16_UNORM => diligent_sys::TEX_FORMAT_D16_UNORM,
            TextureFormat::R16_UNORM => diligent_sys::TEX_FORMAT_R16_UNORM,
            TextureFormat::R16_UINT => diligent_sys::TEX_FORMAT_R16_UINT,
            TextureFormat::R16_SNORM => diligent_sys::TEX_FORMAT_R16_SNORM,
            TextureFormat::R16_SINT => diligent_sys::TEX_FORMAT_R16_SINT,
            TextureFormat::R8_TYPELESS => diligent_sys::TEX_FORMAT_R8_TYPELESS,
            TextureFormat::R8_UNORM => diligent_sys::TEX_FORMAT_R8_UNORM,
            TextureFormat::R8_UINT => diligent_sys::TEX_FORMAT_R8_UINT,
            TextureFormat::R8_SNORM => diligent_sys::TEX_FORMAT_R8_SNORM,
            TextureFormat::R8_SINT => diligent_sys::TEX_FORMAT_R8_SINT,
            TextureFormat::A8_UNORM => diligent_sys::TEX_FORMAT_A8_UNORM,
            TextureFormat::R1_UNORM => diligent_sys::TEX_FORMAT_R1_UNORM,
            TextureFormat::RGB9E5_SHAREDEXP => diligent_sys::TEX_FORMAT_RGB9E5_SHAREDEXP,
            TextureFormat::RG8_B8G8_UNORM => diligent_sys::TEX_FORMAT_RG8_B8G8_UNORM,
            TextureFormat::G8R8_G8B8_UNORM => diligent_sys::TEX_FORMAT_G8R8_G8B8_UNORM,
            TextureFormat::BC1_TYPELESS => diligent_sys::TEX_FORMAT_BC1_TYPELESS,
            TextureFormat::BC1_UNORM => diligent_sys::TEX_FORMAT_BC1_UNORM,
            TextureFormat::BC1_UNORM_SRGB => diligent_sys::TEX_FORMAT_BC1_UNORM_SRGB,
            TextureFormat::BC2_TYPELESS => diligent_sys::TEX_FORMAT_BC2_TYPELESS,
            TextureFormat::BC2_UNORM => diligent_sys::TEX_FORMAT_BC2_UNORM,
            TextureFormat::BC2_UNORM_SRGB => diligent_sys::TEX_FORMAT_BC2_UNORM_SRGB,
            TextureFormat::BC3_TYPELESS => diligent_sys::TEX_FORMAT_BC3_TYPELESS,
            TextureFormat::BC3_UNORM => diligent_sys::TEX_FORMAT_BC3_UNORM,
            TextureFormat::BC3_UNORM_SRGB => diligent_sys::TEX_FORMAT_BC3_UNORM_SRGB,
            TextureFormat::BC4_TYPELESS => diligent_sys::TEX_FORMAT_BC4_TYPELESS,
            TextureFormat::BC4_UNORM => diligent_sys::TEX_FORMAT_BC4_UNORM,
            TextureFormat::BC4_SNORM => diligent_sys::TEX_FORMAT_BC4_SNORM,
            TextureFormat::BC5_TYPELESS => diligent_sys::TEX_FORMAT_BC5_TYPELESS,
            TextureFormat::BC5_UNORM => diligent_sys::TEX_FORMAT_BC5_UNORM,
            TextureFormat::BC5_SNORM => diligent_sys::TEX_FORMAT_BC5_SNORM,
            TextureFormat::B5G6R5_UNORM => diligent_sys::TEX_FORMAT_B5G6R5_UNORM,
            TextureFormat::B5G5R5A1_UNORM => diligent_sys::TEX_FORMAT_B5G5R5A1_UNORM,
            TextureFormat::BGRA8_UNORM => diligent_sys::TEX_FORMAT_BGRA8_UNORM,
            TextureFormat::BGRX8_UNORM => diligent_sys::TEX_FORMAT_BGRX8_UNORM,
            TextureFormat::R10G10B10_XR_BIAS_A2_UNORM => {
                diligent_sys::TEX_FORMAT_R10G10B10_XR_BIAS_A2_UNORM
            }
            TextureFormat::BGRA8_TYPELESS => diligent_sys::TEX_FORMAT_BGRA8_TYPELESS,
            TextureFormat::BGRA8_UNORM_SRGB => diligent_sys::TEX_FORMAT_BGRA8_UNORM_SRGB,
            TextureFormat::BGRX8_TYPELESS => diligent_sys::TEX_FORMAT_BGRX8_TYPELESS,
            TextureFormat::BGRX8_UNORM_SRGB => diligent_sys::TEX_FORMAT_BGRX8_UNORM_SRGB,
            TextureFormat::BC6H_TYPELESS => diligent_sys::TEX_FORMAT_BC6H_TYPELESS,
            TextureFormat::BC6H_UF16 => diligent_sys::TEX_FORMAT_BC6H_UF16,
            TextureFormat::BC6H_SF16 => diligent_sys::TEX_FORMAT_BC6H_SF16,
            TextureFormat::BC7_TYPELESS => diligent_sys::TEX_FORMAT_BC7_TYPELESS,
            TextureFormat::BC7_UNORM => diligent_sys::TEX_FORMAT_BC7_UNORM,
            TextureFormat::BC7_UNORM_SRGB => diligent_sys::TEX_FORMAT_BC7_UNORM_SRGB,
            TextureFormat::ETC2_RGB8_UNORM => diligent_sys::TEX_FORMAT_ETC2_RGB8_UNORM,
            TextureFormat::ETC2_RGB8_UNORM_SRGB => diligent_sys::TEX_FORMAT_ETC2_RGB8_UNORM_SRGB,
            TextureFormat::ETC2_RGB8A1_UNORM => diligent_sys::TEX_FORMAT_ETC2_RGB8A1_UNORM,
            TextureFormat::ETC2_RGB8A1_UNORM_SRGB => {
                diligent_sys::TEX_FORMAT_ETC2_RGB8A1_UNORM_SRGB
            }
            TextureFormat::ETC2_RGBA8_UNORM => diligent_sys::TEX_FORMAT_ETC2_RGBA8_UNORM,
            TextureFormat::ETC2_RGBA8_UNORM_SRGB => diligent_sys::TEX_FORMAT_ETC2_RGBA8_UNORM_SRGB,
        }) as _
    }
}

impl From<diligent_sys::TEXTURE_FORMAT> for TextureFormat {
    fn from(value: diligent_sys::TEXTURE_FORMAT) -> Self {
        match value as _ {
            diligent_sys::TEX_FORMAT_RGBA32_TYPELESS => TextureFormat::RGBA32_TYPELESS,
            diligent_sys::TEX_FORMAT_RGBA32_FLOAT => TextureFormat::RGBA32_FLOAT,
            diligent_sys::TEX_FORMAT_RGBA32_UINT => TextureFormat::RGBA32_UINT,
            diligent_sys::TEX_FORMAT_RGBA32_SINT => TextureFormat::RGBA32_SINT,
            diligent_sys::TEX_FORMAT_RGB32_TYPELESS => TextureFormat::RGB32_TYPELESS,
            diligent_sys::TEX_FORMAT_RGB32_FLOAT => TextureFormat::RGB32_FLOAT,
            diligent_sys::TEX_FORMAT_RGB32_UINT => TextureFormat::RGB32_UINT,
            diligent_sys::TEX_FORMAT_RGB32_SINT => TextureFormat::RGB32_SINT,
            diligent_sys::TEX_FORMAT_RGBA16_TYPELESS => TextureFormat::RGBA16_TYPELESS,
            diligent_sys::TEX_FORMAT_RGBA16_FLOAT => TextureFormat::RGBA16_FLOAT,
            diligent_sys::TEX_FORMAT_RGBA16_UNORM => TextureFormat::RGBA16_UNORM,
            diligent_sys::TEX_FORMAT_RGBA16_UINT => TextureFormat::RGBA16_UINT,
            diligent_sys::TEX_FORMAT_RGBA16_SNORM => TextureFormat::RGBA16_SNORM,
            diligent_sys::TEX_FORMAT_RGBA16_SINT => TextureFormat::RGBA16_SINT,
            diligent_sys::TEX_FORMAT_RG32_TYPELESS => TextureFormat::RG32_TYPELESS,
            diligent_sys::TEX_FORMAT_RG32_FLOAT => TextureFormat::RG32_FLOAT,
            diligent_sys::TEX_FORMAT_RG32_UINT => TextureFormat::RG32_UINT,
            diligent_sys::TEX_FORMAT_RG32_SINT => TextureFormat::RG32_SINT,
            diligent_sys::TEX_FORMAT_R32G8X24_TYPELESS => TextureFormat::R32G8X24_TYPELESS,
            diligent_sys::TEX_FORMAT_D32_FLOAT_S8X24_UINT => TextureFormat::D32_FLOAT_S8X24_UINT,
            diligent_sys::TEX_FORMAT_R32_FLOAT_X8X24_TYPELESS => {
                TextureFormat::R32_FLOAT_X8X24_TYPELESS
            }
            diligent_sys::TEX_FORMAT_X32_TYPELESS_G8X24_UINT => {
                TextureFormat::X32_TYPELESS_G8X24_UINT
            }
            diligent_sys::TEX_FORMAT_RGB10A2_TYPELESS => TextureFormat::RGB10A2_TYPELESS,
            diligent_sys::TEX_FORMAT_RGB10A2_UNORM => TextureFormat::RGB10A2_UNORM,
            diligent_sys::TEX_FORMAT_RGB10A2_UINT => TextureFormat::RGB10A2_UINT,
            diligent_sys::TEX_FORMAT_R11G11B10_FLOAT => TextureFormat::R11G11B10_FLOAT,
            diligent_sys::TEX_FORMAT_RGBA8_TYPELESS => TextureFormat::RGBA8_TYPELESS,
            diligent_sys::TEX_FORMAT_RGBA8_UNORM => TextureFormat::RGBA8_UNORM,
            diligent_sys::TEX_FORMAT_RGBA8_UNORM_SRGB => TextureFormat::RGBA8_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_RGBA8_UINT => TextureFormat::RGBA8_UINT,
            diligent_sys::TEX_FORMAT_RGBA8_SNORM => TextureFormat::RGBA8_SNORM,
            diligent_sys::TEX_FORMAT_RGBA8_SINT => TextureFormat::RGBA8_SINT,
            diligent_sys::TEX_FORMAT_RG16_TYPELESS => TextureFormat::RG16_TYPELESS,
            diligent_sys::TEX_FORMAT_RG16_FLOAT => TextureFormat::RG16_FLOAT,
            diligent_sys::TEX_FORMAT_RG16_UNORM => TextureFormat::RG16_UNORM,
            diligent_sys::TEX_FORMAT_RG16_UINT => TextureFormat::RG16_UINT,
            diligent_sys::TEX_FORMAT_RG16_SNORM => TextureFormat::RG16_SNORM,
            diligent_sys::TEX_FORMAT_RG16_SINT => TextureFormat::RG16_SINT,
            diligent_sys::TEX_FORMAT_R32_TYPELESS => TextureFormat::R32_TYPELESS,
            diligent_sys::TEX_FORMAT_D32_FLOAT => TextureFormat::D32_FLOAT,
            diligent_sys::TEX_FORMAT_R32_FLOAT => TextureFormat::R32_FLOAT,
            diligent_sys::TEX_FORMAT_R32_UINT => TextureFormat::R32_UINT,
            diligent_sys::TEX_FORMAT_R32_SINT => TextureFormat::R32_SINT,
            diligent_sys::TEX_FORMAT_R24G8_TYPELESS => TextureFormat::R24G8_TYPELESS,
            diligent_sys::TEX_FORMAT_D24_UNORM_S8_UINT => TextureFormat::D24_UNORM_S8_UINT,
            diligent_sys::TEX_FORMAT_R24_UNORM_X8_TYPELESS => TextureFormat::R24_UNORM_X8_TYPELESS,
            diligent_sys::TEX_FORMAT_X24_TYPELESS_G8_UINT => TextureFormat::X24_TYPELESS_G8_UINT,
            diligent_sys::TEX_FORMAT_RG8_TYPELESS => TextureFormat::RG8_TYPELESS,
            diligent_sys::TEX_FORMAT_RG8_UNORM => TextureFormat::RG8_UNORM,
            diligent_sys::TEX_FORMAT_RG8_UINT => TextureFormat::RG8_UINT,
            diligent_sys::TEX_FORMAT_RG8_SNORM => TextureFormat::RG8_SNORM,
            diligent_sys::TEX_FORMAT_RG8_SINT => TextureFormat::RG8_SINT,
            diligent_sys::TEX_FORMAT_R16_TYPELESS => TextureFormat::R16_TYPELESS,
            diligent_sys::TEX_FORMAT_R16_FLOAT => TextureFormat::R16_FLOAT,
            diligent_sys::TEX_FORMAT_D16_UNORM => TextureFormat::D16_UNORM,
            diligent_sys::TEX_FORMAT_R16_UNORM => TextureFormat::R16_UNORM,
            diligent_sys::TEX_FORMAT_R16_UINT => TextureFormat::R16_UINT,
            diligent_sys::TEX_FORMAT_R16_SNORM => TextureFormat::R16_SNORM,
            diligent_sys::TEX_FORMAT_R16_SINT => TextureFormat::R16_SINT,
            diligent_sys::TEX_FORMAT_R8_TYPELESS => TextureFormat::R8_TYPELESS,
            diligent_sys::TEX_FORMAT_R8_UNORM => TextureFormat::R8_UNORM,
            diligent_sys::TEX_FORMAT_R8_UINT => TextureFormat::R8_UINT,
            diligent_sys::TEX_FORMAT_R8_SNORM => TextureFormat::R8_SNORM,
            diligent_sys::TEX_FORMAT_R8_SINT => TextureFormat::R8_SINT,
            diligent_sys::TEX_FORMAT_A8_UNORM => TextureFormat::A8_UNORM,
            diligent_sys::TEX_FORMAT_R1_UNORM => TextureFormat::R1_UNORM,
            diligent_sys::TEX_FORMAT_RGB9E5_SHAREDEXP => TextureFormat::RGB9E5_SHAREDEXP,
            diligent_sys::TEX_FORMAT_RG8_B8G8_UNORM => TextureFormat::RG8_B8G8_UNORM,
            diligent_sys::TEX_FORMAT_G8R8_G8B8_UNORM => TextureFormat::G8R8_G8B8_UNORM,
            diligent_sys::TEX_FORMAT_BC1_TYPELESS => TextureFormat::BC1_TYPELESS,
            diligent_sys::TEX_FORMAT_BC1_UNORM => TextureFormat::BC1_UNORM,
            diligent_sys::TEX_FORMAT_BC1_UNORM_SRGB => TextureFormat::BC1_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_BC2_TYPELESS => TextureFormat::BC2_TYPELESS,
            diligent_sys::TEX_FORMAT_BC2_UNORM => TextureFormat::BC2_UNORM,
            diligent_sys::TEX_FORMAT_BC2_UNORM_SRGB => TextureFormat::BC2_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_BC3_TYPELESS => TextureFormat::BC3_TYPELESS,
            diligent_sys::TEX_FORMAT_BC3_UNORM => TextureFormat::BC3_UNORM,
            diligent_sys::TEX_FORMAT_BC3_UNORM_SRGB => TextureFormat::BC3_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_BC4_TYPELESS => TextureFormat::BC4_TYPELESS,
            diligent_sys::TEX_FORMAT_BC4_UNORM => TextureFormat::BC4_UNORM,
            diligent_sys::TEX_FORMAT_BC4_SNORM => TextureFormat::BC4_SNORM,
            diligent_sys::TEX_FORMAT_BC5_TYPELESS => TextureFormat::BC5_TYPELESS,
            diligent_sys::TEX_FORMAT_BC5_UNORM => TextureFormat::BC5_UNORM,
            diligent_sys::TEX_FORMAT_BC5_SNORM => TextureFormat::BC5_SNORM,
            diligent_sys::TEX_FORMAT_B5G6R5_UNORM => TextureFormat::B5G6R5_UNORM,
            diligent_sys::TEX_FORMAT_B5G5R5A1_UNORM => TextureFormat::B5G5R5A1_UNORM,
            diligent_sys::TEX_FORMAT_BGRA8_UNORM => TextureFormat::BGRA8_UNORM,
            diligent_sys::TEX_FORMAT_BGRX8_UNORM => TextureFormat::BGRX8_UNORM,
            diligent_sys::TEX_FORMAT_R10G10B10_XR_BIAS_A2_UNORM => {
                TextureFormat::R10G10B10_XR_BIAS_A2_UNORM
            }
            diligent_sys::TEX_FORMAT_BGRA8_TYPELESS => TextureFormat::BGRA8_TYPELESS,
            diligent_sys::TEX_FORMAT_BGRA8_UNORM_SRGB => TextureFormat::BGRA8_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_BGRX8_TYPELESS => TextureFormat::BGRX8_TYPELESS,
            diligent_sys::TEX_FORMAT_BGRX8_UNORM_SRGB => TextureFormat::BGRX8_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_BC6H_TYPELESS => TextureFormat::BC6H_TYPELESS,
            diligent_sys::TEX_FORMAT_BC6H_UF16 => TextureFormat::BC6H_UF16,
            diligent_sys::TEX_FORMAT_BC6H_SF16 => TextureFormat::BC6H_SF16,
            diligent_sys::TEX_FORMAT_BC7_TYPELESS => TextureFormat::BC7_TYPELESS,
            diligent_sys::TEX_FORMAT_BC7_UNORM => TextureFormat::BC7_UNORM,
            diligent_sys::TEX_FORMAT_BC7_UNORM_SRGB => TextureFormat::BC7_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_ETC2_RGB8_UNORM => TextureFormat::ETC2_RGB8_UNORM,
            diligent_sys::TEX_FORMAT_ETC2_RGB8_UNORM_SRGB => TextureFormat::ETC2_RGB8_UNORM_SRGB,
            diligent_sys::TEX_FORMAT_ETC2_RGB8A1_UNORM => TextureFormat::ETC2_RGB8A1_UNORM,
            diligent_sys::TEX_FORMAT_ETC2_RGB8A1_UNORM_SRGB => {
                TextureFormat::ETC2_RGB8A1_UNORM_SRGB
            }
            diligent_sys::TEX_FORMAT_ETC2_RGBA8_UNORM => TextureFormat::ETC2_RGBA8_UNORM,
            diligent_sys::TEX_FORMAT_ETC2_RGBA8_UNORM_SRGB => TextureFormat::ETC2_RGBA8_UNORM_SRGB,
            _ => panic!("Unknown texture format"),
        }
    }
}

impl TextureFormat {
    pub const fn component_size(self) -> u8 {
        match self {
            TextureFormat::RGBA32_TYPELESS
            | TextureFormat::RGBA32_FLOAT
            | TextureFormat::RGBA32_UINT
            | TextureFormat::RGBA32_SINT
            | TextureFormat::RGB32_TYPELESS
            | TextureFormat::RGB32_FLOAT
            | TextureFormat::RGB32_UINT
            | TextureFormat::RGB32_SINT
            | TextureFormat::RG32_TYPELESS
            | TextureFormat::RG32_FLOAT
            | TextureFormat::RG32_UINT
            | TextureFormat::RG32_SINT
            | TextureFormat::R32G8X24_TYPELESS
            | TextureFormat::D32_FLOAT_S8X24_UINT
            | TextureFormat::R32_FLOAT_X8X24_TYPELESS
            | TextureFormat::X32_TYPELESS_G8X24_UINT
            | TextureFormat::RGB10A2_TYPELESS
            | TextureFormat::RGB10A2_UNORM
            | TextureFormat::RGB10A2_UINT
            | TextureFormat::R11G11B10_FLOAT
            | TextureFormat::R32_TYPELESS
            | TextureFormat::D32_FLOAT
            | TextureFormat::R32_FLOAT
            | TextureFormat::R32_UINT
            | TextureFormat::R32_SINT
            | TextureFormat::R24G8_TYPELESS
            | TextureFormat::D24_UNORM_S8_UINT
            | TextureFormat::R24_UNORM_X8_TYPELESS
            | TextureFormat::X24_TYPELESS_G8_UINT
            | TextureFormat::RGB9E5_SHAREDEXP
            | TextureFormat::R10G10B10_XR_BIAS_A2_UNORM => 4,

            TextureFormat::RGBA16_TYPELESS
            | TextureFormat::RGBA16_FLOAT
            | TextureFormat::RGBA16_UNORM
            | TextureFormat::RGBA16_UINT
            | TextureFormat::RGBA16_SNORM
            | TextureFormat::RGBA16_SINT
            | TextureFormat::RG16_TYPELESS
            | TextureFormat::RG16_FLOAT
            | TextureFormat::RG16_UNORM
            | TextureFormat::RG16_UINT
            | TextureFormat::RG16_SNORM
            | TextureFormat::RG16_SINT
            | TextureFormat::R16_TYPELESS
            | TextureFormat::R16_FLOAT
            | TextureFormat::D16_UNORM
            | TextureFormat::R16_UNORM
            | TextureFormat::R16_UINT
            | TextureFormat::R16_SNORM
            | TextureFormat::R16_SINT
            | TextureFormat::B5G6R5_UNORM
            | TextureFormat::B5G5R5A1_UNORM => 2,

            TextureFormat::RGBA8_TYPELESS
            | TextureFormat::RGBA8_UNORM
            | TextureFormat::RGBA8_UNORM_SRGB
            | TextureFormat::RGBA8_UINT
            | TextureFormat::RGBA8_SNORM
            | TextureFormat::RGBA8_SINT
            | TextureFormat::RG8_TYPELESS
            | TextureFormat::RG8_UNORM
            | TextureFormat::RG8_UINT
            | TextureFormat::RG8_SNORM
            | TextureFormat::RG8_SINT
            | TextureFormat::R8_TYPELESS
            | TextureFormat::R8_UNORM
            | TextureFormat::R8_UINT
            | TextureFormat::R8_SNORM
            | TextureFormat::R8_SINT
            | TextureFormat::A8_UNORM
            | TextureFormat::R1_UNORM
            | TextureFormat::RG8_B8G8_UNORM
            | TextureFormat::G8R8_G8B8_UNORM
            | TextureFormat::BGRA8_UNORM
            | TextureFormat::BGRX8_UNORM
            | TextureFormat::BGRA8_TYPELESS
            | TextureFormat::BGRA8_UNORM_SRGB
            | TextureFormat::BGRX8_TYPELESS
            | TextureFormat::BGRX8_UNORM_SRGB => 1,

            TextureFormat::BC1_TYPELESS
            | TextureFormat::BC1_UNORM
            | TextureFormat::BC1_UNORM_SRGB
            | TextureFormat::BC4_TYPELESS
            | TextureFormat::BC4_UNORM
            | TextureFormat::BC4_SNORM
            | TextureFormat::ETC2_RGB8_UNORM
            | TextureFormat::ETC2_RGB8_UNORM_SRGB
            | TextureFormat::ETC2_RGB8A1_UNORM
            | TextureFormat::ETC2_RGB8A1_UNORM_SRGB => 8,

            TextureFormat::BC2_TYPELESS
            | TextureFormat::BC2_UNORM
            | TextureFormat::BC2_UNORM_SRGB
            | TextureFormat::BC3_TYPELESS
            | TextureFormat::BC3_UNORM
            | TextureFormat::BC3_UNORM_SRGB
            | TextureFormat::BC5_TYPELESS
            | TextureFormat::BC5_UNORM
            | TextureFormat::BC5_SNORM
            | TextureFormat::BC6H_TYPELESS
            | TextureFormat::BC6H_UF16
            | TextureFormat::BC6H_SF16
            | TextureFormat::BC7_TYPELESS
            | TextureFormat::BC7_UNORM
            | TextureFormat::BC7_UNORM_SRGB
            | TextureFormat::ETC2_RGBA8_UNORM
            | TextureFormat::ETC2_RGBA8_UNORM_SRGB => 16,
        }
    }

    pub const fn num_components(self) -> u8 {
        match self {
            TextureFormat::RGBA32_TYPELESS
            | TextureFormat::RGBA32_FLOAT
            | TextureFormat::RGBA32_UINT
            | TextureFormat::RGBA32_SINT
            | TextureFormat::RGBA16_TYPELESS
            | TextureFormat::RGBA16_FLOAT
            | TextureFormat::RGBA16_UNORM
            | TextureFormat::RGBA16_UINT
            | TextureFormat::RGBA16_SNORM
            | TextureFormat::RGBA16_SINT
            | TextureFormat::RGBA8_TYPELESS
            | TextureFormat::RGBA8_UNORM
            | TextureFormat::RGBA8_UNORM_SRGB
            | TextureFormat::RGBA8_UINT
            | TextureFormat::RGBA8_SNORM
            | TextureFormat::RGBA8_SINT
            | TextureFormat::RG8_B8G8_UNORM
            | TextureFormat::G8R8_G8B8_UNORM
            | TextureFormat::BC2_TYPELESS
            | TextureFormat::BC2_UNORM
            | TextureFormat::BC2_UNORM_SRGB
            | TextureFormat::BC3_TYPELESS
            | TextureFormat::BC3_UNORM
            | TextureFormat::BC3_UNORM_SRGB
            | TextureFormat::BGRA8_UNORM
            | TextureFormat::BGRX8_UNORM
            | TextureFormat::BGRA8_TYPELESS
            | TextureFormat::BGRA8_UNORM_SRGB
            | TextureFormat::BGRX8_TYPELESS
            | TextureFormat::BGRX8_UNORM_SRGB
            | TextureFormat::BC7_TYPELESS
            | TextureFormat::BC7_UNORM
            | TextureFormat::BC7_UNORM_SRGB
            | TextureFormat::ETC2_RGB8A1_UNORM
            | TextureFormat::ETC2_RGB8A1_UNORM_SRGB
            | TextureFormat::ETC2_RGBA8_UNORM
            | TextureFormat::ETC2_RGBA8_UNORM_SRGB => 4,

            TextureFormat::RGB32_TYPELESS
            | TextureFormat::RGB32_FLOAT
            | TextureFormat::RGB32_UINT
            | TextureFormat::RGB32_SINT
            | TextureFormat::BC1_TYPELESS
            | TextureFormat::BC1_UNORM
            | TextureFormat::BC1_UNORM_SRGB
            | TextureFormat::BC6H_TYPELESS
            | TextureFormat::BC6H_UF16
            | TextureFormat::BC6H_SF16
            | TextureFormat::ETC2_RGB8_UNORM
            | TextureFormat::ETC2_RGB8_UNORM_SRGB => 3,

            TextureFormat::RG32_TYPELESS
            | TextureFormat::RG32_FLOAT
            | TextureFormat::RG32_UINT
            | TextureFormat::RG32_SINT
            | TextureFormat::R32G8X24_TYPELESS
            | TextureFormat::D32_FLOAT_S8X24_UINT
            | TextureFormat::R32_FLOAT_X8X24_TYPELESS
            | TextureFormat::X32_TYPELESS_G8X24_UINT
            | TextureFormat::RG16_TYPELESS
            | TextureFormat::RG16_FLOAT
            | TextureFormat::RG16_UNORM
            | TextureFormat::RG16_UINT
            | TextureFormat::RG16_SNORM
            | TextureFormat::RG16_SINT
            | TextureFormat::RG8_TYPELESS
            | TextureFormat::RG8_UNORM
            | TextureFormat::RG8_UINT
            | TextureFormat::RG8_SNORM
            | TextureFormat::RG8_SINT
            | TextureFormat::BC5_TYPELESS
            | TextureFormat::BC5_UNORM
            | TextureFormat::BC5_SNORM => 2,

            TextureFormat::RGB10A2_TYPELESS
            | TextureFormat::RGB10A2_UNORM
            | TextureFormat::RGB10A2_UINT
            | TextureFormat::R11G11B10_FLOAT
            | TextureFormat::R32_TYPELESS
            | TextureFormat::D32_FLOAT
            | TextureFormat::R32_FLOAT
            | TextureFormat::R32_UINT
            | TextureFormat::R32_SINT
            | TextureFormat::R24G8_TYPELESS
            | TextureFormat::D24_UNORM_S8_UINT
            | TextureFormat::R24_UNORM_X8_TYPELESS
            | TextureFormat::X24_TYPELESS_G8_UINT
            | TextureFormat::R16_TYPELESS
            | TextureFormat::R16_FLOAT
            | TextureFormat::D16_UNORM
            | TextureFormat::R16_UNORM
            | TextureFormat::R16_UINT
            | TextureFormat::R16_SNORM
            | TextureFormat::R16_SINT
            | TextureFormat::R8_TYPELESS
            | TextureFormat::R8_UNORM
            | TextureFormat::R8_UINT
            | TextureFormat::R8_SNORM
            | TextureFormat::R8_SINT
            | TextureFormat::A8_UNORM
            | TextureFormat::R1_UNORM
            | TextureFormat::RGB9E5_SHAREDEXP
            | TextureFormat::BC4_TYPELESS
            | TextureFormat::BC4_UNORM
            | TextureFormat::BC4_SNORM
            | TextureFormat::B5G6R5_UNORM
            | TextureFormat::B5G5R5A1_UNORM
            | TextureFormat::R10G10B10_XR_BIAS_A2_UNORM => 1,
        }
    }

    pub const fn component_type(self) -> ComponentType {
        match self {
            TextureFormat::RGBA32_TYPELESS
            | TextureFormat::RGB32_TYPELESS
            | TextureFormat::RGBA16_TYPELESS
            | TextureFormat::RG32_TYPELESS
            | TextureFormat::RGBA8_TYPELESS
            | TextureFormat::RG16_TYPELESS
            | TextureFormat::R32_TYPELESS
            | TextureFormat::RG8_TYPELESS
            | TextureFormat::R16_TYPELESS
            | TextureFormat::R8_TYPELESS
            | TextureFormat::BGRA8_TYPELESS
            | TextureFormat::BGRX8_TYPELESS => ComponentType::Undefined,

            TextureFormat::RGBA32_FLOAT
            | TextureFormat::RGB32_FLOAT
            | TextureFormat::RGBA16_FLOAT
            | TextureFormat::RG32_FLOAT
            | TextureFormat::RG16_FLOAT
            | TextureFormat::R32_FLOAT
            | TextureFormat::R16_FLOAT => ComponentType::Float,

            TextureFormat::RGBA32_UINT
            | TextureFormat::RGB32_UINT
            | TextureFormat::RGBA16_UINT
            | TextureFormat::RG32_UINT
            | TextureFormat::RGBA8_UINT
            | TextureFormat::RG16_UINT
            | TextureFormat::R32_UINT
            | TextureFormat::RG8_UINT
            | TextureFormat::R16_UINT
            | TextureFormat::R8_UINT => ComponentType::Uint,

            TextureFormat::RGBA32_SINT
            | TextureFormat::RGB32_SINT
            | TextureFormat::RGBA16_SINT
            | TextureFormat::RG32_SINT
            | TextureFormat::RGBA8_SINT
            | TextureFormat::RG16_SINT
            | TextureFormat::R32_SINT
            | TextureFormat::RG8_SINT
            | TextureFormat::R16_SINT
            | TextureFormat::R8_SINT => ComponentType::Sint,

            TextureFormat::RGBA16_UNORM
            | TextureFormat::RGBA8_UNORM
            | TextureFormat::RGBA8_UNORM_SRGB
            | TextureFormat::RG16_UNORM
            | TextureFormat::RG8_UNORM
            | TextureFormat::R16_UNORM
            | TextureFormat::R8_UNORM
            | TextureFormat::A8_UNORM
            | TextureFormat::R1_UNORM
            | TextureFormat::RG8_B8G8_UNORM
            | TextureFormat::G8R8_G8B8_UNORM
            | TextureFormat::BGRA8_UNORM
            | TextureFormat::BGRX8_UNORM
            | TextureFormat::BGRA8_UNORM_SRGB
            | TextureFormat::BGRX8_UNORM_SRGB => ComponentType::UnormSRGB,

            TextureFormat::RGBA16_SNORM
            | TextureFormat::RGBA8_SNORM
            | TextureFormat::RG16_SNORM
            | TextureFormat::RG8_SNORM
            | TextureFormat::R16_SNORM
            | TextureFormat::R8_SNORM => ComponentType::Snorm,

            TextureFormat::R32G8X24_TYPELESS
            | TextureFormat::D32_FLOAT_S8X24_UINT
            | TextureFormat::R32_FLOAT_X8X24_TYPELESS
            | TextureFormat::X32_TYPELESS_G8X24_UINT
            | TextureFormat::R24G8_TYPELESS
            | TextureFormat::D24_UNORM_S8_UINT
            | TextureFormat::R24_UNORM_X8_TYPELESS
            | TextureFormat::X24_TYPELESS_G8_UINT => ComponentType::DepthStencil,

            TextureFormat::RGB10A2_TYPELESS
            | TextureFormat::RGB10A2_UNORM
            | TextureFormat::RGB10A2_UINT
            | TextureFormat::R11G11B10_FLOAT
            | TextureFormat::RGB9E5_SHAREDEXP
            | TextureFormat::B5G6R5_UNORM
            | TextureFormat::B5G5R5A1_UNORM
            | TextureFormat::R10G10B10_XR_BIAS_A2_UNORM => ComponentType::Compound,

            TextureFormat::D32_FLOAT | TextureFormat::D16_UNORM => ComponentType::Depth,

            TextureFormat::BC1_TYPELESS
            | TextureFormat::BC1_UNORM
            | TextureFormat::BC1_UNORM_SRGB
            | TextureFormat::BC2_TYPELESS
            | TextureFormat::BC2_UNORM
            | TextureFormat::BC2_UNORM_SRGB
            | TextureFormat::BC3_TYPELESS
            | TextureFormat::BC3_UNORM
            | TextureFormat::BC3_UNORM_SRGB
            | TextureFormat::BC4_TYPELESS
            | TextureFormat::BC4_UNORM
            | TextureFormat::BC4_SNORM
            | TextureFormat::BC5_TYPELESS
            | TextureFormat::BC5_UNORM
            | TextureFormat::BC5_SNORM
            | TextureFormat::BC6H_TYPELESS
            | TextureFormat::BC6H_UF16
            | TextureFormat::BC6H_SF16
            | TextureFormat::BC7_TYPELESS
            | TextureFormat::BC7_UNORM
            | TextureFormat::BC7_UNORM_SRGB
            | TextureFormat::ETC2_RGB8_UNORM
            | TextureFormat::ETC2_RGB8_UNORM_SRGB
            | TextureFormat::ETC2_RGB8A1_UNORM
            | TextureFormat::ETC2_RGB8A1_UNORM_SRGB
            | TextureFormat::ETC2_RGBA8_UNORM
            | TextureFormat::ETC2_RGBA8_UNORM_SRGB => ComponentType::Compressed,
        }
    }

    pub const fn is_typeless(self) -> bool {
        match self {
            TextureFormat::RGBA32_TYPELESS
            | TextureFormat::RGB32_TYPELESS
            | TextureFormat::RGBA16_TYPELESS
            | TextureFormat::RG32_TYPELESS
            | TextureFormat::R32G8X24_TYPELESS
            | TextureFormat::RGB10A2_TYPELESS
            | TextureFormat::RGBA8_TYPELESS
            | TextureFormat::RG16_TYPELESS
            | TextureFormat::R32_TYPELESS
            | TextureFormat::R24G8_TYPELESS
            | TextureFormat::RG8_TYPELESS
            | TextureFormat::R16_TYPELESS
            | TextureFormat::R8_TYPELESS
            | TextureFormat::BC1_TYPELESS
            | TextureFormat::BC2_TYPELESS
            | TextureFormat::BC3_TYPELESS
            | TextureFormat::BC4_TYPELESS
            | TextureFormat::BC5_TYPELESS
            | TextureFormat::BGRA8_TYPELESS
            | TextureFormat::BGRX8_TYPELESS
            | TextureFormat::BC6H_TYPELESS
            | TextureFormat::BC7_TYPELESS => true,

            TextureFormat::RGBA32_FLOAT
            | TextureFormat::RGBA32_UINT
            | TextureFormat::RGBA32_SINT
            | TextureFormat::RGB32_FLOAT
            | TextureFormat::RGB32_UINT
            | TextureFormat::RGB32_SINT
            | TextureFormat::RGBA16_FLOAT
            | TextureFormat::RGBA16_UNORM
            | TextureFormat::RGBA16_UINT
            | TextureFormat::RGBA16_SNORM
            | TextureFormat::RGBA16_SINT
            | TextureFormat::RG32_FLOAT
            | TextureFormat::RG32_UINT
            | TextureFormat::RG32_SINT
            | TextureFormat::D32_FLOAT_S8X24_UINT
            | TextureFormat::R32_FLOAT_X8X24_TYPELESS
            | TextureFormat::X32_TYPELESS_G8X24_UINT
            | TextureFormat::RGB10A2_UNORM
            | TextureFormat::RGB10A2_UINT
            | TextureFormat::R11G11B10_FLOAT
            | TextureFormat::RGBA8_UNORM
            | TextureFormat::RGBA8_UNORM_SRGB
            | TextureFormat::RGBA8_UINT
            | TextureFormat::RGBA8_SNORM
            | TextureFormat::RGBA8_SINT
            | TextureFormat::RG16_FLOAT
            | TextureFormat::RG16_UNORM
            | TextureFormat::RG16_UINT
            | TextureFormat::RG16_SNORM
            | TextureFormat::RG16_SINT
            | TextureFormat::D32_FLOAT
            | TextureFormat::R32_FLOAT
            | TextureFormat::R32_UINT
            | TextureFormat::R32_SINT
            | TextureFormat::D24_UNORM_S8_UINT
            | TextureFormat::R24_UNORM_X8_TYPELESS
            | TextureFormat::X24_TYPELESS_G8_UINT
            | TextureFormat::RG8_UNORM
            | TextureFormat::RG8_UINT
            | TextureFormat::RG8_SNORM
            | TextureFormat::RG8_SINT
            | TextureFormat::R16_FLOAT
            | TextureFormat::D16_UNORM
            | TextureFormat::R16_UNORM
            | TextureFormat::R16_UINT
            | TextureFormat::R16_SNORM
            | TextureFormat::R16_SINT
            | TextureFormat::R8_UNORM
            | TextureFormat::R8_UINT
            | TextureFormat::R8_SNORM
            | TextureFormat::R8_SINT
            | TextureFormat::A8_UNORM
            | TextureFormat::R1_UNORM
            | TextureFormat::RGB9E5_SHAREDEXP
            | TextureFormat::RG8_B8G8_UNORM
            | TextureFormat::G8R8_G8B8_UNORM
            | TextureFormat::BC1_UNORM
            | TextureFormat::BC1_UNORM_SRGB
            | TextureFormat::BC2_UNORM
            | TextureFormat::BC2_UNORM_SRGB
            | TextureFormat::BC3_UNORM
            | TextureFormat::BC3_UNORM_SRGB
            | TextureFormat::BC4_UNORM
            | TextureFormat::BC4_SNORM
            | TextureFormat::BC5_UNORM
            | TextureFormat::BC5_SNORM
            | TextureFormat::B5G6R5_UNORM
            | TextureFormat::B5G5R5A1_UNORM
            | TextureFormat::BGRA8_UNORM
            | TextureFormat::BGRX8_UNORM
            | TextureFormat::R10G10B10_XR_BIAS_A2_UNORM
            | TextureFormat::BGRA8_UNORM_SRGB
            | TextureFormat::BGRX8_UNORM_SRGB
            | TextureFormat::BC6H_UF16
            | TextureFormat::BC6H_SF16
            | TextureFormat::BC7_UNORM
            | TextureFormat::BC7_UNORM_SRGB
            | TextureFormat::ETC2_RGB8_UNORM
            | TextureFormat::ETC2_RGB8_UNORM_SRGB
            | TextureFormat::ETC2_RGB8A1_UNORM
            | TextureFormat::ETC2_RGB8A1_UNORM_SRGB
            | TextureFormat::ETC2_RGBA8_UNORM
            | TextureFormat::ETC2_RGBA8_UNORM_SRGB => false,
        }
    }

    pub const fn block_width(self) -> u8 {
        match self {
            TextureFormat::RGBA32_TYPELESS
            | TextureFormat::RGBA32_FLOAT
            | TextureFormat::RGBA32_UINT
            | TextureFormat::RGBA32_SINT
            | TextureFormat::RGB32_TYPELESS
            | TextureFormat::RGB32_FLOAT
            | TextureFormat::RGB32_UINT
            | TextureFormat::RGB32_SINT
            | TextureFormat::RGBA16_TYPELESS
            | TextureFormat::RGBA16_FLOAT
            | TextureFormat::RGBA16_UNORM
            | TextureFormat::RGBA16_UINT
            | TextureFormat::RGBA16_SNORM
            | TextureFormat::RGBA16_SINT
            | TextureFormat::RG32_TYPELESS
            | TextureFormat::RG32_FLOAT
            | TextureFormat::RG32_UINT
            | TextureFormat::RG32_SINT
            | TextureFormat::R32G8X24_TYPELESS
            | TextureFormat::D32_FLOAT_S8X24_UINT
            | TextureFormat::R32_FLOAT_X8X24_TYPELESS
            | TextureFormat::X32_TYPELESS_G8X24_UINT
            | TextureFormat::RGB10A2_TYPELESS
            | TextureFormat::RGB10A2_UNORM
            | TextureFormat::RGB10A2_UINT
            | TextureFormat::R11G11B10_FLOAT
            | TextureFormat::RGBA8_TYPELESS
            | TextureFormat::RGBA8_UNORM
            | TextureFormat::RGBA8_UNORM_SRGB
            | TextureFormat::RGBA8_UINT
            | TextureFormat::RGBA8_SNORM
            | TextureFormat::RGBA8_SINT
            | TextureFormat::RG16_TYPELESS
            | TextureFormat::RG16_FLOAT
            | TextureFormat::RG16_UNORM
            | TextureFormat::RG16_UINT
            | TextureFormat::RG16_SNORM
            | TextureFormat::RG16_SINT
            | TextureFormat::R32_TYPELESS
            | TextureFormat::D32_FLOAT
            | TextureFormat::R32_FLOAT
            | TextureFormat::R32_UINT
            | TextureFormat::R32_SINT
            | TextureFormat::R24G8_TYPELESS
            | TextureFormat::D24_UNORM_S8_UINT
            | TextureFormat::R24_UNORM_X8_TYPELESS
            | TextureFormat::X24_TYPELESS_G8_UINT
            | TextureFormat::RG8_TYPELESS
            | TextureFormat::RG8_UNORM
            | TextureFormat::RG8_UINT
            | TextureFormat::RG8_SNORM
            | TextureFormat::RG8_SINT
            | TextureFormat::R16_TYPELESS
            | TextureFormat::R16_FLOAT
            | TextureFormat::D16_UNORM
            | TextureFormat::R16_UNORM
            | TextureFormat::R16_UINT
            | TextureFormat::R16_SNORM
            | TextureFormat::R16_SINT
            | TextureFormat::R8_TYPELESS
            | TextureFormat::R8_UNORM
            | TextureFormat::R8_UINT
            | TextureFormat::R8_SNORM
            | TextureFormat::R8_SINT
            | TextureFormat::A8_UNORM
            | TextureFormat::R1_UNORM
            | TextureFormat::RGB9E5_SHAREDEXP
            | TextureFormat::RG8_B8G8_UNORM
            | TextureFormat::G8R8_G8B8_UNORM
            | TextureFormat::B5G6R5_UNORM
            | TextureFormat::B5G5R5A1_UNORM
            | TextureFormat::BGRA8_UNORM
            | TextureFormat::BGRX8_UNORM
            | TextureFormat::R10G10B10_XR_BIAS_A2_UNORM
            | TextureFormat::BGRA8_TYPELESS
            | TextureFormat::BGRA8_UNORM_SRGB
            | TextureFormat::BGRX8_TYPELESS
            | TextureFormat::BGRX8_UNORM_SRGB => 1,

            TextureFormat::BC1_TYPELESS
            | TextureFormat::BC1_UNORM
            | TextureFormat::BC1_UNORM_SRGB
            | TextureFormat::BC2_TYPELESS
            | TextureFormat::BC2_UNORM
            | TextureFormat::BC2_UNORM_SRGB
            | TextureFormat::BC3_TYPELESS
            | TextureFormat::BC3_UNORM
            | TextureFormat::BC3_UNORM_SRGB
            | TextureFormat::BC4_TYPELESS
            | TextureFormat::BC4_UNORM
            | TextureFormat::BC4_SNORM
            | TextureFormat::BC5_TYPELESS
            | TextureFormat::BC5_UNORM
            | TextureFormat::BC5_SNORM
            | TextureFormat::BC6H_TYPELESS
            | TextureFormat::BC6H_UF16
            | TextureFormat::BC6H_SF16
            | TextureFormat::BC7_TYPELESS
            | TextureFormat::BC7_UNORM
            | TextureFormat::BC7_UNORM_SRGB
            | TextureFormat::ETC2_RGB8_UNORM
            | TextureFormat::ETC2_RGB8_UNORM_SRGB
            | TextureFormat::ETC2_RGB8A1_UNORM
            | TextureFormat::ETC2_RGB8A1_UNORM_SRGB
            | TextureFormat::ETC2_RGBA8_UNORM
            | TextureFormat::ETC2_RGBA8_UNORM_SRGB => 4,
        }
    }

    pub const fn block_height(self) -> u8 {
        match self {
            TextureFormat::RGBA32_TYPELESS
            | TextureFormat::RGBA32_FLOAT
            | TextureFormat::RGBA32_UINT
            | TextureFormat::RGBA32_SINT
            | TextureFormat::RGB32_TYPELESS
            | TextureFormat::RGB32_FLOAT
            | TextureFormat::RGB32_UINT
            | TextureFormat::RGB32_SINT
            | TextureFormat::RGBA16_TYPELESS
            | TextureFormat::RGBA16_FLOAT
            | TextureFormat::RGBA16_UNORM
            | TextureFormat::RGBA16_UINT
            | TextureFormat::RGBA16_SNORM
            | TextureFormat::RGBA16_SINT
            | TextureFormat::RG32_TYPELESS
            | TextureFormat::RG32_FLOAT
            | TextureFormat::RG32_UINT
            | TextureFormat::RG32_SINT
            | TextureFormat::R32G8X24_TYPELESS
            | TextureFormat::D32_FLOAT_S8X24_UINT
            | TextureFormat::R32_FLOAT_X8X24_TYPELESS
            | TextureFormat::X32_TYPELESS_G8X24_UINT
            | TextureFormat::RGB10A2_TYPELESS
            | TextureFormat::RGB10A2_UNORM
            | TextureFormat::RGB10A2_UINT
            | TextureFormat::R11G11B10_FLOAT
            | TextureFormat::RGBA8_TYPELESS
            | TextureFormat::RGBA8_UNORM
            | TextureFormat::RGBA8_UNORM_SRGB
            | TextureFormat::RGBA8_UINT
            | TextureFormat::RGBA8_SNORM
            | TextureFormat::RGBA8_SINT
            | TextureFormat::RG16_TYPELESS
            | TextureFormat::RG16_FLOAT
            | TextureFormat::RG16_UNORM
            | TextureFormat::RG16_UINT
            | TextureFormat::RG16_SNORM
            | TextureFormat::RG16_SINT
            | TextureFormat::R32_TYPELESS
            | TextureFormat::D32_FLOAT
            | TextureFormat::R32_FLOAT
            | TextureFormat::R32_UINT
            | TextureFormat::R32_SINT
            | TextureFormat::R24G8_TYPELESS
            | TextureFormat::D24_UNORM_S8_UINT
            | TextureFormat::R24_UNORM_X8_TYPELESS
            | TextureFormat::X24_TYPELESS_G8_UINT
            | TextureFormat::RG8_TYPELESS
            | TextureFormat::RG8_UNORM
            | TextureFormat::RG8_UINT
            | TextureFormat::RG8_SNORM
            | TextureFormat::RG8_SINT
            | TextureFormat::R16_TYPELESS
            | TextureFormat::R16_FLOAT
            | TextureFormat::D16_UNORM
            | TextureFormat::R16_UNORM
            | TextureFormat::R16_UINT
            | TextureFormat::R16_SNORM
            | TextureFormat::R16_SINT
            | TextureFormat::R8_TYPELESS
            | TextureFormat::R8_UNORM
            | TextureFormat::R8_UINT
            | TextureFormat::R8_SNORM
            | TextureFormat::R8_SINT
            | TextureFormat::A8_UNORM
            | TextureFormat::R1_UNORM
            | TextureFormat::RGB9E5_SHAREDEXP
            | TextureFormat::RG8_B8G8_UNORM
            | TextureFormat::G8R8_G8B8_UNORM
            | TextureFormat::B5G6R5_UNORM
            | TextureFormat::B5G5R5A1_UNORM
            | TextureFormat::BGRA8_UNORM
            | TextureFormat::BGRX8_UNORM
            | TextureFormat::R10G10B10_XR_BIAS_A2_UNORM
            | TextureFormat::BGRA8_TYPELESS
            | TextureFormat::BGRA8_UNORM_SRGB
            | TextureFormat::BGRX8_TYPELESS
            | TextureFormat::BGRX8_UNORM_SRGB => 1,

            TextureFormat::BC1_TYPELESS
            | TextureFormat::BC1_UNORM
            | TextureFormat::BC1_UNORM_SRGB
            | TextureFormat::BC2_TYPELESS
            | TextureFormat::BC2_UNORM
            | TextureFormat::BC2_UNORM_SRGB
            | TextureFormat::BC3_TYPELESS
            | TextureFormat::BC3_UNORM
            | TextureFormat::BC3_UNORM_SRGB
            | TextureFormat::BC4_TYPELESS
            | TextureFormat::BC4_UNORM
            | TextureFormat::BC4_SNORM
            | TextureFormat::BC5_TYPELESS
            | TextureFormat::BC5_UNORM
            | TextureFormat::BC5_SNORM
            | TextureFormat::BC6H_TYPELESS
            | TextureFormat::BC6H_UF16
            | TextureFormat::BC6H_SF16
            | TextureFormat::BC7_TYPELESS
            | TextureFormat::BC7_UNORM
            | TextureFormat::BC7_UNORM_SRGB
            | TextureFormat::ETC2_RGB8_UNORM
            | TextureFormat::ETC2_RGB8_UNORM_SRGB
            | TextureFormat::ETC2_RGB8A1_UNORM
            | TextureFormat::ETC2_RGB8A1_UNORM_SRGB
            | TextureFormat::ETC2_RGBA8_UNORM
            | TextureFormat::ETC2_RGBA8_UNORM_SRGB => 4,
        }
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct ResourceDimensionSupport : diligent_sys::RESOURCE_DIMENSION_SUPPORT {
        const None          = diligent_sys::RESOURCE_DIMENSION_SUPPORT_NONE           as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const Buffer        = diligent_sys::RESOURCE_DIMENSION_SUPPORT_BUFFER         as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const Tex1D         = diligent_sys::RESOURCE_DIMENSION_SUPPORT_TEX_1D         as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const Tex1DArray    = diligent_sys::RESOURCE_DIMENSION_SUPPORT_TEX_1D_ARRAY   as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const Tex2D         = diligent_sys::RESOURCE_DIMENSION_SUPPORT_TEX_2D         as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const Tex2DArray    = diligent_sys::RESOURCE_DIMENSION_SUPPORT_TEX_2D_ARRAY   as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const Tex3D         = diligent_sys::RESOURCE_DIMENSION_SUPPORT_TEX_3D         as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const TexCube       = diligent_sys::RESOURCE_DIMENSION_SUPPORT_TEX_CUBE       as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
        const TexCube_array = diligent_sys::RESOURCE_DIMENSION_SUPPORT_TEX_CUBE_ARRAY as diligent_sys::RESOURCE_DIMENSION_SUPPORT;
    }
}

#[repr(transparent)]
pub struct TextureFormatInfoExt(diligent_sys::TextureFormatInfoExt);
impl TextureFormatInfoExt {
    pub fn bind_flags(&self) -> BindFlags {
        BindFlags::from_bits_retain(self.0.BindFlags)
    }
    pub fn dimensions(&self) -> ResourceDimensionSupport {
        ResourceDimensionSupport::from_bits_retain(self.0.Dimensions)
    }
    pub fn sample_counts(&self) -> SampleCount {
        SampleCount::from_bits_retain(self.0.SampleCounts)
    }
    pub fn filterable(&self) -> bool {
        self.0.Filterable
    }
}

#[repr(transparent)]
pub struct SparseTextureFormatInfo(diligent_sys::SparseTextureFormatInfo);

impl SparseTextureFormatInfo {
    pub(crate) fn new(sys: diligent_sys::SparseTextureFormatInfo) -> Self {
        Self(sys)
    }
    pub fn bind_flags(&self) -> BindFlags {
        BindFlags::from_bits_retain(self.0.BindFlags)
    }
    pub fn tile_size(&self) -> &[u32; 3usize] {
        &self.0.TileSize
    }
    pub fn flags(&self) -> SparseTextureFlags {
        SparseTextureFlags::from_bits_retain(self.0.Flags)
    }
}

#[derive(Clone, Copy)]
pub enum ScalingMode {
    Unspecified,
    Centered,
    Stretched,
}

impl From<diligent_sys::SCALING_MODE> for ScalingMode {
    fn from(value: diligent_sys::SCALING_MODE) -> Self {
        match value {
            diligent_sys::SCALING_MODE_UNSPECIFIED => ScalingMode::Unspecified,
            diligent_sys::SCALING_MODE_CENTERED => ScalingMode::Centered,
            diligent_sys::SCALING_MODE_STRETCHED => ScalingMode::Stretched,
            _ => panic!("Unknown scaling mode"),
        }
    }
}

impl From<ScalingMode> for diligent_sys::SCALING_MODE {
    fn from(value: ScalingMode) -> Self {
        match value {
            ScalingMode::Unspecified => diligent_sys::SCALING_MODE_UNSPECIFIED,
            ScalingMode::Centered => diligent_sys::SCALING_MODE_CENTERED,
            ScalingMode::Stretched => diligent_sys::SCALING_MODE_STRETCHED,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ScanlineOrder {
    Unspecified,
    Progressive,
    UpperFieldFirst,
    LowerFieldFirst,
}

impl From<diligent_sys::SCANLINE_ORDER> for ScanlineOrder {
    fn from(value: diligent_sys::SCANLINE_ORDER) -> Self {
        match value {
            diligent_sys::SCANLINE_ORDER_UNSPECIFIED => ScanlineOrder::Unspecified,
            diligent_sys::SCANLINE_ORDER_PROGRESSIVE => ScanlineOrder::Progressive,
            diligent_sys::SCANLINE_ORDER_UPPER_FIELD_FIRST => ScanlineOrder::UpperFieldFirst,
            diligent_sys::SCANLINE_ORDER_LOWER_FIELD_FIRST => ScanlineOrder::LowerFieldFirst,
            _ => panic!("Unknown scanline order"),
        }
    }
}

impl From<ScanlineOrder> for diligent_sys::SCANLINE_ORDER {
    fn from(value: ScanlineOrder) -> Self {
        match value {
            ScanlineOrder::Unspecified => diligent_sys::SCANLINE_ORDER_UNSPECIFIED,
            ScanlineOrder::Progressive => diligent_sys::SCANLINE_ORDER_PROGRESSIVE,
            ScanlineOrder::UpperFieldFirst => diligent_sys::SCANLINE_ORDER_UPPER_FIELD_FIRST,
            ScanlineOrder::LowerFieldFirst => diligent_sys::SCANLINE_ORDER_LOWER_FIELD_FIRST,
        }
    }
}

pub struct DisplayModeAttribs(pub(crate) diligent_sys::DisplayModeAttribs);

impl DisplayModeAttribs {
    pub fn width(&self) -> u32 {
        self.0.Width
    }
    pub fn height(&self) -> u32 {
        self.0.Height
    }
    pub fn format(&self) -> TextureFormat {
        self.0.Format.into()
    }
    pub fn refresh_rate_numerator(&self) -> u32 {
        self.0.RefreshRateNumerator
    }
    pub fn refresh_rate_denominator(&self) -> u32 {
        self.0.RefreshRateDenominator
    }
    pub fn scaling_mode(&self) -> ScalingMode {
        match self.0.Scaling as _ {
            diligent_sys::SCALING_MODE_UNSPECIFIED => ScalingMode::Unspecified,
            diligent_sys::SCALING_MODE_CENTERED => ScalingMode::Centered,
            diligent_sys::SCALING_MODE_STRETCHED => ScalingMode::Stretched,
            _ => panic!("Unknown SCALING_MODE value"),
        }
    }
    pub fn scanline_order(&self) -> ScanlineOrder {
        match self.0.ScanlineOrder as _ {
            diligent_sys::SCANLINE_ORDER_UNSPECIFIED => ScanlineOrder::Unspecified,
            diligent_sys::SCANLINE_ORDER_UPPER_FIELD_FIRST => ScanlineOrder::UpperFieldFirst,
            diligent_sys::SCANLINE_ORDER_LOWER_FIELD_FIRST => ScanlineOrder::LowerFieldFirst,
            diligent_sys::SCANLINE_ORDER_PROGRESSIVE => ScanlineOrder::Progressive,
            _ => panic!("Unknown SCANLINE_ORDER value"),
        }
    }
}

#[derive(Builder)]
pub struct FullScreenModeDesc {
    #[builder(default = false)]
    fullscreen: bool,

    #[builder(default = 0)]
    refresh_rate_numerator: u32,

    #[builder(default = 0)]
    refresh_rate_denominator: u32,

    #[builder(default = ScalingMode::Unspecified)]
    scaling: ScalingMode,

    #[builder(default = ScanlineOrder::Unspecified)]
    scanline_order: ScanlineOrder,
}

impl From<&FullScreenModeDesc> for diligent_sys::FullScreenModeDesc {
    fn from(value: &FullScreenModeDesc) -> Self {
        diligent_sys::FullScreenModeDesc {
            Fullscreen: value.fullscreen,
            RefreshRateDenominator: value.refresh_rate_denominator,
            RefreshRateNumerator: value.refresh_rate_numerator,
            Scaling: value.scaling.into(),
            ScanlineOrder: value.scanline_order.into(),
        }
    }
}

impl Default for FullScreenModeDesc {
    fn default() -> Self {
        FullScreenModeDesc {
            fullscreen: false,
            refresh_rate_numerator: 0,
            refresh_rate_denominator: 0,
            scaling: ScalingMode::Unspecified,
            scanline_order: ScanlineOrder::Unspecified,
        }
    }
}

#[derive(Clone, Copy)]
pub enum StateTransitionType {
    Immediate,
    Begin,
    End,
}

impl From<StateTransitionType> for diligent_sys::STATE_TRANSITION_TYPE {
    fn from(value: StateTransitionType) -> Self {
        (match value {
            StateTransitionType::Immediate => diligent_sys::STATE_TRANSITION_TYPE_IMMEDIATE,
            StateTransitionType::Begin => diligent_sys::STATE_TRANSITION_TYPE_BEGIN,
            StateTransitionType::End => diligent_sys::STATE_TRANSITION_TYPE_END,
        }) as _
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct AccessFlags : diligent_sys::ACCESS_FLAGS {
        const None                       = diligent_sys::ACCESS_FLAG_NONE as diligent_sys::ACCESS_FLAGS;
        const IndirectCommandRead        = diligent_sys::ACCESS_FLAG_INDIRECT_COMMAND_READ as diligent_sys::ACCESS_FLAGS;
        const IndexRead                  = diligent_sys::ACCESS_FLAG_INDEX_READ as diligent_sys::ACCESS_FLAGS;
        const VertexRead                 = diligent_sys::ACCESS_FLAG_VERTEX_READ as diligent_sys::ACCESS_FLAGS;
        const UniformRead                = diligent_sys::ACCESS_FLAG_UNIFORM_READ as diligent_sys::ACCESS_FLAGS;
        const InputAttachmentRead        = diligent_sys::ACCESS_FLAG_INPUT_ATTACHMENT_READ as diligent_sys::ACCESS_FLAGS;
        const ShaderRead                 = diligent_sys::ACCESS_FLAG_SHADER_READ as diligent_sys::ACCESS_FLAGS;
        const ShaderWrite                = diligent_sys::ACCESS_FLAG_SHADER_WRITE as diligent_sys::ACCESS_FLAGS;
        const RenderTargetRead           = diligent_sys::ACCESS_FLAG_RENDER_TARGET_READ as diligent_sys::ACCESS_FLAGS;
        const RenderTargetWrite          = diligent_sys::ACCESS_FLAG_RENDER_TARGET_WRITE as diligent_sys::ACCESS_FLAGS;
        const DepthStencilRead           = diligent_sys::ACCESS_FLAG_DEPTH_STENCIL_READ as diligent_sys::ACCESS_FLAGS;
        const DepthStencilWrite          = diligent_sys::ACCESS_FLAG_DEPTH_STENCIL_WRITE as diligent_sys::ACCESS_FLAGS;
        const CopySrc                    = diligent_sys::ACCESS_FLAG_COPY_SRC as diligent_sys::ACCESS_FLAGS;
        const CopyDst                    = diligent_sys::ACCESS_FLAG_COPY_DST as diligent_sys::ACCESS_FLAGS;
        const HostRead                   = diligent_sys::ACCESS_FLAG_HOST_READ as diligent_sys::ACCESS_FLAGS;
        const HostWrite                  = diligent_sys::ACCESS_FLAG_HOST_WRITE as diligent_sys::ACCESS_FLAGS;
        const MemoryRead                 = diligent_sys::ACCESS_FLAG_MEMORY_READ as diligent_sys::ACCESS_FLAGS;
        const MemoryWrite                = diligent_sys::ACCESS_FLAG_MEMORY_WRITE as diligent_sys::ACCESS_FLAGS;
        const ConditionalRenderingRead   = diligent_sys::ACCESS_FLAG_CONDITIONAL_RENDERING_READ as diligent_sys::ACCESS_FLAGS;
        const ShadingRateTextureRead     = diligent_sys::ACCESS_FLAG_SHADING_RATE_TEXTURE_READ as diligent_sys::ACCESS_FLAGS;
        const AccelerationStructureRead  = diligent_sys::ACCESS_FLAG_ACCELERATION_STRUCTURE_READ as diligent_sys::ACCESS_FLAGS;
        const AccelerationStructureWrite = diligent_sys::ACCESS_FLAG_ACCELERATION_STRUCTURE_WRITE as diligent_sys::ACCESS_FLAGS;
        const FragmentDensityMapRead     = diligent_sys::ACCESS_FLAG_FRAGMENT_DENSITY_MAP_READ as diligent_sys::ACCESS_FLAGS;
        const Default                    = diligent_sys::ACCESS_FLAG_DEFAULT as diligent_sys::ACCESS_FLAGS;
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct PipelineStageFlags : diligent_sys::PIPELINE_STAGE_FLAGS {
        const Undefined                  = diligent_sys::PIPELINE_STAGE_FLAG_UNDEFINED                    as diligent_sys::PIPELINE_STAGE_FLAGS;
        const TopOfPipe                  = diligent_sys::PIPELINE_STAGE_FLAG_TOP_OF_PIPE                  as diligent_sys::PIPELINE_STAGE_FLAGS;
        const DrawIndirect               = diligent_sys::PIPELINE_STAGE_FLAG_DRAW_INDIRECT                as diligent_sys::PIPELINE_STAGE_FLAGS;
        const VertexInput                = diligent_sys::PIPELINE_STAGE_FLAG_VERTEX_INPUT                 as diligent_sys::PIPELINE_STAGE_FLAGS;
        const VertexShader               = diligent_sys::PIPELINE_STAGE_FLAG_VERTEX_SHADER                as diligent_sys::PIPELINE_STAGE_FLAGS;
        const HullShader                 = diligent_sys::PIPELINE_STAGE_FLAG_HULL_SHADER                  as diligent_sys::PIPELINE_STAGE_FLAGS;
        const DomainShader               = diligent_sys::PIPELINE_STAGE_FLAG_DOMAIN_SHADER                as diligent_sys::PIPELINE_STAGE_FLAGS;
        const GeometryShader             = diligent_sys::PIPELINE_STAGE_FLAG_GEOMETRY_SHADER              as diligent_sys::PIPELINE_STAGE_FLAGS;
        const PixelShader                = diligent_sys::PIPELINE_STAGE_FLAG_PIXEL_SHADER                 as diligent_sys::PIPELINE_STAGE_FLAGS;
        const EarlyFragmentTests         = diligent_sys::PIPELINE_STAGE_FLAG_EARLY_FRAGMENT_TESTS         as diligent_sys::PIPELINE_STAGE_FLAGS;
        const Late_fragmentTests         = diligent_sys::PIPELINE_STAGE_FLAG_LATE_FRAGMENT_TESTS          as diligent_sys::PIPELINE_STAGE_FLAGS;
        const RenderTarget               = diligent_sys::PIPELINE_STAGE_FLAG_RENDER_TARGET                as diligent_sys::PIPELINE_STAGE_FLAGS;
        const ComputeShader              = diligent_sys::PIPELINE_STAGE_FLAG_COMPUTE_SHADER               as diligent_sys::PIPELINE_STAGE_FLAGS;
        const Transfer                   = diligent_sys::PIPELINE_STAGE_FLAG_TRANSFER                     as diligent_sys::PIPELINE_STAGE_FLAGS;
        const BottomOfPipe               = diligent_sys::PIPELINE_STAGE_FLAG_BOTTOM_OF_PIPE               as diligent_sys::PIPELINE_STAGE_FLAGS;
        const Host                       = diligent_sys::PIPELINE_STAGE_FLAG_HOST                         as diligent_sys::PIPELINE_STAGE_FLAGS;
        const ConditionalRendering       = diligent_sys::PIPELINE_STAGE_FLAG_CONDITIONAL_RENDERING        as diligent_sys::PIPELINE_STAGE_FLAGS;
        const ShadingRateTexture         = diligent_sys::PIPELINE_STAGE_FLAG_SHADING_RATE_TEXTURE         as diligent_sys::PIPELINE_STAGE_FLAGS;
        const RayTracingShader           = diligent_sys::PIPELINE_STAGE_FLAG_RAY_TRACING_SHADER           as diligent_sys::PIPELINE_STAGE_FLAGS;
        const AccelerationStructureBuild = diligent_sys::PIPELINE_STAGE_FLAG_ACCELERATION_STRUCTURE_BUILD as diligent_sys::PIPELINE_STAGE_FLAGS;
        const TaskShader                 = diligent_sys::PIPELINE_STAGE_FLAG_TASK_SHADER                  as diligent_sys::PIPELINE_STAGE_FLAGS;
        const MeshShader                 = diligent_sys::PIPELINE_STAGE_FLAG_MESH_SHADER                  as diligent_sys::PIPELINE_STAGE_FLAGS;
        const FragmentDensityProcess     = diligent_sys::PIPELINE_STAGE_FLAG_FRAGMENT_DENSITY_PROCESS     as diligent_sys::PIPELINE_STAGE_FLAGS;
        const Default                    = diligent_sys::PIPELINE_STAGE_FLAG_DEFAULT                      as diligent_sys::PIPELINE_STAGE_FLAGS;
    }
}

#[derive(Clone, Copy)]
pub enum MemoryProperty {
    HostCoherent,
}

#[repr(transparent)]
pub struct ImmediateContextCreateInfo(diligent_sys::ImmediateContextCreateInfo);

#[bon::bon]
impl ImmediateContextCreateInfo {
    #[builder]
    pub fn new(name: Option<&CStr>, queue_id: u8, priority: Option<QueuePriority>) -> Self {
        Self(diligent_sys::ImmediateContextCreateInfo {
            Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
            QueueId: queue_id,
            Priority: priority.map_or(diligent_sys::QUEUE_PRIORITY_UNKNOWN, |priority| {
                match priority {
                    QueuePriority::Low => diligent_sys::QUEUE_PRIORITY_LOW,
                    QueuePriority::Medium => diligent_sys::QUEUE_PRIORITY_MEDIUM,
                    QueuePriority::High => diligent_sys::QUEUE_PRIORITY_HIGH,
                    QueuePriority::RealTime => diligent_sys::QUEUE_PRIORITY_REALTIME,
                }
            }) as _,
        })
    }
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct ValidationFlags : diligent_sys::VALIDATION_FLAGS {
        const None                  = diligent_sys::VALIDATION_FLAG_NONE                     as diligent_sys::VALIDATION_FLAGS;
        const CheckShaderBufferSize = diligent_sys::VALIDATION_FLAG_CHECK_SHADER_BUFFER_SIZE as diligent_sys::VALIDATION_FLAGS;
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Box(pub(crate) diligent_sys::Box);

#[bon::bon]
impl Box {
    #[builder]
    pub fn new(
        #[builder(default = 0)] min_x: u32,
        #[builder(default = 0)] max_x: u32,
        #[builder(default = 0)] min_y: u32,
        #[builder(default = 0)] max_y: u32,
        #[builder(default = 0)] min_z: u32,
        #[builder(default = 1)] max_z: u32,
    ) -> Self {
        Self(diligent_sys::Box {
            MinX: min_x,
            MaxX: max_x,
            MinY: min_y,
            MaxY: max_y,
            MinZ: min_z,
            MaxZ: max_z,
        })
    }
}

impl Box {
    pub fn width(&self) -> u32 {
        self.0.MaxX - self.0.MinX
    }

    pub fn height(&self) -> u32 {
        self.0.MaxY - self.0.MinY
    }

    pub fn depth(&self) -> u32 {
        self.0.MaxZ - self.0.MinZ
    }

    pub fn is_valid(&self) -> bool {
        self.0.MaxX > self.0.MinX && self.0.MaxY > self.0.MinY && self.0.MaxZ > self.0.MinZ
    }
}
