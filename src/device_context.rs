use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{
    Boxed, BoxedFromNulError, CommandQueueType, DeviceMemory, Ported, PrimitiveTopology,
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
    texture::{Texture, TextureSubResource},
    texture_view::TextureView,
    tlas::{HitGroupBindingMode, TLASBuildInstanceData, TopLevelAS},
};

#[repr(transparent)]
pub struct DeviceContextDesc(diligent_sys::DeviceContextDesc);

impl DeviceContextDesc {
    pub fn queue_type(&self) -> CommandQueueType {
        CommandQueueType::from_bits_retain(self.0.QueueType)
    }
    pub fn is_deferred(&self) -> bool {
        self.0.IsDeferred
    }
    pub fn context_id(&self) -> u8 {
        self.0.ContextId
    }
    pub fn queue_id(&self) -> u8 {
        self.0.QueueId
    }
    pub fn texture_copy_granularity(&self) -> &[u32; 3usize] {
        &self.0.TextureCopyGranularity
    }
}

#[repr(transparent)]
pub struct DeviceContextCommandCounters(diligent_sys::DeviceContextCommandCounters);

impl DeviceContextCommandCounters {
    pub fn set_pipeline_state(&self) -> u32 {
        self.0.SetPipelineState
    }
    pub fn commit_shader_resources(&self) -> u32 {
        self.0.CommitShaderResources
    }
    pub fn set_vertex_buffers(&self) -> u32 {
        self.0.SetVertexBuffers
    }
    pub fn set_index_buffer(&self) -> u32 {
        self.0.SetIndexBuffer
    }
    pub fn set_render_targets(&self) -> u32 {
        self.0.SetRenderTargets
    }
    pub fn set_blend_factors(&self) -> u32 {
        self.0.SetBlendFactors
    }
    pub fn set_stencil_ref(&self) -> u32 {
        self.0.SetStencilRef
    }
    pub fn set_viewports(&self) -> u32 {
        self.0.SetViewports
    }
    pub fn set_scissor_rects(&self) -> u32 {
        self.0.SetScissorRects
    }
    pub fn clear_render_target(&self) -> u32 {
        self.0.ClearRenderTarget
    }
    pub fn clear_depth_stencil(&self) -> u32 {
        self.0.ClearDepthStencil
    }
    pub fn draw(&self) -> u32 {
        self.0.Draw
    }
    pub fn draw_indexed(&self) -> u32 {
        self.0.DrawIndexed
    }
    pub fn draw_indirect(&self) -> u32 {
        self.0.DrawIndirect
    }
    pub fn draw_indexed_indirect(&self) -> u32 {
        self.0.DrawIndexedIndirect
    }
    pub fn multi_draw(&self) -> u32 {
        self.0.MultiDraw
    }
    pub fn multi_draw_indexed(&self) -> u32 {
        self.0.MultiDrawIndexed
    }
    pub fn dispatch_compute(&self) -> u32 {
        self.0.DispatchCompute
    }
    pub fn dispatch_compute_indirect(&self) -> u32 {
        self.0.DispatchComputeIndirect
    }
    pub fn dispatch_tile(&self) -> u32 {
        self.0.DispatchTile
    }
    pub fn draw_mesh(&self) -> u32 {
        self.0.DrawMesh
    }
    pub fn draw_mesh_indirect(&self) -> u32 {
        self.0.DrawMeshIndirect
    }
    pub fn build_blas(&self) -> u32 {
        self.0.BuildBLAS
    }
    pub fn build_tlas(&self) -> u32 {
        self.0.BuildTLAS
    }
    pub fn copy_blas(&self) -> u32 {
        self.0.CopyBLAS
    }
    pub fn copy_tlas(&self) -> u32 {
        self.0.CopyTLAS
    }
    pub fn write_blas_compacted_size(&self) -> u32 {
        self.0.WriteBLASCompactedSize
    }
    pub fn write_tlas_compacted_size(&self) -> u32 {
        self.0.WriteTLASCompactedSize
    }
    pub fn trace_rays(&self) -> u32 {
        self.0.TraceRays
    }
    pub fn trace_rays_indirect(&self) -> u32 {
        self.0.TraceRaysIndirect
    }
    pub fn update_sbt(&self) -> u32 {
        self.0.UpdateSBT
    }
    pub fn update_buffer(&self) -> u32 {
        self.0.UpdateBuffer
    }
    pub fn copy_buffer(&self) -> u32 {
        self.0.CopyBuffer
    }
    pub fn map_buffer(&self) -> u32 {
        self.0.MapBuffer
    }
    pub fn update_texture(&self) -> u32 {
        self.0.UpdateTexture
    }
    pub fn copy_texture(&self) -> u32 {
        self.0.CopyTexture
    }
    pub fn map_texture_subresource(&self) -> u32 {
        self.0.MapTextureSubresource
    }
    pub fn begin_query(&self) -> u32 {
        self.0.BeginQuery
    }
    pub fn generate_mips(&self) -> u32 {
        self.0.GenerateMips
    }
    pub fn resolve_texture_subresource(&self) -> u32 {
        self.0.ResolveTextureSubresource
    }
    pub fn bind_sparse_resource_memory(&self) -> u32 {
        self.0.BindSparseResourceMemory
    }
}

#[repr(transparent)]
pub struct DeviceContextStats(diligent_sys::DeviceContextStats);

impl DeviceContextStats {
    pub fn primitive_counts(&self, primitive_topology: PrimitiveTopology) -> u32 {
        let primitive_topology = diligent_sys::PRIMITIVE_TOPOLOGY::from(primitive_topology);
        unsafe {
            *self
                .0
                .PrimitiveCounts
                .get_unchecked(primitive_topology as usize)
        }
    }
    pub fn command_counters(&self) -> &DeviceContextCommandCounters {
        unsafe { std::mem::transmute(&self.0.CommandCounters) }
    }
}

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
pub struct DrawIndirectAttribs<'attibs_buffer>(
    diligent_sys::DrawIndirectAttribs,
    PhantomData<&'attibs_buffer ()>,
);

#[bon::bon]
impl<'attibs_buffer> DrawIndirectAttribs<'attibs_buffer> {
    #[builder]
    pub fn new(
        attribs_buffer: &'attibs_buffer Buffer,

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
pub struct DrawIndexedIndirectAttribs<'attibs_buffer, 'counter_buffer>(
    diligent_sys::DrawIndexedIndirectAttribs,
    PhantomData<(&'attibs_buffer (), &'counter_buffer ())>,
);

#[bon::bon]
impl<'attibs_buffer, 'counter_buffer> DrawIndexedIndirectAttribs<'attibs_buffer, 'counter_buffer> {
    #[builder]
    pub fn new(
        index_type: ValueType,

        attribs_buffer: &'attibs_buffer Buffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] draw_count: u32,

        #[builder(default = 20)] draw_args_stride: u32,

        #[builder(default = ResourceStateTransitionMode::None)]
        attribs_buffer_state_transition_mode: ResourceStateTransitionMode,

        counter_buffer: Option<&'counter_buffer Buffer>,

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
pub struct DrawMeshIndirectAttribs<'attibs_buffer, 'counter_buffer>(
    diligent_sys::DrawMeshIndirectAttribs,
    PhantomData<(&'attibs_buffer (), &'counter_buffer ())>,
);

#[bon::bon]
impl<'attibs_buffer, 'counter_buffer> DrawMeshIndirectAttribs<'attibs_buffer, 'counter_buffer> {
    #[builder]
    pub fn new(
        attribs_buffer: &'attibs_buffer Buffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] command_count: u32,

        #[builder(default = ResourceStateTransitionMode::None)]
        attribs_buffer_state_transition_mode: ResourceStateTransitionMode,

        counter_buffer: Option<&'counter_buffer Buffer>,

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

#[repr(transparent)]
pub struct MultiDrawItem(diligent_sys::MultiDrawItem);

#[bon::bon]
impl MultiDrawItem {
    #[builder]
    pub fn new(num_vertices: u32, start_vertex_location: u32) -> Self {
        Self(diligent_sys::MultiDrawItem {
            NumVertices: num_vertices,
            StartVertexLocation: start_vertex_location,
        })
    }
}

#[repr(transparent)]
pub struct MultiDrawAttribs<'draw_items>(
    diligent_sys::MultiDrawAttribs,
    PhantomData<&'draw_items ()>,
);

#[bon::bon]
impl<'draw_items> MultiDrawAttribs<'draw_items> {
    #[builder]
    pub fn new(
        draw_items: &'draw_items [MultiDrawItem],

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] num_instances: u32,

        #[builder(default = 0)] first_instance_location: u32,
    ) -> Self {
        Self(
            diligent_sys::MultiDrawAttribs {
                DrawCount: draw_items.len() as u32,
                pDrawItems: draw_items.first().map_or(std::ptr::null(), |item| &item.0),
                Flags: flags.bits(),
                NumInstances: num_instances,
                FirstInstanceLocation: first_instance_location,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct MultiDrawIndexedItem(diligent_sys::MultiDrawIndexedItem);
#[bon::bon]
impl MultiDrawIndexedItem {
    #[builder]
    pub fn new(num_vertices: u32, first_index_location: u32, base_vertex: u32) -> Self {
        Self(diligent_sys::MultiDrawIndexedItem {
            NumIndices: num_vertices,
            FirstIndexLocation: first_index_location,
            BaseVertex: base_vertex,
        })
    }
}

#[repr(transparent)]
pub struct MultiDrawIndexedAttribs<'draw_items>(
    diligent_sys::MultiDrawIndexedAttribs,
    PhantomData<&'draw_items ()>,
);

#[bon::bon]
impl<'draw_items> MultiDrawIndexedAttribs<'draw_items> {
    #[builder]
    pub fn new(
        draw_items: &'draw_items [MultiDrawIndexedItem],

        index_type: ValueType,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] num_instances: u32,

        #[builder(default = 0)] first_instance_location: u32,
    ) -> Self {
        Self(
            diligent_sys::MultiDrawIndexedAttribs {
                DrawCount: draw_items.len() as u32,
                pDrawItems: draw_items.first().map_or(std::ptr::null(), |item| &item.0),
                IndexType: index_type.into(),
                Flags: flags.bits(),
                NumInstances: num_instances,
                FirstInstanceLocation: first_instance_location,
            },
            PhantomData,
        )
    }
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
pub struct DispatchComputeIndirectAttribs<'attibs_buffer>(
    diligent_sys::DispatchComputeIndirectAttribs,
    PhantomData<&'attibs_buffer ()>,
);

#[bon::bon]
impl<'attibs_buffer> DispatchComputeIndirectAttribs<'attibs_buffer> {
    #[builder]
    pub fn new(
        attribs_buffer: &'attibs_buffer Buffer,

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
pub struct BLASBuildBoundingBoxData<'name, 'buffer>(
    diligent_sys::BLASBuildBoundingBoxData,
    PhantomData<(&'name (), &'buffer ())>,
);
#[bon::bon]
impl<'name, 'buffer> BLASBuildBoundingBoxData<'name, 'buffer> {
    #[builder]
    pub fn new(
        geometry_name: &'name CStr,

        box_buffer: &'buffer Buffer,

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
pub struct BLASBuildTriangleData<'geometry_name, 'vertex_buffer, 'index_buffer, 'transform_buffer>(
    diligent_sys::BLASBuildTriangleData,
    PhantomData<(
        &'geometry_name (),
        &'vertex_buffer (),
        &'index_buffer (),
        &'transform_buffer (),
    )>,
);
#[bon::bon]
impl<'geometry_name, 'vertex_buffer, 'index_buffer, 'transform_buffer>
    BLASBuildTriangleData<'geometry_name, 'vertex_buffer, 'index_buffer, 'transform_buffer>
{
    #[builder]
    pub fn new(
        geometry_name: &'geometry_name CStr,

        vertex_buffer: &'vertex_buffer Buffer,

        #[builder(default = 0)] vertex_offset: u64,

        vertex_stride: u32,

        vertex_count: usize,

        vertex_value_type: Option<ValueType>,

        #[builder(default = 0)] vertex_component_count: u8,

        primitive_count: usize,

        index_buffer: Option<&'index_buffer Buffer>,

        #[builder(default = 0)] index_offset: u64,

        index_type: Option<ValueType>,

        transform_buffer: Option<&'transform_buffer Buffer>,

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
pub struct BuildBLASAttribs<
    'blas,
    'scratch_buffer,
    'triangles,
    'triangles_geometry_name,
    'triangles_vertex_buffer,
    'triangles_index_buffer,
    'triangles_transform_buffer,
    'bounding_boxes,
    'bb_name,
    'bb_buffer,
>(
    diligent_sys::BuildBLASAttribs,
    PhantomData<(
        &'blas (),
        &'scratch_buffer (),
        &'triangles (),
        &'triangles_geometry_name (),
        &'triangles_vertex_buffer (),
        &'triangles_index_buffer (),
        &'triangles_transform_buffer (),
        &'bounding_boxes (),
        &'bb_name (),
        &'bb_buffer (),
    )>,
);
#[bon::bon]
impl<
    'blas,
    'scratch_buffer,
    'triangles,
    'triangles_geometry_name,
    'triangles_vertex_buffer,
    'triangles_index_buffer,
    'triangles_transform_buffer,
    'bounding_boxes,
    'bb_name,
    'bb_buffer,
>
    BuildBLASAttribs<
        'blas,
        'scratch_buffer,
        'triangles,
        'triangles_geometry_name,
        'triangles_vertex_buffer,
        'triangles_index_buffer,
        'triangles_transform_buffer,
        'bounding_boxes,
        'bb_name,
        'bb_buffer,
    >
{
    #[builder]
    pub fn new(
        blas: &'blas BottomLevelAS,

        #[builder(default = ResourceStateTransitionMode::None)]
        blas_transition_mode: ResourceStateTransitionMode,

        #[builder(default = ResourceStateTransitionMode::None)]
        geometry_transition_mode: ResourceStateTransitionMode,

        #[builder(default)] triangle_data: &'triangles [BLASBuildTriangleData<
            'triangles_geometry_name,
            'triangles_vertex_buffer,
            'triangles_index_buffer,
            'triangles_transform_buffer,
        >],

        #[builder(default)] box_data: &'bounding_boxes [BLASBuildBoundingBoxData<
            'bb_name,
            'bb_buffer,
        >],

        scratch_buffer: &'scratch_buffer Buffer,

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
                pTriangleData: triangle_data
                    .first()
                    .map_or(std::ptr::null(), |triangle| &triangle.0),
                TriangleDataCount: triangle_data.len() as u32,
                pBoxData: box_data
                    .first()
                    .map_or(std::ptr::null(), |box_data| &box_data.0),
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
pub struct BuildTLASAttribs<
    'tlas,
    'tlas_instance_name,
    'blas,
    'instance_buffer,
    'scratch_buffer,
    'instances,
>(
    pub(crate) diligent_sys::BuildTLASAttribs,
    PhantomData<(
        &'tlas (),
        &'tlas_instance_name (),
        &'blas (),
        &'instance_buffer (),
        &'scratch_buffer (),
        &'instances (),
    )>,
);
#[bon::bon]
impl<'tlas, 'tlas_instance_name, 'blas, 'instance_buffer, 'scratch_buffer, 'instances>
    BuildTLASAttribs<
        'tlas,
        'tlas_instance_name,
        'blas,
        'instance_buffer,
        'scratch_buffer,
        'instances,
    >
{
    #[builder]
    pub fn new(
        tlas: &'tlas TopLevelAS,

        #[builder(default = ResourceStateTransitionMode::None)]
        tlas_transition_mode: ResourceStateTransitionMode,

        #[builder(default = ResourceStateTransitionMode::None)]
        blas_transition_mode: ResourceStateTransitionMode,

        instances: &'instances [TLASBuildInstanceData<'tlas_instance_name, 'blas>],

        instance_buffer: &'instance_buffer Buffer,

        #[builder(default = 0)] instance_buffer_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        instance_buffer_transition_mode: ResourceStateTransitionMode,

        #[builder(default = 1)] hit_group_stride: u32,

        #[builder(default = 0)] base_contribution_to_hit_group_index: u32,

        #[builder(default = HitGroupBindingMode::PerGeometry)] binding_mode: HitGroupBindingMode,

        scratch_buffer: &'scratch_buffer Buffer,

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
                pInstances: instances
                    .first()
                    .map_or(std::ptr::null(), |instance| &instance.0),
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

#[repr(transparent)]
pub struct UpdateIndirectRTBufferAttribs<'buffer>(
    diligent_sys::UpdateIndirectRTBufferAttribs,
    PhantomData<&'buffer ()>,
);

#[bon::bon]
impl<'buffer> UpdateIndirectRTBufferAttribs<'buffer> {
    #[builder]
    pub fn new(
        attribs_buffer: &'buffer Buffer,

        #[builder(default = 0)] attribs_buffer_offset: u64,

        #[builder(default = ResourceStateTransitionMode::None)]
        transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        Self(
            diligent_sys::UpdateIndirectRTBufferAttribs {
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                AttribsBufferOffset: attribs_buffer_offset,
                TransitionMode: transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct TraceRaysAttribs<'shader_binding_table>(
    diligent_sys::TraceRaysAttribs,
    PhantomData<&'shader_binding_table ()>,
);
#[bon::bon]
impl<'shader_binding_table> TraceRaysAttribs<'shader_binding_table> {
    #[builder]
    pub fn new(
        sbt: &'shader_binding_table ShaderBindingTable,

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
pub struct TraceRaysIndirectAttribs<'shader_binding_table, 'attribs_buffer>(
    diligent_sys::TraceRaysIndirectAttribs,
    PhantomData<(&'shader_binding_table (), &'attribs_buffer ())>,
);
#[bon::bon]
impl<'shader_binding_table, 'attribs_buffer>
    TraceRaysIndirectAttribs<'shader_binding_table, 'attribs_buffer>
{
    #[builder]
    pub fn new(
        sbt: &'shader_binding_table ShaderBindingTable,
        attribs_buffer: &'attribs_buffer Buffer,
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

#[repr(transparent)]
pub struct CopyTextureAttribs<'src_texture, 'dst_texture, 'region>(
    diligent_sys::CopyTextureAttribs,
    PhantomData<(&'src_texture (), &'dst_texture (), &'region ())>,
);

#[bon::bon]
impl<'src_texture, 'dst_texture, 'region> CopyTextureAttribs<'src_texture, 'dst_texture, 'region> {
    #[builder]
    pub fn new(
        src_texture: &'src_texture Texture,
        src_mip_level: u32,
        src_slice: u32,
        src_box: &'region crate::Box,
        src_texture_transition_mode: ResourceStateTransitionMode,
        dst_texture: &'dst_texture Texture,
        dst_mip_level: u32,
        dst_slice: u32,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        dst_texture_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        CopyTextureAttribs(
            diligent_sys::CopyTextureAttribs {
                pSrcTexture: src_texture.sys_ptr(),
                SrcMipLevel: src_mip_level,
                SrcSlice: src_slice,
                pSrcBox: &src_box.0,
                SrcTextureTransitionMode: src_texture_transition_mode.into(),
                pDstTexture: dst_texture.sys_ptr(),
                DstMipLevel: dst_mip_level,
                DstSlice: dst_slice,
                DstX: dst_x,
                DstY: dst_y,
                DstZ: dst_z,
                DstTextureTransitionMode: dst_texture_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct ResolveTextureSubresourceAttribs(diligent_sys::ResolveTextureSubresourceAttribs);

#[bon::bon]
impl ResolveTextureSubresourceAttribs {
    #[builder]
    pub fn new(
        src_mip_level: u32,
        src_slice: u32,
        src_texture_transition_mode: ResourceStateTransitionMode,
        dst_mip_level: u32,
        dst_slice: u32,
        dst_texture_transition_mode: ResourceStateTransitionMode,
        format: TextureFormat,
    ) -> Self {
        Self(diligent_sys::ResolveTextureSubresourceAttribs {
            SrcMipLevel: src_mip_level,
            SrcSlice: src_slice,
            SrcTextureTransitionMode: src_texture_transition_mode.into(),
            DstMipLevel: dst_mip_level,
            DstSlice: dst_slice,
            DstTextureTransitionMode: dst_texture_transition_mode.into(),
            Format: format.into(),
        })
    }
}

#[repr(transparent)]
pub struct WriteBLASCompactedSizeAttribs<'blas, 'buffer>(
    diligent_sys::WriteBLASCompactedSizeAttribs,
    PhantomData<(&'blas (), &'buffer ())>,
);

#[bon::bon]
impl<'blas, 'buffer> WriteBLASCompactedSizeAttribs<'blas, 'buffer> {
    #[builder]
    pub fn new(
        blas: &'blas BottomLevelAS,
        dest_buffer: &'buffer Buffer,
        dest_buffer_offset: u64,
        blas_transition_mode: ResourceStateTransitionMode,
        buffer_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        Self(
            diligent_sys::WriteBLASCompactedSizeAttribs {
                pBLAS: blas.sys_ptr(),
                pDestBuffer: dest_buffer.sys_ptr(),
                DestBufferOffset: dest_buffer_offset,
                BLASTransitionMode: blas_transition_mode.into(),
                BufferTransitionMode: buffer_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct WriteTLASCompactedSizeAttribs<'tlas, 'buffer>(
    diligent_sys::WriteTLASCompactedSizeAttribs,
    PhantomData<(&'tlas (), &'buffer ())>,
);

#[bon::bon]
impl<'tlas, 'buffer> WriteTLASCompactedSizeAttribs<'tlas, 'buffer> {
    #[builder]
    pub fn new(
        tlas: &'tlas TopLevelAS,
        dest_buffer: &'buffer Buffer,
        dest_buffer_offset: u64,
        tlas_transition_mode: ResourceStateTransitionMode,
        buffer_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        Self(
            diligent_sys::WriteTLASCompactedSizeAttribs {
                pTLAS: tlas.sys_ptr(),
                pDestBuffer: dest_buffer.sys_ptr(),
                DestBufferOffset: dest_buffer_offset,
                TLASTransitionMode: tlas_transition_mode.into(),
                BufferTransitionMode: buffer_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct CopyBLASAttribs<'src_blas, 'dst_blas>(
    diligent_sys::CopyBLASAttribs,
    PhantomData<(&'src_blas (), &'dst_blas ())>,
);

#[bon::bon]
impl<'src_blas, 'dst_blas> CopyBLASAttribs<'src_blas, 'dst_blas> {
    #[builder]
    pub fn new(
        src: &'src_blas BottomLevelAS,
        dst: &'dst_blas BottomLevelAS,
        mode: CopyAsMode,
        src_transition_mode: ResourceStateTransitionMode,
        dst_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        CopyBLASAttribs(
            diligent_sys::CopyBLASAttribs {
                pSrc: src.sys_ptr(),
                pDst: dst.sys_ptr(),
                Mode: mode.into(),
                SrcTransitionMode: src_transition_mode.into(),
                DstTransitionMode: dst_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct CopyTLASAttribs<'src_tlas, 'dst_tlas>(
    diligent_sys::CopyTLASAttribs,
    PhantomData<(&'src_tlas (), &'dst_tlas ())>,
);

#[bon::bon]
impl<'src_tlas, 'dst_tlas> CopyTLASAttribs<'src_tlas, 'dst_tlas> {
    #[builder]
    pub fn new(
        src: &'src_tlas TopLevelAS,
        dst: &'dst_tlas TopLevelAS,
        mode: CopyAsMode,
        src_transition_mode: ResourceStateTransitionMode,
        dst_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        CopyTLASAttribs(
            diligent_sys::CopyTLASAttribs {
                pSrc: src.sys_ptr(),
                pDst: dst.sys_ptr(),
                Mode: mode.into(),
                SrcTransitionMode: src_transition_mode.into(),
                DstTransitionMode: dst_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[derive(Clone, Copy)]
pub enum CopyAsMode {
    Clone,
    Compact,
}

const_assert_eq!(diligent_sys::COPY_AS_MODE_LAST, 1);

impl From<CopyAsMode> for diligent_sys::COPY_AS_MODE {
    fn from(value: CopyAsMode) -> Self {
        (match value {
            CopyAsMode::Clone => diligent_sys::COPY_AS_MODE_CLONE,
            CopyAsMode::Compact => diligent_sys::COPY_AS_MODE_COMPACT,
        }) as _
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

#[repr(transparent)]
pub struct Viewport(diligent_sys::Viewport);

#[bon::bon]
impl Viewport {
    #[builder]
    pub fn new(
        top_left_x: f32,
        top_left_y: f32,
        width: f32,
        height: f32,
        #[builder(default = 0.0)] min_depth: f32,
        #[builder(default = 1.0)] max_depth: f32,
    ) -> Self {
        Self(diligent_sys::Viewport {
            TopLeftX: top_left_x,
            TopLeftY: top_left_y,
            Width: width,
            Height: height,
            MinDepth: min_depth,
            MaxDepth: max_depth,
        })
    }
}

#[repr(transparent)]
pub struct Rect(diligent_sys::Rect);

#[bon::bon]
impl Rect {
    #[builder]
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self(diligent_sys::Rect {
            left,
            top,
            right,
            bottom,
        })
    }
}

impl Rect {
    pub fn is_valid(&self) -> bool {
        self.0.right > self.0.left && self.0.bottom > self.0.top
    }
}

pub struct ScopedDebugGroup<'context> {
    device_context: &'context DeviceContext,
}

impl<'context> ScopedDebugGroup<'context> {
    fn new(device_context: &'context DeviceContext, name: &CStr, color: Option<[f32; 4]>) -> Self {
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

impl Drop for ScopedDebugGroup<'_> {
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
pub struct StateTransitionDesc<'resource>(
    diligent_sys::StateTransitionDesc,
    PhantomData<&'resource ()>,
);
#[bon::bon]
impl<'resource> StateTransitionDesc<'resource> {
    #[builder(derive(Clone))]
    pub fn new(
        resource: &'resource DeviceObject,

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

define_ported!(CommandList, diligent_sys::ICommandList);

#[repr(transparent)]
pub struct DepthStencilClearValue(pub(crate) diligent_sys::DepthStencilClearValue);

#[bon::bon]
impl DepthStencilClearValue {
    #[builder]
    pub fn new(depth: f32, stencil: u8) -> Self {
        Self(diligent_sys::DepthStencilClearValue {
            Depth: depth,
            Stencil: stencil,
        })
    }
}

#[repr(transparent)]
pub struct OptimizedClearValue(pub(crate) diligent_sys::OptimizedClearValue);

#[bon::bon]
impl OptimizedClearValue {
    #[builder]
    pub fn new(
        format: TextureFormat,
        color: [f32; 4usize],
        depth_stencil: DepthStencilClearValue,
    ) -> Self {
        Self(diligent_sys::OptimizedClearValue {
            Format: format.into(),
            Color: color,
            DepthStencil: depth_stencil.0,
        })
    }
}

#[repr(transparent)]
pub struct BeginRenderPassAttribs<'render_pass, 'frame_buffer, 'clear_values>(
    diligent_sys::BeginRenderPassAttribs,
    PhantomData<(&'render_pass (), &'frame_buffer (), &'clear_values ())>,
);

#[bon::bon]
impl<'render_pass, 'frame_buffer, 'clear_values>
    BeginRenderPassAttribs<'render_pass, 'frame_buffer, 'clear_values>
{
    #[builder]
    pub fn new(
        render_pass: &'render_pass RenderPass,
        frame_buffer: &'frame_buffer Framebuffer,
        clear_values: &'clear_values [OptimizedClearValue],
        state_transition_mode: ResourceStateTransitionMode,
    ) -> Self {
        Self(
            diligent_sys::BeginRenderPassAttribs {
                pRenderPass: render_pass.sys_ptr(),
                pFramebuffer: frame_buffer.sys_ptr(),
                ClearValueCount: clear_values.len() as u32,
                pClearValues: clear_values.first().map_or(std::ptr::null_mut(), |value| {
                    std::ptr::from_ref(&value.0) as *mut _
                }),
                StateTransitionMode: state_transition_mode.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct SparseBufferMemoryBindRange(diligent_sys::SparseBufferMemoryBindRange);

#[bon::bon]
impl SparseBufferMemoryBindRange {
    #[builder]
    pub fn new(
        buffer_offset: u64,
        memory_offset: u64,
        memory_size: u64,
        memory: &DeviceMemory,
    ) -> Self {
        Self(diligent_sys::SparseBufferMemoryBindRange {
            BufferOffset: buffer_offset,
            MemoryOffset: memory_offset,
            MemorySize: memory_size,
            pMemory: memory.sys_ptr(),
        })
    }
}

#[repr(transparent)]
pub struct SparseTextureMemoryBindRange(diligent_sys::SparseTextureMemoryBindRange);

#[bon::bon]
impl SparseTextureMemoryBindRange {
    #[builder]
    pub fn new(
        mip_level: u32,
        array_slice: u32,
        region: crate::Box,
        offset_in_mip_tail: u64,
        memory_size: u64,
        memory_offset: u64,
        memory: &DeviceMemory,
    ) -> Self {
        Self(diligent_sys::SparseTextureMemoryBindRange {
            MipLevel: mip_level,
            ArraySlice: array_slice,
            Region: region.0,
            OffsetInMipTail: offset_in_mip_tail,
            MemorySize: memory_size,
            MemoryOffset: memory_offset,
            pMemory: memory.sys_ptr(),
        })
    }
}

#[repr(transparent)]
pub struct SparseBufferMemoryBindInfo(diligent_sys::SparseBufferMemoryBindInfo);

#[bon::bon]
impl SparseBufferMemoryBindInfo {
    #[builder]
    pub fn new(buffer: &Buffer, ranges: &[SparseBufferMemoryBindRange]) -> Self {
        Self(diligent_sys::SparseBufferMemoryBindInfo {
            pBuffer: buffer.sys_ptr(),
            pRanges: ranges.first().map_or(std::ptr::null(), |r| &r.0),
            NumRanges: ranges.len() as u32,
        })
    }
}

#[repr(transparent)]
pub struct SparseTextureMemoryBindInfo(diligent_sys::SparseTextureMemoryBindInfo);

#[bon::bon]
impl SparseTextureMemoryBindInfo {
    #[builder]
    pub fn new(texture: &Texture, ranges: &[SparseTextureMemoryBindRange]) -> Self {
        Self(diligent_sys::SparseTextureMemoryBindInfo {
            pTexture: texture.sys_ptr(),
            pRanges: ranges.first().map_or(std::ptr::null(), |r| &r.0),
            NumRanges: ranges.len() as u32,
        })
    }
}

#[repr(transparent)]
pub struct BindSparseResourceMemoryAttribs(diligent_sys::BindSparseResourceMemoryAttribs);

#[bon::bon]
impl BindSparseResourceMemoryAttribs {
    #[builder]
    pub fn new(
        buffer_binds: &[SparseBufferMemoryBindInfo],
        texture_binds: &[SparseTextureMemoryBindInfo],
        wait_fences: &[&Fence],
        wait_fence_values: &[u64],
        signal_fences: &[&Fence],
        signal_fence_values: &[u64],
    ) -> Self {
        Self(diligent_sys::BindSparseResourceMemoryAttribs {
            pBufferBinds: buffer_binds
                .first()
                .map_or(std::ptr::null(), |binds| &binds.0),
            NumBufferBinds: buffer_binds.len() as u32,
            pTextureBinds: texture_binds
                .first()
                .map_or(std::ptr::null(), |binds| &binds.0),
            NumTextureBinds: texture_binds.len() as u32,
            ppWaitFences: unsafe {
                std::mem::transmute::<&[&Fence], &[*mut diligent_sys::IFence]>(wait_fences)
            }
            .first()
            .map_or(std::ptr::null_mut(), |&fence| {
                std::ptr::addr_of!(fence) as *mut _
            }),
            pWaitFenceValues: wait_fence_values
                .first()
                .map_or(std::ptr::null_mut(), std::ptr::from_ref),
            NumWaitFences: wait_fences.len() as u32,
            ppSignalFences: unsafe {
                std::mem::transmute::<&[&Fence], &[*mut diligent_sys::IFence]>(signal_fences)
            }
            .first()
            .map_or(std::ptr::null_mut(), |&fence| {
                std::ptr::addr_of!(fence) as *mut _
            }),
            pSignalFenceValues: signal_fence_values
                .first()
                .map_or(std::ptr::null_mut(), std::ptr::from_ref),
            NumSignalFences: signal_fences.len() as u32,
        })
    }
}

pub struct RenderPassToken<'context> {
    context: &'context DeviceContext,
}

impl<'context> RenderPassToken<'context> {
    pub fn new(context: &'context DeviceContext, attribs: &BeginRenderPassAttribs) -> Self {
        unsafe_member_call!(context, DeviceContext, BeginRenderPass, &attribs.0);

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

pub struct GraphicsPipelineToken<'context> {
    context: &'context DeviceContext,
}

impl GraphicsPipelineToken<'_> {
    pub fn draw(&self, attribs: &DrawAttribs) {
        unsafe_member_call!(self.context, DeviceContext, Draw, &attribs.0)
    }

    pub fn draw_indexed(&self, attribs: &DrawIndexedAttribs) {
        unsafe_member_call!(self.context, DeviceContext, DrawIndexed, &attribs.0)
    }

    pub fn draw_indirect(&self, attribs: &DrawIndirectAttribs) {
        unsafe_member_call!(self.context, DeviceContext, DrawIndirect, &attribs.0)
    }

    pub fn draw_indexed_indirect(&self, attribs: &DrawIndexedIndirectAttribs) {
        unsafe_member_call!(self.context, DeviceContext, DrawIndexedIndirect, &attribs.0)
    }

    pub fn multi_draw(&self, attribs: &MultiDrawAttribs) {
        unsafe_member_call!(self.context, DeviceContext, MultiDraw, &attribs.0)
    }

    pub fn multi_draw_indexed(&self, attribs: &MultiDrawIndexedAttribs) {
        unsafe_member_call!(self.context, DeviceContext, MultiDrawIndexed, &attribs.0)
    }
}

pub struct MeshPipelineToken<'context> {
    context: &'context DeviceContext,
}

impl MeshPipelineToken<'_> {
    pub fn draw_mesh(&self, attribs: &DrawMeshAttribs) {
        unsafe_member_call!(self.context, DeviceContext, DrawMesh, &attribs.0)
    }

    pub fn draw_mesh_indirect(&self, attribs: &DrawMeshIndirectAttribs) {
        unsafe_member_call!(self.context, DeviceContext, DrawMeshIndirect, &attribs.0)
    }
}

pub struct ComputePipelineToken<'context> {
    context: &'context DeviceContext,
}

impl ComputePipelineToken<'_> {
    pub fn dispatch_compute(&self, attribs: &DispatchComputeAttribs) {
        unsafe_member_call!(self.context, DeviceContext, DispatchCompute, &attribs.0)
    }

    pub fn dispatch_compute_indirect(&self, attribs: &DispatchComputeIndirectAttribs) {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            DispatchComputeIndirect,
            &attribs.0
        )
    }
}

pub struct TilePipelineToken<'context> {
    context: &'context DeviceContext,
}

impl TilePipelineToken<'_> {
    pub fn dispatch_tile(&self, attribs: &DispatchTileAttribs) {
        unsafe_member_call!(self.context, DeviceContext, DispatchTile, &attribs.0)
    }
}

pub struct RayTracingPipelineToken<'context> {
    context: &'context DeviceContext,
}

impl RayTracingPipelineToken<'_> {
    pub fn trace_rays(&self, attribs: &TraceRaysAttribs) {
        unsafe_member_call!(self.context, DeviceContext, TraceRays, &attribs.0)
    }

    pub fn trace_rays_indirect(&self, attribs: &TraceRaysIndirectAttribs) {
        unsafe_member_call!(self.context, DeviceContext, TraceRaysIndirect, &attribs.0)
    }
}

define_ported!(
    DeviceContext,
    diligent_sys::IDeviceContext,
    diligent_sys::IDeviceContextMethods : 72,
    Object
);

impl DeviceContext {
    pub fn desc(&self) -> &DeviceContextDesc {
        let desc_ptr = unsafe_member_call!(self, DeviceContext, GetDesc);
        unsafe { &*(desc_ptr as *const DeviceContextDesc) }
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
            buffer_pointers.as_ptr(),
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
        viewports: &[Viewport],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetViewports,
            viewports.len() as u32,
            viewports
                .first()
                .map_or(std::ptr::null(), |viewport| &viewport.0),
            render_target_width,
            render_target_height
        )
    }

    pub fn set_scissor_rects(
        &self,
        rects: &[Rect],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetScissorRects,
            rects.len() as u32,
            rects.first().map_or(std::ptr::null(), |rect| &rect.0),
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
            &mut tile_size_x,
            &mut tile_size_y
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
            rgba.as_ptr() as *const std::os::raw::c_void,
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

    pub fn map_buffer_read<'buffer, T: Sized>(
        &self,
        buffer: &'buffer Buffer,
        map_flags: MapFlags,
    ) -> BufferMapReadToken<'_, 'buffer, T>
    where
        Self: Sized,
    {
        BufferMapReadToken::new(self, buffer, map_flags.bits())
    }

    pub fn map_buffer_write<'buffer, T: Sized>(
        &self,
        buffer: &'buffer Buffer,
        map_flags: MapFlags,
    ) -> BufferMapWriteToken<'_, 'buffer, T>
    where
        Self: Sized,
    {
        BufferMapWriteToken::new(self, buffer, map_flags.bits())
    }

    pub fn map_buffer_read_write<'buffer, T: Sized>(
        &self,
        buffer: &'buffer Buffer,
        map_flags: MapFlags,
    ) -> BufferMapReadWriteToken<'_, 'buffer, T>
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
        dst_box: &crate::Box,
        subres_data: &TextureSubResource,
        src_buffer_transition_mode: ResourceStateTransitionMode,
        texture_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateTexture,
            texture.sys_ptr(),
            mip_level,
            slice,
            &dst_box.0,
            &subres_data.0,
            src_buffer_transition_mode.into(),
            texture_transition_mode.into()
        )
    }

    pub fn copy_texture(&self, copy_attribs: &CopyTextureAttribs) {
        unsafe_member_call!(self, DeviceContext, CopyTexture, &copy_attribs.0)
    }

    #[cfg(any(feature = "d3d11", feature = "d3d12", feature = "vulkan"))]
    pub fn map_texture_subresource_read<'texture, T>(
        &self,
        texture: &'texture Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<crate::Box>,
    ) -> crate::texture::TextureMapReadToken<'_, 'texture, T> {
        crate::texture::TextureMapReadToken::new(
            self,
            texture,
            mip_level,
            array_slice,
            map_flags,
            map_region,
        )
    }

    #[cfg(any(feature = "d3d11", feature = "d3d12", feature = "vulkan"))]
    pub fn map_texture_subresource_write<'texture, T>(
        &self,
        texture: &'texture Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<crate::Box>,
    ) -> crate::texture::TextureMapWriteToken<'_, 'texture, T> {
        crate::texture::TextureMapWriteToken::new(
            self,
            texture,
            mip_level,
            array_slice,
            map_flags,
            map_region,
        )
    }

    #[cfg(any(feature = "d3d11", feature = "d3d12", feature = "vulkan"))]
    pub fn map_texture_subresource_read_write<'texture, T>(
        &self,
        texture: &'texture Texture,
        mip_level: u32,
        array_slice: u32,
        map_flags: MapFlags,
        map_region: Option<crate::Box>,
    ) -> crate::texture::TextureMapReadWriteToken<'_, 'texture, T> {
        crate::texture::TextureMapReadWriteToken::new(
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
            barriers
                .first()
                .map_or(std::ptr::null(), |barrier| &barrier.0)
        )
    }

    pub fn resolve_texture_subresource(
        &self,
        src_texture: &Texture,
        dst_texture: &mut Texture,
        resolve_attribs: &ResolveTextureSubresourceAttribs,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            ResolveTextureSubresource,
            src_texture.sys_ptr(),
            dst_texture.sys_ptr(),
            &resolve_attribs.0
        )
    }

    pub fn build_blas(&self, attribs: &BuildBLASAttribs) {
        unsafe_member_call!(self, DeviceContext, BuildBLAS, &attribs.0)
    }

    pub fn build_tlas<
        'tlas,
        'tlas_instance_name,
        'blas,
        'instance_buffer,
        'scratch_buffer,
        'instances,
    >(
        &self,
        attribs: &BuildTLASAttribs<
            'tlas,
            'tlas_instance_name,
            'blas,
            'instance_buffer,
            'scratch_buffer,
            'instances,
        >,
    ) {
        unsafe_member_call!(self, DeviceContext, BuildTLAS, &attribs.0)
    }

    pub fn copy_blas(&self, attribs: &CopyBLASAttribs) {
        unsafe_member_call!(self, DeviceContext, CopyBLAS, &attribs.0)
    }

    pub fn copy_tlas(&self, attribs: &CopyTLASAttribs) {
        unsafe_member_call!(self, DeviceContext, CopyTLAS, &attribs.0)
    }

    pub fn write_blas_compacted_size(&self, attribs: &WriteBLASCompactedSizeAttribs) {
        unsafe_member_call!(self, DeviceContext, WriteBLASCompactedSize, &attribs.0)
    }

    pub fn write_tlas_compacted_size(&self, attribs: &WriteTLASCompactedSizeAttribs) {
        unsafe_member_call!(self, DeviceContext, WriteTLASCompactedSize, &attribs.0)
    }

    pub fn update_sbt(
        &self,
        sbt: &ShaderBindingTable,
        attribs: Option<&UpdateIndirectRTBufferAttribs>,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateSBT,
            sbt.sys_ptr(),
            attribs.map_or(std::ptr::null_mut(), |attribs| &attribs.0)
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

    pub fn get_stats(&self) -> &DeviceContextStats {
        let stats = unsafe_member_call!(self, DeviceContext, GetStats);

        unsafe { &*(stats as *const DeviceContextStats) }
    }
}

impl Ported for ImmediateDeviceContext {
    type SysType = diligent_sys::IDeviceContext;
}

impl Ported for DeferredDeviceContext {
    type SysType = diligent_sys::IDeviceContext;
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

    pub fn execute_command_lists(&self, command_lists: &[&CommandList]) {
        unsafe_member_call!(
            self.0,
            DeviceContext,
            ExecuteCommandLists,
            command_lists.len() as u32,
            command_lists.first().map_or(std::ptr::null(), |_| {
                command_lists.as_ptr() as *const *mut _
            })
        )
    }

    pub fn wait_for_idle(&self) {
        unsafe_member_call!(self.0, DeviceContext, WaitForIdle)
    }

    // TODO : make command queue locking RAII
    pub fn lock_command_queue(&self) -> Result<Boxed<CommandQueue>, BoxedFromNulError> {
        Boxed::new(unsafe_member_call!(self.0, DeviceContext, LockCommandQueue))
    }

    pub fn unlock_command_queue(&self) {
        unsafe_member_call!(self.0, DeviceContext, UnlockCommandQueue);
    }

    pub fn begin_query<'query, QueryDataType: GetSysQueryType>(
        &self,
        query: &'query Query<QueryDataType>,
    ) -> ScopedQueryToken<'query, '_, QueryDataType> {
        ScopedQueryToken::<QueryDataType>::new(self, query)
    }

    pub fn query_timestamp<'query>(
        &self,
        query: &'query DurationQueryHelper,
    ) -> TimeStampQueryToken<'query, '_> {
        TimeStampQueryToken::new(query, self)
    }

    pub fn bind_sparse_resource_memory(&self, attribs: &BindSparseResourceMemoryAttribs) {
        unsafe_member_call!(self.0, DeviceContext, BindSparseResourceMemory, &attribs.0)
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

    pub fn finish_command_list(&self) -> Result<Boxed<CommandList>, BoxedFromNulError> {
        let mut command_list_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self.0,
            DeviceContext,
            FinishCommandList,
            &mut command_list_ptr
        );

        Boxed::new(command_list_ptr)
    }
}
