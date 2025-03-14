//use std::ffi::CStr;

use bitflags::bitflags;
use static_assertions::const_assert;

bitflags! {
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
const_assert!(diligent_sys::SHADER_TYPE_LAST == 16384);

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

impl From<&ShaderType> for diligent_sys::SHADER_TYPE {
    fn from(value: &ShaderType) -> Self {
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
        }) as diligent_sys::SHADER_TYPE
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
const_assert!(diligent_sys::FILTER_TYPE_NUM_FILTERS == 13);

impl From<&FilterType> for diligent_sys::FILTER_TYPE {
    fn from(value: &FilterType) -> Self {
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
        }) as diligent_sys::FILTER_TYPE
    }
}

pub enum TextureAddressMode {
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}
const_assert!(diligent_sys::TEXTURE_ADDRESS_NUM_MODES == 6);

impl From<&TextureAddressMode> for diligent_sys::TEXTURE_ADDRESS_MODE {
    fn from(value: &TextureAddressMode) -> Self {
        (match value {
            TextureAddressMode::Wrap => diligent_sys::TEXTURE_ADDRESS_WRAP,
            TextureAddressMode::Mirror => diligent_sys::TEXTURE_ADDRESS_MIRROR,
            TextureAddressMode::Clamp => diligent_sys::TEXTURE_ADDRESS_CLAMP,
            TextureAddressMode::Border => diligent_sys::TEXTURE_ADDRESS_BORDER,
            TextureAddressMode::MirrorOnce => diligent_sys::TEXTURE_ADDRESS_MIRROR_ONCE,
        }) as diligent_sys::TEXTURE_ADDRESS_MODE
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
const_assert!(diligent_sys::PRIMITIVE_TOPOLOGY_NUM_TOPOLOGIES == 42);

impl From<&PrimitiveTopology> for diligent_sys::PRIMITIVE_TOPOLOGY {
    fn from(value: &PrimitiveTopology) -> Self {
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
        }) as diligent_sys::PRIMITIVE_TOPOLOGY
    }
}

bitflags! {
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
const_assert!(diligent_sys::BIND_FLAG_LAST == 2048);

pub enum Usage {
    Immutable,
    Default,
    Dynamic,
    Staging,
    Unified,
    Sparse,
}
const_assert!(diligent_sys::USAGE_NUM_USAGES == 6);

impl From<&Usage> for diligent_sys::USAGE {
    fn from(value: &Usage) -> Self {
        (match value {
            Usage::Immutable => diligent_sys::USAGE_IMMUTABLE,
            Usage::Default => diligent_sys::USAGE_DEFAULT,
            Usage::Dynamic => diligent_sys::USAGE_DYNAMIC,
            Usage::Staging => diligent_sys::USAGE_STAGING,
            Usage::Unified => diligent_sys::USAGE_UNIFIED,
            Usage::Sparse => diligent_sys::USAGE_SPARSE,
        }) as diligent_sys::USAGE
    }
}

bitflags! {
    pub struct CpuAccessFlags: diligent_sys::CPU_ACCESS_FLAGS {
        const None  = diligent_sys::CPU_ACCESS_NONE as diligent_sys::CPU_ACCESS_FLAGS;
        const Read  = diligent_sys::CPU_ACCESS_READ as diligent_sys::CPU_ACCESS_FLAGS;
        const Write = diligent_sys::CPU_ACCESS_WRITE as diligent_sys::CPU_ACCESS_FLAGS;
    }
}
const_assert!(diligent_sys::CPU_ACCESS_FLAG_LAST == 2);

bitflags! {
    pub struct SetShaderResourceFlags: diligent_sys::SET_SHADER_RESOURCE_FLAGS {
        const None          = diligent_sys::SET_SHADER_RESOURCE_FLAG_NONE as diligent_sys::SET_SHADER_RESOURCE_FLAGS;
        const AllowOverrite = diligent_sys::SET_SHADER_RESOURCE_FLAG_ALLOW_OVERWRITE as diligent_sys::SET_SHADER_RESOURCE_FLAGS;
    }
}

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
const_assert!(diligent_sys::RENDER_DEVICE_TYPE_COUNT == 8);

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
const_assert!(diligent_sys::VT_NUM_TYPES == 10);

impl From<&ValueType> for diligent_sys::VALUE_TYPE {
    fn from(value: &ValueType) -> Self {
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
        }) as diligent_sys::VALUE_TYPE
    }
}

bitflags! {
    pub struct MapFlags: diligent_sys::MAP_FLAGS {
        const None        = diligent_sys::MAP_FLAG_NONE as diligent_sys::MAP_FLAGS;
        const DoNotWait   = diligent_sys::MAP_FLAG_DO_NOT_WAIT as diligent_sys::MAP_FLAGS;
        const Discard     = diligent_sys::MAP_FLAG_DISCARD as diligent_sys::MAP_FLAGS;
        const NoOverwrite = diligent_sys::MAP_FLAG_NO_OVERWRITE as diligent_sys::MAP_FLAGS;
    }
}

pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Version { major, minor }
    }
}

#[derive(PartialEq, Eq)]
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
        Some(self.cmp(&other))
    }
}

impl From<&AdapterType> for diligent_sys::ADAPTER_TYPE {
    fn from(value: &AdapterType) -> Self {
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

const_assert!(diligent_sys::ADAPTER_TYPE_COUNT == 4);

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
const_assert!(diligent_sys::ADAPTER_VENDOR_LAST == 10);

pub struct AdapterMemoryInfo {
    pub local_memory: u64,
    pub host_visible_memory: u64,
    pub unified_memory: u64,
    pub max_memory_allocation: u64,
    pub unified_memory_cpu_access: CpuAccessFlags,
    pub memoryless_texture_bind_flags: BindFlags,
}

bitflags! {
    pub struct RaytracingCapFlags : diligent_sys::RAY_TRACING_CAP_FLAGS {
        const None               = diligent_sys::RAY_TRACING_CAP_FLAG_NONE as diligent_sys::RAY_TRACING_CAP_FLAGS;
        const StandaloneShaders  = diligent_sys::RAY_TRACING_CAP_FLAG_STANDALONE_SHADERS as diligent_sys::RAY_TRACING_CAP_FLAGS;
        const InlineRayTracing   = diligent_sys::RAY_TRACING_CAP_FLAG_INLINE_RAY_TRACING as diligent_sys::RAY_TRACING_CAP_FLAGS;
        const IndirectRayTracing = diligent_sys::RAY_TRACING_CAP_FLAG_INDIRECT_RAY_TRACING as diligent_sys::RAY_TRACING_CAP_FLAGS;
    }
}

pub struct RayTracingProperties {
    pub max_recursion_depth: u32,
    pub shader_group_handle_size: u32,
    pub max_shader_record_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_ray_gen_threads: u32,
    pub max_instances_per_tlas: u32,
    pub max_primitives_per_blas: u32,
    pub max_geometries_per_blas: u32,
    pub vertex_buffer_alignment: u32,
    pub index_buffer_alignment: u32,
    pub transform_buffer_alignment: u32,
    pub box_buffer_alignment: u32,
    pub scratch_buffer_alignment: u32,
    pub instance_buffer_alignment: u32,
    pub cap_flags: RaytracingCapFlags,
}

bitflags! {
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
const_assert!(diligent_sys::WAVE_FEATURE_LAST == 128);

pub struct WaveOpProperties {
    pub min_size: u32,
    pub max_size: u32,
    pub supported_stages: ShaderTypes,
    pub features: WaveFeature,
}

pub struct BufferProperties {
    pub constant_buffer_offset_alignment: u32,
    pub structured_buffer_offset_alignment: u32,
}

pub struct TextureProperties {
    pub max_texture1d_dimension: u32,
    pub max_texture1d_array_slices: u32,
    pub max_texture2d_dimension: u32,
    pub max_texture2d_array_slices: u32,
    pub max_texture3d_dimension: u32,
    pub max_texture_cube_dimension: u32,
    pub texture2dms_supported: bool,
    pub texture2dms_array_supported: bool,
    pub texture_view_supported: bool,
    pub cubemap_arrays_supported: bool,
    pub texture_view2d_on3d_supported: bool,
}

pub struct SamplerProperties {
    pub border_sampling_mode_supported: bool,
    pub max_anisotropy: u8,
    pub lod_bias_supported: bool,
}

pub struct MeshShaderProperties {
    pub max_thread_group_count_x: u32,
    pub max_thread_group_count_y: u32,
    pub max_thread_group_count_z: u32,
    pub max_thread_group_total_count: u32,
}

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
const_assert!(diligent_sys::SHADING_RATE_MAX == 10);

impl From<&ShadingRate> for diligent_sys::SHADING_RATE {
    fn from(value: &ShadingRate) -> Self {
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
        }) as diligent_sys::SHADING_RATE
    }
}

bitflags! {
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
const_assert!(diligent_sys::SAMPLE_COUNT_MAX == 64);

pub struct ShadingRateMode {
    pub rate: ShadingRate,
    pub sample_bits: SampleCount,
}

bitflags! {
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
    pub struct ShadingRateCombiner : diligent_sys::SHADING_RATE_COMBINER {
        const Passthrough = diligent_sys::SHADING_RATE_COMBINER_PASSTHROUGH as diligent_sys::SHADING_RATE_COMBINER;
        const Override    = diligent_sys::SHADING_RATE_COMBINER_OVERRIDE as diligent_sys::SHADING_RATE_COMBINER;
        const Min         = diligent_sys::SHADING_RATE_COMBINER_MIN as diligent_sys::SHADING_RATE_COMBINER;
        const Max         = diligent_sys::SHADING_RATE_COMBINER_MAX as diligent_sys::SHADING_RATE_COMBINER;
        const Sum         = diligent_sys::SHADING_RATE_COMBINER_SUM as diligent_sys::SHADING_RATE_COMBINER;
        const Mul         = diligent_sys::SHADING_RATE_COMBINER_MUL as diligent_sys::SHADING_RATE_COMBINER;
    }
}
const_assert!(diligent_sys::SAMPLE_COUNT_MAX == 64);

bitflags! {
    pub struct ShadingRateFormat : diligent_sys::SHADING_RATE_FORMAT {
        const Unknown    = diligent_sys::SHADING_RATE_FORMAT_UNKNOWN as diligent_sys::SHADING_RATE_FORMAT;
        const Palette    = diligent_sys::SHADING_RATE_FORMAT_PALETTE as diligent_sys::SHADING_RATE_FORMAT;
        const Unorm8     = diligent_sys::SHADING_RATE_FORMAT_UNORM8 as diligent_sys::SHADING_RATE_FORMAT;
        const ColRowFp32 = diligent_sys::SHADING_RATE_FORMAT_COL_ROW_FP32 as diligent_sys::SHADING_RATE_FORMAT;
    }
}

bitflags! {
    pub struct ShadingRateTextureAccess : diligent_sys::SHADING_RATE_TEXTURE_ACCESS {
        const Unknown  = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_UNKNOWN as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
        const OnGpu    = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_ON_GPU as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
        const OnSubmit = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_ON_SUBMIT as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
        const OnSetRtv = diligent_sys::SHADING_RATE_TEXTURE_ACCESS_ON_SET_RTV as diligent_sys::SHADING_RATE_TEXTURE_ACCESS;
    }
}

pub struct ShadingRateProperties {
    pub shading_rates: Vec<ShadingRateMode>,
    pub cap_flags: ShadingRateCapFlags,
    pub combiners: ShadingRateCombiner,
    pub format: ShadingRateFormat,
    pub shading_rate_texture_access: ShadingRateTextureAccess,
    pub bind_flags: BindFlags,
    pub min_tile_size: [u32; 2usize],
    pub max_tile_size: [u32; 2usize],
    pub max_subsampled_array_slices: u32,
}

pub struct ComputeShaderProperties {
    pub shared_memory_size: u32,
    pub max_thread_group_invocations: u32,
    pub max_thread_group_size_x: u32,
    pub max_thread_group_size_y: u32,
    pub max_thread_group_size_z: u32,
    pub max_thread_group_count_x: u32,
    pub max_thread_group_count_y: u32,
    pub max_thread_group_count_z: u32,
}

bitflags! {
    pub struct DrawCommandCapFlags : diligent_sys::DRAW_COMMAND_CAP_FLAGS {
        const None                      = diligent_sys::DRAW_COMMAND_CAP_FLAG_NONE as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const BaseVertex                = diligent_sys::DRAW_COMMAND_CAP_FLAG_BASE_VERTEX as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const DrawIndirect              = diligent_sys::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const DrawIndirectFirstInstance = diligent_sys::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT_FIRST_INSTANCE as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const NativeMultiDrawIndirect   = diligent_sys::DRAW_COMMAND_CAP_FLAG_NATIVE_MULTI_DRAW_INDIRECT as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
        const DrawIndirectCounterBuffer = diligent_sys::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT_COUNTER_BUFFER as diligent_sys::DRAW_COMMAND_CAP_FLAGS;
    }
}

pub struct DrawCommandProperties {
    pub cap_flags: DrawCommandCapFlags,
    pub max_index_value: u32,
    pub max_draw_indirect_count: u32,
}

bitflags! {
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

pub struct SparseResourceProperties {
    pub address_space_size: u64,
    pub resource_space_size: u64,
    pub cap_flags: SparseResourceCapFlags,
    pub standard_block_size: u32,
    pub buffer_bind_flags: BindFlags,
}

pub enum DeviceFeatureState {
    Disabled,
    Enabled,
    Optional,
}

impl Into<DeviceFeatureState> for diligent_sys::DEVICE_FEATURE_STATE {
    fn into(self) -> DeviceFeatureState {
        match self as diligent_sys::_DEVICE_FEATURE_STATE {
            diligent_sys::DEVICE_FEATURE_STATE_DISABLED => DeviceFeatureState::Disabled,
            diligent_sys::DEVICE_FEATURE_STATE_ENABLED => DeviceFeatureState::Enabled,
            diligent_sys::DEVICE_FEATURE_STATE_OPTIONAL => DeviceFeatureState::Optional,
            _ => panic!(),
        }
    }
}

impl From<&DeviceFeatureState> for diligent_sys::DEVICE_FEATURE_STATE {
    fn from(value: &DeviceFeatureState) -> Self {
        (match value {
            DeviceFeatureState::Disabled => diligent_sys::DEVICE_FEATURE_STATE_DISABLED,
            DeviceFeatureState::Enabled => diligent_sys::DEVICE_FEATURE_STATE_ENABLED,
            DeviceFeatureState::Optional => diligent_sys::DEVICE_FEATURE_STATE_OPTIONAL,
        }) as diligent_sys::DEVICE_FEATURE_STATE
    }
}

pub struct DeviceFeatures {
    pub separable_programs: DeviceFeatureState,
    pub shader_resource_queries: DeviceFeatureState,
    pub wireframe_fill: DeviceFeatureState,
    pub multithreaded_resource_creation: DeviceFeatureState,
    pub compute_shaders: DeviceFeatureState,
    pub geometry_shaders: DeviceFeatureState,
    pub tessellation: DeviceFeatureState,
    pub mesh_shaders: DeviceFeatureState,
    pub ray_tracing: DeviceFeatureState,
    pub bindless_resources: DeviceFeatureState,
    pub occlusion_queries: DeviceFeatureState,
    pub binary_occlusion_queries: DeviceFeatureState,
    pub timestamp_queries: DeviceFeatureState,
    pub pipeline_statistics_queries: DeviceFeatureState,
    pub duration_queries: DeviceFeatureState,
    pub depth_bias_clamp: DeviceFeatureState,
    pub depth_clamp: DeviceFeatureState,
    pub independent_blend: DeviceFeatureState,
    pub dual_source_blend: DeviceFeatureState,
    pub multi_viewport: DeviceFeatureState,
    pub texture_compression_bc: DeviceFeatureState,
    pub texture_compression_etc2: DeviceFeatureState,
    pub vertex_pipeline_uav_writes_and_atomics: DeviceFeatureState,
    pub pixel_uav_writes_and_atomics: DeviceFeatureState,
    pub texture_uav_extended_formats: DeviceFeatureState,
    pub shader_float16: DeviceFeatureState,
    pub resource_buffer16_bit_access: DeviceFeatureState,
    pub uniform_buffer16_bit_access: DeviceFeatureState,
    pub shader_input_output16: DeviceFeatureState,
    pub shader_int8: DeviceFeatureState,
    pub resource_buffer8_bit_access: DeviceFeatureState,
    pub uniform_buffer8_bit_access: DeviceFeatureState,
    pub shader_resource_static_arrays: DeviceFeatureState,
    pub shader_resource_runtime_arrays: DeviceFeatureState,
    pub wave_op: DeviceFeatureState,
    pub instance_data_step_rate: DeviceFeatureState,
    pub native_fence: DeviceFeatureState,
    pub tile_shaders: DeviceFeatureState,
    pub transfer_queue_timestamp_queries: DeviceFeatureState,
    pub variable_rate_shading: DeviceFeatureState,
    pub sparse_resources: DeviceFeatureState,
    pub subpass_framebuffer_fetch: DeviceFeatureState,
    pub texture_component_swizzle: DeviceFeatureState,
    pub texture_subresource_views: DeviceFeatureState,
    pub native_multi_draw: DeviceFeatureState,
    pub async_shader_compilation: DeviceFeatureState,
    pub formatted_buffers: DeviceFeatureState,
}

impl Default for DeviceFeatures {
    fn default() -> Self {
        DeviceFeatures {
            separable_programs: DeviceFeatureState::Disabled,
            shader_resource_queries: DeviceFeatureState::Disabled,
            wireframe_fill: DeviceFeatureState::Disabled,
            multithreaded_resource_creation: DeviceFeatureState::Disabled,
            compute_shaders: DeviceFeatureState::Disabled,
            geometry_shaders: DeviceFeatureState::Disabled,
            tessellation: DeviceFeatureState::Disabled,
            mesh_shaders: DeviceFeatureState::Disabled,
            ray_tracing: DeviceFeatureState::Disabled,
            bindless_resources: DeviceFeatureState::Disabled,
            occlusion_queries: DeviceFeatureState::Disabled,
            binary_occlusion_queries: DeviceFeatureState::Disabled,
            timestamp_queries: DeviceFeatureState::Disabled,
            pipeline_statistics_queries: DeviceFeatureState::Disabled,
            duration_queries: DeviceFeatureState::Disabled,
            depth_bias_clamp: DeviceFeatureState::Disabled,
            depth_clamp: DeviceFeatureState::Disabled,
            independent_blend: DeviceFeatureState::Disabled,
            dual_source_blend: DeviceFeatureState::Disabled,
            multi_viewport: DeviceFeatureState::Disabled,
            texture_compression_bc: DeviceFeatureState::Disabled,
            texture_compression_etc2: DeviceFeatureState::Disabled,
            vertex_pipeline_uav_writes_and_atomics: DeviceFeatureState::Disabled,
            pixel_uav_writes_and_atomics: DeviceFeatureState::Disabled,
            texture_uav_extended_formats: DeviceFeatureState::Disabled,
            shader_float16: DeviceFeatureState::Disabled,
            resource_buffer16_bit_access: DeviceFeatureState::Disabled,
            uniform_buffer16_bit_access: DeviceFeatureState::Disabled,
            shader_input_output16: DeviceFeatureState::Disabled,
            shader_int8: DeviceFeatureState::Disabled,
            resource_buffer8_bit_access: DeviceFeatureState::Disabled,
            uniform_buffer8_bit_access: DeviceFeatureState::Disabled,
            shader_resource_static_arrays: DeviceFeatureState::Disabled,
            shader_resource_runtime_arrays: DeviceFeatureState::Disabled,
            wave_op: DeviceFeatureState::Disabled,
            instance_data_step_rate: DeviceFeatureState::Disabled,
            native_fence: DeviceFeatureState::Disabled,
            tile_shaders: DeviceFeatureState::Disabled,
            transfer_queue_timestamp_queries: DeviceFeatureState::Disabled,
            variable_rate_shading: DeviceFeatureState::Disabled,
            sparse_resources: DeviceFeatureState::Disabled,
            subpass_framebuffer_fetch: DeviceFeatureState::Disabled,
            texture_component_swizzle: DeviceFeatureState::Disabled,
            texture_subresource_views: DeviceFeatureState::Disabled,
            native_multi_draw: DeviceFeatureState::Disabled,
            async_shader_compilation: DeviceFeatureState::Disabled,
            formatted_buffers: DeviceFeatureState::Disabled,
        }
    }
}

impl From<&DeviceFeatures> for diligent_sys::DeviceFeatures {
    fn from(value: &DeviceFeatures) -> Self {
        diligent_sys::DeviceFeatures {
            SeparablePrograms: diligent_sys::DEVICE_FEATURE_STATE::from(&value.separable_programs),
            ShaderResourceQueries: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.shader_resource_queries,
            ),
            WireframeFill: diligent_sys::DEVICE_FEATURE_STATE::from(&value.wireframe_fill),
            MultithreadedResourceCreation: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.multithreaded_resource_creation,
            ),
            ComputeShaders: diligent_sys::DEVICE_FEATURE_STATE::from(&value.compute_shaders),
            GeometryShaders: diligent_sys::DEVICE_FEATURE_STATE::from(&value.geometry_shaders),
            Tessellation: diligent_sys::DEVICE_FEATURE_STATE::from(&value.tessellation),
            MeshShaders: diligent_sys::DEVICE_FEATURE_STATE::from(&value.mesh_shaders),
            RayTracing: diligent_sys::DEVICE_FEATURE_STATE::from(&value.ray_tracing),
            BindlessResources: diligent_sys::DEVICE_FEATURE_STATE::from(&value.bindless_resources),
            OcclusionQueries: diligent_sys::DEVICE_FEATURE_STATE::from(&value.occlusion_queries),
            BinaryOcclusionQueries: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.binary_occlusion_queries,
            ),
            TimestampQueries: diligent_sys::DEVICE_FEATURE_STATE::from(&value.timestamp_queries),
            PipelineStatisticsQueries: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.pipeline_statistics_queries,
            ),
            DurationQueries: diligent_sys::DEVICE_FEATURE_STATE::from(&value.duration_queries),
            DepthBiasClamp: diligent_sys::DEVICE_FEATURE_STATE::from(&value.depth_bias_clamp),
            DepthClamp: diligent_sys::DEVICE_FEATURE_STATE::from(&value.depth_clamp),
            IndependentBlend: diligent_sys::DEVICE_FEATURE_STATE::from(&value.independent_blend),
            DualSourceBlend: diligent_sys::DEVICE_FEATURE_STATE::from(&value.dual_source_blend),
            MultiViewport: diligent_sys::DEVICE_FEATURE_STATE::from(&value.multi_viewport),
            TextureCompressionBC: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.texture_compression_bc,
            ),
            TextureCompressionETC2: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.texture_compression_etc2,
            ),
            VertexPipelineUAVWritesAndAtomics: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.vertex_pipeline_uav_writes_and_atomics,
            ),
            PixelUAVWritesAndAtomics: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.pixel_uav_writes_and_atomics,
            ),
            TextureUAVExtendedFormats: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.texture_uav_extended_formats,
            ),
            ShaderFloat16: diligent_sys::DEVICE_FEATURE_STATE::from(&value.shader_float16),
            ResourceBuffer16BitAccess: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.resource_buffer16_bit_access,
            ),
            UniformBuffer16BitAccess: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.uniform_buffer16_bit_access,
            ),
            ShaderInputOutput16: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.shader_input_output16,
            ),
            ShaderInt8: diligent_sys::DEVICE_FEATURE_STATE::from(&value.shader_int8),
            ResourceBuffer8BitAccess: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.resource_buffer8_bit_access,
            ),
            UniformBuffer8BitAccess: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.uniform_buffer8_bit_access,
            ),
            ShaderResourceStaticArrays: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.shader_resource_static_arrays,
            ),
            ShaderResourceRuntimeArrays: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.shader_resource_runtime_arrays,
            ),
            WaveOp: diligent_sys::DEVICE_FEATURE_STATE::from(&value.wave_op),
            InstanceDataStepRate: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.instance_data_step_rate,
            ),
            NativeFence: diligent_sys::DEVICE_FEATURE_STATE::from(&value.native_fence),
            TileShaders: diligent_sys::DEVICE_FEATURE_STATE::from(&value.tile_shaders),
            TransferQueueTimestampQueries: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.transfer_queue_timestamp_queries,
            ),
            VariableRateShading: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.variable_rate_shading,
            ),
            SparseResources: diligent_sys::DEVICE_FEATURE_STATE::from(&value.sparse_resources),
            SubpassFramebufferFetch: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.subpass_framebuffer_fetch,
            ),
            TextureComponentSwizzle: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.texture_component_swizzle,
            ),
            TextureSubresourceViews: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.texture_subresource_views,
            ),
            NativeMultiDraw: diligent_sys::DEVICE_FEATURE_STATE::from(&value.native_multi_draw),
            AsyncShaderCompilation: diligent_sys::DEVICE_FEATURE_STATE::from(
                &value.async_shader_compilation,
            ),
            FormattedBuffers: diligent_sys::DEVICE_FEATURE_STATE::from(&value.formatted_buffers),
        }
    }
}

bitflags! {
    pub struct CommandQueueType : diligent_sys::COMMAND_QUEUE_TYPE {
        const Unknown       = diligent_sys::COMMAND_QUEUE_TYPE_UNKNOWN as diligent_sys::COMMAND_QUEUE_TYPE;
        const Transfer      = diligent_sys::COMMAND_QUEUE_TYPE_TRANSFER as diligent_sys::COMMAND_QUEUE_TYPE;
        const Compute       = diligent_sys::COMMAND_QUEUE_TYPE_COMPUTE as diligent_sys::COMMAND_QUEUE_TYPE;
        const Graphics      = diligent_sys::COMMAND_QUEUE_TYPE_GRAPHICS as diligent_sys::COMMAND_QUEUE_TYPE;
        const PrimaryMask   = diligent_sys::COMMAND_QUEUE_TYPE_PRIMARY_MASK as diligent_sys::COMMAND_QUEUE_TYPE;
        const SparseBinding = diligent_sys::COMMAND_QUEUE_TYPE_SPARSE_BINDING as diligent_sys::COMMAND_QUEUE_TYPE;
    }
}
const_assert!(diligent_sys::COMMAND_QUEUE_TYPE_MAX_BIT == 7);

pub struct CommandQueueInfo {
    pub queue_type: CommandQueueType,
    pub max_device_contexts: u32,
    pub texture_copy_granularity: [u32; 3usize],
}

impl Into<CommandQueueInfo> for diligent_sys::CommandQueueInfo {
    fn into(self) -> CommandQueueInfo {
        CommandQueueInfo {
            queue_type: CommandQueueType::from_bits_retain(self.QueueType.into()),
            max_device_contexts: self.MaxDeviceContexts,
            texture_copy_granularity: self.TextureCopyGranularity,
        }
    }
}

pub struct GraphicsAdapterInfo {
    pub description: String,
    pub adapter_type: AdapterType,
    pub vendor: AdapterVendor,
    pub vendor_id: u32,
    pub device_id: u32,
    pub num_outputs: u32,
    pub memory: AdapterMemoryInfo,
    pub ray_tracing: RayTracingProperties,
    pub wave_op: WaveOpProperties,
    pub buffer: BufferProperties,
    pub texture: TextureProperties,
    pub sampler: SamplerProperties,
    pub mesh_shader: MeshShaderProperties,
    pub shading_rate: ShadingRateProperties,
    pub compute_shader: ComputeShaderProperties,
    pub draw_command: DrawCommandProperties,
    pub sparse_resources: SparseResourceProperties,
    pub features: DeviceFeatures,
    pub queues: Vec<CommandQueueInfo>,
}

impl Into<GraphicsAdapterInfo> for diligent_sys::GraphicsAdapterInfo {
    fn into(self) -> GraphicsAdapterInfo {
        let desc_vec = Vec::from_iter(
            self.Description
                .split_inclusive(|&c| c == 0)
                .next()
                .unwrap()
                .iter()
                .map(|&c| c as u8),
        );

        let desc_cstring = std::ffi::CString::from_vec_with_nul(desc_vec).unwrap();

        GraphicsAdapterInfo {
            description: desc_cstring.into_string().unwrap(), //desc.to_str().unwrap().to_owned(),
            adapter_type: match self.Type as diligent_sys::_ADAPTER_TYPE {
                diligent_sys::ADAPTER_TYPE_UNKNOWN => AdapterType::Unknown,
                diligent_sys::ADAPTER_TYPE_SOFTWARE => AdapterType::Software,
                diligent_sys::ADAPTER_TYPE_INTEGRATED => AdapterType::Integrated,
                diligent_sys::ADAPTER_TYPE_DISCRETE => AdapterType::Discrete,
                _ => panic!(),
            },
            vendor: match self.Vendor as diligent_sys::_ADAPTER_VENDOR {
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
            },
            vendor_id: self.VendorId,
            device_id: self.DeviceId,
            num_outputs: self.NumOutputs,
            memory: AdapterMemoryInfo {
                local_memory: self.Memory.LocalMemory,
                host_visible_memory: self.Memory.HostVisibleMemory,
                unified_memory: self.Memory.UnifiedMemory,
                max_memory_allocation: self.Memory.MaxMemoryAllocation,
                unified_memory_cpu_access: CpuAccessFlags::from_bits_retain(
                    self.Memory.UnifiedMemoryCPUAccess.into(),
                ),
                memoryless_texture_bind_flags: BindFlags::from_bits_retain(
                    self.Memory.MemorylessTextureBindFlags.into(),
                ),
            },
            ray_tracing: RayTracingProperties {
                max_recursion_depth: self.RayTracing.MaxRecursionDepth,
                shader_group_handle_size: self.RayTracing.ShaderGroupHandleSize,
                max_shader_record_stride: self.RayTracing.MaxShaderRecordStride,
                shader_group_base_alignment: self.RayTracing.ShaderGroupBaseAlignment,
                max_ray_gen_threads: self.RayTracing.MaxRayGenThreads,
                max_instances_per_tlas: self.RayTracing.MaxInstancesPerTLAS,
                max_primitives_per_blas: self.RayTracing.MaxPrimitivesPerBLAS,
                max_geometries_per_blas: self.RayTracing.MaxGeometriesPerBLAS,
                vertex_buffer_alignment: self.RayTracing.VertexBufferAlignment,
                index_buffer_alignment: self.RayTracing.IndexBufferAlignment,
                transform_buffer_alignment: self.RayTracing.TransformBufferAlignment,
                box_buffer_alignment: self.RayTracing.BoxBufferAlignment,
                scratch_buffer_alignment: self.RayTracing.ScratchBufferAlignment,
                instance_buffer_alignment: self.RayTracing.InstanceBufferAlignment,
                cap_flags: RaytracingCapFlags::from_bits_retain(self.RayTracing.CapFlags.into()),
            },
            wave_op: WaveOpProperties {
                min_size: self.WaveOp.MinSize,
                max_size: self.WaveOp.MaxSize,
                supported_stages: ShaderTypes::from_bits_retain(self.WaveOp.SupportedStages),
                features: WaveFeature::from_bits_retain(self.WaveOp.Features),
            },
            buffer: BufferProperties {
                constant_buffer_offset_alignment: self.Buffer.ConstantBufferOffsetAlignment,
                structured_buffer_offset_alignment: self.Buffer.StructuredBufferOffsetAlignment,
            },
            texture: TextureProperties {
                max_texture1d_dimension: self.Texture.MaxTexture1DDimension,
                max_texture1d_array_slices: self.Texture.MaxTexture1DArraySlices,
                max_texture2d_dimension: self.Texture.MaxTexture2DDimension,
                max_texture2d_array_slices: self.Texture.MaxTexture2DArraySlices,
                max_texture3d_dimension: self.Texture.MaxTexture3DDimension,
                max_texture_cube_dimension: self.Texture.MaxTextureCubeDimension,
                texture2dms_supported: self.Texture.Texture2DMSSupported,
                texture2dms_array_supported: self.Texture.Texture2DMSArraySupported,
                texture_view_supported: self.Texture.TextureViewSupported,
                cubemap_arrays_supported: self.Texture.CubemapArraysSupported,
                texture_view2d_on3d_supported: self.Texture.TextureView2DOn3DSupported,
            },
            sampler: SamplerProperties {
                border_sampling_mode_supported: self.Sampler.BorderSamplingModeSupported,
                max_anisotropy: self.Sampler.MaxAnisotropy,
                lod_bias_supported: self.Sampler.LODBiasSupported,
            },
            mesh_shader: MeshShaderProperties {
                max_thread_group_count_x: self.MeshShader.MaxThreadGroupCountX,
                max_thread_group_count_y: self.MeshShader.MaxThreadGroupCountY,
                max_thread_group_count_z: self.MeshShader.MaxThreadGroupCountZ,
                max_thread_group_total_count: self.MeshShader.MaxThreadGroupTotalCount,
            },
            shading_rate: ShadingRateProperties {
                shading_rates: Vec::from_iter(
                    self.ShadingRate
                        .ShadingRates
                        .into_iter()
                        .map(|sr| ShadingRateMode {
                            rate: match sr.Rate as diligent_sys::_SHADING_RATE {
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
                            },
                            sample_bits: SampleCount::from_bits_retain(sr.SampleBits.into()),
                        })
                        .take(self.ShadingRate.NumShadingRates.into()),
                ),
                cap_flags: ShadingRateCapFlags::from_bits_retain(self.ShadingRate.CapFlags.into()),
                combiners: ShadingRateCombiner::from_bits_retain(self.ShadingRate.Combiners.into()),
                format: ShadingRateFormat::from_bits_retain(self.ShadingRate.Format.into()),
                shading_rate_texture_access: ShadingRateTextureAccess::from_bits_retain(
                    self.ShadingRate.ShadingRateTextureAccess.into(),
                ),
                bind_flags: BindFlags::from_bits_retain(self.ShadingRate.BindFlags),
                min_tile_size: self.ShadingRate.MinTileSize,
                max_tile_size: self.ShadingRate.MaxTileSize,
                max_subsampled_array_slices: self.ShadingRate.MaxSabsampledArraySlices,
            },
            compute_shader: ComputeShaderProperties {
                shared_memory_size: self.ComputeShader.SharedMemorySize,
                max_thread_group_invocations: self.ComputeShader.MaxThreadGroupInvocations,
                max_thread_group_size_x: self.ComputeShader.MaxThreadGroupSizeX,
                max_thread_group_size_y: self.ComputeShader.MaxThreadGroupSizeY,
                max_thread_group_size_z: self.ComputeShader.MaxThreadGroupSizeZ,
                max_thread_group_count_x: self.ComputeShader.MaxThreadGroupCountX,
                max_thread_group_count_y: self.ComputeShader.MaxThreadGroupCountY,
                max_thread_group_count_z: self.ComputeShader.MaxThreadGroupCountZ,
            },
            draw_command: DrawCommandProperties {
                cap_flags: DrawCommandCapFlags::from_bits_retain(self.DrawCommand.CapFlags.into()),
                max_index_value: self.DrawCommand.MaxIndexValue,
                max_draw_indirect_count: self.DrawCommand.MaxDrawIndirectCount,
            },
            sparse_resources: SparseResourceProperties {
                address_space_size: self.SparseResources.AddressSpaceSize,
                resource_space_size: self.SparseResources.ResourceSpaceSize,
                cap_flags: SparseResourceCapFlags::from_bits_retain(self.SparseResources.CapFlags),
                standard_block_size: self.SparseResources.StandardBlockSize,
                buffer_bind_flags: BindFlags::from_bits_retain(
                    self.SparseResources.BufferBindFlags,
                ),
            },
            features: DeviceFeatures {
                separable_programs: self.Features.SeparablePrograms.into(),
                shader_resource_queries: self.Features.ShaderResourceQueries.into(),
                wireframe_fill: self.Features.WireframeFill.into(),
                multithreaded_resource_creation: self.Features.MultithreadedResourceCreation.into(),
                compute_shaders: self.Features.ComputeShaders.into(),
                geometry_shaders: self.Features.GeometryShaders.into(),
                tessellation: self.Features.Tessellation.into(),
                mesh_shaders: self.Features.MeshShaders.into(),
                ray_tracing: self.Features.RayTracing.into(),
                bindless_resources: self.Features.BindlessResources.into(),
                occlusion_queries: self.Features.OcclusionQueries.into(),
                binary_occlusion_queries: self.Features.BinaryOcclusionQueries.into(),
                timestamp_queries: self.Features.TimestampQueries.into(),
                pipeline_statistics_queries: self.Features.PipelineStatisticsQueries.into(),
                duration_queries: self.Features.DurationQueries.into(),
                depth_bias_clamp: self.Features.DepthBiasClamp.into(),
                depth_clamp: self.Features.DepthClamp.into(),
                independent_blend: self.Features.IndependentBlend.into(),
                dual_source_blend: self.Features.DualSourceBlend.into(),
                multi_viewport: self.Features.MultiViewport.into(),
                texture_compression_bc: self.Features.TextureCompressionBC.into(),
                texture_compression_etc2: self.Features.TextureCompressionETC2.into(),
                vertex_pipeline_uav_writes_and_atomics: self
                    .Features
                    .VertexPipelineUAVWritesAndAtomics
                    .into(),
                pixel_uav_writes_and_atomics: self.Features.PixelUAVWritesAndAtomics.into(),
                texture_uav_extended_formats: self.Features.TextureUAVExtendedFormats.into(),
                shader_float16: self.Features.ShaderFloat16.into(),
                resource_buffer16_bit_access: self.Features.ResourceBuffer16BitAccess.into(),
                uniform_buffer16_bit_access: self.Features.UniformBuffer16BitAccess.into(),
                shader_input_output16: self.Features.ShaderInputOutput16.into(),
                shader_int8: self.Features.ShaderInt8.into(),
                resource_buffer8_bit_access: self.Features.ResourceBuffer8BitAccess.into(),
                uniform_buffer8_bit_access: self.Features.UniformBuffer8BitAccess.into(),
                shader_resource_static_arrays: self.Features.ShaderResourceStaticArrays.into(),
                shader_resource_runtime_arrays: self.Features.ShaderResourceRuntimeArrays.into(),
                wave_op: self.Features.WaveOp.into(),
                instance_data_step_rate: self.Features.InstanceDataStepRate.into(),
                native_fence: self.Features.NativeFence.into(),
                tile_shaders: self.Features.TileShaders.into(),
                transfer_queue_timestamp_queries: self
                    .Features
                    .TransferQueueTimestampQueries
                    .into(),
                variable_rate_shading: self.Features.VariableRateShading.into(),
                sparse_resources: self.Features.SparseResources.into(),
                subpass_framebuffer_fetch: self.Features.SubpassFramebufferFetch.into(),
                texture_component_swizzle: self.Features.TextureComponentSwizzle.into(),
                texture_subresource_views: self.Features.TextureSubresourceViews.into(),
                native_multi_draw: self.Features.NativeMultiDraw.into(),
                async_shader_compilation: self.Features.AsyncShaderCompilation.into(),
                formatted_buffers: self.Features.FormattedBuffers.into(),
            },
            queues: Vec::from_iter(self.Queues.into_iter().map(|queue| queue.into())),
        }
    }
}

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

impl From<&SurfaceTransform> for diligent_sys::SURFACE_TRANSFORM {
    fn from(value: &SurfaceTransform) -> Self {
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
        }) as diligent_sys::SURFACE_TRANSFORM
    }
}

impl From<&diligent_sys::SURFACE_TRANSFORM> for SurfaceTransform {
    fn from(value: &diligent_sys::SURFACE_TRANSFORM) -> Self {
        match *value as diligent_sys::_SURFACE_TRANSFORM {
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

const_assert!(diligent_sys::RESOURCE_STATE_MAX_BIT == 2097152);

pub enum QueuePriority {
    Low,
    Medium,
    High,
    RealTime,
}
const_assert!(diligent_sys::QUEUE_PRIORITY_LAST == 4);

//pub struct ImmediateContextCreateInfo<'a> {
//    name: &'a CStr,
//    queue_id: u8,
//    priority: QueuePriority,
//}
