use bitflags::bitflags;

use super::{
    buffer::{Buffer, BufferMapReadToken, BufferMapReadWriteToken, BufferMapWriteToken},
    fence::Fence,
    graphics_types::{MapFlags, ShadingRate, ShadingRateCombiner, ValueType},
    object::{AsObject, Object},
    pipeline_state::PipelineState,
    shader_resource_binding::ShaderResourceBinding,
    texture::{Texture, TextureSubResource},
    texture_view::TextureView,
};

bitflags! {
    pub struct DrawFlags: diligent_sys::DRAW_FLAGS {
        const None                         = diligent_sys::DRAW_FLAG_NONE as diligent_sys::DRAW_FLAGS;
        const VerifyStates                 = diligent_sys::DRAW_FLAG_VERIFY_STATES as diligent_sys::DRAW_FLAGS;
        const VerifyDrawAttribs            = diligent_sys::DRAW_FLAG_VERIFY_DRAW_ATTRIBS as diligent_sys::DRAW_FLAGS;
        const VerifyRenderTargets          = diligent_sys::DRAW_FLAG_VERIFY_RENDER_TARGETS as diligent_sys::DRAW_FLAGS;
        const VerifyAll                    = diligent_sys::DRAW_FLAG_VERIFY_ALL as diligent_sys::DRAW_FLAGS;
        const DynamicResourceBuffersIntact = diligent_sys::DRAW_FLAG_DYNAMIC_RESOURCE_BUFFERS_INTACT as diligent_sys::DRAW_FLAGS;
    }
}

bitflags! {
    pub struct SetVertexBufferFlags: diligent_sys::SET_VERTEX_BUFFERS_FLAGS {
        const None  = diligent_sys::SET_VERTEX_BUFFERS_FLAG_NONE as diligent_sys::SET_VERTEX_BUFFERS_FLAGS;
        const Reset = diligent_sys::SET_VERTEX_BUFFERS_FLAG_RESET as diligent_sys::SET_VERTEX_BUFFERS_FLAGS;
    }
}

pub struct DrawAttribs {
    num_vertices: u32,
    flags: DrawFlags,
    num_instances: u32,
    start_vertex_location: u32,
    first_instance_location: u32,
}

impl DrawAttribs {
    pub fn new(num_vertices: u32) -> Self {
        DrawAttribs {
            num_vertices: num_vertices,
            flags: DrawFlags::None,
            num_instances: 1,
            start_vertex_location: 0,
            first_instance_location: 0,
        }
    }

    pub fn flags(mut self, flags: DrawFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn num_instances(mut self, num_instances: u32) -> Self {
        self.num_instances = num_instances;
        self
    }
    pub fn start_vertex_location(mut self, start_vertex_location: u32) -> Self {
        self.start_vertex_location = start_vertex_location;
        self
    }
    pub fn first_instance_location(mut self, first_instance_location: u32) -> Self {
        self.first_instance_location = first_instance_location;
        self
    }
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

pub struct DrawIndexedAttribs {
    num_indices: u32,
    index_type: ValueType,

    flags: DrawFlags,
    num_instances: u32,
    first_index_location: u32,
    base_vertex: u32,
    first_instance_location: u32,
}

impl DrawIndexedAttribs {
    pub fn new(num_indices: u32, index_type: ValueType) -> Self {
        DrawIndexedAttribs {
            num_indices,
            index_type,

            flags: DrawFlags::None,
            num_instances: 1,
            first_index_location: 0,
            base_vertex: 0,
            first_instance_location: 0,
        }
    }

    pub fn flags(mut self, flags: DrawFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn num_instances(mut self, num_instances: u32) -> Self {
        self.num_instances = num_instances;
        self
    }
    pub fn first_index_location(mut self, first_index_location: u32) -> Self {
        self.first_index_location = first_index_location;
        self
    }
    pub fn base_vertex(mut self, base_vertex: u32) -> Self {
        self.base_vertex = base_vertex;
        self
    }
    pub fn first_instance_location(mut self, first_instance_location: u32) -> Self {
        self.first_instance_location = first_instance_location;
        self
    }
}

impl From<&DrawIndexedAttribs> for diligent_sys::DrawIndexedAttribs {
    fn from(value: &DrawIndexedAttribs) -> Self {
        diligent_sys::DrawIndexedAttribs {
            BaseVertex: value.base_vertex,
            FirstIndexLocation: value.first_index_location,
            FirstInstanceLocation: value.first_instance_location,
            Flags: value.flags.bits(),
            IndexType: diligent_sys::VALUE_TYPE::from(&value.index_type),
            NumIndices: value.num_indices,
            NumInstances: value.num_instances,
        }
    }
}

pub enum ResourceStateTransitionMode {
    None,
    Transition,
    Verify,
}

impl From<&ResourceStateTransitionMode> for diligent_sys::RESOURCE_STATE_TRANSITION_MODE {
    fn from(value: &ResourceStateTransitionMode) -> Self {
        (match value {
            ResourceStateTransitionMode::None => diligent_sys::RESOURCE_STATE_TRANSITION_MODE_NONE,
            ResourceStateTransitionMode::Transition => {
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE_TRANSITION
            }
            ResourceStateTransitionMode::Verify => {
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE_VERIFY
            }
        }) as diligent_sys::RESOURCE_STATE_TRANSITION_MODE
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
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }

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

pub(crate) struct DeviceContextCommon {
    pub(crate) device_context_ptr: *mut diligent_sys::IDeviceContext,
    pub(crate) virtual_functions: *mut diligent_sys::IDeviceContextVtbl,

    object: Object,
}
impl DeviceContextCommon {
    fn new(device_context_ptr: *mut diligent_sys::IDeviceContext) -> Self {
        DeviceContextCommon {
            device_context_ptr,
            virtual_functions: unsafe { (*device_context_ptr).pVtbl },
            object: Object::new(device_context_ptr as *mut diligent_sys::IObject),
        }
    }
}

pub(crate) trait AsDeviceContextCommon {
    fn as_device_context(&self) -> &DeviceContextCommon;
}
impl AsDeviceContextCommon for ImmediateDeviceContext {
    fn as_device_context(&self) -> &DeviceContextCommon {
        &self.device_context
    }
}
impl AsDeviceContextCommon for DeferredDeviceContext {
    fn as_device_context(&self) -> &DeviceContextCommon {
        &self.device_context
    }
}

// I really don't know why this is a warning ? It would be worse if I make everything public, right ?
#[allow(private_bounds)]
pub trait DeviceContext
where
    Self: AsDeviceContextCommon,
{
    fn get_desc(&self) -> &diligent_sys::DeviceContextDesc {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .GetDesc
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn set_pipeline_state(&self, pipeline_state: &PipelineState) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetPipelineState
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                pipeline_state.pipeline_state,
            )
        }
    }

    fn transition_shader_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .TransitionShaderResources
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                shader_resource_binding.shader_resource_binding,
            )
        }
    }

    fn commit_shader_resources(
        &self,
        shader_resource_binding: &ShaderResourceBinding,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .CommitShaderResources
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                shader_resource_binding.shader_resource_binding,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn set_stencil_ref(&self, stencil_ref: u32) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetStencilRef
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr, stencil_ref
            )
        }
    }

    fn set_blend_factors(&self, blend_factors: &[f32; 4]) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetBlendFactors
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                blend_factors.as_ptr(),
            )
        }
    }

    fn set_vertex_buffers(
        &self,
        buffers: &[&Buffer],
        offsets: &[u64],
        state_transition_mode: ResourceStateTransitionMode,
        flags: SetVertexBufferFlags,
    ) {
        let num_buffers = buffers.len();
        let buffer_pointers = Vec::from_iter(buffers.iter().map(|buffer| buffer.buffer));
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetVertexBuffers
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                0,
                num_buffers as u32,
                buffer_pointers.as_ptr(),
                offsets.as_ptr(),
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
                flags.bits(),
            )
        }
    }

    fn invalidate_state(&self) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .InvalidateState
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn set_index_buffer(
        &self,
        index_buffer: &Buffer,
        offset: u64,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetIndexBuffer
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                index_buffer.buffer,
                offset,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn set_viewports(
        &self,
        viewports: &[&Viewport],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        let viewports: Vec<_> = viewports
            .iter()
            .map(|&viewport| diligent_sys::Viewport::from(viewport))
            .collect();
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetViewports
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                viewports.len() as u32,
                viewports.as_ptr(),
                render_target_width,
                render_target_height,
            )
        }
    }

    fn set_scissor_rects(
        &self,
        rects: &[&Rect],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        let rects: Vec<_> = rects
            .iter()
            .map(|&rect| diligent_sys::Rect::from(rect))
            .collect();

        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetScissorRects
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                rects.len() as u32,
                rects.as_ptr(),
                render_target_width,
                render_target_height,
            )
        }
    }

    fn set_render_targets(
        &self,
        render_targets: &[&TextureView],
        depth_stencil: Option<&TextureView>,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        let num_render_targets = render_targets.len();
        let mut render_target_pointers = Vec::from_iter(
            render_targets
                .iter()
                .map(|render_targets| render_targets.texture_view),
        );

        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetRenderTargets
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                num_render_targets as u32,
                render_target_pointers.as_mut_ptr(),
                depth_stencil.map_or(std::ptr::null_mut(), |v| v.texture_view),
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn begin_render_pass(&self, attribs: &diligent_sys::BeginRenderPassAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .BeginRenderPass
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn next_subpass(&self) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .NextSubpass
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn end_render_pass(&self) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .EndRenderPass
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn draw(&self, attribs: &DrawAttribs) {
        let attribs = diligent_sys::DrawAttribs::from(attribs);
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .Draw
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::addr_of!(attribs),
            )
        }
    }

    fn draw_indexed(&self, attribs: &DrawIndexedAttribs) {
        let attribs = diligent_sys::DrawIndexedAttribs::from(attribs);
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DrawIndexed
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::addr_of!(attribs),
            )
        }
    }

    fn draw_indirect(&self, attribs: &diligent_sys::DrawIndirectAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DrawIndirect
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn draw_indexed_indirect(&self, attribs: &diligent_sys::DrawIndexedIndirectAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DrawIndexedIndirect
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn draw_mesh(&self, attribs: &diligent_sys::DrawMeshAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DrawMesh
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn draw_mesh_indirect(&self, attribs: &diligent_sys::DrawMeshIndirectAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DrawMeshIndirect
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn multi_draw(&self, attribs: &diligent_sys::MultiDrawAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .MultiDraw
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn multi_draw_indexed(&self, attribs: &diligent_sys::MultiDrawIndexedAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .MultiDrawIndexed
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn dispatch_compute(&self, attribs: &diligent_sys::DispatchComputeAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DispatchCompute
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn dispatch_compute_indirect(&self, attribs: &diligent_sys::DispatchComputeIndirectAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DispatchComputeIndirect
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn dispatch_tile(&self, attribs: &diligent_sys::DispatchTileAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DispatchTile
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn get_tile_size(&self) -> (u32, u32) {
        let mut tile_size_x: u32 = 0;
        let mut tile_size_y: u32 = 0;
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .GetTileSize
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::addr_of_mut!(tile_size_x),
                std::ptr::addr_of_mut!(tile_size_y),
            )
        };
        (tile_size_x, tile_size_y)
    }

    fn clear_depth(
        &self,
        view: &mut TextureView,
        depth: f32,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .ClearDepthStencil
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                view.texture_view,
                diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
                depth,
                0,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn clear_stencil(
        &self,
        view: &mut TextureView,
        stencil: u8,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .ClearDepthStencil
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                view.texture_view,
                diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
                0.0,
                stencil,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn clear_depth_stencil(
        &self,
        view: &mut TextureView,
        depth: f32,
        stencil: u8,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .ClearDepthStencil
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                view.texture_view,
                diligent_sys::CLEAR_STENCIL_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS
                    | diligent_sys::CLEAR_DEPTH_FLAG as diligent_sys::CLEAR_DEPTH_STENCIL_FLAGS,
                depth,
                stencil,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn clear_render_target<T>(
        &self,
        view: &mut TextureView,
        rgba: &[T; 4],
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .ClearRenderTarget
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                view.texture_view,
                (*rgba).as_ptr() as *const std::os::raw::c_void,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn enqueue_signal(&self, fence: &Fence, value: u64) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .EnqueueSignal
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                fence.fence,
                value,
            )
        }
    }

    fn device_wait_for_fence(&self, fence: &Fence, value: u64) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .DeviceWaitForFence
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                fence.fence,
                value,
            )
        }
    }

    //fn begin_query(&self, query: &mut Query) {
    //    todo!()
    //}
    //fn end_query(&self, query: &mut Query) {
    //    todo!()
    //}

    fn update_buffer<T>(
        &self,
        buffer: &mut Buffer,
        offset: u64,
        size: u64,
        data: &T,
        state_transition_mode: ResourceStateTransitionMode,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .UpdateBuffer
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                buffer.buffer,
                offset,
                size,
                std::ptr::from_ref(data) as *const std::os::raw::c_void,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&state_transition_mode),
            )
        }
    }

    fn copy_buffer(
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
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .CopyBuffer
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                src_buffer.buffer,
                src_offset,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&src_buffer_transition_mode),
                dst_buffer.buffer,
                dst_offset,
                size,
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&dst_buffer_transition_mode),
            )
        }
    }

    fn map_buffer_read<'a, T>(
        &'a self,
        buffer: &'a Buffer,
        map_flags: MapFlags,
    ) -> BufferMapReadToken<'a, T, Self>
    where
        Self: Sized,
    {
        BufferMapReadToken::new(self, buffer, map_flags.bits())
    }

    fn map_buffer_write<'a, T>(
        &'a self,
        buffer: &'a Buffer,
        map_flags: MapFlags,
    ) -> BufferMapWriteToken<'a, T, Self>
    where
        Self: Sized,
    {
        BufferMapWriteToken::new(self, buffer, map_flags.bits())
    }

    fn map_buffer_read_write<'a, T>(
        &'a self,
        buffer: &'a Buffer,
        map_flags: MapFlags,
    ) -> BufferMapReadWriteToken<'a, T, Self>
    where
        Self: Sized,
    {
        BufferMapReadWriteToken::new(self, buffer, map_flags.bits())
    }

    fn update_texture(
        &self,
        texture: &mut Texture,
        mip_level: u32,
        slice: u32,
        dst_box: &diligent_sys::Box,
        subres_data: &TextureSubResource,
        src_buffer_transition_mode: ResourceStateTransitionMode,
        texture_transition_mode: ResourceStateTransitionMode,
    ) {
        let subres_data = diligent_sys::TextureSubResData::from(subres_data);

        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .UpdateTexture
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                texture.texture,
                mip_level,
                slice,
                std::ptr::from_ref(dst_box),
                std::ptr::addr_of!(subres_data),
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&src_buffer_transition_mode),
                diligent_sys::RESOURCE_STATE_TRANSITION_MODE::from(&texture_transition_mode),
            )
        }
    }

    fn copy_texture(&self, copy_attribs: &diligent_sys::CopyTextureAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .CopyTexture
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(copy_attribs),
            )
        }
    }

    // TODO
    //fn map_texture_subresource(&self, )
    //{
    //
    //}

    fn unmap_texture_subresource(&self, texture: &mut Texture, mip_level: u32, array_slice: u32) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .UnmapTextureSubresource
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                texture.texture,
                mip_level,
                array_slice,
            )
        }
    }

    fn generate_mips(&self, texture_view: &mut TextureView) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .GenerateMips
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                texture_view.texture_view,
            )
        }
    }

    fn finish_frame(&self) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .FinishFrame
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn get_frame_number(&self) -> u64 {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .GetFrameNumber
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn transition_resource_states(&self, barriers: &[diligent_sys::StateTransitionDesc]) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .TransitionResourceStates
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                barriers.len() as u32,
                barriers.as_ptr(),
            )
        }
    }

    fn resolve_texture_subresource(
        &self,
        src_texture: &Texture,
        dst_texture: &mut Texture,
        resolve_attribs: &diligent_sys::ResolveTextureSubresourceAttribs,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .ResolveTextureSubresource
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                src_texture.texture,
                dst_texture.texture,
                std::ptr::from_ref(resolve_attribs),
            )
        }
    }

    fn build_blas(&self, attribs: &diligent_sys::BuildBLASAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .BuildBLAS
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn build_tlas(&self, attribs: &diligent_sys::BuildTLASAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .BuildTLAS
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn copy_blas(&self, attribs: &diligent_sys::CopyBLASAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .CopyBLAS
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn copy_tlas(&self, attribs: &diligent_sys::CopyTLASAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .CopyTLAS
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn write_blas_compacted_size(&self, attribs: &diligent_sys::WriteBLASCompactedSizeAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .WriteBLASCompactedSize
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn write_tlas_compacted_size(&self, attribs: &diligent_sys::WriteTLASCompactedSizeAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .WriteTLASCompactedSize
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn trace_rays(&self, attribs: &diligent_sys::TraceRaysAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .TraceRays
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn trace_rays_indirect(&self, attribs: &diligent_sys::TraceRaysIndirectAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .TraceRaysIndirect
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    // TODO
    // fn update_sbt(&self, sbt : &mut ShaderBindingTable) {}

    #[allow(private_bounds)]
    fn set_user_data<Data>(&self, user_data: &Data)
    where
        Data: AsObject,
    {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetUserData
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                user_data.as_object().object,
            )
        }
    }

    // TODO
    // fn get_user_data(&self);

    fn begin_debug_group(&self, name: &str, color: [f32; 4]) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .BeginDebugGroup
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                name.as_ptr(),
                color.as_ptr(),
            )
        }
    }

    fn end_debug_group(&self) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .EndDebugGroup
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn insert_debug_label(&self, name: &str, color: [f32; 4]) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .InsertDebugLabel
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                name.as_ptr(),
                color.as_ptr(),
            )
        }
    }

    //fn lock_command_queue(&self) -> CommandQueue
    //{
    //}

    fn unlock_command_queue(&self) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .UnlockCommandQueue
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn set_shading_rate(
        &self,
        base_rate: &ShadingRate,
        primitive_combiner: &ShadingRateCombiner,
        texture_combiner: &ShadingRateCombiner,
    ) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .SetShadingRate
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                diligent_sys::SHADING_RATE::from(base_rate),
                primitive_combiner.bits(),
                texture_combiner.bits(),
            )
        }
    }

    fn bind_sparse_resource_memory(&self, attribs: &diligent_sys::BindSparseResourceMemoryAttribs) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .BindSparseResourceMemory
                .unwrap_unchecked()(
                self.as_device_context().device_context_ptr,
                std::ptr::from_ref(attribs),
            )
        }
    }

    fn clear_stats(&self) {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .ClearStats
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
        }
    }

    fn get_stats(&self) -> &diligent_sys::DeviceContextStats {
        unsafe {
            (*self.as_device_context().virtual_functions)
                .DeviceContext
                .GetStats
                .unwrap_unchecked()(self.as_device_context().device_context_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }
}

pub struct ImmediateDeviceContext {
    device_context: DeviceContextCommon,
}

impl AsObject for ImmediateDeviceContext {
    fn as_object(&self) -> &Object {
        &self.device_context.object
    }
}

impl DeviceContext for ImmediateDeviceContext {}

impl ImmediateDeviceContext {
    pub(crate) fn new(device_context_ptr: *mut diligent_sys::IDeviceContext) -> Self {
        ImmediateDeviceContext {
            device_context: DeviceContextCommon::new(device_context_ptr),
        }
    }

    pub fn flush(&self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .Flush
                .unwrap_unchecked()(self.device_context.device_context_ptr)
        }
    }

    //pub fn execute_command_lists(&self, command_lists: &[&CommandList]) {
    //    todo!()
    //}

    pub fn wait_for_idle(&self) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .WaitForIdle
                .unwrap_unchecked()(self.device_context.device_context_ptr)
        }
    }
}

pub struct DeferredDeviceContext {
    device_context: DeviceContextCommon,
}

impl AsObject for DeferredDeviceContext {
    fn as_object(&self) -> &Object {
        &self.device_context.object
    }
}

impl DeviceContext for DeferredDeviceContext {}

impl DeferredDeviceContext {
    pub(crate) fn new(device_context_ptr: *mut diligent_sys::IDeviceContext) -> Self {
        DeferredDeviceContext {
            device_context: DeviceContextCommon::new(device_context_ptr),
        }
    }

    pub fn begin(&self, immediate_context_id: u32) {
        unsafe {
            (*self.device_context.virtual_functions)
                .DeviceContext
                .Begin
                .unwrap_unchecked()(
                self.device_context.device_context_ptr, immediate_context_id
            )
        }
    }

    //pub fn finish_command_list(&self) -> CommandList {
    //    todo!()
    //}
}
