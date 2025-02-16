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

impl From<&ShaderType> for bindings::SHADER_TYPE {
    fn from(value: &ShaderType) -> Self {
        (match value {
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

impl From<&FilterType> for bindings::FILTER_TYPE {
    fn from(value: &FilterType) -> Self {
        (match value {
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

impl From<&TextureAddressMode> for bindings::TEXTURE_ADDRESS_MODE {
    fn from(value: &TextureAddressMode) -> Self {
        (match value {
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

impl From<&PrimitiveTopology> for bindings::PRIMITIVE_TOPOLOGY {
    fn from(value: &PrimitiveTopology) -> Self {
        (match value {
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

impl From<&Usage> for bindings::USAGE {
    fn from(value: &Usage) -> Self {
        (match value {
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

impl From<&ValueType> for bindings::VALUE_TYPE {
    fn from(value: &ValueType) -> Self {
        (match value {
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

impl From<&MapType> for bindings::MAP_TYPE {
    fn from(value: &MapType) -> Self {
        (match value {
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

pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Version { major, minor }
    }
}

pub enum AdapterType {
    Unkndown,
    Software,
    Integrated,
    Discrete,
}
const_assert!(bindings::ADAPTER_TYPE_COUNT == 4);

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
const_assert!(bindings::ADAPTER_VENDOR_LAST == 10);

pub struct AdapterMemoryInfo {
    pub local_memory: u64,
    pub host_visible_memory: u64,
    pub unified_memory: u64,
    pub max_memory_allocation: u64,
    pub unified_memory_cpu_access: CpuAccessFlags,
    pub memoryless_texture_bind_flags: BindFlags,
}

bitflags! {
    pub struct RaytracingCapFlags : bindings::_RAY_TRACING_CAP_FLAGS {
        const None               = bindings::RAY_TRACING_CAP_FLAG_NONE;
        const StandaloneShaders  = bindings::RAY_TRACING_CAP_FLAG_STANDALONE_SHADERS;
        const InlineRayTracing   = bindings::RAY_TRACING_CAP_FLAG_INLINE_RAY_TRACING;
        const IndirectRayTracing = bindings::RAY_TRACING_CAP_FLAG_INDIRECT_RAY_TRACING;
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
    pub struct WaveFeature : bindings::_WAVE_FEATURE {
        const Unknown         = bindings::WAVE_FEATURE_UNKNOWN;
        const Basic           = bindings::WAVE_FEATURE_BASIC;
        const Vote            = bindings::WAVE_FEATURE_VOTE;
        const Arithmetic      = bindings::WAVE_FEATURE_ARITHMETIC;
        const Ballout         = bindings::WAVE_FEATURE_BALLOUT;
        const Shuffle         = bindings::WAVE_FEATURE_SHUFFLE;
        const ShuffleRelative = bindings::WAVE_FEATURE_SHUFFLE_RELATIVE;
        const Clustered       = bindings::WAVE_FEATURE_CLUSTERED;
        const Quad            = bindings::WAVE_FEATURE_QUAD;
    }
}
const_assert!(bindings::WAVE_FEATURE_LAST == 128);

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
const_assert!(bindings::SHADING_RATE_MAX == 10);

bitflags! {
    pub struct SampleCount : bindings::_SAMPLE_COUNT {
        const None = bindings::SAMPLE_COUNT_NONE;
        const _1   = bindings::SAMPLE_COUNT_1;
        const _2   = bindings::SAMPLE_COUNT_2;
        const _4   = bindings::SAMPLE_COUNT_4;
        const _8   = bindings::SAMPLE_COUNT_8;
        const _16  = bindings::SAMPLE_COUNT_16;
        const _32  = bindings::SAMPLE_COUNT_32;
        const _64  = bindings::SAMPLE_COUNT_64;
    }
}
const_assert!(bindings::SAMPLE_COUNT_MAX == 64);

pub struct ShadingRateMode {
    pub rate: ShadingRate,
    pub sample_bits: SampleCount,
}

bitflags! {
    pub struct ShadingRateCapFlags : bindings::_SHADING_RATE_CAP_FLAGS {
        const None                              = bindings::SHADING_RATE_CAP_FLAG_NONE;
        const PerDraw                           = bindings::SHADING_RATE_CAP_FLAG_PER_DRAW;
        const PerPrimitive                      = bindings::SHADING_RATE_CAP_FLAG_PER_PRIMITIVE;
        const TextureBased                      = bindings::SHADING_RATE_CAP_FLAG_TEXTURE_BASED;
        const SampleMask                        = bindings::SHADING_RATE_CAP_FLAG_SAMPLE_MASK;
        const ShaderSampleMask                  = bindings::SHADING_RATE_CAP_FLAG_SHADER_SAMPLE_MASK;
        const ShaderDepthStencilWrite           = bindings::SHADING_RATE_CAP_FLAG_SHADER_DEPTH_STENCIL_WRITE;
        const PerPrimitiveWithMultipleViewports = bindings::SHADING_RATE_CAP_FLAG_PER_PRIMITIVE_WITH_MULTIPLE_VIEWPORTS;
        const SameTextureForWholeRenderpass     = bindings::SHADING_RATE_CAP_FLAG_SAME_TEXTURE_FOR_WHOLE_RENDERPASS;
        const TextureArray                      = bindings::SHADING_RATE_CAP_FLAG_TEXTURE_ARRAY;
        const ShadingRateShaderInput            = bindings::SHADING_RATE_CAP_FLAG_SHADING_RATE_SHADER_INPUT;
        const AdditionalInvocations             = bindings::SHADING_RATE_CAP_FLAG_ADDITIONAL_INVOCATIONS;
        const Non_subsampledRenderTarget        = bindings::SHADING_RATE_CAP_FLAG_NON_SUBSAMPLED_RENDER_TARGET;
        const Subsampled_renderTarget           = bindings::SHADING_RATE_CAP_FLAG_SUBSAMPLED_RENDER_TARGET;
    }
}

bitflags! {
    pub struct ShadingRayeCombiner : bindings::_SHADING_RATE_COMBINER {
        const Passthrough = bindings::SHADING_RATE_COMBINER_PASSTHROUGH;
        const Override    = bindings::SHADING_RATE_COMBINER_OVERRIDE;
        const Min         = bindings::SHADING_RATE_COMBINER_MIN;
        const Max         = bindings::SHADING_RATE_COMBINER_MAX;
        const Sum         = bindings::SHADING_RATE_COMBINER_SUM;
        const Mul         = bindings::SHADING_RATE_COMBINER_MUL;
    }
}
const_assert!(bindings::SAMPLE_COUNT_MAX == 64);

bitflags! {
    pub struct ShadingRateFormat : bindings::_SHADING_RATE_FORMAT {
        const Unknown    = bindings::SHADING_RATE_FORMAT_UNKNOWN;
        const Palette    = bindings::SHADING_RATE_FORMAT_PALETTE;
        const Unorm8     = bindings::SHADING_RATE_FORMAT_UNORM8;
        const ColRowFp32 = bindings::SHADING_RATE_FORMAT_COL_ROW_FP32;
    }
}

bitflags! {
    pub struct ShadingRateTextureAccess : bindings::_SHADING_RATE_TEXTURE_ACCESS {
        const Unknown  = bindings::SHADING_RATE_TEXTURE_ACCESS_UNKNOWN;
        const OnGpu    = bindings::SHADING_RATE_TEXTURE_ACCESS_ON_GPU;
        const OnSubmit = bindings::SHADING_RATE_TEXTURE_ACCESS_ON_SUBMIT;
        const OnSetRtv = bindings::SHADING_RATE_TEXTURE_ACCESS_ON_SET_RTV;
    }
}

pub struct ShadingRateProperties {
    pub shading_rates: Vec<ShadingRateMode>,
    pub cap_flags: ShadingRateCapFlags,
    pub combiners: ShadingRayeCombiner,
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
    pub struct DrawCommandCapFlags : bindings::_DRAW_COMMAND_CAP_FLAGS {
        const None                      = bindings::DRAW_COMMAND_CAP_FLAG_NONE;
        const BaseVertex                = bindings::DRAW_COMMAND_CAP_FLAG_BASE_VERTEX;
        const DrawIndirect              = bindings::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT;
        const DrawIndirectFirstInstance = bindings::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT_FIRST_INSTANCE;
        const NativeMultiDrawIndirect   = bindings::DRAW_COMMAND_CAP_FLAG_NATIVE_MULTI_DRAW_INDIRECT;
        const DrawIndirectCounterBuffer = bindings::DRAW_COMMAND_CAP_FLAG_DRAW_INDIRECT_COUNTER_BUFFER;
    }
}

pub struct DrawCommandProperties {
    pub cap_flags: DrawCommandCapFlags,
    pub max_index_value: u32,
    pub max_draw_indirect_count: u32,
}

bitflags! {
    pub struct SparseResourceCapFlags : bindings::_SPARSE_RESOURCE_CAP_FLAGS {
        const None                     = bindings::SPARSE_RESOURCE_CAP_FLAG_NONE;
        const ShaderResourceResidency  = bindings::SPARSE_RESOURCE_CAP_FLAG_SHADER_RESOURCE_RESIDENCY;
        const Buffer                   = bindings::SPARSE_RESOURCE_CAP_FLAG_BUFFER;
        const Texture2D                = bindings::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_2D;
        const Texture3D                = bindings::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_3D;
        const Texture2Samples          = bindings::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_2_SAMPLES;
        const Texture4Samples          = bindings::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_4_SAMPLES;
        const Texture8Samples          = bindings::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_8_SAMPLES;
        const Texture16Samples         = bindings::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_16_SAMPLES;
        const Aliased                  = bindings::SPARSE_RESOURCE_CAP_FLAG_ALIASED;
        const Standard2DTileShape      = bindings::SPARSE_RESOURCE_CAP_FLAG_STANDARD_2D_TILE_SHAPE;
        const Standard2DMSTileShape    = bindings::SPARSE_RESOURCE_CAP_FLAG_STANDARD_2DMS_TILE_SHAPE;
        const Standard3DTileShape      = bindings::SPARSE_RESOURCE_CAP_FLAG_STANDARD_3D_TILE_SHAPE;
        const AlignedMipSize           = bindings::SPARSE_RESOURCE_CAP_FLAG_ALIGNED_MIP_SIZE;
        const NonResidentStrict        = bindings::SPARSE_RESOURCE_CAP_FLAG_NON_RESIDENT_STRICT;
        const Texture2dArrayMipTail    = bindings::SPARSE_RESOURCE_CAP_FLAG_TEXTURE_2D_ARRAY_MIP_TAIL;
        const BufferStandardBlock      = bindings::SPARSE_RESOURCE_CAP_FLAG_BUFFER_STANDARD_BLOCK;
        const NonResidentSafe          = bindings::SPARSE_RESOURCE_CAP_FLAG_NON_RESIDENT_SAFE;
        const MixedResourceTypeSupport = bindings::SPARSE_RESOURCE_CAP_FLAG_MIXED_RESOURCE_TYPE_SUPPORT;
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

impl Into<DeviceFeatureState> for bindings::DEVICE_FEATURE_STATE {
    fn into(self) -> DeviceFeatureState {
        match self as bindings::_DEVICE_FEATURE_STATE {
            bindings::DEVICE_FEATURE_STATE_DISABLED => DeviceFeatureState::Disabled,
            bindings::DEVICE_FEATURE_STATE_ENABLED => DeviceFeatureState::Enabled,
            bindings::DEVICE_FEATURE_STATE_OPTIONAL => DeviceFeatureState::Optional,
            _ => panic!(),
        }
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

bitflags! {
    pub struct CommandQueueType : bindings::_COMMAND_QUEUE_TYPE {
        const Unknown       = bindings::COMMAND_QUEUE_TYPE_UNKNOWN;
        const Transfer      = bindings::COMMAND_QUEUE_TYPE_TRANSFER;
        const Compute       = bindings::COMMAND_QUEUE_TYPE_COMPUTE;
        const Graphics      = bindings::COMMAND_QUEUE_TYPE_GRAPHICS;
        const PrimaryMask   = bindings::COMMAND_QUEUE_TYPE_PRIMARY_MASK;
        const SparseBinding = bindings::COMMAND_QUEUE_TYPE_SPARSE_BINDING;
    }
}
const_assert!(bindings::COMMAND_QUEUE_TYPE_MAX_BIT == 7);

pub struct CommandQueueInfo {
    pub queue_type: CommandQueueType,
    pub max_device_contexts: u32,
    pub texture_copy_granularity: [u32; 3usize],
}

impl Into<CommandQueueInfo> for bindings::CommandQueueInfo {
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

impl Into<GraphicsAdapterInfo> for bindings::GraphicsAdapterInfo {
    fn into(self) -> GraphicsAdapterInfo {
        GraphicsAdapterInfo {
            description: std::ffi::CString::from_vec_with_nul(Vec::from_iter(
                self.Description.into_iter().map(|c| c as u8),
            ))
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned(),
            adapter_type: match self.Type as bindings::_ADAPTER_TYPE {
                bindings::ADAPTER_TYPE_UNKNOWN => AdapterType::Unkndown,
                bindings::ADAPTER_TYPE_SOFTWARE => AdapterType::Software,
                bindings::ADAPTER_TYPE_INTEGRATED => AdapterType::Integrated,
                bindings::ADAPTER_TYPE_DISCRETE => AdapterType::Discrete,
                _ => panic!(),
            },
            vendor: match self.Vendor as bindings::_ADAPTER_VENDOR {
                bindings::ADAPTER_VENDOR_UNKNOWN => AdapterVendor::Unknown,
                bindings::ADAPTER_VENDOR_NVIDIA => AdapterVendor::Nvidia,
                bindings::ADAPTER_VENDOR_AMD => AdapterVendor::AMD,
                bindings::ADAPTER_VENDOR_INTEL => AdapterVendor::Intel,
                bindings::ADAPTER_VENDOR_ARM => AdapterVendor::ARM,
                bindings::ADAPTER_VENDOR_QUALCOMM => AdapterVendor::Qualcomm,
                bindings::ADAPTER_VENDOR_IMGTECH => AdapterVendor::Imgtech,
                bindings::ADAPTER_VENDOR_MSFT => AdapterVendor::Msft,
                bindings::ADAPTER_VENDOR_APPLE => AdapterVendor::Apple,
                bindings::ADAPTER_VENDOR_MESA => AdapterVendor::Mesa,
                bindings::ADAPTER_VENDOR_BROADCOM => AdapterVendor::Broadcom,
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
                            rate: match sr.Rate as bindings::_SHADING_RATE {
                                bindings::SHADING_RATE_1X1 => ShadingRate::_1X1,
                                bindings::SHADING_RATE_1X2 => ShadingRate::_1X2,
                                bindings::SHADING_RATE_1X4 => ShadingRate::_1X4,
                                bindings::SHADING_RATE_2X1 => ShadingRate::_2X1,
                                bindings::SHADING_RATE_2X2 => ShadingRate::_2X2,
                                bindings::SHADING_RATE_2X4 => ShadingRate::_2X4,
                                bindings::SHADING_RATE_4X1 => ShadingRate::_4X1,
                                bindings::SHADING_RATE_4X2 => ShadingRate::_4X2,
                                bindings::SHADING_RATE_4X4 => ShadingRate::_4X4,
                                _ => panic!(),
                            },
                            sample_bits: SampleCount::from_bits_retain(sr.SampleBits.into()),
                        })
                        .take(self.ShadingRate.NumShadingRates.into()),
                ),
                cap_flags: ShadingRateCapFlags::from_bits_retain(self.ShadingRate.CapFlags.into()),
                combiners: ShadingRayeCombiner::from_bits_retain(self.ShadingRate.Combiners.into()),
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
