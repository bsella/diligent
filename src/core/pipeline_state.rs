use crate::core::bindings;

use super::device_object::{AsDeviceObject, DeviceObject};
use super::pipeline_resource_signature::PipelineResourceSignature;
use super::resource_mapping::ResourceMapping;
use super::shader_resource_binding::ShaderResourceBinding;
use super::shader_resource_variable::ShaderResourceVariable;

pub struct PipelineState {
    pub(crate) m_pipeline_state: *mut bindings::IPipelineState,
    m_virtual_functions: *mut bindings::IPipelineStateVtbl,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for PipelineState {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl PipelineState {
    pub(crate) fn new(pipeline_state_ptr: *mut bindings::IPipelineState) -> Self {
        PipelineState {
            m_pipeline_state: pipeline_state_ptr,
            m_virtual_functions: unsafe { (*pipeline_state_ptr).pVtbl },
            m_device_object: DeviceObject::new(pipeline_state_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> &bindings::PipelineStateDesc {
        unsafe {
            ((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.m_pipeline_state as *mut bindings::IDeviceObject
            ) as *const bindings::PipelineStateDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    fn get_graphics_pipeline_desc(&self) -> &bindings::GraphicsPipelineDesc {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .GetGraphicsPipelineDesc
                .unwrap_unchecked()(self.m_pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_ray_tracing_pipeline_desc(&self) -> &bindings::RayTracingPipelineDesc {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .GetRayTracingPipelineDesc
                .unwrap_unchecked()(self.m_pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn get_tile_pipeline_desc(&self) -> &bindings::TilePipelineDesc {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .GetTilePipelineDesc
                .unwrap_unchecked()(self.m_pipeline_state)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    fn bind_static_resources(
        &mut self,
        shader_type: bindings::SHADER_TYPE,
        resource_mapping: &ResourceMapping,
        flags: bindings::BIND_SHADER_RESOURCES_FLAGS,
    ) {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .BindStaticResources
                .unwrap_unchecked()(
                self.m_pipeline_state,
                shader_type,
                resource_mapping.m_resource_mapping,
                flags,
            )
        }
    }

    // TODO
    //fn get_static_variables(
    //    &self,
    //    shader_type: bindings::SHADER_TYPE,
    //) -> Option<&[ShaderResourceVariable]> {
    //}

    fn create_shader_resource_binding(
        &mut self,
        init_static_resources: Option<bool>,
    ) -> Option<ShaderResourceBinding> {
        let mut shader_resource_binding_ptr: *mut bindings::IShaderResourceBinding =
            std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .CreateShaderResourceBinding
                .unwrap_unchecked()(
                self.m_pipeline_state,
                std::ptr::addr_of_mut!(shader_resource_binding_ptr),
                init_static_resources.unwrap_or(false),
            );
        }
        if shader_resource_binding_ptr.is_null() {
            None
        } else {
            Some(ShaderResourceBinding::new(shader_resource_binding_ptr))
        }
    }

    fn initialize_static_srb_resources(&self, shader_resource_binding: &mut ShaderResourceBinding) {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .InitializeStaticSRBResources
                .unwrap_unchecked()(
                self.m_pipeline_state,
                shader_resource_binding.m_shader_resource_binding,
            )
        }
    }

    fn copy_static_resources(&self, pipeline_state: &mut PipelineState) {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .CopyStaticResources
                .unwrap_unchecked()(
                self.m_pipeline_state, pipeline_state.m_pipeline_state
            )
        }
    }

    fn is_compatible_with(&self, pipeline_state: &PipelineState) -> bool {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .IsCompatibleWith
                .unwrap_unchecked()(
                self.m_pipeline_state, pipeline_state.m_pipeline_state
            )
        }
    }

    // TODO
    //fn get_resource_signatures(&self) -> &[PipelineResourceSignature] {
    //}

    fn get_status(&self, wait_for_completion: Option<bool>) -> bindings::PIPELINE_STATE_STATUS {
        unsafe {
            (*self.m_virtual_functions)
                .PipelineState
                .GetStatus
                .unwrap_unchecked()(
                self.m_pipeline_state, wait_for_completion.unwrap_or(false)
            )
        }
    }
}
