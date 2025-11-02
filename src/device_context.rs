use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use bitflags::bitflags;
use bon::{Builder, builder};
use static_assertions::const_assert_eq;

use crate::{
    Boxed,
    blas::BottomLevelAS,
    buffer::{Buffer, BufferMapReadToken, BufferMapReadWriteToken, BufferMapWriteToken},
    command_queue::CommandQueue,
    device_object::DeviceObject,
    fence::Fence,
    frame_buffer::Framebuffer,
    graphics_types::{
        MapFlags, ResourceState, ShadingRate, ShadingRateCombiner, StateTransitionType,
        TextureFormat, ValueType,
    },
    object::Object,
    pipeline_state::{
        ComputePipelineState, GraphicsPipelineState, RayTracingPipelineState, TilePipelineState,
    },
    query::{DurationQueryHelper, GetSysQueryType, Query, ScopedQueryToken, TimeStampQueryToken},
    render_pass::RenderPass,
    shader_binding_table::ShaderBindingTable,
    shader_resource_binding::ShaderResourceBinding,
    texture::{
        Texture, TextureSubResource, TextureSubresourceReadMapToken,
        TextureSubresourceReadWriteMapToken, TextureSubresourceWriteMapToken,
    },
    texture_view::TextureView,
    tlas::{HitGroupBindingMode, TLASBuildInstanceData, TopLevelAS},
};

bitflags! {
    #[derive(Clone, Copy)]
    pub struct DrawFlags: diligent_sys::DRAW_FLAGS {
        const None                         = diligent_sys::DRAW_FLAG_NONE as diligent_sys::DRAW_FLAGS;
        const VerifyStates                 = diligent_sys::DRAW_FLAG_VERIFY_STATES as diligent_sys::DRAW_FLAGS;
        const VerifyDrawAttribs            = diligent_sys::DRAW_FLAG_VERIFY_DRAW_ATTRIBS as diligent_sys::DRAW_FLAGS;
        const VerifyAll                    = diligent_sys::DRAW_FLAG_VERIFY_ALL as diligent_sys::DRAW_FLAGS;
        const DynamicResourceBuffersIntact = diligent_sys::DRAW_FLAG_DYNAMIC_RESOURCE_BUFFERS_INTACT as diligent_sys::DRAW_FLAGS;
    }
}

impl Default for DrawFlags {
    fn default() -> Self {
        DrawFlags::None
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct SetVertexBufferFlags: diligent_sys::SET_VERTEX_BUFFERS_FLAGS {
        const None  = diligent_sys::SET_VERTEX_BUFFERS_FLAG_NONE as diligent_sys::SET_VERTEX_BUFFERS_FLAGS;
        const Reset = diligent_sys::SET_VERTEX_BUFFERS_FLAG_RESET as diligent_sys::SET_VERTEX_BUFFERS_FLAGS;
    }
}

impl Default for SetVertexBufferFlags {
    fn default() -> Self {
        SetVertexBufferFlags::None
    }
}

#[repr(transparent)]
pub struct DrawAttribs(diligent_sys::DrawAttribs);

#[bon::bon]
impl DrawAttribs {
    #[builder]
    pub fn new(
        num_vertices: u32,
        #[builder(default)] flags: DrawFlags,
        #[builder(default = 1)] num_instances: u32,
        #[builder(default = 0)] start_vertex_location: u32,
        #[builder(default = 0)] first_instance_location: u32,
    ) -> Self {
        Self(diligent_sys::DrawAttribs {
            NumVertices: num_vertices,
            Flags: flags.bits(),
            NumInstances: num_instances,
            StartVertexLocation: start_vertex_location,
            FirstInstanceLocation: first_instance_location,
        })
    }
}

#[repr(transparent)]
pub struct DrawIndexedAttribs(diligent_sys::DrawIndexedAttribs);

#[bon::bon]
impl DrawIndexedAttribs {
    #[builder]
    pub fn new(
        num_indices: u32,
        index_type: ValueType,

        #[builder(default)] flags: DrawFlags,

        #[builder(default = 1)] num_instances: u32,

        #[builder(default = 0)] first_index_location: u32,

        #[builder(default = 0)] base_vertex: u32,

        #[builder(default = 0)] first_instance_location: u32,
    ) -> Self {
        Self(diligent_sys::DrawIndexedAttribs {
            NumIndices: num_indices,
            IndexType: index_type.into(),
            Flags: flags.bits(),
            NumInstances: num_instances,
            FirstIndexLocation: first_index_location,
            BaseVertex: base_vertex,
            FirstInstanceLocation: first_instance_location,
        })
    }
}

#[repr(transparent)]
pub struct DrawIndirectAttribs<'a>(diligent_sys::DrawIndirectAttribs, PhantomData<&'a ()>);

#[bon::bon]
impl<'a> DrawIndirectAttribs<'a> {
    #[builder]
    pub fn new(
        attribs_buffer: &'a Buffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] draw_count: u32,

        #[builder(default = 16)] draw_args_stride: u32,

        #[builder(default = ResourceStateTransitionMode::None)]
        attribs_buffer_state_transition_mode: ResourceStateTransitionMode,

        counter_buffer: Option<&Buffer>,

        #[builder(default = 0)] counter_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        counter_buffer_state_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        Self(
            diligent_sys::DrawIndirectAttribs {
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                DrawArgsOffset: draw_args_offset,
                Flags: flags.bits(),
                DrawCount: draw_count,
                DrawArgsStride: draw_args_stride,
                AttribsBufferStateTransitionMode: attribs_buffer_state_transition_mode.into(),
                pCounterBuffer: counter_buffer
                    .map_or(std::ptr::null_mut(), |buffer| buffer.sys_ptr()),
                CounterOffset: counter_offset,
                CounterBufferStateTransitionMode: counter_buffer_state_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct DrawIndexedIndirectAttribs<'a>(
    diligent_sys::DrawIndexedIndirectAttribs,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> DrawIndexedIndirectAttribs<'a> {
    #[builder]
    pub fn new(
        index_type: ValueType,

        attribs_buffer: &'a Buffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] draw_count: u32,

        #[builder(default = 20)] draw_args_stride: u32,

        #[builder(default = ResourceStateTransitionMode::None)]
        attribs_buffer_state_transition_mode: ResourceStateTransitionMode,

        counter_buffer: Option<&'a Buffer>,

        #[builder(default = 0)] counter_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        counter_buffer_state_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        Self(
            diligent_sys::DrawIndexedIndirectAttribs {
                IndexType: index_type.into(),
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                DrawArgsOffset: draw_args_offset,
                Flags: flags.bits(),
                DrawCount: draw_count,
                DrawArgsStride: draw_args_stride,
                AttribsBufferStateTransitionMode: attribs_buffer_state_transition_mode.into(),
                pCounterBuffer: counter_buffer
                    .map_or(std::ptr::null_mut(), |buffer| buffer.sys_ptr()),
                CounterOffset: counter_offset,
                CounterBufferStateTransitionMode: counter_buffer_state_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct DrawMeshAttribs(diligent_sys::DrawMeshAttribs);

#[bon::bon]
impl DrawMeshAttribs {
    #[builder]
    pub fn new(
        #[builder(default = 1)] thread_group_count_x: u32,

        #[builder(default = 1)] thread_group_count_y: u32,

        #[builder(default = 1)] thread_group_count_z: u32,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,
    ) -> Self {
        Self(diligent_sys::DrawMeshAttribs {
            ThreadGroupCountX: thread_group_count_x,
            ThreadGroupCountY: thread_group_count_y,
            ThreadGroupCountZ: thread_group_count_z,
            Flags: flags.bits(),
        })
    }
}

#[repr(transparent)]
pub struct DrawMeshIndirectAttribs<'a>(diligent_sys::DrawMeshIndirectAttribs, PhantomData<&'a ()>);

#[bon::bon]
impl<'a> DrawMeshIndirectAttribs<'a> {
    #[builder]
    pub fn new(
        attribs_buffer: &'a Buffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] command_count: u32,

        #[builder(default = ResourceStateTransitionMode::None)]
        attribs_buffer_state_transition_mode: ResourceStateTransitionMode,

        counter_buffer: Option<&'a Buffer>,

        #[builder(default = 0)] counter_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        counter_buffer_state_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        Self(
            diligent_sys::DrawMeshIndirectAttribs {
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                DrawArgsOffset: draw_args_offset,
                Flags: flags.bits(),
                CommandCount: command_count,
                AttribsBufferStateTransitionMode: attribs_buffer_state_transition_mode.into(),
                pCounterBuffer: counter_buffer
                    .map_or(std::ptr::null_mut(), |buffer| buffer.sys_ptr()),
                CounterOffset: counter_offset,
                CounterBufferStateTransitionMode: counter_buffer_state_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[derive(Clone, Copy)]
pub struct MultiDrawItem {
    pub num_vertices: u32,
    pub start_vertex_location: u32,
}

#[derive(Builder)]
pub struct MultiDrawAttribs {
    draw_items: Vec<MultiDrawItem>,

    #[builder(default = DrawFlags::None)]
    flags: DrawFlags,

    #[builder(default = 1)]
    num_instances: u32,

    #[builder(default = 0)]
    first_instance_location: u32,
}

#[derive(Clone, Copy)]
pub struct MultiDrawIndexedItem {
    pub num_vertices: u32,
    pub first_index_location: u32,
    pub base_vertex: u32,
}

#[derive(Builder)]
pub struct MultiDrawIndexedAttribs {
    draw_items: Vec<MultiDrawIndexedItem>,

    index_type: ValueType,

    #[builder(default = DrawFlags::None)]
    flags: DrawFlags,

    #[builder(default = 1)]
    num_instances: u32,

    #[builder(default = 0)]
    first_instance_location: u32,
}

#[repr(transparent)]
pub struct DispatchComputeAttribs(diligent_sys::DispatchComputeAttribs);
#[bon::bon]
impl DispatchComputeAttribs {
    #[builder]
    pub fn new(
        #[builder(default = 1)] thread_group_count_x: u32,

        #[builder(default = 1)] thread_group_count_y: u32,

        #[builder(default = 1)] thread_group_count_z: u32,

        #[cfg(feature = "metal")]
        #[builder(default = 0)]
        mtl_thread_group_size_x: u32,

        #[cfg(feature = "metal")]
        #[builder(default = 0)]
        mtl_thread_group_size_y: u32,

        #[cfg(feature = "metal")]
        #[builder(default = 0)]
        mtl_thread_group_size_z: u32,
    ) -> Self {
        Self(diligent_sys::DispatchComputeAttribs {
            ThreadGroupCountX: thread_group_count_x,
            ThreadGroupCountY: thread_group_count_y,
            ThreadGroupCountZ: thread_group_count_z,
            #[cfg(feature = "metal")]
            MtlThreadGroupSizeX: mtl_thread_group_size_x,
            #[cfg(feature = "metal")]
            MtlThreadGroupSizeY: mtl_thread_group_size_y,
            #[cfg(feature = "metal")]
            MtlThreadGroupSizeZ: mtl_thread_group_size_z,
            #[cfg(not(feature = "metal"))]
            MtlThreadGroupSizeX: 0,
            #[cfg(not(feature = "metal"))]
            MtlThreadGroupSizeY: 0,
            #[cfg(not(feature = "metal"))]
            MtlThreadGroupSizeZ: 0,
        })
    }
}

#[repr(transparent)]
pub struct DispatchComputeIndirectAttribs<'a>(
    diligent_sys::DispatchComputeIndirectAttribs,
    PhantomData<&'a ()>,
);

#[bon::bon]
impl<'a> DispatchComputeIndirectAttribs<'a> {
    #[builder]
    pub fn new(
        attribs_buffer: &'a Buffer,

        #[builder(default = ResourceStateTransitionMode::None)]
        attribs_buffer_state_transition_mode: ResourceStateTransitionMode,

        #[builder(default = 0)] dispatch_args_byte_offset: u64,

        #[cfg(feature = "metal")]
        #[builder(default = 0)]
        mtl_thread_group_size_x: u32,

        #[cfg(feature = "metal")]
        #[builder(default = 0)]
        mtl_thread_group_size_y: u32,

        #[cfg(feature = "metal")]
        #[builder(default = 0)]
        mtl_thread_group_size_z: u32,
    ) -> Self {
        Self(
            diligent_sys::DispatchComputeIndirectAttribs {
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                AttribsBufferStateTransitionMode: attribs_buffer_state_transition_mode.into(),
                DispatchArgsByteOffset: dispatch_args_byte_offset,
                #[cfg(feature = "metal")]
                MtlThreadGroupSizeX: mtl_thread_group_size_x,
                #[cfg(feature = "metal")]
                MtlThreadGroupSizeY: mtl_thread_group_size_y,
                #[cfg(feature = "metal")]
                MtlThreadGroupSizeZ: mtl_thread_group_size_z,
                #[cfg(not(feature = "metal"))]
                MtlThreadGroupSizeX: 0,
                #[cfg(not(feature = "metal"))]
                MtlThreadGroupSizeY: 0,
                #[cfg(not(feature = "metal"))]
                MtlThreadGroupSizeZ: 0,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct DispatchTileAttribs(diligent_sys::DispatchTileAttribs);
#[bon::bon]
impl DispatchTileAttribs {
    #[builder]
    pub fn new(
        #[builder(default = 1)] threads_per_tile_x: u32,

        #[builder(default = 1)] threads_per_tile_y: u32,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,
    ) -> Self {
        Self(diligent_sys::DispatchTileAttribs {
            ThreadsPerTileX: threads_per_tile_x,
            ThreadsPerTileY: threads_per_tile_y,
            Flags: flags.bits(),
        })
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct RaytracingGeometryFlags: diligent_sys::RAYTRACING_GEOMETRY_FLAGS {
        const None                        = diligent_sys::RAYTRACING_GEOMETRY_FLAG_NONE as _;
        const Opaque                      = diligent_sys::RAYTRACING_GEOMETRY_FLAG_OPAQUE as _;
        const NoDuplicateAnyHitInvocation = diligent_sys::RAYTRACING_GEOMETRY_FLAG_NO_DUPLICATE_ANY_HIT_INVOCATION as _;
    }
}

impl Default for RaytracingGeometryFlags {
    fn default() -> Self {
        RaytracingGeometryFlags::None
    }
}

const_assert_eq!(diligent_sys::RAYTRACING_GEOMETRY_FLAG_LAST, 2);

#[repr(transparent)]
pub struct BLASBuildBoundingBoxData<'a>(
    diligent_sys::BLASBuildBoundingBoxData,
    PhantomData<&'a ()>,
);
#[bon::bon]
impl<'a> BLASBuildBoundingBoxData<'a> {
    #[builder]
    pub fn new(
        geometry_name: &'a CStr,

        box_buffer: &'a Buffer,

        #[builder(default = 0)] box_offset: u64,

        box_stride: u32,

        box_count: u32,

        #[builder(default)] flags: RaytracingGeometryFlags,
    ) -> Self {
        Self(
            diligent_sys::BLASBuildBoundingBoxData {
                GeometryName: geometry_name.as_ptr(),
                pBoxBuffer: box_buffer.sys_ptr(),
                BoxOffset: box_offset,
                BoxStride: box_stride,
                BoxCount: box_count,
                Flags: flags.bits(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct BLASBuildTriangleData<'a>(diligent_sys::BLASBuildTriangleData, PhantomData<&'a ()>);
#[bon::bon]
impl<'a> BLASBuildTriangleData<'a> {
    #[builder]
    pub fn new(
        geometry_name: &'a CStr,

        vertex_buffer: &'a Buffer,

        #[builder(default = 0)] vertex_offset: u64,

        vertex_stride: u32,

        vertex_count: usize,

        vertex_value_type: Option<ValueType>,

        #[builder(default = 0)] vertex_component_count: u8,

        primitive_count: usize,

        index_buffer: Option<&'a Buffer>,

        #[builder(default = 0)] index_offset: u64,

        index_type: Option<ValueType>,

        transform_buffer: Option<&'a Buffer>,

        #[builder(default = 0)] transform_buffer_offset: u64,

        #[builder(default)] flags: RaytracingGeometryFlags,
    ) -> Self {
        Self(
            diligent_sys::BLASBuildTriangleData {
                GeometryName: geometry_name.as_ptr(),
                pVertexBuffer: vertex_buffer.sys_ptr(),
                VertexOffset: vertex_offset,
                VertexStride: vertex_stride,
                VertexCount: vertex_count as u32,
                VertexValueType: vertex_value_type
                    .map_or(diligent_sys::VT_UNDEFINED as _, |vt| vt.into()),
                VertexComponentCount: vertex_component_count,
                PrimitiveCount: primitive_count as u32,
                pIndexBuffer: index_buffer.map_or(std::ptr::null_mut(), |ib| ib.sys_ptr()),
                IndexOffset: index_offset,
                IndexType: index_type.map_or(diligent_sys::VT_UNDEFINED as _, |vt| vt.into()),
                pTransformBuffer: transform_buffer
                    .as_ref()
                    .map_or(std::ptr::null_mut(), |tb| tb.sys_ptr()),
                TransformBufferOffset: transform_buffer_offset,
                Flags: flags.bits(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct BuildBLASAttribs<'a>(diligent_sys::BuildBLASAttribs, PhantomData<&'a ()>);
#[bon::bon]
impl<'a> BuildBLASAttribs<'a> {
    #[builder]
    pub fn new(
        blas: &'a BottomLevelAS,

        #[builder(default = ResourceStateTransitionMode::None)]
        blas_transition_mode: ResourceStateTransitionMode,

        #[builder(default = ResourceStateTransitionMode::None)]
        geometry_transition_mode: ResourceStateTransitionMode,

        #[builder(default)] triangle_data: &[BLASBuildTriangleData<'a>],

        #[builder(default)] box_data: &[BLASBuildBoundingBoxData<'a>],

        scratch_buffer: &'a Buffer,

        #[builder(default = 0)] scratch_buffer_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        scratch_buffer_transition_mode: ResourceStateTransitionMode,

        #[builder(default = false)] update: bool,
    ) -> Self {
        Self(
            diligent_sys::BuildBLASAttribs {
                pBLAS: blas.sys_ptr(),
                BLASTransitionMode: blas_transition_mode.into(),
                GeometryTransitionMode: geometry_transition_mode.into(),
                pTriangleData: if triangle_data.is_empty() {
                    std::ptr::null()
                } else {
                    triangle_data.as_ptr() as _
                },
                TriangleDataCount: triangle_data.len() as u32,
                pBoxData: if box_data.is_empty() {
                    std::ptr::null()
                } else {
                    box_data.as_ptr() as _
                },
                BoxDataCount: box_data.len() as u32,
                pScratchBuffer: scratch_buffer.sys_ptr(),
                ScratchBufferOffset: scratch_buffer_offset,
                ScratchBufferTransitionMode: scratch_buffer_transition_mode.into(),
                Update: update,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct BuildTLASAttribs<'a>(diligent_sys::BuildTLASAttribs, PhantomData<&'a ()>);
#[bon::bon]
impl<'a> BuildTLASAttribs<'a> {
    #[builder]
    pub fn new(
        tlas: &'a TopLevelAS,

        #[builder(default = ResourceStateTransitionMode::None)]
        tlas_transition_mode: ResourceStateTransitionMode,

        #[builder(default = ResourceStateTransitionMode::None)]
        blas_transition_mode: ResourceStateTransitionMode,

        instances: &'a [TLASBuildInstanceData<'a>],

        instance_buffer: &'a Buffer,

        #[builder(default = 0)] instance_buffer_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        instance_buffer_transition_mode: ResourceStateTransitionMode,

        #[builder(default = 1)] hit_group_stride: u32,

        #[builder(default = 0)] base_contribution_to_hit_group_index: u32,

        #[builder(default = HitGroupBindingMode::PerGeometry)] binding_mode: HitGroupBindingMode,

        scratch_buffer: &'a Buffer,

        #[builder(default = 0)] scratch_buffer_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        scratch_buffer_transition_mode: ResourceStateTransitionMode,

        #[builder(default = false)] update: bool,
    ) -> Self {
        Self(
            diligent_sys::BuildTLASAttribs {
                pTLAS: tlas.sys_ptr(),
                TLASTransitionMode: tlas_transition_mode.into(),
                BLASTransitionMode: blas_transition_mode.into(),
                pInstances: if instances.is_empty() {
                    std::ptr::null()
                } else {
                    instances.as_ptr() as _
                },
                InstanceCount: instances.len() as u32,
                pInstanceBuffer: instance_buffer.sys_ptr(),
                InstanceBufferOffset: instance_buffer_offset,
                InstanceBufferTransitionMode: instance_buffer_transition_mode.into(),
                HitGroupStride: hit_group_stride,
                BaseContributionToHitGroupIndex: base_contribution_to_hit_group_index,
                BindingMode: binding_mode.into(),
                pScratchBuffer: scratch_buffer.sys_ptr(),
                ScratchBufferOffset: scratch_buffer_offset,
                ScratchBufferTransitionMode: scratch_buffer_transition_mode.into(),
                Update: update,
            },
            PhantomData,
        )
    }
}

#[derive(Builder)]
pub struct UpdateIndirectRTBufferAttribs<'a> {
    attribs_buffer: &'a Buffer,

    #[builder(default = 0)]
    attribs_buffer_offset: u64,

    #[builder(default = ResourceStateTransitionMode::None)]
    transition_mode: ResourceStateTransitionMode,
}

#[repr(transparent)]
pub struct TraceRaysAttribs<'a>(diligent_sys::TraceRaysAttribs, PhantomData<&'a ()>);
#[bon::bon]
impl<'a> TraceRaysAttribs<'a> {
    #[builder]
    pub fn new(
        sbt: &'a ShaderBindingTable,

        #[builder(default = 1)] dimension_x: u32,

        #[builder(default = 1)] dimension_y: u32,

        #[builder(default = 1)] dimension_z: u32,
    ) -> Self {
        Self(
            diligent_sys::TraceRaysAttribs {
                pSBT: sbt.sys_ptr(),
                DimensionX: dimension_x,
                DimensionY: dimension_y,
                DimensionZ: dimension_z,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct TraceRaysIndirectAttribs<'a>(
    diligent_sys::TraceRaysIndirectAttribs,
    PhantomData<&'a ()>,
);
#[bon::bon]
impl<'a> TraceRaysIndirectAttribs<'a> {
    #[builder]
    pub fn new(
        sbt: &'a ShaderBindingTable,
        attribs_buffer: &'a Buffer,
        #[builder(default = ResourceStateTransitionMode::None)]
        attribs_buffer_state_transition_mode: ResourceStateTransitionMode,
        #[builder(default = 0)] args_byte_offset: u64,
    ) -> Self {
        Self(
            diligent_sys::TraceRaysIndirectAttribs {
                pSBT: sbt.sys_ptr(),
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                AttribsBufferStateTransitionMode: attribs_buffer_state_transition_mode.into(),
                ArgsByteOffset: args_byte_offset,
            },
            PhantomData,
        )
    }
}

#[derive(Clone, Copy)]
pub enum ResourceStateTransitionMode {
    None,
    Transition,
    Verify,
}

impl From<ResourceStateTransitionMode> for diligent_sys::RESOURCE_STATE_TRANSITION_MODE {
    fn from(value: ResourceStateTransitionMode) -> Self {
        (match value {
            ResourceStateTransitionMode::None => diligent_sys::RESOURCE_STATE_TRANSITION_MODE_NONE,
            ResourceStateTransitionMode::Transition => {
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE_TRANSITION
            }
            ResourceStateTransitionMode::Verify => {
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE_VERIFY
            }
        }) as _
    }
}

pub struct Viewport {
    top_left_x: f32,
    top_left_y: f32,
    width: f32,
    height: f32,
    min_depth: f32,
    max_depth: f32,
}

impl Viewport {
    pub fn new(top_left_x: f32, top_left_y: f32, width: f32, height: f32) -> Self {
        Viewport {
            top_left_x,
            top_left_y,
            width,
            height,
            min_depth: 0.0,
            max_depth: 1.0,
        }
    }
    pub fn min_depth(mut self, min_depth: f32) -> Self {
        self.min_depth = min_depth;
        self
    }
    pub fn max_depth(mut self, max_depth: f32) -> Self {
        self.max_depth = max_depth;
        self
    }
}

impl From<&Viewport> for diligent_sys::Viewport {
    fn from(value: &Viewport) -> Self {
        diligent_sys::Viewport {
            TopLeftX: value.top_left_x,
            TopLeftY: value.top_left_y,
            Width: value.width,
            Height: value.height,
            MinDepth: value.min_depth,
            MaxDepth: value.max_depth,
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn is_valid(&self) -> bool {
        self.right > self.left && self.bottom > self.top
    }
}

impl From<&Rect> for diligent_sys::Rect {
    fn from(value: &Rect) -> Self {
        diligent_sys::Rect {
            bottom: value.bottom,
            left: value.left,
            right: value.right,
            top: value.top,
        }
    }
}

pub struct ScopedDebugGroup<'a> {
    device_context: &'a DeviceContext,
}

impl<'a> ScopedDebugGroup<'a> {
    fn new(device_context: &'a DeviceContext, name: &CStr, color: Option<[f32; 4]>) -> Self {
        unsafe_member_call!(
            device_context,
            DeviceContext,
            BeginDebugGroup,
            name.as_ptr(),
            color.map_or(std::ptr::null(), |color| color.as_ptr())
        );

        ScopedDebugGroup { device_context }
    }
}

impl<'a> Drop for ScopedDebugGroup<'a> {
    fn drop(&mut self) {
        unsafe_member_call!(self.device_context, DeviceContext, EndDebugGroup)
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct StateTransitionFlags: diligent_sys::STATE_TRANSITION_FLAGS {
        const None           = diligent_sys::STATE_TRANSITION_FLAG_NONE as diligent_sys::STATE_TRANSITION_FLAGS;
        const UpdateState    = diligent_sys::STATE_TRANSITION_FLAG_UPDATE_STATE as diligent_sys::STATE_TRANSITION_FLAGS;
        const DiscardContent = diligent_sys::STATE_TRANSITION_FLAG_DISCARD_CONTENT as diligent_sys::STATE_TRANSITION_FLAGS;
        const Aliasing       = diligent_sys::STATE_TRANSITION_FLAG_ALIASING as diligent_sys::STATE_TRANSITION_FLAGS;
    }
}

impl Default for StateTransitionFlags {
    fn default() -> Self {
        StateTransitionFlags::None
    }
}

#[repr(transparent)]
pub struct StateTransitionDesc<'a>(diligent_sys::StateTransitionDesc, PhantomData<&'a ()>);
#[bon::bon]
impl<'a> StateTransitionDesc<'a> {
    #[builder(derive(Clone))]
    pub fn new(
        resource: &'a DeviceObject,

        new_state: ResourceState,

        #[builder(default = 0)] first_mip_level: u32,

        #[builder(default = diligent_sys::REMAINING_MIP_LEVELS)] mip_levels_count: u32,

        #[builder(default = 0)] first_array_slice: u32,

        #[builder(default = diligent_sys::REMAINING_ARRAY_SLICES)] array_slice_count: u32,

        old_state: Option<ResourceState>,

        #[builder(default = StateTransitionType::Immediate)] transition_type: StateTransitionType,

        #[builder(default)] flags: StateTransitionFlags,
    ) -> Self {
        Self(
            diligent_sys::StateTransitionDesc {
                pResourceBefore: std::ptr::null_mut(), // TODO
                pResource: resource.sys_ptr(),
                FirstMipLevel: first_mip_level,
                MipLevelsCount: mip_levels_count,
                FirstArraySlice: first_array_slice,
                ArraySliceCount: array_slice_count,
                OldState: old_state.map_or(diligent_sys::RESOURCE_STATE_UNKNOWN as _, |state| {
                    state.bits()
                }),
                NewState: new_state.bits(),
                TransitionType: transition_type.into(),
                Flags: flags.bits(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct CommandList(diligent_sys::ICommandList);

pub struct DepthStencilClearValue {
    pub depth: f32,
    pub stencil: u8,
}

pub struct OptimizedClearValue {
    pub format: TextureFormat,
    pub color: [f32; 4usize],
    pub depth_stencil: DepthStencilClearValue,
}

pub struct BeginRenderPassAttribs<'a> {
    pub render_pass: &'a RenderPass,
    pub frame_buffer: &'a Framebuffer,
    pub clear_values: Vec<OptimizedClearValue>,
    pub state_transition_mode: ResourceStateTransitionMode,
}

pub struct RenderPassToken<'a> {
    context: &'a DeviceContext,
}

impl<'a> RenderPassToken<'a> {
    pub fn new(context: &'a DeviceContext, attribs: &BeginRenderPassAttribs) -> Self {
        let clear_values = attribs
            .clear_values
            .iter()
            .map(|clear_value| diligent_sys::OptimizedClearValue {
                Color: clear_value.color,
                DepthStencil: diligent_sys::DepthStencilClearValue {
                    Depth: clear_value.depth_stencil.depth,
                    Stencil: clear_value.depth_stencil.stencil,
                },
                Format: clear_value.format.into(),
            })
            .collect::<Vec<_>>();

        let attribs = diligent_sys::BeginRenderPassAttribs {
            pRenderPass: attribs.render_pass.sys_ptr(),
            ClearValueCount: attribs.clear_values.len() as u32,
            pClearValues: clear_values.as_ptr() as *mut diligent_sys::OptimizedClearValue,
            StateTransitionMode: attribs.state_transition_mode.into(),
            pFramebuffer: attribs.frame_buffer.sys_ptr(),
        };

        unsafe_member_call!(
            context,
            DeviceContext,
            BeginRenderPass,
            std::ptr::from_ref(&attribs)
        );

        RenderPassToken { context }
    }

    pub fn next_subpass(&self) {
        unsafe_member_call!(self.context, DeviceContext, NextSubpass)
    }
}

impl Drop for RenderPassToken<'_> {
    fn drop(&mut self) {
        unsafe_member_call!(self.context, DeviceContext, EndRenderPass)
    }
}

pub struct GraphicsPipelineToken<'a> {
    context: &'a DeviceContext,
}

impl<'a> GraphicsPipelineToken<'a> {
    pub fn draw(&self, attribs: &DrawAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            Draw,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn draw_indexed(&self, attribs: &DrawIndexedAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DrawIndexed,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn draw_indirect(&self, attribs: &DrawIndirectAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DrawIndirect,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn draw_indexed_indirect(&self, attribs: &DrawIndexedIndirectAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DrawIndexedIndirect,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn multi_draw(&self, attribs: &MultiDrawAttribs) {
        let draw_items = attribs
            .draw_items
            .iter()
            .map(|item| diligent_sys::MultiDrawItem {
                NumVertices: item.num_vertices,
                StartVertexLocation: item.start_vertex_location,
            })
            .collect::<Vec<_>>();
        let attribs = diligent_sys::MultiDrawAttribs {
            DrawCount: draw_items.len() as u32,
            pDrawItems: draw_items.as_ptr(),
            Flags: attribs.flags.bits(),
            NumInstances: attribs.num_instances,
            FirstInstanceLocation: attribs.first_instance_location,
        };
        unsafe_member_call!(
            self.context,
            DeviceContext,
            MultiDraw,
            std::ptr::from_ref(&attribs)
        )
    }

    pub fn multi_draw_indexed(&self, attribs: &MultiDrawIndexedAttribs) {
        let draw_items = attribs
            .draw_items
            .iter()
            .map(|item| diligent_sys::MultiDrawIndexedItem {
                NumIndices: item.num_vertices,
                FirstIndexLocation: item.first_index_location,
                BaseVertex: item.base_vertex,
            })
            .collect::<Vec<_>>();
        let attribs = diligent_sys::MultiDrawIndexedAttribs {
            DrawCount: draw_items.len() as u32,
            pDrawItems: draw_items.as_ptr(),
            IndexType: attribs.index_type.into(),
            Flags: attribs.flags.bits(),
            NumInstances: attribs.num_instances,
            FirstInstanceLocation: attribs.first_instance_location,
        };
        unsafe_member_call!(
            self.context,
            DeviceContext,
            MultiDrawIndexed,
            std::ptr::from_ref(&attribs)
        )
    }
}

pub struct MeshPipelineToken<'a> {
    context: &'a DeviceContext,
}

impl<'a> MeshPipelineToken<'a> {
    pub fn draw_mesh(&self, attribs: &DrawMeshAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DrawMesh,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn draw_mesh_indirect(&self, attribs: &DrawMeshIndirectAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DrawMeshIndirect,
            std::ptr::from_ref(attribs) as _
        )
    }
}

pub struct ComputePipelineToken<'a> {
    context: &'a DeviceContext,
}

impl<'a> ComputePipelineToken<'a> {
    pub fn dispatch_compute(&self, attribs: &DispatchComputeAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DispatchCompute,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn dispatch_compute_indirect(&self, attribs: &DispatchComputeIndirectAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DispatchComputeIndirect,
            std::ptr::from_ref(attribs) as _
        )
    }
}

pub struct TilePipelineToken<'a> {
    context: &'a DeviceContext,
}

impl<'a> TilePipelineToken<'a> {
    pub fn dispatch_tile(&self, attribs: &DispatchTileAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DispatchTile,
            std::ptr::from_ref(attribs) as _
        )
    }
}

pub struct RayTracingPipelineToken<'a> {
    context: &'a DeviceContext,
}

impl<'a> RayTracingPipelineToken<'a> {
    pub fn trace_rays(&self, attribs: &TraceRaysAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            TraceRays,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn trace_rays_indirect(&self, attribs: &TraceRaysIndirectAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            TraceRaysIndirect,
            std::ptr::from_ref(attribs) as _
        )
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IDeviceContextMethods>(),
    72 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct DeviceContext(pub(crate) diligent_sys::IDeviceContext);

impl Deref for DeviceContext {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IObject as *const Object) }
    }
}

impl DeviceContext {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IDeviceContext {
        std::ptr::addr_of!(self.0) as _
    }

    pub fn set_graphics_pipeline_state(
        &self,
        pipeline_state: &GraphicsPipelineState,
    ) -> GraphicsPipelineToken<'_> {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        GraphicsPipelineToken { context: self }
    }

    pub fn set_mesh_pipeline_state(
        &self,
        pipeline_state: &GraphicsPipelineState,
    ) -> GraphicsPipelineToken<'_> {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        GraphicsPipelineToken { context: self }
    }

    pub fn set_compute_pipeline_state(
        &self,
        pipeline_state: &ComputePipelineState,
    ) -> ComputePipelineToken<'_> {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        ComputePipelineToken { context: self }
    }

    pub fn set_tile_pipeline_state(
        &self,
        pipeline_state: &TilePipelineState,
    ) -> TilePipelineToken<'_> {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        TilePipelineToken { context: self }
    }

    pub fn set_ray_tracing_pipeline_state(
        &self,
        pipeline_state: &RayTracingPipelineState,
    ) -> RayTracingPipelineToken<'_> {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        RayTracingPipelineToken { context: self }
    }

    pub fn transition_shader_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe_member_call!(
            self,
            DeviceContext,
            TransitionShaderResources,
            shader_resource_binding.sys_ptr()
        )
    }

    pub fn commit_shader_resources(
        &self,
        shader_resource_binding: &ShaderResourceBinding,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            CommitShaderResources,
            shader_resource_binding.sys_ptr(),
            state_transition_mode.into()
        )
    }

    pub fn set_stencil_ref(&self, stencil_ref: u32) {
        unsafe_member_call!(self, DeviceContext, SetStencilRef, stencil_ref);
    }

    pub fn set_blend_factors(&self, blend_factors: Option<&[f32; 4]>) {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetBlendFactors,
            blend_factors.map_or(std::ptr::null(), |factors| factors.as_ptr())
        );
    }

    pub fn set_vertex_buffers(
        &self,
        buffers: &[(&Buffer, u64)],
        state_transition_mode: ResourceStateTransitionMode,
        flags: SetVertexBufferFlags,
    ) {
        let num_buffers = buffers.as_ref().len();
        let (buffer_pointers, offsets): (Vec<_>, Vec<_>) = buffers
            .as_ref()
            .iter()
            .map(|&(buffer, offset)| (buffer.sys_ptr(), offset))
            .unzip();

        unsafe_member_call!(
            self,
            DeviceContext,
            SetVertexBuffers,
            0,
            num_buffers as u32,
            buffer_pointers.as_ptr() as _,
            offsets.as_ptr(),
            state_transition_mode.into(),
            flags.bits()
        )
    }

    pub fn invalidate_state(&self) {
        unsafe_member_call!(self, DeviceContext, InvalidateState)
    }

    pub fn set_index_buffer(
        &self,
        index_buffer: &Buffer,
        offset: u64,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetIndexBuffer,
            index_buffer.sys_ptr(),
            offset,
            state_transition_mode.into()
        )
    }

    pub fn set_viewports(
        &self,
        viewports: &[&Viewport],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        let viewports: Vec<_> = viewports.iter().map(|&viewport| viewport.into()).collect();
        unsafe_member_call!(
            self,
            DeviceContext,
            SetViewports,
            viewports.len() as u32,
            viewports.as_ptr(),
            render_target_width,
            render_target_height
        )
    }

    pub fn set_scissor_rects(
        &self,
        rects: &[&Rect],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        let rects: Vec<_> = rects.iter().map(|&rect| rect.into()).collect();

        unsafe_member_call!(
            self,
            DeviceContext,
            SetScissorRects,
            rects.len() as u32,
            rects.as_ptr(),
            render_target_width,
            render_target_height
        )
    }

    pub fn set_render_targets(
        &self,
        render_targets: &[&TextureView],
        depth_stencil: Option<&TextureView>,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetRenderTargets,
            render_targets.len() as u32,
            render_targets.as_ptr() as _,
            depth_stencil.map_or(std::ptr::null_mut(), |v| v.sys_ptr()),
            state_transition_mode.into()
        )
    }

    pub fn new_render_pass(&self, attribs: &BeginRenderPassAttribs) -> RenderPassToken<'_> {
        RenderPassToken::new(self, attribs)
    }

    pub fn get_tile_size(&self) -> (u32, u32) {
        let mut tile_size_x: u32 = 0;
        let mut tile_size_y: u32 = 0;
        unsafe_member_call!(
            self,
            DeviceContext,
            GetTileSize,
            std::ptr::addr_of_mut!(tile_size_x),
            std::ptr::addr_of_mut!(tile_size_y)
        );
        (tile_size_x, tile_size_y)
    }

    pub fn clear_depth(
        &self,
        view: &TextureView,
        depth: f32,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearDepthStencil,
            view.sys_ptr(),
            diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
            depth,
            0,
            state_transition_mode.into()
        )
    }

    pub fn clear_stencil(
        &self,
        view: &mut TextureView,
        stencil: u8,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearDepthStencil,
            view.sys_ptr(),
            diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
            0.0,
            stencil,
            state_transition_mode.into()
        )
    }

    pub fn clear_depth_stencil(
        &self,
        view: &mut TextureView,
        depth: f32,
        stencil: u8,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearDepthStencil,
            view.sys_ptr(),
            diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS
                | diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
            depth,
            stencil,
            state_transition_mode.into()
        )
    }

    pub fn clear_render_target<T>(
        &self,
        view: &TextureView,
        rgba: &[T; 4],
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearRenderTarget,
            view.sys_ptr(),
            (*rgba).as_ptr() as *const std::os::raw::c_void,
            state_transition_mode.into()
        )
    }

    pub fn enqueue_signal(&self, fence: &Fence, value: u64) {
        unsafe_member_call!(self, DeviceContext, EnqueueSignal, fence.sys_ptr(), value);
    }

    pub fn update_buffer<T>(
        &self,
        buffer: &mut Buffer,
        offset: u64,
        size: u64,
        data: &T,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateBuffer,
            buffer.sys_ptr(),
            offset,
            size,
            std::ptr::from_ref(data) as *const std::os::raw::c_void,
            state_transition_mode.into()
        )
    }

    pub fn update_buffer_from_slice<T>(
        &self,
        buffer: &mut Buffer,
        data: &[T],
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateBuffer,
            buffer.sys_ptr(),
            0,
            data.len() as u64 * std::mem::size_of::<T>() as u64,
            data.as_ptr() as *const std::os::raw::c_void,
            state_transition_mode.into()
        )
    }

    pub fn copy_buffer(
        &self,
        src_buffer: &Buffer,
        src_offset: u64,
        src_buffer_transition_mode: ResourceStateTransitionMode,
        dst_buffer: &mut Buffer,
        dst_offset: u64,
        size: u64,
        dst_buffer_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            CopyBuffer,
            src_buffer.sys_ptr(),
            src_offset,
            src_buffer_transition_mode.into(),
            dst_buffer.sys_ptr(),
            dst_offset,
            size,
            dst_buffer_transition_mode.into()
        )
    }

    pub fn map_buffer_read<'a, T>(
        &'a self,
        buffer: &'a Buffer,
        map_flags: MapFlags,
    ) -> BufferMapReadToken<'a, T>
    where
        Self: Sized,
    {
        BufferMapReadToken::new(self, buffer, map_flags.bits())
    }

    pub fn map_buffer_write<'a, T>(
        &'a self,
        buffer: &'a Buffer,
        map_flags: MapFlags,
    ) -> BufferMapWriteToken<'a, T>
    where
        Self: Sized,
    {
        BufferMapWriteToken::new(self, buffer, map_flags.bits())
    }

    pub fn map_buffer_read_write<'a, T>(
        &'a self,
        buffer: &'a Buffer,
        map_flags: MapFlags,
    ) -> BufferMapReadWriteToken<'a, T>
    where
        Self: Sized,
    {
        BufferMapReadWriteToken::new(self, buffer, map_flags.bits())
    }

    pub fn update_texture(
        &self,
        texture: &mut Texture,
        mip_level: u32,
        slice: u32,
        dst_box: &diligent_sys::Box,
        subres_data: &TextureSubResource,
        src_buffer_transition_mode: ResourceStateTransitionMode,
        texture_transition_mode: ResourceStateTransitionMode,
    ) {
        let subres_data = subres_data.into();

        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateTexture,
            texture.sys_ptr(),
            mip_level,
            slice,
            std::ptr::from_ref(dst_box),
            std::ptr::addr_of!(subres_data),
            src_buffer_transition_mode.into(),
            texture_transition_mode.into()
        )
    }

    pub fn copy_texture(&self, copy_attribs: &diligent_sys::CopyTextureAttribs) {
        unsafe_member_call!(
            self,
            DeviceContext,
            CopyTexture,
            std::ptr::from_ref(copy_attribs)
        )
    }

    pub fn map_texture_subresource_read<'a, T>(
        &'a self,
        texture: &'a Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<diligent_sys::Box>,
    ) -> TextureSubresourceReadMapToken<'a, T> {
        TextureSubresourceReadMapToken::new(
            self,
            texture,
            mip_level,
            array_slice,
            map_flags,
            map_region,
        )
    }

    pub fn map_texture_subresource_write<'a, T>(
        &'a self,
        texture: &'a Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<diligent_sys::Box>,
    ) -> TextureSubresourceWriteMapToken<'a, T> {
        TextureSubresourceWriteMapToken::new(
            self,
            texture,
            mip_level,
            array_slice,
            map_flags,
            map_region,
        )
    }

    pub fn map_texture_subresource_read_write<'a, T>(
        &'a self,
        texture: &'a Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<diligent_sys::Box>,
    ) -> TextureSubresourceReadWriteMapToken<'a, T> {
        TextureSubresourceReadWriteMapToken::new(
            self,
            texture,
            mip_level,
            array_slice,
            map_flags,
            map_region,
        )
    }

    pub fn generate_mips(&self, texture_view: &mut TextureView) {
        unsafe_member_call!(self, DeviceContext, GenerateMips, texture_view.sys_ptr())
    }

    pub fn finish_frame(&self) {
        unsafe_member_call!(self, DeviceContext, FinishFrame)
    }

    pub fn get_frame_number(&self) -> u64 {
        unsafe_member_call!(self, DeviceContext, GetFrameNumber)
    }

    pub fn transition_resource_states<'a>(&self, barriers: &[StateTransitionDesc<'a>]) {
        unsafe_member_call!(
            self,
            DeviceContext,
            TransitionResourceStates,
            barriers.len() as u32,
            barriers.as_ptr() as _
        )
    }

    pub fn resolve_texture_subresource(
        &self,
        src_texture: &Texture,
        dst_texture: &mut Texture,
        resolve_attribs: &diligent_sys::ResolveTextureSubresourceAttribs,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            ResolveTextureSubresource,
            src_texture.sys_ptr(),
            dst_texture.sys_ptr(),
            std::ptr::from_ref(resolve_attribs)
        )
    }

    pub fn build_blas(&self, attribs: &BuildBLASAttribs) {
        unsafe_member_call!(
            self,
            DeviceContext,
            BuildBLAS,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn build_tlas<'a>(&self, attribs: &BuildTLASAttribs<'a>) {
        unsafe_member_call!(
            self,
            DeviceContext,
            BuildTLAS,
            std::ptr::from_ref(attribs) as _
        )
    }

    pub fn copy_blas(&self, attribs: &diligent_sys::CopyBLASAttribs) {
        unsafe_member_call!(self, DeviceContext, CopyBLAS, std::ptr::from_ref(attribs))
    }

    pub fn copy_tlas(&self, attribs: &diligent_sys::CopyTLASAttribs) {
        unsafe_member_call!(self, DeviceContext, CopyTLAS, std::ptr::from_ref(attribs))
    }

    pub fn write_blas_compacted_size(&self, attribs: &diligent_sys::WriteBLASCompactedSizeAttribs) {
        unsafe_member_call!(
            self,
            DeviceContext,
            WriteBLASCompactedSize,
            std::ptr::from_ref(attribs)
        )
    }

    pub fn write_tlas_compacted_size(&self, attribs: &diligent_sys::WriteTLASCompactedSizeAttribs) {
        unsafe_member_call!(
            self,
            DeviceContext,
            WriteTLASCompactedSize,
            std::ptr::from_ref(attribs)
        )
    }

    pub fn update_sbt(
        &self,
        sbt: &ShaderBindingTable,
        attribs: Option<&UpdateIndirectRTBufferAttribs>,
    ) {
        let attribs = attribs.map(|attribs| diligent_sys::UpdateIndirectRTBufferAttribs {
            pAttribsBuffer: attribs.attribs_buffer.sys_ptr(),
            AttribsBufferOffset: attribs.attribs_buffer_offset,
            TransitionMode: attribs.transition_mode.into(),
        });
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateSBT,
            sbt.sys_ptr(),
            attribs.map_or(std::ptr::null_mut(), |attribs| std::ptr::from_ref(&attribs))
        )
    }

    pub fn debug_group(&self, name: &CStr, color: Option<[f32; 4]>) -> ScopedDebugGroup<'_> {
        ScopedDebugGroup::new(self, name, color)
    }

    pub fn insert_debug_label(&self, label: &CStr, color: Option<[f32; 4]>) {
        unsafe_member_call!(
            self,
            DeviceContext,
            InsertDebugLabel,
            label.as_ptr(),
            color.map_or(std::ptr::null(), |color| color.as_ptr())
        )
    }

    pub fn set_shading_rate(
        &self,
        base_rate: ShadingRate,
        primitive_combiner: ShadingRateCombiner,
        texture_combiner: ShadingRateCombiner,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetShadingRate,
            base_rate.into(),
            primitive_combiner.bits(),
            texture_combiner.bits()
        )
    }

    pub fn clear_stats(&self) {
        unsafe_member_call!(self, DeviceContext, ClearStats)
    }

    pub fn get_stats(&self) -> &diligent_sys::DeviceContextStats {
        // TODO
        let stats = unsafe_member_call!(self, DeviceContext, GetStats);

        unsafe { stats.as_ref().unwrap_unchecked() }
    }
}

#[repr(transparent)]
pub struct ImmediateDeviceContext(DeviceContext);

impl Deref for ImmediateDeviceContext {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ImmediateDeviceContext {
    pub fn flush(&self) {
        unsafe_member_call!(self.0, DeviceContext, Flush)
    }

    pub fn execute_command_lists(&self, command_lists: &[CommandList]) {
        unsafe_member_call!(
            self.0,
            DeviceContext,
            ExecuteCommandLists,
            command_lists.len() as u32,
            command_lists.as_ptr() as _
        )
    }

    pub fn wait_for_idle(&self) {
        unsafe_member_call!(self.0, DeviceContext, WaitForIdle)
    }

    // TODO : make command queue locking RAII
    pub fn lock_command_queue(&self) -> Result<Boxed<CommandQueue>, ()> {
        let command_queue_ptr = unsafe_member_call!(self.0, DeviceContext, LockCommandQueue);
        if command_queue_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<CommandQueue>::new(command_queue_ptr as _))
        }
    }

    pub fn unlock_command_queue(&self) {
        unsafe_member_call!(self.0, DeviceContext, UnlockCommandQueue);
    }

    pub fn begin_query<'a, QueryDataType: GetSysQueryType>(
        &'a self,
        query: &'a Query<QueryDataType>,
    ) -> ScopedQueryToken<'a, QueryDataType> {
        ScopedQueryToken::<QueryDataType>::new(self, query)
    }

    pub fn query_timestamp<'a>(
        &'a self,
        query: &'a DurationQueryHelper,
    ) -> TimeStampQueryToken<'a> {
        TimeStampQueryToken::new(query, self)
    }

    pub fn bind_sparse_resource_memory(
        &self,
        attribs: &diligent_sys::BindSparseResourceMemoryAttribs,
    ) {
        unsafe_member_call!(
            self.0,
            DeviceContext,
            BindSparseResourceMemory,
            std::ptr::from_ref(attribs)
        )
    }

    pub fn device_wait_for_fence(&self, fence: &Fence, value: u64) {
        unsafe_member_call!(
            self.0,
            DeviceContext,
            DeviceWaitForFence,
            fence.sys_ptr(),
            value
        )
    }
}

#[repr(transparent)]
pub struct DeferredDeviceContext(DeviceContext);

impl Deref for DeferredDeviceContext {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DeferredDeviceContext {
    pub fn begin(&self, immediate_context_id: u32) {
        unsafe_member_call!(self.0, DeviceContext, Begin, immediate_context_id)
    }

    pub fn finish_command_list(&self) -> Result<Boxed<CommandList>, ()> {
        let mut command_list_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self.0,
            DeviceContext,
            FinishCommandList,
            std::ptr::addr_of_mut!(command_list_ptr)
        );

        if command_list_ptr.is_null() {
            Err(())
        } else {
            Ok(Boxed::<CommandList>::new(command_list_ptr as _))
        }
    }
}
