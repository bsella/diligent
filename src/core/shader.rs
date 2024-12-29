use std::os::raw::c_void;

use crate::bindings;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct Shader {
    pub(crate) m_shader: *mut bindings::IShader,
    m_virtual_functions: *mut bindings::IShaderVtbl,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for Shader {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl Shader {
    pub(crate) fn new(shader_ptr: *mut bindings::IShader) -> Self {
        Shader {
            m_shader: shader_ptr,
            m_virtual_functions: unsafe { (*shader_ptr).pVtbl },
            m_device_object: DeviceObject::new(shader_ptr as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> bindings::ShaderDesc {
        unsafe {
            *((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.m_shader as *mut bindings::IDeviceObject)
                as *const bindings::ShaderDesc)
        }
    }

    fn get_resources(&self) -> Vec<bindings::ShaderResourceDesc> {
        unsafe {
            let num_resources = (*self.m_virtual_functions)
                .Shader
                .GetResourceCount
                .unwrap_unchecked()(self.m_shader);

            let mut resources = Vec::with_capacity(num_resources as usize);

            for index in 0..num_resources {
                let resources_ptr = std::ptr::null_mut();
                (*self.m_virtual_functions)
                    .Shader
                    .GetResourceDesc
                    .unwrap_unchecked()(self.m_shader, index, resources_ptr);
                resources.push(*resources_ptr);
            }
            resources
        }
    }

    fn get_constant_buffer_desc(&self, index: u32) -> Option<&bindings::ShaderCodeBufferDesc> {
        unsafe {
            (*self.m_virtual_functions)
                .Shader
                .GetConstantBufferDesc
                .unwrap_unchecked()(self.m_shader, index)
            .as_ref()
        }
    }

    fn get_bytecode(&self, bytecode: *mut *const u8) -> u64 {
        unsafe {
            let mut size: u64 = 0;
            (*self.m_virtual_functions)
                .Shader
                .GetBytecode
                .unwrap_unchecked()(
                self.m_shader,
                bytecode as *mut *const c_void,
                std::ptr::addr_of_mut!(size),
            );
            size
        }
    }

    fn get_status(&self, wait_for_completion: bool) -> bindings::SHADER_STATUS {
        unsafe {
            (*self.m_virtual_functions)
                .Shader
                .GetStatus
                .unwrap_unchecked()(self.m_shader, wait_for_completion)
        }
    }
}
