use crate::bindings::{self, RESOURCE_STATE_TRANSITION_MODE};

use super::{
    buffer::Buffer,
    fence::Fence,
    object::{AsObject, Object},
    pipeline_state::PipelineState,
    shader_resource_binding::ShaderResourceBinding,
    texture::Texture,
    texture_view::TextureView,
};

pub struct DrawAttribs {
    pub num_vertices: u32,
    pub flags: bindings::DRAW_FLAGS,
    pub num_instances: u32,
    pub start_vertex_location: u32,
    pub first_instance_location: u32,
}

impl Into<bindings::DrawAttribs> for DrawAttribs {
    fn into(self) -> bindings::DrawAttribs {
        bindings::DrawAttribs {
            NumVertices: self.num_vertices,
            Flags: self.flags,
            NumInstances: self.num_instances,
            StartVertexLocation: self.start_vertex_location,
            FirstInstanceLocation: self.first_instance_location,
        }
    }
}

pub struct DeviceContext {
    pub(crate) m_device_context: *mut bindings::IDeviceContext,
    m_virtual_functions: *mut bindings::IDeviceContextVtbl,

    m_object: Object,
}

impl AsObject for DeviceContext {
    fn as_object(&self) -> &Object {
        &self.m_object
    }
}

impl DeviceContext {
    pub(crate) fn new(device_context: *mut bindings::IDeviceContext) -> Self {
        DeviceContext {
            m_device_context: device_context,
            m_virtual_functions: unsafe { (*device_context).pVtbl },
            m_object: Object::new(device_context as *mut bindings::IObject),
        }
    }

    pub fn get_desc(&self) -> &bindings::DeviceContextDesc {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .GetDesc
                .unwrap_unchecked()(self.m_device_context)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn begin(&self, immediate_context_id: u32) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .Begin
                .unwrap_unchecked()(self.m_device_context, immediate_context_id)
        }
    }

    pub fn set_pipeline_state(&self, pipeline_state: &PipelineState) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetPipelineState
                .unwrap_unchecked()(
                self.m_device_context, pipeline_state.m_pipeline_state
            )
        }
    }

    pub fn transition_shader_resources(&self, shader_resource_binding: &ShaderResourceBinding) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .TransitionShaderResources
                .unwrap_unchecked()(
                self.m_device_context,
                shader_resource_binding.m_shader_resource_binding,
            )
        }
    }

    pub fn commit_shader_resources(
        &self,
        shader_resource_binding: &ShaderResourceBinding,
        state_transition_mode: bindings::RESOURCE_STATE_TRANSITION_MODE,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .CommitShaderResources
                .unwrap_unchecked()(
                self.m_device_context,
                shader_resource_binding.m_shader_resource_binding,
                state_transition_mode,
            )
        }
    }

    pub fn set_stencil_ref(&self, stencil_ref: u32) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetStencilRef
                .unwrap_unchecked()(self.m_device_context, stencil_ref)
        }
    }

    pub fn set_blend_factors(&self, blend_factors: &[f32; 4]) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetBlendFactors
                .unwrap_unchecked()(self.m_device_context, blend_factors.as_ptr())
        }
    }

    pub fn set_vertex_buffers(
        &self,
        buffers: &[&Buffer],
        offsets: &[u64],
        state_transition_mode: bindings::RESOURCE_STATE_TRANSITION_MODE,
        flags: bindings::SET_VERTEX_BUFFERS_FLAGS,
    ) {
        let num_buffers = buffers.len();
        let buffer_pointers = Vec::from_iter(buffers.iter().map(|buffer| buffer.m_buffer));
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetVertexBuffers
                .unwrap_unchecked()(
                self.m_device_context,
                0,
                num_buffers as u32,
                buffer_pointers.as_ptr(),
                offsets.as_ptr(),
                state_transition_mode,
                flags,
            )
        }
    }

    pub fn invalidate_state(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .InvalidateState
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn set_index_buffer(
        &self,
        index_buffer: &Buffer,
        offset: u64,
        state_transition_mode: bindings::RESOURCE_STATE_TRANSITION_MODE,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetIndexBuffer
                .unwrap_unchecked()(
                self.m_device_context,
                index_buffer.m_buffer,
                offset,
                state_transition_mode,
            )
        }
    }

    pub fn set_viewports(
        &self,
        viewports: &[bindings::Viewport],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetViewports
                .unwrap_unchecked()(
                self.m_device_context,
                viewports.len() as u32,
                viewports.as_ptr(),
                render_target_width,
                render_target_height,
            )
        }
    }

    pub fn set_scissor_rects(
        &self,
        rects: &[bindings::Rect],
        render_target_width: u32,
        render_target_height: u32,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetScissorRects
                .unwrap_unchecked()(
                self.m_device_context,
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
        state_transition_mode: bindings::_RESOURCE_STATE_TRANSITION_MODE,
    ) {
        let num_render_targets = render_targets.len();
        let mut render_target_pointers = Vec::from_iter(
            render_targets
                .iter()
                .map(|render_targets| render_targets.m_texture_view),
        );

        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetRenderTargets
                .unwrap_unchecked()(
                self.m_device_context,
                num_render_targets as u32,
                render_target_pointers.as_mut_ptr(),
                depth_stencil.map_or(std::ptr::null_mut(), |v| v.m_texture_view),
                state_transition_mode as bindings::RESOURCE_STATE_TRANSITION_MODE,
            )
        }
    }

    pub fn begin_render_pass(&self, attribs: &bindings::BeginRenderPassAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .BeginRenderPass
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn next_subpass(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .NextSubpass
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn end_render_pass(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .EndRenderPass
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn draw(&self, attribs: DrawAttribs) {
        let attribs: bindings::DrawAttribs = attribs.into();
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .Draw
                .unwrap_unchecked()(self.m_device_context, std::ptr::addr_of!(attribs))
        }
    }

    pub fn draw_indexed(&self, attribs: &bindings::DrawIndexedAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DrawIndexed
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn draw_indirect(&self, attribs: &bindings::DrawIndirectAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DrawIndirect
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn draw_indexed_indirect(&self, attribs: &bindings::DrawIndexedIndirectAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DrawIndexedIndirect
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn draw_mesh(&self, attribs: &bindings::DrawMeshAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DrawMesh
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn draw_mesh_indirect(&self, attribs: &bindings::DrawMeshIndirectAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DrawMeshIndirect
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn multi_draw(&self, attribs: &bindings::MultiDrawAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .MultiDraw
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn multi_draw_indexed(&self, attribs: &bindings::MultiDrawIndexedAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .MultiDrawIndexed
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn dispatch_compute(&self, attribs: &bindings::DispatchComputeAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DispatchCompute
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn dispatch_compute_indirect(&self, attribs: &bindings::DispatchComputeIndirectAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DispatchComputeIndirect
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn dispatch_tile(&self, attribs: &bindings::DispatchTileAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DispatchTile
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn get_tile_size(&self) -> (u32, u32) {
        let mut tile_size_x: u32 = 0;
        let mut tile_size_y: u32 = 0;
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .GetTileSize
                .unwrap_unchecked()(
                self.m_device_context,
                std::ptr::addr_of_mut!(tile_size_x),
                std::ptr::addr_of_mut!(tile_size_y),
            )
        };
        (tile_size_x, tile_size_y)
    }

    pub fn clear_depth_stencil(
        &self,
        view: &mut TextureView,
        clear_flags: bindings::CLEAR_DEPTH_STENCIL_FLAGS,
        depth: f32,
        stencil: u8,
        state_transition_mode: bindings::_RESOURCE_STATE_TRANSITION_MODE,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .ClearDepthStencil
                .unwrap_unchecked()(
                self.m_device_context,
                view.m_texture_view,
                clear_flags,
                depth,
                stencil,
                state_transition_mode as bindings::RESOURCE_STATE_TRANSITION_MODE,
            )
        }
    }

    pub fn clear_render_target<T>(
        &self,
        view: &mut TextureView,
        rgba: &[T; 4],
        state_transition_mode: bindings::_RESOURCE_STATE_TRANSITION_MODE,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .ClearRenderTarget
                .unwrap_unchecked()(
                self.m_device_context,
                view.m_texture_view,
                (*rgba).as_ptr() as *const std::os::raw::c_void,
                state_transition_mode as bindings::RESOURCE_STATE_TRANSITION_MODE,
            )
        }
    }

    //pub fn finish_command_list(&self) -> CommandList {
    //    todo!()
    //}
    //pub fn execute_command_lists(&self, command_lists: &[&CommandList]) {
    //    todo!()
    //}

    pub fn enqueue_signal(&self, fence: &Fence, value: u64) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .EnqueueSignal
                .unwrap_unchecked()(self.m_device_context, fence.m_fence, value)
        }
    }

    pub fn device_wait_for_fence(&self, fence: &Fence, value: u64) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .DeviceWaitForFence
                .unwrap_unchecked()(self.m_device_context, fence.m_fence, value)
        }
    }

    pub fn wait_for_idle(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .WaitForIdle
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    //pub fn begin_query(&self, query: &mut Query) {
    //    todo!()
    //}
    //pub fn end_query(&self, query: &mut Query) {
    //    todo!()
    //}

    pub fn flush(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .Flush
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn update_buffer<T>(
        &self,
        buffer: &mut Buffer,
        offset: u64,
        size: u64,
        data: &T,
        state_transition_mode: RESOURCE_STATE_TRANSITION_MODE,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .UpdateBuffer
                .unwrap_unchecked()(
                self.m_device_context,
                buffer.m_buffer,
                offset,
                size,
                std::ptr::from_ref(data) as *const std::os::raw::c_void,
                state_transition_mode,
            )
        }
    }

    pub fn copy_buffer(
        &self,
        src_buffer: &Buffer,
        src_offset: u64,
        src_buffer_transition_mode: RESOURCE_STATE_TRANSITION_MODE,
        dst_buffer: &mut Buffer,
        dst_offset: u64,
        size: u64,
        dst_buffer_transition_mode: RESOURCE_STATE_TRANSITION_MODE,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .CopyBuffer
                .unwrap_unchecked()(
                self.m_device_context,
                src_buffer.m_buffer,
                src_offset,
                src_buffer_transition_mode,
                dst_buffer.m_buffer,
                dst_offset,
                size,
                dst_buffer_transition_mode,
            )
        }
    }

    pub fn map_buffer(
        &self,
        buffer: &mut Buffer,
        map_type: bindings::_MAP_TYPE,
        map_flags: bindings::_MAP_FLAGS,
    ) -> *mut u8 {
        let mut ptr = std::ptr::null_mut() as *mut std::os::raw::c_void;
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .MapBuffer
                .unwrap_unchecked()(
                self.m_device_context,
                buffer.m_buffer,
                map_type as bindings::MAP_TYPE,
                map_flags as bindings::MAP_FLAGS,
                std::ptr::addr_of_mut!(ptr),
            );
        }
        ptr as *mut u8
    }

    pub fn unmap_buffer(&self, buffer: &mut Buffer, map_type: bindings::_MAP_TYPE) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .UnmapBuffer
                .unwrap_unchecked()(
                self.m_device_context,
                buffer.m_buffer,
                map_type as bindings::MAP_TYPE,
            )
        }
    }

    pub fn update_texture(
        &self,
        texture: &mut Texture,
        mip_level: u32,
        slice: u32,
        dst_box: &bindings::Box,
        subres_data: &bindings::TextureSubResData,
        src_buffer_transition_mode: bindings::_RESOURCE_STATE_TRANSITION_MODE,
        texture_transition_mode: bindings::_RESOURCE_STATE_TRANSITION_MODE,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .UpdateTexture
                .unwrap_unchecked()(
                self.m_device_context,
                texture.m_texture,
                mip_level,
                slice,
                std::ptr::from_ref(dst_box),
                std::ptr::from_ref(subres_data),
                src_buffer_transition_mode as bindings::RESOURCE_STATE_TRANSITION_MODE,
                texture_transition_mode as bindings::RESOURCE_STATE_TRANSITION_MODE,
            )
        }
    }

    pub fn copy_texture(&self, copy_attribs: &bindings::CopyTextureAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .CopyTexture
                .unwrap_unchecked()(
                self.m_device_context, std::ptr::from_ref(copy_attribs)
            )
        }
    }

    // TODO
    //pub fn map_texture_subresource(&self, )
    //{
    //
    //}

    pub fn unmap_texture_subresource(
        &self,
        texture: &mut Texture,
        mip_level: u32,
        array_slice: u32,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .UnmapTextureSubresource
                .unwrap_unchecked()(
                self.m_device_context,
                texture.m_texture,
                mip_level,
                array_slice,
            )
        }
    }

    pub fn generate_mips(&self, texture_view: &mut TextureView) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .GenerateMips
                .unwrap_unchecked()(self.m_device_context, texture_view.m_texture_view)
        }
    }

    pub fn finish_frame(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .FinishFrame
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn get_frame_number(&self) -> u64 {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .GetFrameNumber
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn transition_resource_states(&self, barriers: &[bindings::StateTransitionDesc]) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .TransitionResourceStates
                .unwrap_unchecked()(
                self.m_device_context,
                barriers.len() as u32,
                barriers.as_ptr(),
            )
        }
    }

    pub fn resolve_texture_subresource(
        &self,
        src_texture: &Texture,
        dst_texture: &mut Texture,
        resolve_attribs: &bindings::ResolveTextureSubresourceAttribs,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .ResolveTextureSubresource
                .unwrap_unchecked()(
                self.m_device_context,
                src_texture.m_texture,
                dst_texture.m_texture,
                std::ptr::from_ref(resolve_attribs),
            )
        }
    }

    pub fn build_blas(&self, attribs: &bindings::BuildBLASAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .BuildBLAS
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn build_tlas(&self, attribs: &bindings::BuildTLASAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .BuildTLAS
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn copy_blas(&self, attribs: &bindings::CopyBLASAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .CopyBLAS
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn copy_tlas(&self, attribs: &bindings::CopyTLASAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .CopyTLAS
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn write_blas_compacted_size(&self, attribs: &bindings::WriteBLASCompactedSizeAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .WriteBLASCompactedSize
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn write_tlas_compacted_size(&self, attribs: &bindings::WriteTLASCompactedSizeAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .WriteTLASCompactedSize
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn trace_rays(&self, attribs: &bindings::TraceRaysAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .TraceRays
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn trace_rays_indirect(&self, attribs: &bindings::TraceRaysIndirectAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .TraceRaysIndirect
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    // TODO
    // pub fn update_sbt(&self, sbt : &mut ShaderBindingTable) {}

    pub fn set_user_data<Data>(&self, user_data: &Data)
    where
        Data: AsObject,
    {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetUserData
                .unwrap_unchecked()(
                self.m_device_context, user_data.as_object().m_object
            )
        }
    }

    // TODO
    // pub fn get_user_data(&self);

    pub fn begin_debug_group(&self, name: &str, color: [f32; 4]) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .BeginDebugGroup
                .unwrap_unchecked()(
                self.m_device_context,
                name.as_bytes().as_ptr() as *const i8,
                color.as_ptr(),
            )
        }
    }

    pub fn end_debug_group(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .EndDebugGroup
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn insert_debug_label(&self, name: &str, color: [f32; 4]) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .InsertDebugLabel
                .unwrap_unchecked()(
                self.m_device_context,
                name.as_bytes().as_ptr() as *const i8,
                color.as_ptr(),
            )
        }
    }

    //pub fn lock_command_queue(&self) -> CommandQueue
    //{
    //}

    pub fn unlock_command_queue(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .UnlockCommandQueue
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn set_shading_rate(
        &self,
        base_rate: bindings::_SHADING_RATE,
        primitive_combiner: bindings::_SHADING_RATE_COMBINER,
        texture_combiner: bindings::_SHADING_RATE_COMBINER,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .SetShadingRate
                .unwrap_unchecked()(
                self.m_device_context,
                base_rate as bindings::SHADING_RATE,
                primitive_combiner as bindings::SHADING_RATE_COMBINER,
                texture_combiner as bindings::SHADING_RATE_COMBINER,
            )
        }
    }

    pub fn bind_sparse_resource_memory(&self, attribs: &bindings::BindSparseResourceMemoryAttribs) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .BindSparseResourceMemory
                .unwrap_unchecked()(self.m_device_context, std::ptr::from_ref(attribs))
        }
    }

    pub fn clear_stats(&self) {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .ClearStats
                .unwrap_unchecked()(self.m_device_context)
        }
    }

    pub fn get_stats(&self) -> &bindings::DeviceContextStats {
        unsafe {
            (*self.m_virtual_functions)
                .DeviceContext
                .GetStats
                .unwrap_unchecked()(self.m_device_context)
            .as_ref()
            .unwrap_unchecked()
        }
    }
}
