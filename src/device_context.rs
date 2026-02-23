use std::{borrow::Borrow, ffi::CStr, marker::PhantomData, ops::Deref};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{
    Boxed, BoxedFromNulError, CommandQueueType, DeviceMemory, Ported, PrimitiveTopology,
    blas::BottomLevelAS,
    buffer::{Buffer, BufferMapReadToken, BufferMapReadWriteToken, BufferMapWriteToken},
    command_queue::CommandQueue,
    device_object::{DeviceObject, ResourceTransition},
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
#[derive(Clone)]
pub struct DrawAttribs(diligent_sys::DrawAttribs);

#[bon::bon]
impl DrawAttribs {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct DrawIndexedAttribs(diligent_sys::DrawIndexedAttribs);

#[bon::bon]
impl DrawIndexedAttribs {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct DrawIndirectAttribs<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>(
    diligent_sys::DrawIndirectAttribs,
    PhantomData<(
        &'attibs_buffer AttribsBuffer,
        &'counter_buffer CounterBuffer,
    )>,
);

#[bon::bon]
impl<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>
    DrawIndirectAttribs<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>
where
    AttribsBuffer: ResourceTransition<'attibs_buffer, Buffer>,
    CounterBuffer: ResourceTransition<'counter_buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        attribs_buffer: AttribsBuffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] draw_count: u32,

        #[builder(default = 16)] draw_args_stride: u32,

        counter_buffer: Option<(CounterBuffer, u64)>,
    ) -> Self {
        let (counter_buffer, counter_offset) = counter_buffer
            .map_or((std::ptr::null_mut(), 0), |(buffer, offset)| {
                (buffer.sys_ptr(), offset)
            });

        Self(
            diligent_sys::DrawIndirectAttribs {
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                DrawArgsOffset: draw_args_offset,
                Flags: flags.bits(),
                DrawCount: draw_count,
                DrawArgsStride: draw_args_stride,
                AttribsBufferStateTransitionMode: AttribsBuffer::TRANSITION_MODE,
                pCounterBuffer: counter_buffer,
                CounterOffset: counter_offset,
                CounterBufferStateTransitionMode: CounterBuffer::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DrawIndexedIndirectAttribs<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>(
    diligent_sys::DrawIndexedIndirectAttribs,
    PhantomData<(
        &'attibs_buffer AttribsBuffer,
        &'counter_buffer CounterBuffer,
    )>,
);

#[bon::bon]
impl<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>
    DrawIndexedIndirectAttribs<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>
where
    AttribsBuffer: ResourceTransition<'attibs_buffer, Buffer>,
    CounterBuffer: ResourceTransition<'counter_buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        index_type: ValueType,

        attribs_buffer: AttribsBuffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] draw_count: u32,

        #[builder(default = 20)] draw_args_stride: u32,

        counter_buffer: Option<(CounterBuffer, u64)>,
    ) -> Self {
        let (counter_buffer, counter_offset) = counter_buffer
            .map_or((std::ptr::null_mut(), 0), |(buffer, offset)| {
                (buffer.sys_ptr(), offset)
            });
        Self(
            diligent_sys::DrawIndexedIndirectAttribs {
                IndexType: index_type.into(),
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                DrawArgsOffset: draw_args_offset,
                Flags: flags.bits(),
                DrawCount: draw_count,
                DrawArgsStride: draw_args_stride,
                AttribsBufferStateTransitionMode: AttribsBuffer::TRANSITION_MODE,
                pCounterBuffer: counter_buffer,
                CounterOffset: counter_offset,
                CounterBufferStateTransitionMode: CounterBuffer::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DrawMeshAttribs(diligent_sys::DrawMeshAttribs);

#[bon::bon]
impl DrawMeshAttribs {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct DrawMeshIndirectAttribs<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>(
    diligent_sys::DrawMeshIndirectAttribs,
    PhantomData<(
        &'attibs_buffer AttribsBuffer,
        &'counter_buffer CounterBuffer,
    )>,
);

#[bon::bon]
impl<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>
    DrawMeshIndirectAttribs<'attibs_buffer, 'counter_buffer, AttribsBuffer, CounterBuffer>
where
    AttribsBuffer: ResourceTransition<'attibs_buffer, Buffer>,
    CounterBuffer: ResourceTransition<'counter_buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        attribs_buffer: AttribsBuffer,

        #[builder(default = 0)] draw_args_offset: u64,

        #[builder(default = DrawFlags::None)] flags: DrawFlags,

        #[builder(default = 1)] command_count: u32,

        counter_buffer: Option<(CounterBuffer, u64)>,
    ) -> Self {
        let (counter_buffer, counter_offset) = counter_buffer
            .map_or((std::ptr::null_mut(), 0), |(buffer, offset)| {
                (buffer.sys_ptr(), offset)
            });
        Self(
            diligent_sys::DrawMeshIndirectAttribs {
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                DrawArgsOffset: draw_args_offset,
                Flags: flags.bits(),
                CommandCount: command_count,
                AttribsBufferStateTransitionMode: AttribsBuffer::TRANSITION_MODE,
                pCounterBuffer: counter_buffer,
                CounterOffset: counter_offset,
                CounterBufferStateTransitionMode: CounterBuffer::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MultiDrawItem(diligent_sys::MultiDrawItem);

#[bon::bon]
impl MultiDrawItem {
    #[builder(derive(Clone))]
    pub fn new(num_vertices: u32, start_vertex_location: u32) -> Self {
        Self(diligent_sys::MultiDrawItem {
            NumVertices: num_vertices,
            StartVertexLocation: start_vertex_location,
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MultiDrawAttribs<'draw_items>(
    diligent_sys::MultiDrawAttribs,
    PhantomData<&'draw_items ()>,
);

#[bon::bon]
impl<'draw_items> MultiDrawAttribs<'draw_items> {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct MultiDrawIndexedItem(diligent_sys::MultiDrawIndexedItem);
#[bon::bon]
impl MultiDrawIndexedItem {
    #[builder(derive(Clone))]
    pub fn new(num_vertices: u32, first_index_location: u32, base_vertex: u32) -> Self {
        Self(diligent_sys::MultiDrawIndexedItem {
            NumIndices: num_vertices,
            FirstIndexLocation: first_index_location,
            BaseVertex: base_vertex,
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MultiDrawIndexedAttribs<'draw_items>(
    diligent_sys::MultiDrawIndexedAttribs,
    PhantomData<&'draw_items ()>,
);

#[bon::bon]
impl<'draw_items> MultiDrawIndexedAttribs<'draw_items> {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct DispatchComputeAttribs(diligent_sys::DispatchComputeAttribs);
#[bon::bon]
impl DispatchComputeAttribs {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct DispatchComputeIndirectAttribs<'attibs_buffer, AttribsBuffer>(
    diligent_sys::DispatchComputeIndirectAttribs,
    PhantomData<&'attibs_buffer AttribsBuffer>,
);

#[bon::bon]
impl<'attibs_buffer, AttribsBuffer> DispatchComputeIndirectAttribs<'attibs_buffer, AttribsBuffer>
where
    AttribsBuffer: ResourceTransition<'attibs_buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        attribs_buffer: AttribsBuffer,

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
                AttribsBufferStateTransitionMode: AttribsBuffer::TRANSITION_MODE,
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
#[derive(Clone)]
pub struct DispatchTileAttribs(diligent_sys::DispatchTileAttribs);
#[bon::bon]
impl DispatchTileAttribs {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct BLASBuildBoundingBoxData<'name, 'buffer, GeometryBufferTransition>(
    diligent_sys::BLASBuildBoundingBoxData,
    PhantomData<(&'name (), &'buffer GeometryBufferTransition)>,
);
#[bon::bon]
impl<'name, 'buffer, GeometryBufferTransition>
    BLASBuildBoundingBoxData<'name, 'buffer, GeometryBufferTransition>
where
    GeometryBufferTransition: ResourceTransition<'buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        geometry_name: &'name CStr,

        box_buffer: GeometryBufferTransition,

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
#[derive(Clone)]
pub struct BLASBuildTriangleData<'geometry_name, 'geometry, GeometryBufferTransition>(
    diligent_sys::BLASBuildTriangleData,
    PhantomData<(&'geometry_name (), &'geometry GeometryBufferTransition)>,
);
#[bon::bon]
impl<'geometry_name, 'geometry, GeometryBufferTransition>
    BLASBuildTriangleData<'geometry_name, 'geometry, GeometryBufferTransition>
where
    GeometryBufferTransition: ResourceTransition<'geometry, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        geometry_name: &'geometry_name CStr,

        vertex_buffer: GeometryBufferTransition,

        #[builder(default = 0)] vertex_offset: u64,

        vertex_stride: u32,

        vertex_count: usize,

        vertex_value_type: Option<ValueType>,

        #[builder(default = 0)] vertex_component_count: u8,

        primitive_count: usize,

        index_buffer: Option<(GeometryBufferTransition, u64, Option<ValueType>)>,

        transform_buffer: Option<(GeometryBufferTransition, u64)>,

        #[builder(default)] flags: RaytracingGeometryFlags,
    ) -> Self {
        let (index_buffer, index_offset, index_value_type) = index_buffer.map_or(
            (std::ptr::null_mut(), 0, diligent_sys::VT_UNDEFINED as _),
            |(buffer, offset, value_type)| {
                (
                    buffer.sys_ptr(),
                    offset,
                    value_type.map_or(diligent_sys::VT_UNDEFINED as _, |vt| vt.into()),
                )
            },
        );

        let (transform_buffer, transform_offset) = transform_buffer
            .map_or((std::ptr::null_mut(), 0), |(buffer, offset)| {
                (buffer.sys_ptr(), offset)
            });

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
                pIndexBuffer: index_buffer,
                IndexOffset: index_offset,
                IndexType: index_value_type,
                pTransformBuffer: transform_buffer,
                TransformBufferOffset: transform_offset,
                Flags: flags.bits(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct BuildBLASAttribs<
    'blas,
    'scratch_buffer,
    'triangles,
    'triangles_geometry_name,
    'triangles_geometry_buffers,
    'bounding_boxes,
    'bb_name,
    'bb_buffer,
    BLASTransition,
    ScratchBufferTransition,
    GeometryBufferTransition,
>(
    diligent_sys::BuildBLASAttribs,
    PhantomData<(
        &'blas BLASTransition,
        &'scratch_buffer ScratchBufferTransition,
        &'triangles GeometryBufferTransition,
        &'triangles_geometry_name (),
        &'triangles_geometry_buffers (),
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
    'triangles_geometry_buffers,
    'bounding_boxes,
    'bb_name,
    'bb_buffer,
    BLASTransition,
    ScratchBufferTransition,
    GeometryBufferTransition,
>
    BuildBLASAttribs<
        'blas,
        'scratch_buffer,
        'triangles,
        'triangles_geometry_name,
        'triangles_geometry_buffers,
        'bounding_boxes,
        'bb_name,
        'bb_buffer,
        BLASTransition,
        ScratchBufferTransition,
        GeometryBufferTransition,
    >
where
    BLASTransition: ResourceTransition<'blas, BottomLevelAS>,
    ScratchBufferTransition: ResourceTransition<'scratch_buffer, Buffer>,
    GeometryBufferTransition: ResourceTransition<'triangles, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        blas: BLASTransition,

        #[builder(default)] triangle_data: &'triangles [BLASBuildTriangleData<
            'triangles_geometry_name,
            'triangles_geometry_buffers,
            GeometryBufferTransition,
        >],

        #[builder(default)] box_data: &'bounding_boxes [BLASBuildBoundingBoxData<
            'bb_name,
            'bb_buffer,
            GeometryBufferTransition,
        >],

        scratch_buffer: ScratchBufferTransition,

        #[builder(default = 0)] scratch_buffer_offset: u64,

        #[builder(default = false)] update: bool,
    ) -> Self {
        Self(
            diligent_sys::BuildBLASAttribs {
                pBLAS: blas.sys_ptr(),
                BLASTransitionMode: BLASTransition::TRANSITION_MODE,
                GeometryTransitionMode: GeometryBufferTransition::TRANSITION_MODE,
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
                ScratchBufferTransitionMode: ScratchBufferTransition::TRANSITION_MODE,
                Update: update,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct BuildTLASAttribs<
    'tlas,
    'tlas_instance_name,
    'blas,
    'instance_buffer,
    'scratch_buffer,
    'instances,
    TLASTransition,
    InstanceBufferTransition,
    ScratchBufferTransition,
    BlasTriansition,
>(
    pub(crate) diligent_sys::BuildTLASAttribs,
    PhantomData<(
        &'tlas TLASTransition,
        &'tlas_instance_name (),
        &'blas BlasTriansition,
        &'instance_buffer InstanceBufferTransition,
        &'scratch_buffer ScratchBufferTransition,
        &'instances (),
    )>,
);
#[bon::bon]
impl<
    'tlas,
    'tlas_instance_name,
    'blas,
    'instance_buffer,
    'scratch_buffer,
    'instances,
    TLASTransition,
    InstanceBufferTransition,
    ScratchBufferTransition,
    BlasTriansition,
>
    BuildTLASAttribs<
        'tlas,
        'tlas_instance_name,
        'blas,
        'instance_buffer,
        'scratch_buffer,
        'instances,
        TLASTransition,
        InstanceBufferTransition,
        ScratchBufferTransition,
        BlasTriansition,
    >
where
    TLASTransition: ResourceTransition<'tlas, TopLevelAS>,
    InstanceBufferTransition: ResourceTransition<'instance_buffer, Buffer>,
    ScratchBufferTransition: ResourceTransition<'scratch_buffer, Buffer>,
    BlasTriansition: ResourceTransition<'blas, BottomLevelAS>,
{
    #[builder(derive(Clone))]
    pub fn new(
        tlas: TLASTransition,

        instances: &'instances [TLASBuildInstanceData<
            'tlas_instance_name,
            'blas,
            BlasTriansition,
        >],

        instance_buffer: InstanceBufferTransition,

        #[builder(default = 0)] instance_buffer_offset: u64,

        #[builder(default = 1)] hit_group_stride: u32,

        #[builder(default = 0)] base_contribution_to_hit_group_index: u32,

        #[builder(default = HitGroupBindingMode::PerGeometry)] binding_mode: HitGroupBindingMode,

        scratch_buffer: ScratchBufferTransition,

        #[builder(default = 0)] scratch_buffer_offset: u64,

        #[builder(default = false)] update: bool,
    ) -> Self {
        Self(
            diligent_sys::BuildTLASAttribs {
                pTLAS: tlas.sys_ptr(),
                TLASTransitionMode: TLASTransition::TRANSITION_MODE,
                BLASTransitionMode: BlasTriansition::TRANSITION_MODE,
                pInstances: instances
                    .first()
                    .map_or(std::ptr::null(), |instance| &instance.0),
                InstanceCount: instances.len() as u32,
                pInstanceBuffer: instance_buffer.sys_ptr(),
                InstanceBufferOffset: instance_buffer_offset,
                InstanceBufferTransitionMode: InstanceBufferTransition::TRANSITION_MODE,
                HitGroupStride: hit_group_stride,
                BaseContributionToHitGroupIndex: base_contribution_to_hit_group_index,
                BindingMode: binding_mode.into(),
                pScratchBuffer: scratch_buffer.sys_ptr(),
                ScratchBufferOffset: scratch_buffer_offset,
                ScratchBufferTransitionMode: ScratchBufferTransition::TRANSITION_MODE,
                Update: update,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct UpdateIndirectRTBufferAttribs<'buffer, AttribsBufferTransition>(
    diligent_sys::UpdateIndirectRTBufferAttribs,
    PhantomData<&'buffer AttribsBufferTransition>,
);

#[bon::bon]
impl<'buffer, AttribsBufferTransition>
    UpdateIndirectRTBufferAttribs<'buffer, AttribsBufferTransition>
where
    AttribsBufferTransition: ResourceTransition<'buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        attribs_buffer: AttribsBufferTransition,

        #[builder(default = 0)] attribs_buffer_offset: u64,
    ) -> Self {
        Self(
            diligent_sys::UpdateIndirectRTBufferAttribs {
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                AttribsBufferOffset: attribs_buffer_offset,
                TransitionMode: AttribsBufferTransition::TRANSITION_MODE,
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
        sbt: &'shader_binding_table mut ShaderBindingTable,

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
pub struct TraceRaysIndirectAttribs<'shader_binding_table, 'attribs_buffer, AttribsBufferTransition>(
    diligent_sys::TraceRaysIndirectAttribs,
    PhantomData<(
        &'shader_binding_table (),
        &'attribs_buffer AttribsBufferTransition,
    )>,
);
#[bon::bon]
impl<'shader_binding_table, 'attribs_buffer, AttribsBufferTransition>
    TraceRaysIndirectAttribs<'shader_binding_table, 'attribs_buffer, AttribsBufferTransition>
where
    AttribsBufferTransition: ResourceTransition<'attribs_buffer, Buffer>,
{
    #[builder]
    pub fn new(
        sbt: &'shader_binding_table mut ShaderBindingTable,
        attribs_buffer: AttribsBufferTransition,
        #[builder(default = 0)] args_byte_offset: u64,
    ) -> Self {
        Self(
            diligent_sys::TraceRaysIndirectAttribs {
                pSBT: sbt.sys_ptr(),
                pAttribsBuffer: attribs_buffer.sys_ptr(),
                AttribsBufferStateTransitionMode: AttribsBufferTransition::TRANSITION_MODE,
                ArgsByteOffset: args_byte_offset,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct CopyTextureAttribs<
    'src_texture,
    'dst_texture,
    'region,
    SrcTextureTransition,
    DstTextureTransition,
>(
    diligent_sys::CopyTextureAttribs,
    PhantomData<(
        &'src_texture SrcTextureTransition,
        &'dst_texture DstTextureTransition,
        &'region (),
    )>,
);

#[bon::bon]
impl<'src_texture, 'dst_texture, 'region, SrcTextureTransition, DstTextureTransition>
    CopyTextureAttribs<
        'src_texture,
        'dst_texture,
        'region,
        SrcTextureTransition,
        DstTextureTransition,
    >
where
    SrcTextureTransition: ResourceTransition<'src_texture, Texture>,
    DstTextureTransition: ResourceTransition<'src_texture, Texture>,
{
    #[builder(derive(Clone))]
    pub fn new(
        src_texture: SrcTextureTransition,
        src_mip_level: u32,
        src_slice: u32,
        src_box: &'region crate::Box,
        dst_texture: DstTextureTransition,
        dst_mip_level: u32,
        dst_slice: u32,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
    ) -> Self {
        CopyTextureAttribs(
            diligent_sys::CopyTextureAttribs {
                pSrcTexture: src_texture.sys_ptr(),
                SrcMipLevel: src_mip_level,
                SrcSlice: src_slice,
                pSrcBox: &src_box.0,
                SrcTextureTransitionMode: SrcTextureTransition::TRANSITION_MODE,
                pDstTexture: dst_texture.sys_ptr(),
                DstMipLevel: dst_mip_level,
                DstSlice: dst_slice,
                DstX: dst_x,
                DstY: dst_y,
                DstZ: dst_z,
                DstTextureTransitionMode: DstTextureTransition::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ResolveTextureSubresourceAttribs<SrcTextureTransition, DstTextureTransition>(
    diligent_sys::ResolveTextureSubresourceAttribs,
    PhantomData<(SrcTextureTransition, DstTextureTransition)>,
);

#[bon::bon]
impl<'src, 'dst, SrcTextureTransition, DstTextureTransition>
    ResolveTextureSubresourceAttribs<SrcTextureTransition, DstTextureTransition>
where
    SrcTextureTransition: ResourceTransition<'src, Texture>,
    DstTextureTransition: ResourceTransition<'dst, Texture>,
{
    #[builder(derive(Clone))]
    pub fn new(
        src_mip_level: u32,
        src_slice: u32,
        dst_mip_level: u32,
        dst_slice: u32,
        format: TextureFormat,
    ) -> Self {
        Self(
            diligent_sys::ResolveTextureSubresourceAttribs {
                SrcMipLevel: src_mip_level,
                SrcSlice: src_slice,
                SrcTextureTransitionMode: SrcTextureTransition::TRANSITION_MODE,
                DstMipLevel: dst_mip_level,
                DstSlice: dst_slice,
                DstTextureTransitionMode: DstTextureTransition::TRANSITION_MODE,
                Format: format.into(),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct WriteBLASCompactedSizeAttribs<'blas, 'buffer, BLASTransition, BufferTransition>(
    diligent_sys::WriteBLASCompactedSizeAttribs,
    PhantomData<(&'blas BLASTransition, &'buffer BufferTransition)>,
);

#[bon::bon]
impl<'blas, 'buffer, BLASTransition, BufferTransition>
    WriteBLASCompactedSizeAttribs<'blas, 'buffer, BLASTransition, BufferTransition>
where
    BLASTransition: ResourceTransition<'blas, BottomLevelAS>,
    BufferTransition: ResourceTransition<'buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        blas: BLASTransition,
        dest_buffer: BufferTransition,
        dest_buffer_offset: u64,
    ) -> Self {
        Self(
            diligent_sys::WriteBLASCompactedSizeAttribs {
                pBLAS: blas.sys_ptr(),
                pDestBuffer: dest_buffer.sys_ptr(),
                DestBufferOffset: dest_buffer_offset,
                BLASTransitionMode: BLASTransition::TRANSITION_MODE,
                BufferTransitionMode: BufferTransition::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct WriteTLASCompactedSizeAttribs<'tlas, 'buffer, TLASTransition, BufferTransition>(
    diligent_sys::WriteTLASCompactedSizeAttribs,
    PhantomData<(&'tlas TLASTransition, &'buffer BufferTransition)>,
);

#[bon::bon]
impl<'tlas, 'buffer, TLASTransition, BufferTransition>
    WriteTLASCompactedSizeAttribs<'tlas, 'buffer, TLASTransition, BufferTransition>
where
    TLASTransition: ResourceTransition<'tlas, TopLevelAS>,
    BufferTransition: ResourceTransition<'buffer, Buffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        tlas: TLASTransition,
        dest_buffer: BufferTransition,
        dest_buffer_offset: u64,
    ) -> Self {
        Self(
            diligent_sys::WriteTLASCompactedSizeAttribs {
                pTLAS: tlas.sys_ptr(),
                pDestBuffer: dest_buffer.sys_ptr(),
                DestBufferOffset: dest_buffer_offset,
                TLASTransitionMode: TLASTransition::TRANSITION_MODE,
                BufferTransitionMode: BufferTransition::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct CopyBLASAttribs<'src_blas, 'dst_blas, SrcTransition, DstTransition>(
    diligent_sys::CopyBLASAttribs,
    PhantomData<(&'src_blas SrcTransition, &'dst_blas DstTransition)>,
);

#[bon::bon]
impl<'src_blas, 'dst_blas, SrcTransition, DstTransition>
    CopyBLASAttribs<'src_blas, 'dst_blas, SrcTransition, DstTransition>
where
    SrcTransition: ResourceTransition<'src_blas, BottomLevelAS>,
    DstTransition: ResourceTransition<'dst_blas, BottomLevelAS>,
{
    #[builder(derive(Clone))]
    pub fn new(src: SrcTransition, dst: DstTransition, mode: CopyAsMode) -> Self {
        CopyBLASAttribs(
            diligent_sys::CopyBLASAttribs {
                pSrc: src.sys_ptr(),
                pDst: dst.sys_ptr(),
                Mode: mode.into(),
                SrcTransitionMode: SrcTransition::TRANSITION_MODE,
                DstTransitionMode: DstTransition::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct CopyTLASAttribs<'src_tlas, 'dst_tlas, SrcTransition, DstTransition>(
    diligent_sys::CopyTLASAttribs,
    PhantomData<(&'src_tlas SrcTransition, &'dst_tlas DstTransition)>,
);

#[bon::bon]
impl<'src_tlas, 'dst_tlas, SrcTransition, DstTransition>
    CopyTLASAttribs<'src_tlas, 'dst_tlas, SrcTransition, DstTransition>
where
    SrcTransition: ResourceTransition<'src_tlas, TopLevelAS>,
    DstTransition: ResourceTransition<'dst_tlas, TopLevelAS>,
{
    #[builder(derive(Clone))]
    pub fn new(src: SrcTransition, dst: DstTransition, mode: CopyAsMode) -> Self {
        CopyTLASAttribs(
            diligent_sys::CopyTLASAttribs {
                pSrc: src.sys_ptr(),
                pDst: dst.sys_ptr(),
                Mode: mode.into(),
                SrcTransitionMode: SrcTransition::TRANSITION_MODE,
                DstTransitionMode: DstTransition::TRANSITION_MODE,
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

#[repr(transparent)]
#[derive(Clone)]
pub struct Viewport(diligent_sys::Viewport);

#[bon::bon]
impl Viewport {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct Rect(diligent_sys::Rect);

#[bon::bon]
impl Rect {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct DepthStencilClearValue(pub(crate) diligent_sys::DepthStencilClearValue);

#[bon::bon]
impl DepthStencilClearValue {
    #[builder(derive(Clone))]
    pub fn new(depth: f32, stencil: u8) -> Self {
        Self(diligent_sys::DepthStencilClearValue {
            Depth: depth,
            Stencil: stencil,
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct OptimizedClearValue(pub(crate) diligent_sys::OptimizedClearValue);

#[bon::bon]
impl OptimizedClearValue {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct BeginRenderPassAttribs<'render_pass, 'frame_buffer, 'clear_values, FramebufferTransition>(
    diligent_sys::BeginRenderPassAttribs,
    PhantomData<(
        &'render_pass (),
        &'frame_buffer FramebufferTransition,
        &'clear_values (),
    )>,
);

#[bon::bon]
impl<'render_pass, 'frame_buffer, 'clear_values, FramebufferTransition>
    BeginRenderPassAttribs<'render_pass, 'frame_buffer, 'clear_values, FramebufferTransition>
where
    FramebufferTransition: ResourceTransition<'frame_buffer, Framebuffer>,
{
    #[builder(derive(Clone))]
    pub fn new(
        render_pass: &'render_pass RenderPass,
        frame_buffer: FramebufferTransition,
        clear_values: &'clear_values [OptimizedClearValue],
    ) -> Self {
        Self(
            diligent_sys::BeginRenderPassAttribs {
                pRenderPass: render_pass.sys_ptr(),
                pFramebuffer: frame_buffer.sys_ptr(),
                ClearValueCount: clear_values.len() as u32,
                pClearValues: clear_values.first().map_or(std::ptr::null_mut(), |value| {
                    std::ptr::from_ref(&value.0) as *mut _
                }),
                StateTransitionMode: FramebufferTransition::TRANSITION_MODE,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct SparseBufferMemoryBindRange(diligent_sys::SparseBufferMemoryBindRange);

#[bon::bon]
impl SparseBufferMemoryBindRange {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct SparseTextureMemoryBindRange(diligent_sys::SparseTextureMemoryBindRange);

#[bon::bon]
impl SparseTextureMemoryBindRange {
    #[builder(derive(Clone))]
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
#[derive(Clone)]
pub struct SparseBufferMemoryBindInfo(diligent_sys::SparseBufferMemoryBindInfo);

#[bon::bon]
impl SparseBufferMemoryBindInfo {
    #[builder(derive(Clone))]
    pub fn new(buffer: &Buffer, ranges: &[SparseBufferMemoryBindRange]) -> Self {
        Self(diligent_sys::SparseBufferMemoryBindInfo {
            pBuffer: buffer.sys_ptr(),
            pRanges: ranges.first().map_or(std::ptr::null(), |r| &r.0),
            NumRanges: ranges.len() as u32,
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct SparseTextureMemoryBindInfo(diligent_sys::SparseTextureMemoryBindInfo);

#[bon::bon]
impl SparseTextureMemoryBindInfo {
    #[builder(derive(Clone))]
    pub fn new(texture: &Texture, ranges: &[SparseTextureMemoryBindRange]) -> Self {
        Self(diligent_sys::SparseTextureMemoryBindInfo {
            pTexture: texture.sys_ptr(),
            pRanges: ranges.first().map_or(std::ptr::null(), |r| &r.0),
            NumRanges: ranges.len() as u32,
        })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct BindSparseResourceMemoryAttribs(diligent_sys::BindSparseResourceMemoryAttribs);

#[bon::bon]
impl BindSparseResourceMemoryAttribs {
    #[builder(derive(Clone))]
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
            ppWaitFences: wait_fences.first().map_or(std::ptr::null_mut(), |&fence| {
                std::ptr::from_ref(fence) as *mut _
            }),
            pWaitFenceValues: wait_fence_values
                .first()
                .map_or(std::ptr::null(), std::ptr::from_ref),
            NumWaitFences: wait_fences.len() as u32,
            ppSignalFences: signal_fences
                .first()
                .map_or(std::ptr::null_mut(), |&fence| {
                    std::ptr::from_ref(fence) as *mut _
                }),
            pSignalFenceValues: signal_fence_values
                .first()
                .map_or(std::ptr::null(), std::ptr::from_ref),
            NumSignalFences: signal_fences.len() as u32,
        })
    }
}

pub struct RenderPassToken<'context> {
    context: &'context DeviceContext,
}

impl<'context> RenderPassToken<'context> {
    pub fn new<FramebufferTransition>(
        context: &'context DeviceContext,
        attribs: &BeginRenderPassAttribs<FramebufferTransition>,
    ) -> Self {
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

pub struct GraphicsPipelineToken<Context: Borrow<DeviceContext>>(Context);

impl<Context: Borrow<DeviceContext>> GraphicsPipelineToken<Context> {
    pub fn draw(&self, attribs: &DrawAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, Draw, &attribs.0)
    }

    pub fn draw_indexed(&self, attribs: &DrawIndexedAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, DrawIndexed, &attribs.0)
    }

    pub fn draw_indirect<AttribsBuffer, CounterBuffer>(
        &self,
        attribs: &DrawIndirectAttribs<AttribsBuffer, CounterBuffer>,
    ) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, DrawIndirect, &attribs.0)
    }

    pub fn draw_indexed_indirect<AttribsBuffer, CounterBuffer>(
        &self,
        attribs: &DrawIndexedIndirectAttribs<AttribsBuffer, CounterBuffer>,
    ) {
        unsafe_member_call!(
            self.0.borrow(),
            DeviceContext,
            DrawIndexedIndirect,
            &attribs.0
        )
    }

    pub fn multi_draw(&self, attribs: &MultiDrawAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, MultiDraw, &attribs.0)
    }

    pub fn multi_draw_indexed(&self, attribs: &MultiDrawIndexedAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, MultiDrawIndexed, &attribs.0)
    }

    pub fn finish(self) -> Context {
        self.0
    }
}

impl<Context: Borrow<DeviceContext>> Deref for GraphicsPipelineToken<Context> {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct MeshPipelineToken<Context: Borrow<DeviceContext>>(Context);

impl<Context: Borrow<DeviceContext>> MeshPipelineToken<Context> {
    pub fn draw_mesh(&self, attribs: &DrawMeshAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, DrawMesh, &attribs.0)
    }

    pub fn draw_mesh_indirect<AttribsBuffer, CounterBuffer>(
        &self,
        attribs: &DrawMeshIndirectAttribs<AttribsBuffer, CounterBuffer>,
    ) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, DrawMeshIndirect, &attribs.0)
    }

    pub fn finish(self) -> Context {
        self.0
    }
}

impl<Context: Borrow<DeviceContext>> Deref for MeshPipelineToken<Context> {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ComputePipelineToken<Context: Borrow<DeviceContext>>(Context);

impl<Context: Borrow<DeviceContext>> ComputePipelineToken<Context> {
    pub fn dispatch_compute(&self, attribs: &DispatchComputeAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, DispatchCompute, &attribs.0)
    }

    pub fn dispatch_compute_indirect<AttribsBuffer>(
        &self,
        attribs: &DispatchComputeIndirectAttribs<AttribsBuffer>,
    ) {
        unsafe_member_call!(
            self.0.borrow(),
            DeviceContext,
            DispatchComputeIndirect,
            &attribs.0
        )
    }

    pub fn finish(self) -> Context {
        self.0
    }
}

impl<Context: Borrow<DeviceContext>> Deref for ComputePipelineToken<Context> {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct TilePipelineToken<Context: Borrow<DeviceContext>>(Context);

impl<Context: Borrow<DeviceContext>> TilePipelineToken<Context> {
    pub fn dispatch_tile(&self, attribs: &DispatchTileAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, DispatchTile, &attribs.0)
    }

    pub fn finish(self) -> Context {
        self.0
    }
}

impl<Context: Borrow<DeviceContext>> Deref for TilePipelineToken<Context> {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RayTracingPipelineToken<Context: Borrow<DeviceContext>>(Context);

impl<Context: Borrow<DeviceContext>> RayTracingPipelineToken<Context> {
    pub fn trace_rays(&self, attribs: &TraceRaysAttribs) {
        unsafe_member_call!(self.0.borrow(), DeviceContext, TraceRays, &attribs.0)
    }

    pub fn trace_rays_indirect<'buffer, AttribsBufferTransition>(
        &self,
        attribs: &TraceRaysIndirectAttribs<AttribsBufferTransition>,
    ) where
        AttribsBufferTransition: ResourceTransition<'buffer, Buffer>,
    {
        unsafe_member_call!(
            self.0.borrow(),
            DeviceContext,
            TraceRaysIndirect,
            &attribs.0
        )
    }

    pub fn finish(self) -> Context {
        self.0
    }
}

impl<Context: Borrow<DeviceContext>> Deref for RayTracingPipelineToken<Context> {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

define_ported!(
    DeviceContext,
    diligent_sys::IDeviceContext,
    diligent_sys::IDeviceContextMethods : 72,
    Object
);

pub trait GraphicsContext: Borrow<DeviceContext> + Sized {
    fn set_graphics_pipeline_state(
        self,
        pipeline_state: &GraphicsPipelineState,
    ) -> GraphicsPipelineToken<Self> {
        unsafe_member_call!(
            self.borrow(),
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        GraphicsPipelineToken(self)
    }
}

pub trait MeshContext: Borrow<DeviceContext> + Sized {
    fn set_mesh_pipeline_state(
        self,
        pipeline_state: &GraphicsPipelineState,
    ) -> GraphicsPipelineToken<Self> {
        unsafe_member_call!(
            self.borrow(),
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        GraphicsPipelineToken(self)
    }
}

pub trait ComputeContext: Borrow<DeviceContext> + Sized {
    fn set_compute_pipeline_state(
        self,
        pipeline_state: &ComputePipelineState,
    ) -> ComputePipelineToken<Self> {
        unsafe_member_call!(
            self.borrow(),
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        ComputePipelineToken(self)
    }
}

pub trait TileContext: Borrow<DeviceContext> + Sized {
    fn set_tile_pipeline_state(
        self,
        pipeline_state: &TilePipelineState,
    ) -> TilePipelineToken<Self> {
        unsafe_member_call!(
            self.borrow(),
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        TilePipelineToken(self)
    }
}

pub trait RayTracingContext: Borrow<DeviceContext> + Sized {
    fn set_ray_tracing_pipeline_state(
        self,
        pipeline_state: &RayTracingPipelineState,
    ) -> RayTracingPipelineToken<Self> {
        unsafe_member_call!(
            self.borrow(),
            DeviceContext,
            SetPipelineState,
            pipeline_state.sys_ptr()
        );

        RayTracingPipelineToken(self)
    }
}

impl Borrow<DeviceContext> for Boxed<ImmediateDeviceContext> {
    fn borrow(&self) -> &DeviceContext {
        self
    }
}
impl Borrow<DeviceContext> for Boxed<DeferredDeviceContext> {
    fn borrow(&self) -> &DeviceContext {
        self
    }
}

impl GraphicsContext for Boxed<ImmediateDeviceContext> {}
impl GraphicsContext for Boxed<DeferredDeviceContext> {}
impl MeshContext for Boxed<ImmediateDeviceContext> {}
impl MeshContext for Boxed<DeferredDeviceContext> {}
impl ComputeContext for Boxed<ImmediateDeviceContext> {}
impl ComputeContext for Boxed<DeferredDeviceContext> {}
impl TileContext for Boxed<ImmediateDeviceContext> {}
impl TileContext for Boxed<DeferredDeviceContext> {}
impl RayTracingContext for Boxed<ImmediateDeviceContext> {}
impl RayTracingContext for Boxed<DeferredDeviceContext> {}

impl DeviceContext {
    pub fn desc(&self) -> &DeviceContextDesc {
        let desc_ptr = unsafe_member_call!(self, DeviceContext, GetDesc);
        unsafe { &*(desc_ptr as *const DeviceContextDesc) }
    }

    pub fn transition_shader_resources(&self, shader_resource_binding: &mut ShaderResourceBinding) {
        unsafe_member_call!(
            self,
            DeviceContext,
            TransitionShaderResources,
            shader_resource_binding.sys_ptr()
        )
    }

    pub fn commit_shader_resources<'srb, SRBTransition>(&self, srb: SRBTransition)
    where
        SRBTransition: ResourceTransition<'srb, ShaderResourceBinding>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            CommitShaderResources,
            srb.sys_ptr(),
            SRBTransition::TRANSITION_MODE
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

    pub fn set_vertex_buffers<'buffer, BufferTransition, const N: usize>(
        &self,
        buffers: [(BufferTransition, u64); N],
        flags: SetVertexBufferFlags,
    ) where
        BufferTransition: ResourceTransition<'buffer, Buffer>,
    {
        let num_buffers = buffers.as_ref().len();
        let (buffer_pointers, offsets): (Vec<_>, Vec<_>) = buffers
            .into_iter()
            .map(|(buffer, offset)| (buffer.sys_ptr() as *mut _, offset))
            .unzip();

        unsafe_member_call!(
            self,
            DeviceContext,
            SetVertexBuffers,
            0,
            num_buffers as u32,
            buffer_pointers
                .first()
                .map_or(std::ptr::null(), std::ptr::from_ref),
            offsets.first().map_or(std::ptr::null(), std::ptr::from_ref),
            BufferTransition::TRANSITION_MODE,
            flags.bits()
        )
    }

    pub fn invalidate_state(&self) {
        unsafe_member_call!(self, DeviceContext, InvalidateState)
    }

    pub fn set_index_buffer<'buffer, BufferTransition>(
        &self,
        index_buffer: BufferTransition,
        offset: u64,
    ) where
        BufferTransition: ResourceTransition<'buffer, Buffer>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetIndexBuffer,
            index_buffer.sys_ptr(),
            offset,
            BufferTransition::TRANSITION_MODE
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

    pub fn set_render_targets<'texture_view, TextureViewTransition>(
        &self,
        render_targets: &[TextureViewTransition],
        depth_stencil: Option<TextureViewTransition>,
    ) where
        TextureViewTransition: ResourceTransition<'texture_view, TextureView>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            SetRenderTargets,
            render_targets.len() as u32,
            render_targets.first().map_or(std::ptr::null_mut(), |rt| {
                std::ptr::from_ref(rt) as *mut _
            }),
            depth_stencil.map_or(std::ptr::null_mut(), |v| v.sys_ptr()),
            TextureViewTransition::TRANSITION_MODE
        )
    }

    pub fn new_render_pass<FramebufferTransition>(
        &self,
        attribs: &BeginRenderPassAttribs<FramebufferTransition>,
    ) -> RenderPassToken<'_> {
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

    pub fn clear_depth<'view, TextureViewTransition>(&self, view: TextureViewTransition, depth: f32)
    where
        TextureViewTransition: ResourceTransition<'view, TextureView>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearDepthStencil,
            view.sys_ptr(),
            diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
            depth,
            0,
            TextureViewTransition::TRANSITION_MODE
        )
    }

    pub fn clear_stencil<'view, TextureViewTransition>(
        &self,
        view: TextureViewTransition,
        stencil: u8,
    ) where
        TextureViewTransition: ResourceTransition<'view, TextureView>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearDepthStencil,
            view.sys_ptr(),
            diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
            0.0,
            stencil,
            TextureViewTransition::TRANSITION_MODE
        )
    }

    pub fn clear_depth_stencil<'view, TextureViewTransition>(
        &self,
        view: TextureViewTransition,
        depth: f32,
        stencil: u8,
    ) where
        TextureViewTransition: ResourceTransition<'view, TextureView>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearDepthStencil,
            view.sys_ptr(),
            diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS
                | diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
            depth,
            stencil,
            TextureViewTransition::TRANSITION_MODE
        )
    }

    pub fn clear_render_target<'view, T, TextureViewTransition>(
        &self,
        view: TextureViewTransition,
        rgba: &[T; 4],
    ) where
        TextureViewTransition: ResourceTransition<'view, TextureView>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            ClearRenderTarget,
            view.sys_ptr(),
            rgba.as_ptr() as *const std::os::raw::c_void,
            TextureViewTransition::TRANSITION_MODE
        )
    }

    pub fn enqueue_signal(&self, fence: &Fence, value: u64) {
        unsafe_member_call!(self, DeviceContext, EnqueueSignal, fence.sys_ptr(), value);
    }

    pub fn update_buffer<'buffer, T, BufferTransition: ResourceTransition<'buffer, Buffer>>(
        &self,
        buffer: BufferTransition,
        offset: u64,
        size: u64,
        data: &T,
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateBuffer,
            buffer.sys_ptr(),
            offset,
            size,
            std::ptr::from_ref(data) as *const std::os::raw::c_void,
            BufferTransition::TRANSITION_MODE
        )
    }

    pub fn update_buffer_from_slice<
        'buffer,
        T,
        BufferTransition: ResourceTransition<'buffer, Buffer>,
    >(
        &self,
        buffer: BufferTransition,
        data: &[T],
    ) {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateBuffer,
            buffer.sys_ptr(),
            0,
            std::mem::size_of_val(data) as u64,
            data.first()
                .map_or(std::ptr::null_mut(), |rt| { std::ptr::from_ref(rt) as _ }),
            BufferTransition::TRANSITION_MODE
        )
    }

    pub fn copy_buffer<'src_buffer, 'dst_buffer, SrcBufferTransition, DstBufferTransition>(
        &self,
        src_buffer: SrcBufferTransition,
        src_offset: u64,
        dst_buffer: DstBufferTransition,
        dst_offset: u64,
        size: u64,
    ) where
        SrcBufferTransition: ResourceTransition<'src_buffer, Buffer>,
        DstBufferTransition: ResourceTransition<'src_buffer, Buffer>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            CopyBuffer,
            src_buffer.sys_ptr(),
            src_offset,
            SrcBufferTransition::TRANSITION_MODE,
            dst_buffer.sys_ptr(),
            dst_offset,
            size,
            DstBufferTransition::TRANSITION_MODE
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

    pub fn update_texture<'texture, 'buffer, SrcBufferTransition, DstTextureTransition>(
        &self,
        texture: DstTextureTransition,
        mip_level: u32,
        slice: u32,
        dst_box: &crate::Box,
        subres_data: &TextureSubResource<'buffer, SrcBufferTransition>,
    ) where
        SrcBufferTransition: ResourceTransition<'buffer, Buffer>,
        DstTextureTransition: ResourceTransition<'texture, Texture>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateTexture,
            texture.sys_ptr(),
            mip_level,
            slice,
            &dst_box.0,
            &subres_data.0,
            SrcBufferTransition::TRANSITION_MODE,
            DstTextureTransition::TRANSITION_MODE
        )
    }

    pub fn copy_texture<SrcTextureTransition, DstTextureTransition>(
        &self,
        copy_attribs: &CopyTextureAttribs<SrcTextureTransition, DstTextureTransition>,
    ) {
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

    pub fn resolve_texture_subresource<'src, 'dst, SrcTexture, DstTexture>(
        &self,
        src_texture: SrcTexture,
        dst_texture: DstTexture,
        resolve_attribs: &ResolveTextureSubresourceAttribs<SrcTexture, DstTexture>,
    ) where
        SrcTexture: ResourceTransition<'src, Texture>,
        DstTexture: ResourceTransition<'dst, Texture>,
    {
        unsafe_member_call!(
            self,
            DeviceContext,
            ResolveTextureSubresource,
            src_texture.sys_ptr(),
            dst_texture.sys_ptr(),
            &resolve_attribs.0
        )
    }

    pub fn build_blas<BLASTransition, ScratchBufferTransition, GeometryTransition>(
        &self,
        attribs: &BuildBLASAttribs<BLASTransition, ScratchBufferTransition, GeometryTransition>,
    ) {
        unsafe_member_call!(self, DeviceContext, BuildBLAS, &attribs.0)
    }

    pub fn build_tlas<
        TLASTransition,
        InstanceBufferTransition,
        ScratchBufferTransition,
        BlasTriansition,
    >(
        &self,
        attribs: &BuildTLASAttribs<
            TLASTransition,
            InstanceBufferTransition,
            ScratchBufferTransition,
            BlasTriansition,
        >,
    ) {
        unsafe_member_call!(self, DeviceContext, BuildTLAS, &attribs.0)
    }

    pub fn copy_blas<SrcTransition, DstTransition>(
        &self,
        attribs: &CopyBLASAttribs<SrcTransition, DstTransition>,
    ) {
        unsafe_member_call!(self, DeviceContext, CopyBLAS, &attribs.0)
    }

    pub fn copy_tlas<SrcTransition, DstTransition>(
        &self,
        attribs: &CopyTLASAttribs<SrcTransition, DstTransition>,
    ) {
        unsafe_member_call!(self, DeviceContext, CopyTLAS, &attribs.0)
    }

    pub fn write_blas_compacted_size<BLASTransition, BufferTransition>(
        &self,
        attribs: &WriteBLASCompactedSizeAttribs<BLASTransition, BufferTransition>,
    ) {
        unsafe_member_call!(self, DeviceContext, WriteBLASCompactedSize, &attribs.0)
    }

    pub fn write_tlas_compacted_size<TLASTransition, BufferTransition>(
        &self,
        attribs: &WriteTLASCompactedSizeAttribs<TLASTransition, BufferTransition>,
    ) {
        unsafe_member_call!(self, DeviceContext, WriteTLASCompactedSize, &attribs.0)
    }

    pub fn update_sbt(&self, sbt: &mut ShaderBindingTable) {
        unsafe_member_call!(
            self,
            DeviceContext,
            UpdateSBT,
            sbt.sys_ptr(),
            std::ptr::null_mut()
        )
    }

    pub fn update_sbt_with_attribs<Transition>(
        &self,
        sbt: &mut ShaderBindingTable,
        attribs: &UpdateIndirectRTBufferAttribs<Transition>,
    ) {
        unsafe_member_call!(self, DeviceContext, UpdateSBT, sbt.sys_ptr(), &attribs.0)
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
    fn sys_ptr(&self) -> *mut Self::SysType {
        self.0.sys_ptr()
    }
}

impl Ported for DeferredDeviceContext {
    type SysType = diligent_sys::IDeviceContext;
    fn sys_ptr(&self) -> *mut Self::SysType {
        self.0.sys_ptr()
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
