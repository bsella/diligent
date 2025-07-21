use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert;

use crate::{
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
    pipeline_state::PipelineState,
    query::{GetSysQueryType, Query},
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

#[derive(Builder)]
pub struct DrawAttribs {
    num_vertices: u32,

    #[builder(default)]
    flags: DrawFlags,

    #[builder(default = 1)]
    num_instances: u32,

    #[builder(default = 0)]
    start_vertex_location: u32,

    #[builder(default = 0)]
    first_instance_location: u32,
}

impl From<&DrawAttribs> for diligent_sys::DrawAttribs {
    fn from(value: &DrawAttribs) -> Self {
        diligent_sys::DrawAttribs {
            NumVertices: value.num_vertices,
            Flags: value.flags.bits(),
            NumInstances: value.num_instances,
            StartVertexLocation: value.start_vertex_location,
            FirstInstanceLocation: value.first_instance_location,
        }
    }
}

#[derive(Builder)]
pub struct DrawIndexedAttribs {
    num_indices: u32,
    index_type: ValueType,

    #[builder(default)]
    flags: DrawFlags,

    #[builder(default = 1)]
    num_instances: u32,

    #[builder(default = 0)]
    first_index_location: u32,

    #[builder(default = 0)]
    base_vertex: u32,

    #[builder(default = 0)]
    first_instance_location: u32,
}

impl From<&DrawIndexedAttribs> for diligent_sys::DrawIndexedAttribs {
    fn from(value: &DrawIndexedAttribs) -> Self {
        diligent_sys::DrawIndexedAttribs {
            BaseVertex: value.base_vertex,
            FirstIndexLocation: value.first_index_location,
            FirstInstanceLocation: value.first_instance_location,
            Flags: value.flags.bits(),
            IndexType: value.index_type.into(),
            NumIndices: value.num_indices,
            NumInstances: value.num_instances,
        }
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

const_assert!(diligent_sys::RAYTRACING_GEOMETRY_FLAG_LAST == 2);

#[derive(Builder)]
pub struct BLASBuildBoundingBoxData<'a> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    geometry_name: CString,

    box_buffer: &'a Buffer,

    #[builder(default = 0)]
    box_offset: u64,

    box_stride: u32,

    box_count: u32,

    #[builder(default)]
    flags: RaytracingGeometryFlags,
}

#[derive(Builder)]
pub struct BLASBuildTriangleData<'a> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    geometry_name: CString,

    vertex_buffer: &'a Buffer,

    #[builder(default = 0)]
    vertex_offset: u64,

    vertex_stride: u32,

    vertex_count: usize,

    vertex_value_type: Option<ValueType>,

    #[builder(default = 0)]
    vertex_component_count: u8,

    primitive_count: usize,

    index_buffer: Option<&'a Buffer>,

    #[builder(default = 0)]
    index_offset: u64,

    index_type: Option<ValueType>,

    transform_buffer: Option<Buffer>,

    #[builder(default = 0)]
    transform_buffer_offset: u64,

    #[builder(default)]
    flags: RaytracingGeometryFlags,
}

#[derive(Builder)]
pub struct BuildBLASAttribs<'a> {
    blas: &'a BottomLevelAS,

    #[builder(default = ResourceStateTransitionMode::None)]
    blas_transition_mode: ResourceStateTransitionMode,

    #[builder(default = ResourceStateTransitionMode::None)]
    geometry_transition_mode: ResourceStateTransitionMode,

    #[builder(default)]
    #[builder(into)]
    triangle_data: Vec<BLASBuildTriangleData<'a>>,

    #[builder(default)]
    #[builder(into)]
    box_data: Vec<BLASBuildBoundingBoxData<'a>>,

    scratch_buffer: &'a Buffer,

    #[builder(default = 0)]
    scratch_buffer_offset: u64,

    #[builder(default = ResourceStateTransitionMode::None)]
    scratch_buffer_transition_mode: ResourceStateTransitionMode,

    #[builder(default = false)]
    update: bool,
}

#[derive(Builder)]
pub struct BuildTLASAttribs<'a> {
    tlas: &'a TopLevelAS,

    #[builder(default = ResourceStateTransitionMode::None)]
    tlas_transition_mode: ResourceStateTransitionMode,

    #[builder(default = ResourceStateTransitionMode::None)]
    blas_transition_mode: ResourceStateTransitionMode,

    #[builder(into)]
    instances: Vec<TLASBuildInstanceData<'a>>,

    instance_buffer: &'a Buffer,

    #[builder(default = 0)]
    instance_buffer_offset: u64,

    #[builder(default = ResourceStateTransitionMode::None)]
    instance_buffer_transition_mode: ResourceStateTransitionMode,

    #[builder(default = 1)]
    hit_group_stride: u32,

    #[builder(default = 0)]
    base_contribution_to_hit_group_index: u32,

    #[builder(default = HitGroupBindingMode::PerGeometry)]
    binding_mode: HitGroupBindingMode,

    scratch_buffer: &'a Buffer,

    #[builder(default = 0)]
    scratch_buffer_offset: u64,

    #[builder(default = ResourceStateTransitionMode::None)]
    scratch_buffer_transition_mode: ResourceStateTransitionMode,

    #[builder(default = false)]
    update: bool,
}

#[derive(Builder)]
pub struct UpdateIndirectRTBufferAttribs<'a> {
    attribs_buffer: &'a Buffer,

    #[builder(default = 0)]
    attribs_buffer_offset: u64,

    #[builder(default = ResourceStateTransitionMode::None)]
    transition_mode: ResourceStateTransitionMode,
}

#[derive(Builder)]
pub struct TraceRaysAttribs<'a> {
    sbt: &'a ShaderBindingTable,

    #[builder(default = 1)]
    dimension_x: u32,

    #[builder(default = 1)]
    dimension_y: u32,

    #[builder(default = 1)]
    dimension_z: u32,
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
    fn new(device_context: &'a DeviceContext, name: &CString, color: Option<[f32; 4]>) -> Self {
        unsafe {
            (*device_context.virtual_functions)
                .DeviceContext
                .BeginDebugGroup
                .unwrap_unchecked()(
                device_context.sys_ptr,
                name.as_ptr(),
                color.map_or(std::ptr::null(), |color| color.as_ptr()),
            )
        }
        ScopedDebugGroup { device_context }
    }
}

impl<'a> Drop for ScopedDebugGroup<'a> {
    fn drop(&mut self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .EndDebugGroup
                .unwrap_unchecked()(self.device_context.sys_ptr)
        }
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

#[derive(Builder)]
#[builder(derive(Clone))]
pub struct StateTransitionDesc<'a> {
    #[builder(with =|resource : &'a impl AsRef<DeviceObject>| resource.as_ref())]
    resource: &'a DeviceObject,

    new_state: ResourceState,

    #[builder(default = 0)]
    first_mip_level: u32,

    #[builder(default = diligent_sys::REMAINING_MIP_LEVELS)]
    mip_levels_count: u32,

    #[builder(default = 0)]
    first_array_slice: u32,

    #[builder(default = diligent_sys::REMAINING_ARRAY_SLICES)]
    array_slice_count: u32,

    old_state: Option<ResourceState>,

    #[builder(default = StateTransitionType::Immediate)]
    transition_type: StateTransitionType,

    #[builder(default)]
    flags: StateTransitionFlags,
}

impl<'a> From<&StateTransitionDesc<'a>> for diligent_sys::StateTransitionDesc {
    fn from(value: &StateTransitionDesc) -> Self {
        diligent_sys::StateTransitionDesc {
            pResource: value.resource.sys_ptr,
            NewState: value.new_state.bits(),
            OldState: value.old_state.as_ref().map_or(
                diligent_sys::RESOURCE_STATE_UNKNOWN as diligent_sys::RESOURCE_STATE,
                |state| state.bits(),
            ),
            FirstArraySlice: value.first_array_slice,
            ArraySliceCount: value.array_slice_count,
            FirstMipLevel: value.first_mip_level,
            MipLevelsCount: value.mip_levels_count,
            TransitionType: match value.transition_type {
                StateTransitionType::Immediate => diligent_sys::STATE_TRANSITION_TYPE_IMMEDIATE,
                StateTransitionType::Begin => diligent_sys::STATE_TRANSITION_TYPE_BEGIN,
                StateTransitionType::End => diligent_sys::STATE_TRANSITION_TYPE_END,
            } as diligent_sys::STATE_TRANSITION_TYPE,
            Flags: value.flags.bits(),
            // TODO
            pResourceBefore: std::ptr::null_mut(),
        }
    }
}

pub struct CommandList {
    pub(crate) sys_ptr: *mut diligent_sys::ICommandList,

    device_object: DeviceObject,
}

impl AsRef<DeviceObject> for CommandList {
    fn as_ref(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl CommandList {
    fn new(sys_ptr: *mut diligent_sys::ICommandList) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::ICommandList>()
        );

        CommandList {
            sys_ptr,
            device_object: DeviceObject::new(sys_ptr as *mut diligent_sys::IDeviceObject),
        }
    }
}

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
            pRenderPass: attribs.render_pass.sys_ptr,
            ClearValueCount: attribs.clear_values.len() as u32,
            pClearValues: clear_values.as_ptr() as *mut diligent_sys::OptimizedClearValue,
            StateTransitionMode: attribs.state_transition_mode.into(),
            pFramebuffer: attribs.frame_buffer.sys_ptr,
        };

        unsafe {
            (*context.virtual_functions)
                .DeviceContext
                .BeginRenderPass
                .unwrap_unchecked()(context.sys_ptr, std::ptr::from_ref(&attribs))
        }
        RenderPassToken { context }
    }

    pub fn next_subpass(&self) {
        unsafe {
            (*self.context.virtual_functions)
                .DeviceContext
                .NextSubpass
                .unwrap_unchecked()(self.context.sys_ptr)
        }
    }
}

impl Drop for RenderPassToken<'_> {
    fn drop(&mut self) {
        unsafe {
            (*self.context.virtual_functions)
                .DeviceContext
                .EndRenderPass
                .unwrap_unchecked()(self.context.sys_ptr)
        }
    }
}

pub struct ScopedQueryToken<'a, QueryDataType: GetSysQueryType + Default> {
    query: &'a Query<QueryDataType>,
    context: &'a DeviceContext,
}

impl<'a, QueryDataType: GetSysQueryType + Default> ScopedQueryToken<'a, QueryDataType> {
    pub fn new(context: &'a DeviceContext, query: &'a Query<QueryDataType>) -> Self {
        unsafe {
            (*context.virtual_functions)
                .DeviceContext
                .BeginQuery
                .unwrap_unchecked()(context.sys_ptr, query.sys_ptr);
        }

        Self { query, context }
    }
}

impl<'a, QueryDataType: GetSysQueryType + Default> Drop for ScopedQueryToken<'a, QueryDataType> {
    fn drop(&mut self) {
        unsafe {
            (*self.context.virtual_functions)
                .DeviceContext
                .EndQuery
                .unwrap_unchecked()(self.context.sys_ptr, self.query.sys_ptr);
        }
    }
}

pub struct DeviceContext {
    pub(crate) sys_ptr: *mut diligent_sys::IDeviceContext,
    pub(crate) virtual_functions: *mut diligent_sys::IDeviceContextVtbl,

    object: Object,
}

impl DeviceContext {
    pub(crate) fn new(device_context_ptr: *mut diligent_sys::IDeviceContext) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IDeviceContext>()
        );
        DeviceContext {
            sys_ptr: device_context_ptr,
            virtual_functions: unsafe { (*device_context_ptr).pVtbl },
            object: Object::new(device_context_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn get_desc(&self) -> &diligent_sys::DeviceContextDesc {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .GetDesc
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn set_pipeline_state(&self, pipeline_state: &PipelineState) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetPipelineState
                .unwrap_unchecked()(self.sys_ptr, pipeline_state.sys_ptr)
        }
    }

    pub fn transition_shader_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .TransitionShaderResources
                .unwrap_unchecked()(self.sys_ptr, shader_resource_binding.sys_ptr)
        }
    }

    pub fn commit_shader_resources(
        &self,
        shader_resource_binding: &ShaderResourceBinding,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .CommitShaderResources
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_resource_binding.sys_ptr,
                state_transition_mode.into(),
            )
        }
    }

    pub fn set_stencil_ref(&self, stencil_ref: u32) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetStencilRef
                .unwrap_unchecked()(self.sys_ptr, stencil_ref)
        }
    }

    pub fn set_blend_factors(&self, blend_factors: Option<&[f32; 4]>) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetBlendFactors
                .unwrap_unchecked()(
                self.sys_ptr,
                blend_factors.map_or(std::ptr::null(), |factors| factors.as_ptr()),
            )
        }
    }

    pub fn set_vertex_buffers<'a>(
        &self,
        buffers: &impl AsRef<[(&'a Buffer, u64)]>,
        state_transition_mode: ResourceStateTransitionMode,
        flags: SetVertexBufferFlags,
    ) {
        let num_buffers = buffers.as_ref().len();
        let (buffer_pointers, offsets): (Vec<_>, Vec<_>) = buffers
            .as_ref()
            .iter()
            .map(|&(buffer, offset)| (buffer.sys_ptr, offset))
            .unzip();

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetVertexBuffers
                .unwrap_unchecked()(
                self.sys_ptr,
                0,
                num_buffers as u32,
                buffer_pointers.as_ptr(),
                offsets.as_ptr(),
                state_transition_mode.into(),
                flags.bits(),
            )
        }
    }

    pub fn invalidate_state(&self) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .InvalidateState
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn set_index_buffer(
        &self,
        index_buffer: &Buffer,
        offset: u64,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetIndexBuffer
                .unwrap_unchecked()(
                self.sys_ptr,
                index_buffer.sys_ptr,
                offset,
                state_transition_mode.into(),
            )
        }
    }

    pub fn set_viewports(
        &self,
        viewports: &[&Viewport],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        let viewports: Vec<_> = viewports.iter().map(|&viewport| viewport.into()).collect();
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetViewports
                .unwrap_unchecked()(
                self.sys_ptr,
                viewports.len() as u32,
                viewports.as_ptr(),
                render_target_width,
                render_target_height,
            )
        }
    }

    pub fn set_scissor_rects(
        &self,
        rects: &[&Rect],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        let rects: Vec<_> = rects.iter().map(|&rect| rect.into()).collect();

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetScissorRects
                .unwrap_unchecked()(
                self.sys_ptr,
                rects.len() as u32,
                rects.as_ptr(),
                render_target_width,
                render_target_height,
            )
        }
    }

    pub fn set_render_targets(
        &self,
        render_targets: &[&TextureView],
        depth_stencil: Option<&TextureView>,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        let num_render_targets = render_targets.len();
        let mut render_target_pointers = Vec::from_iter(
            render_targets
                .iter()
                .map(|render_targets| render_targets.sys_ptr),
        );

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetRenderTargets
                .unwrap_unchecked()(
                self.sys_ptr,
                num_render_targets as u32,
                render_target_pointers.as_mut_ptr(),
                depth_stencil.map_or(std::ptr::null_mut(), |v| v.sys_ptr),
                state_transition_mode.into(),
            )
        }
    }

    pub fn new_render_pass(&self, attribs: &BeginRenderPassAttribs) -> RenderPassToken {
        RenderPassToken::new(self, attribs)
    }

    pub fn draw(&self, attribs: &DrawAttribs) {
        let attribs = attribs.into();
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .Draw
                .unwrap_unchecked()(self.sys_ptr, std::ptr::addr_of!(attribs))
        }
    }

    pub fn draw_indexed(&self, attribs: &DrawIndexedAttribs) {
        let attribs = attribs.into();
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DrawIndexed
                .unwrap_unchecked()(self.sys_ptr, std::ptr::addr_of!(attribs))
        }
    }

    pub fn draw_indirect(&self, attribs: &diligent_sys::DrawIndirectAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DrawIndirect
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn draw_indexed_indirect(&self, attribs: &diligent_sys::DrawIndexedIndirectAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DrawIndexedIndirect
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn draw_mesh(&self, attribs: &diligent_sys::DrawMeshAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DrawMesh
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn draw_mesh_indirect(&self, attribs: &diligent_sys::DrawMeshIndirectAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DrawMeshIndirect
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn multi_draw(&self, attribs: &diligent_sys::MultiDrawAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .MultiDraw
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn multi_draw_indexed(&self, attribs: &diligent_sys::MultiDrawIndexedAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .MultiDrawIndexed
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn dispatch_compute(&self, attribs: &diligent_sys::DispatchComputeAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DispatchCompute
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn dispatch_compute_indirect(
        &self,
        attribs: &diligent_sys::DispatchComputeIndirectAttribs,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DispatchComputeIndirect
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn dispatch_tile(&self, attribs: &diligent_sys::DispatchTileAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DispatchTile
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn get_tile_size(&self) -> (u32, u32) {
        let mut tile_size_x: u32 = 0;
        let mut tile_size_y: u32 = 0;
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .GetTileSize
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::addr_of_mut!(tile_size_x),
                std::ptr::addr_of_mut!(tile_size_y),
            )
        };
        (tile_size_x, tile_size_y)
    }

    pub fn clear_depth(
        &self,
        view: &mut TextureView,
        depth: f32,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .ClearDepthStencil
                .unwrap_unchecked()(
                self.sys_ptr,
                view.sys_ptr,
                diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
                depth,
                0,
                state_transition_mode.into(),
            )
        }
    }

    pub fn clear_stencil(
        &self,
        view: &mut TextureView,
        stencil: u8,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .ClearDepthStencil
                .unwrap_unchecked()(
                self.sys_ptr,
                view.sys_ptr,
                diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
                0.0,
                stencil,
                state_transition_mode.into(),
            )
        }
    }

    pub fn clear_depth_stencil(
        &self,
        view: &mut TextureView,
        depth: f32,
        stencil: u8,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .ClearDepthStencil
                .unwrap_unchecked()(
                self.sys_ptr,
                view.sys_ptr,
                diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS
                    | diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
                depth,
                stencil,
                state_transition_mode.into(),
            )
        }
    }

    pub fn clear_render_target<T>(
        &self,
        view: &mut TextureView,
        rgba: &[T; 4],
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .ClearRenderTarget
                .unwrap_unchecked()(
                self.sys_ptr,
                view.sys_ptr,
                (*rgba).as_ptr() as *const std::os::raw::c_void,
                state_transition_mode.into(),
            )
        }
    }

    pub fn enqueue_signal(&self, fence: &Fence, value: u64) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .EnqueueSignal
                .unwrap_unchecked()(self.sys_ptr, fence.sys_ptr, value)
        }
    }

    pub fn update_buffer<T>(
        &self,
        buffer: &mut Buffer,
        offset: u64,
        size: u64,
        data: &T,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .UpdateBuffer
                .unwrap_unchecked()(
                self.sys_ptr,
                buffer.sys_ptr,
                offset,
                size,
                std::ptr::from_ref(data) as *const std::os::raw::c_void,
                state_transition_mode.into(),
            )
        }
    }

    pub fn update_buffer_from_slice<T>(
        &self,
        buffer: &mut Buffer,
        data: &[T],
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .UpdateBuffer
                .unwrap_unchecked()(
                self.sys_ptr,
                buffer.sys_ptr,
                0,
                data.len() as u64 * std::mem::size_of::<T>() as u64,
                data.as_ptr() as *const std::os::raw::c_void,
                state_transition_mode.into(),
            )
        }
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
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .CopyBuffer
                .unwrap_unchecked()(
                self.sys_ptr,
                src_buffer.sys_ptr,
                src_offset,
                src_buffer_transition_mode.into(),
                dst_buffer.sys_ptr,
                dst_offset,
                size,
                dst_buffer_transition_mode.into(),
            )
        }
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

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .UpdateTexture
                .unwrap_unchecked()(
                self.sys_ptr,
                texture.sys_ptr,
                mip_level,
                slice,
                std::ptr::from_ref(dst_box),
                std::ptr::addr_of!(subres_data),
                src_buffer_transition_mode.into(),
                texture_transition_mode.into(),
            )
        }
    }

    pub fn copy_texture(&self, copy_attribs: &diligent_sys::CopyTextureAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .CopyTexture
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(copy_attribs))
        }
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
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .GenerateMips
                .unwrap_unchecked()(self.sys_ptr, texture_view.sys_ptr)
        }
    }

    pub fn finish_frame(&self) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .FinishFrame
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_frame_number(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .GetFrameNumber
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn transition_resource_states<'a>(&self, barriers: &impl AsRef<[StateTransitionDesc<'a>]>) {
        let barriers = barriers
            .as_ref()
            .iter()
            .map(|state_transition_desc| state_transition_desc.into())
            .collect::<Vec<_>>();

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .TransitionResourceStates
                .unwrap_unchecked()(
                self.sys_ptr, barriers.len() as u32, barriers.as_ptr()
            )
        }
    }

    pub fn resolve_texture_subresource(
        &self,
        src_texture: &Texture,
        dst_texture: &mut Texture,
        resolve_attribs: &diligent_sys::ResolveTextureSubresourceAttribs,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .ResolveTextureSubresource
                .unwrap_unchecked()(
                self.sys_ptr,
                src_texture.sys_ptr,
                dst_texture.sys_ptr,
                std::ptr::from_ref(resolve_attribs),
            )
        }
    }

    pub fn build_blas(&self, attribs: &BuildBLASAttribs) {
        let triangles = attribs
            .triangle_data
            .iter()
            .map(|triangle| diligent_sys::BLASBuildTriangleData {
                GeometryName: triangle.geometry_name.as_ptr(),
                pVertexBuffer: triangle.vertex_buffer.sys_ptr,
                VertexOffset: triangle.vertex_offset,
                VertexStride: triangle.vertex_stride,
                VertexCount: triangle.vertex_count as u32,
                VertexValueType: triangle
                    .vertex_value_type
                    .map_or(diligent_sys::VT_UNDEFINED as _, |vt| vt.into()),
                VertexComponentCount: triangle.vertex_component_count,
                PrimitiveCount: triangle.primitive_count as u32,
                pIndexBuffer: triangle
                    .index_buffer
                    .map_or(std::ptr::null_mut(), |ib| ib.sys_ptr),
                IndexOffset: triangle.index_offset,
                IndexType: triangle
                    .index_type
                    .map_or(diligent_sys::VT_UNDEFINED as _, |vt| vt.into()),
                pTransformBuffer: triangle
                    .transform_buffer
                    .as_ref()
                    .map_or(std::ptr::null_mut(), |tb| tb.sys_ptr),
                TransformBufferOffset: triangle.transform_buffer_offset,
                Flags: triangle.flags.bits(),
            })
            .collect::<Vec<_>>();

        let boxes = attribs
            .box_data
            .iter()
            .map(|box_data| diligent_sys::BLASBuildBoundingBoxData {
                GeometryName: box_data.geometry_name.as_ptr(),
                pBoxBuffer: box_data.box_buffer.sys_ptr,
                BoxOffset: box_data.box_offset,
                BoxStride: box_data.box_stride,
                BoxCount: box_data.box_count,
                Flags: box_data.flags.bits(),
            })
            .collect::<Vec<_>>();

        let attribs = diligent_sys::BuildBLASAttribs {
            pBLAS: attribs.blas.sys_ptr,
            BLASTransitionMode: attribs.blas_transition_mode.into(),
            GeometryTransitionMode: attribs.geometry_transition_mode.into(),
            pTriangleData: if triangles.is_empty() {
                std::ptr::null()
            } else {
                triangles.as_ptr()
            },
            TriangleDataCount: triangles.len() as u32,
            pBoxData: if boxes.is_empty() {
                std::ptr::null()
            } else {
                boxes.as_ptr()
            },
            BoxDataCount: boxes.len() as u32,
            pScratchBuffer: attribs.scratch_buffer.sys_ptr,
            ScratchBufferOffset: attribs.scratch_buffer_offset,
            ScratchBufferTransitionMode: attribs.scratch_buffer_transition_mode.into(),
            Update: attribs.update,
        };

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .BuildBLAS
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(&attribs))
        }
    }

    pub fn build_tlas<'a>(&self, attribs: &BuildTLASAttribs<'a>) {
        let instances = attribs
            .instances
            .iter()
            .map(|instance| instance.into())
            .collect::<Vec<_>>();
        let attribs = diligent_sys::BuildTLASAttribs {
            pTLAS: attribs.tlas.sys_ptr,
            TLASTransitionMode: attribs.tlas_transition_mode.into(),
            BLASTransitionMode: attribs.blas_transition_mode.into(),
            pInstances: instances.as_ptr(),
            InstanceCount: instances.len() as u32,
            pInstanceBuffer: attribs.instance_buffer.sys_ptr,
            InstanceBufferOffset: attribs.instance_buffer_offset,
            InstanceBufferTransitionMode: attribs.instance_buffer_transition_mode.into(),
            HitGroupStride: attribs.hit_group_stride,
            BaseContributionToHitGroupIndex: attribs.base_contribution_to_hit_group_index,
            BindingMode: attribs.binding_mode.into(),
            pScratchBuffer: attribs.scratch_buffer.sys_ptr,
            ScratchBufferOffset: attribs.scratch_buffer_offset,
            ScratchBufferTransitionMode: attribs.scratch_buffer_transition_mode.into(),
            Update: attribs.update,
        };

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .BuildTLAS
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(&attribs))
        }
    }

    pub fn copy_blas(&self, attribs: &diligent_sys::CopyBLASAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .CopyBLAS
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn copy_tlas(&self, attribs: &diligent_sys::CopyTLASAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .CopyTLAS
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn write_blas_compacted_size(&self, attribs: &diligent_sys::WriteBLASCompactedSizeAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .WriteBLASCompactedSize
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn write_tlas_compacted_size(&self, attribs: &diligent_sys::WriteTLASCompactedSizeAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .WriteTLASCompactedSize
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn trace_rays(&self, attribs: &TraceRaysAttribs) {
        let attribs = diligent_sys::TraceRaysAttribs {
            pSBT: attribs.sbt.sys_ptr,
            DimensionX: attribs.dimension_x,
            DimensionY: attribs.dimension_y,
            DimensionZ: attribs.dimension_z,
        };

        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .TraceRays
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(&attribs))
        }
    }

    pub fn trace_rays_indirect(&self, attribs: &diligent_sys::TraceRaysIndirectAttribs) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .TraceRaysIndirect
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn update_sbt(
        &self,
        sbt: &ShaderBindingTable,
        attribs: Option<&UpdateIndirectRTBufferAttribs>,
    ) {
        let attribs = attribs.map(|attribs| diligent_sys::UpdateIndirectRTBufferAttribs {
            pAttribsBuffer: attribs.attribs_buffer.sys_ptr,
            AttribsBufferOffset: attribs.attribs_buffer_offset,
            TransitionMode: attribs.transition_mode.into(),
        });
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .UpdateSBT
                .unwrap_unchecked()(
                self.sys_ptr,
                sbt.sys_ptr,
                attribs.map_or(std::ptr::null_mut(), |attribs| std::ptr::from_ref(&attribs)),
            )
        }
    }

    #[allow(private_bounds)]
    pub fn set_user_data(&self, user_data: &impl AsRef<Object>) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetUserData
                .unwrap_unchecked()(self.sys_ptr, user_data.as_ref().object)
        }
    }

    // TODO
    // pub fn get_user_data(&self);

    pub fn debug_group(&self, name: impl AsRef<str>, color: Option<[f32; 4]>) -> ScopedDebugGroup {
        let name = CString::new(name.as_ref()).unwrap();
        ScopedDebugGroup::new(self, &name, color)
    }

    pub fn insert_debug_label(&self, label: impl AsRef<str>, color: Option<[f32; 4]>) {
        let label = CString::new(label.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .InsertDebugLabel
                .unwrap_unchecked()(
                self.sys_ptr,
                label.as_ptr(),
                color.map_or(std::ptr::null(), |color| color.as_ptr()),
            )
        }
    }

    pub fn set_shading_rate(
        &self,
        base_rate: ShadingRate,
        primitive_combiner: &ShadingRateCombiner,
        texture_combiner: &ShadingRateCombiner,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .SetShadingRate
                .unwrap_unchecked()(
                self.sys_ptr,
                base_rate.into(),
                primitive_combiner.bits(),
                texture_combiner.bits(),
            )
        }
    }

    pub fn clear_stats(&self) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .ClearStats
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_stats(&self) -> &diligent_sys::DeviceContextStats {
        // TODO
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .GetStats
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }
}

impl AsRef<Object> for DeviceContext {
    fn as_ref(&self) -> &Object {
        &self.object
    }
}

pub struct ImmediateDeviceContext {
    device_context: DeviceContext,
}

impl Deref for ImmediateDeviceContext {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        &self.device_context
    }
}
impl ImmediateDeviceContext {
    pub(crate) fn new(device_context_ptr: *mut diligent_sys::IDeviceContext) -> Self {
        ImmediateDeviceContext {
            device_context: DeviceContext::new(device_context_ptr),
        }
    }

    pub fn flush(&self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .Flush
                .unwrap_unchecked()(self.device_context.sys_ptr)
        }
    }

    pub fn execute_command_lists<'a>(&self, command_lists: &impl AsRef<[&'a CommandList]>) {
        let command_lists = command_lists
            .as_ref()
            .iter()
            .map(|&command_list| command_list.sys_ptr)
            .collect::<Vec<_>>();

        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .ExecuteCommandLists
                .unwrap_unchecked()(
                self.device_context.sys_ptr,
                command_lists.len() as u32,
                command_lists.as_ptr(),
            )
        }
    }

    pub fn wait_for_idle(&self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .WaitForIdle
                .unwrap_unchecked()(self.device_context.sys_ptr)
        }
    }

    pub fn lock_command_queue(&self) -> Result<CommandQueue, ()> {
        CommandQueue::new(self)
    }

    pub fn begin_query<'a, QueryDataType: GetSysQueryType + Default>(
        &'a self,
        query: &'a Query<QueryDataType>,
    ) -> ScopedQueryToken<'a, QueryDataType> {
        ScopedQueryToken::<QueryDataType>::new(&self, query)
    }

    pub fn bind_sparse_resource_memory(
        &self,
        attribs: &diligent_sys::BindSparseResourceMemoryAttribs,
    ) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .BindSparseResourceMemory
                .unwrap_unchecked()(self.sys_ptr, std::ptr::from_ref(attribs))
        }
    }

    pub fn device_wait_for_fence(&self, fence: &Fence, value: u64) {
        unsafe {
            (*self.virtual_functions)
                .DeviceContext
                .DeviceWaitForFence
                .unwrap_unchecked()(self.sys_ptr, fence.sys_ptr, value)
        }
    }
}

pub struct DeferredDeviceContext {
    device_context: DeviceContext,
}

impl Deref for DeferredDeviceContext {
    type Target = DeviceContext;
    fn deref(&self) -> &Self::Target {
        &self.device_context
    }
}

impl DeferredDeviceContext {
    #[allow(dead_code)] // In case backends that doesn't support deffered contexts like OpenGL are used
    pub(crate) fn new(device_context_ptr: *mut diligent_sys::IDeviceContext) -> Self {
        DeferredDeviceContext {
            device_context: DeviceContext::new(device_context_ptr),
        }
    }

    pub fn begin(&self, immediate_context_id: u32) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .Begin
                .unwrap_unchecked()(self.device_context.sys_ptr, immediate_context_id)
        }
    }

    pub fn finish_command_list(&self) -> Result<CommandList, ()> {
        let mut command_list_ptr = std::ptr::null_mut();
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .FinishCommandList
                .unwrap_unchecked()(
                self.device_context.sys_ptr,
                std::ptr::addr_of_mut!(command_list_ptr),
            );
        }
        if command_list_ptr.is_null() {
            Err(())
        } else {
            Ok(CommandList::new(command_list_ptr))
        }
    }
}
