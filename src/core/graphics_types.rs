use bitflags::bitflags;
use static_assertions::const_assert;

use crate::bindings;

impl Default for bindings::DeviceFeatures {
    fn default() -> Self {
        bindings::DeviceFeatures {
            SeparablePrograms: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ShaderResourceQueries: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            WireframeFill: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            MultithreadedResourceCreation: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ComputeShaders: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            GeometryShaders: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            Tessellation: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            MeshShaders: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            RayTracing: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            BindlessResources: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            OcclusionQueries: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            BinaryOcclusionQueries: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TimestampQueries: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            PipelineStatisticsQueries: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            DurationQueries: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            DepthBiasClamp: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            DepthClamp: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            IndependentBlend: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            DualSourceBlend: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            MultiViewport: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TextureCompressionBC: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TextureCompressionETC2: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            VertexPipelineUAVWritesAndAtomics: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            PixelUAVWritesAndAtomics: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TextureUAVExtendedFormats: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ShaderFloat16: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ResourceBuffer16BitAccess: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            UniformBuffer16BitAccess: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ShaderInputOutput16: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ShaderInt8: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ResourceBuffer8BitAccess: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            UniformBuffer8BitAccess: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ShaderResourceStaticArrays: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            ShaderResourceRuntimeArrays: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            WaveOp: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            InstanceDataStepRate: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            NativeFence: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TileShaders: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TransferQueueTimestampQueries: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            VariableRateShading: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            SparseResources: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            SubpassFramebufferFetch: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TextureComponentSwizzle: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            TextureSubresourceViews: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            NativeMultiDraw: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            AsyncShaderCompilation: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
            FormattedBuffers: bindings::DEVICE_FEATURE_STATE_DISABLED as u8,
        }
    }
}

bitflags! {
    pub struct ShaderTypes: bindings::_SHADER_TYPE {
        const Vertex          = bindings::SHADER_TYPE_VERTEX;
        const Pixel           = bindings::SHADER_TYPE_PIXEL;
        const Geometry        = bindings::SHADER_TYPE_GEOMETRY;
        const Hull            = bindings::SHADER_TYPE_HULL;
        const Domain          = bindings::SHADER_TYPE_DOMAIN;
        const Compute         = bindings::SHADER_TYPE_COMPUTE;
        const Amplification   = bindings::SHADER_TYPE_AMPLIFICATION;
        const Mesh            = bindings::SHADER_TYPE_MESH;
        const RayGen          = bindings::SHADER_TYPE_RAY_GEN;
        const RayMiss         = bindings::SHADER_TYPE_RAY_MISS;
        const RayClosestHit   = bindings::SHADER_TYPE_RAY_CLOSEST_HIT;
        const RayAnyHit       = bindings::SHADER_TYPE_RAY_ANY_HIT;
        const RayIntersection = bindings::SHADER_TYPE_RAY_INTERSECTION;
        const Callable        = bindings::SHADER_TYPE_CALLABLE;
        const Tile            = bindings::SHADER_TYPE_TILE;

        const VertexPixel   = bindings::SHADER_TYPE_VS_PS;
        const AllGraphics   = bindings::SHADER_TYPE_ALL_GRAPHICS;
        const AllMesh       = bindings::SHADER_TYPE_ALL_MESH;
        const AllRayTracing = bindings::SHADER_TYPE_ALL_RAY_TRACING;
        const All           = bindings::SHADER_TYPE_ALL;
    }
}
const_assert!(bindings::SHADER_TYPE_LAST == 16384);

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

impl Into<bindings::SHADER_TYPE> for ShaderType {
    fn into(self) -> bindings::SHADER_TYPE {
        (match self {
            ShaderType::Vertex => bindings::SHADER_TYPE_VERTEX,
            ShaderType::Pixel => bindings::SHADER_TYPE_PIXEL,
            ShaderType::Geometry => bindings::SHADER_TYPE_GEOMETRY,
            ShaderType::Hull => bindings::SHADER_TYPE_HULL,
            ShaderType::Domain => bindings::SHADER_TYPE_DOMAIN,
            ShaderType::Compute => bindings::SHADER_TYPE_COMPUTE,
            ShaderType::Amplification => bindings::SHADER_TYPE_AMPLIFICATION,
            ShaderType::Mesh => bindings::SHADER_TYPE_MESH,
            ShaderType::RayGen => bindings::SHADER_TYPE_RAY_GEN,
            ShaderType::RayMiss => bindings::SHADER_TYPE_RAY_MISS,
            ShaderType::RayClosestHit => bindings::SHADER_TYPE_RAY_CLOSEST_HIT,
            ShaderType::RayAnyHit => bindings::SHADER_TYPE_RAY_ANY_HIT,
            ShaderType::RayIntersection => bindings::SHADER_TYPE_RAY_INTERSECTION,
            ShaderType::Callable => bindings::SHADER_TYPE_CALLABLE,
            ShaderType::Tile => bindings::SHADER_TYPE_TILE,
        }) as bindings::SHADER_TYPE
    }
}

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
const_assert!(bindings::FILTER_TYPE_NUM_FILTERS == 13);

impl Into<bindings::FILTER_TYPE> for FilterType {
    fn into(self) -> bindings::FILTER_TYPE {
        (match self {
            FilterType::Point => bindings::FILTER_TYPE_POINT,
            FilterType::Linear => bindings::FILTER_TYPE_LINEAR,
            FilterType::Anisotropic => bindings::FILTER_TYPE_ANISOTROPIC,
            FilterType::ComparisonPoint => bindings::FILTER_TYPE_COMPARISON_POINT,
            FilterType::ComparisonLinear => bindings::FILTER_TYPE_COMPARISON_LINEAR,
            FilterType::ComparisonAnisotropic => bindings::FILTER_TYPE_COMPARISON_ANISOTROPIC,
            FilterType::MinimumPoint => bindings::FILTER_TYPE_MINIMUM_POINT,
            FilterType::MinimumLinear => bindings::FILTER_TYPE_MINIMUM_LINEAR,
            FilterType::MinimumAnisotropic => bindings::FILTER_TYPE_MINIMUM_ANISOTROPIC,
            FilterType::MaximumPoint => bindings::FILTER_TYPE_MAXIMUM_POINT,
            FilterType::MaximumLinear => bindings::FILTER_TYPE_MAXIMUM_LINEAR,
            FilterType::MaximumAnisotropic => bindings::FILTER_TYPE_MAXIMUM_ANISOTROPIC,
        }) as bindings::FILTER_TYPE
    }
}

pub enum TextureAddressMode {
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}
const_assert!(bindings::TEXTURE_ADDRESS_NUM_MODES == 6);

impl Into<bindings::TEXTURE_ADDRESS_MODE> for TextureAddressMode {
    fn into(self) -> bindings::TEXTURE_ADDRESS_MODE {
        (match self {
            TextureAddressMode::Wrap => bindings::TEXTURE_ADDRESS_WRAP,
            TextureAddressMode::Mirror => bindings::TEXTURE_ADDRESS_MIRROR,
            TextureAddressMode::Clamp => bindings::TEXTURE_ADDRESS_CLAMP,
            TextureAddressMode::Border => bindings::TEXTURE_ADDRESS_BORDER,
            TextureAddressMode::MirrorOnce => bindings::TEXTURE_ADDRESS_MIRROR_ONCE,
        }) as bindings::TEXTURE_ADDRESS_MODE
    }
}

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
const_assert!(bindings::PRIMITIVE_TOPOLOGY_NUM_TOPOLOGIES == 42);

impl Into<bindings::PRIMITIVE_TOPOLOGY> for PrimitiveTopology {
    fn into(self) -> bindings::PRIMITIVE_TOPOLOGY {
        (match self {
            PrimitiveTopology::TriangleList => bindings::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            PrimitiveTopology::TriangleStrip => bindings::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            PrimitiveTopology::PointList => bindings::PRIMITIVE_TOPOLOGY_POINT_LIST,
            PrimitiveTopology::LineList => bindings::PRIMITIVE_TOPOLOGY_LINE_LIST,
            PrimitiveTopology::LineStrip => bindings::PRIMITIVE_TOPOLOGY_LINE_STRIP,
            PrimitiveTopology::TriangleListAdj => bindings::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_ADJ,
            PrimitiveTopology::TriangleStripAdj => bindings::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_ADJ,
            PrimitiveTopology::LineListAdj => bindings::PRIMITIVE_TOPOLOGY_LINE_LIST_ADJ,
            PrimitiveTopology::LineStripAdj => bindings::PRIMITIVE_TOPOLOGY_LINE_STRIP_ADJ,
            PrimitiveTopology::ControlPointPatchList1 => {
                bindings::PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList2 => {
                bindings::PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList3 => {
                bindings::PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList4 => {
                bindings::PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList5 => {
                bindings::PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList6 => {
                bindings::PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList7 => {
                bindings::PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList8 => {
                bindings::PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList9 => {
                bindings::PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList10 => {
                bindings::PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList11 => {
                bindings::PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList12 => {
                bindings::PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList13 => {
                bindings::PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList14 => {
                bindings::PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList15 => {
                bindings::PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList16 => {
                bindings::PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList17 => {
                bindings::PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList18 => {
                bindings::PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList19 => {
                bindings::PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList20 => {
                bindings::PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList21 => {
                bindings::PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList22 => {
                bindings::PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList23 => {
                bindings::PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList24 => {
                bindings::PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList25 => {
                bindings::PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList26 => {
                bindings::PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList27 => {
                bindings::PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList28 => {
                bindings::PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList29 => {
                bindings::PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList30 => {
                bindings::PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList31 => {
                bindings::PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST
            }
            PrimitiveTopology::ControlPointPatchList32 => {
                bindings::PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST
            }
        }) as bindings::PRIMITIVE_TOPOLOGY
    }
}

bitflags! {
    pub struct BindFlags: bindings::BIND_FLAGS {
        const None             = bindings::BIND_NONE;
        const VertexBuffer     = bindings::BIND_VERTEX_BUFFER;
        const IndexBuffer      = bindings::BIND_INDEX_BUFFER;
        const UniformBuffer    = bindings::BIND_UNIFORM_BUFFER;
        const ShaderResourcec  = bindings::BIND_SHADER_RESOURCE;
        const StreamOutput     = bindings::BIND_STREAM_OUTPUT;
        const RenderTarget     = bindings::BIND_RENDER_TARGET;
        const DepthStencil     = bindings::BIND_DEPTH_STENCIL;
        const UnorderedAccess  = bindings::BIND_UNORDERED_ACCESS;
        const IndirectDrawArgs = bindings::BIND_INDIRECT_DRAW_ARGS;
        const InputAttachement = bindings::BIND_INPUT_ATTACHMENT;
        const RayTracing       = bindings::BIND_RAY_TRACING;
        const ShadingRate      = bindings::BIND_SHADING_RATE;
    }
}
const_assert!(bindings::BIND_FLAG_LAST == 2048);

pub enum Usage {
    Immutable,
    Default,
    Dynamic,
    Staging,
    Unified,
    Sparse,
}
const_assert!(bindings::USAGE_NUM_USAGES == 6);

impl Into<bindings::USAGE> for Usage {
    fn into(self) -> bindings::USAGE {
        (match self {
            Usage::Immutable => bindings::USAGE_IMMUTABLE,
            Usage::Default => bindings::USAGE_DEFAULT,
            Usage::Dynamic => bindings::USAGE_DYNAMIC,
            Usage::Staging => bindings::USAGE_STAGING,
            Usage::Unified => bindings::USAGE_UNIFIED,
            Usage::Sparse => bindings::USAGE_SPARSE,
        }) as bindings::USAGE
    }
}

bitflags! {
    pub struct CpuAccessFlags: bindings::_CPU_ACCESS_FLAGS {
        const None  = bindings::CPU_ACCESS_NONE;
        const Read  = bindings::CPU_ACCESS_READ;
        const Write = bindings::CPU_ACCESS_WRITE;
    }
}
const_assert!(bindings::CPU_ACCESS_FLAG_LAST == 2);

bitflags! {
    pub struct SetShaderResourceFlags: bindings::_SET_SHADER_RESOURCE_FLAGS {
        const None          = bindings::SET_SHADER_RESOURCE_FLAG_NONE;
        const AllowOverrite = bindings::SET_SHADER_RESOURCE_FLAG_ALLOW_OVERWRITE;
    }
}

pub enum RenderDeviceType {
    D3D11,
    D3D12,
    GL,
    GLES,
    VULKAN,
    METAL,
    WEBGPU,
}
const_assert!(bindings::RENDER_DEVICE_TYPE_COUNT == 8);

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
const_assert!(bindings::VT_NUM_TYPES == 10);

impl Into<bindings::VALUE_TYPE> for ValueType {
    fn into(self) -> bindings::VALUE_TYPE {
        (match self {
            ValueType::Int8 => bindings::VT_INT8,
            ValueType::Int16 => bindings::VT_INT16,
            ValueType::Int32 => bindings::VT_INT32,
            ValueType::Uint8 => bindings::VT_UINT8,
            ValueType::Uint16 => bindings::VT_UINT16,
            ValueType::Uint32 => bindings::VT_UINT32,
            ValueType::Float16 => bindings::VT_FLOAT16,
            ValueType::Float32 => bindings::VT_FLOAT32,
            ValueType::Float64 => bindings::VT_FLOAT64,
        }) as bindings::VALUE_TYPE
    }
}

pub enum MapType {
    Read,
    Write,
    ReadWrite,
}

impl Into<bindings::MAP_TYPE> for MapType {
    fn into(self) -> bindings::MAP_TYPE {
        (match self {
            MapType::Read => bindings::MAP_READ,
            MapType::Write => bindings::MAP_WRITE,
            MapType::ReadWrite => bindings::MAP_READ_WRITE,
        }) as bindings::MAP_TYPE
    }
}

bitflags! {
    pub struct MapFlags: bindings::_MAP_FLAGS {
        const None        = bindings::MAP_FLAG_NONE;
        const DoNotWait   = bindings::MAP_FLAG_DO_NOT_WAIT;
        const Discard     = bindings::MAP_FLAG_DISCARD;
        const NoOverwrite = bindings::MAP_FLAG_NO_OVERWRITE;
    }
}
