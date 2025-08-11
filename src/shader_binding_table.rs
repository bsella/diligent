use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use static_assertions::const_assert;

use crate::{device_object::DeviceObject, pipeline_state::PipelineState, tlas::TopLevelAS};

bitflags! {
    #[derive(Clone, Copy)]
    pub struct VerifySBTFlags : diligent_sys::VERIFY_SBT_FLAGS
    {
        const ShaderOnly   = diligent_sys::VERIFY_SBT_FLAG_SHADER_ONLY as _;
        const ShaderRecord = diligent_sys::VERIFY_SBT_FLAG_SHADER_RECORD as _;
        const TLAS         = diligent_sys::VERIFY_SBT_FLAG_TLAS as _;
        const All          = diligent_sys::VERIFY_SBT_FLAG_ALL as _;
    }
}

pub struct ShaderBindingTableDesc<'a> {
    name: CString,
    raytracing_pso: &'a PipelineState,
}

impl<'a> ShaderBindingTableDesc<'a> {
    pub fn new(name: impl AsRef<str>, raytracing_pso: &'a PipelineState) -> Self {
        Self {
            name: CString::new(name.as_ref()).unwrap(),
            raytracing_pso,
        }
    }
}

impl From<&ShaderBindingTableDesc<'_>> for diligent_sys::ShaderBindingTableDesc {
    fn from(value: &ShaderBindingTableDesc<'_>) -> Self {
        Self {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value.name.as_ptr(),
            },
            pPSO: value.raytracing_pso.sys_ptr,
        }
    }
}

pub struct ShaderBindingTable {
    pub(crate) sys_ptr: *mut diligent_sys::IShaderBindingTable,
    virtual_functions: *mut diligent_sys::IShaderBindingTableVtbl,

    device_object: DeviceObject,
}

impl Deref for ShaderBindingTable {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.device_object
    }
}

impl ShaderBindingTable {
    pub(crate) fn new(sys_ptr: *mut diligent_sys::IShaderBindingTable) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IShaderBindingTable>()
        );

        Self {
            sys_ptr,
            virtual_functions: unsafe { (*sys_ptr).pVtbl },
            device_object: DeviceObject::new(sys_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    //TODO pub fn GetDesc() -> ShaderBindingTableDesc{}

    pub fn verify(&self, flags: VerifySBTFlags) -> bool {
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .Verify
                .unwrap_unchecked()(self.sys_ptr, flags.bits())
        }
    }
    pub fn reset(&self, pso: &PipelineState) {
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .Reset
                .unwrap_unchecked()(self.sys_ptr, pso.sys_ptr)
        }
    }
    pub fn reset_hit_groups(&self) {
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .ResetHitGroups
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn bind_ray_gen_shader(&self, shader_group_name: impl AsRef<str>) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindRayGenShader
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_group_name.as_ptr(),
                std::ptr::null(),
                0,
            )
        }
    }

    pub fn bind_ray_gen_shader_with_data<T>(&self, shader_group_name: impl AsRef<str>, data: &T) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindRayGenShader
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_group_name.as_ptr(),
                std::ptr::from_ref(data) as _,
                std::mem::size_of_val(data) as u32,
            )
        }
    }

    pub fn bind_miss_shader(&self, shader_group_name: impl AsRef<str>, miss_index: u32) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindMissShader
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_group_name.as_ptr(),
                miss_index,
                std::ptr::null(),
                0,
            )
        }
    }

    pub fn bind_miss_shader_with_data<T>(
        &self,
        shader_group_name: impl AsRef<str>,
        miss_index: u32,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindMissShader
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_group_name.as_ptr(),
                miss_index,
                std::ptr::from_ref(data) as _,
                std::mem::size_of_val(data) as u32,
            )
        }
    }

    pub fn bind_hit_group_for_geometry(
        &self,
        tlas: &TopLevelAS,
        instance_name: impl AsRef<str>,
        geometry_name: impl AsRef<str>,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: impl AsRef<str>,
    ) {
        let instance_name = CString::new(instance_name.as_ref()).unwrap();
        let geometry_name = CString::new(geometry_name.as_ref()).unwrap();
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupForGeometry
                .unwrap_unchecked()(
                self.sys_ptr,
                tlas.sys_ptr,
                instance_name.as_ptr(),
                geometry_name.as_ptr(),
                ray_offset_in_hit_group_index,
                shader_group_name.as_ptr(),
                std::ptr::null(),
                0,
            )
        }
    }

    pub fn bind_hit_group_for_geometry_with_data<T>(
        &self,
        tlas: &TopLevelAS,
        instance_name: impl AsRef<str>,
        geometry_name: impl AsRef<str>,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: impl AsRef<str>,
        data: &T,
    ) {
        let instance_name = CString::new(instance_name.as_ref()).unwrap();
        let geometry_name = CString::new(geometry_name.as_ref()).unwrap();
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupForGeometry
                .unwrap_unchecked()(
                self.sys_ptr,
                tlas.sys_ptr,
                instance_name.as_ptr(),
                geometry_name.as_ptr(),
                ray_offset_in_hit_group_index,
                shader_group_name.as_ptr(),
                std::ptr::from_ref(data) as _,
                std::mem::size_of_val(data) as u32,
            )
        }
    }

    pub fn bind_hit_group_by_index(&self, binding_index: u32, shader_group_name: impl AsRef<str>) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupByIndex
                .unwrap_unchecked()(
                self.sys_ptr,
                binding_index,
                shader_group_name.as_ptr(),
                std::ptr::null(),
                0,
            )
        }
    }

    pub fn bind_hit_group_by_index_with_data<T>(
        &self,
        binding_index: u32,
        shader_group_name: impl AsRef<str>,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupByIndex
                .unwrap_unchecked()(
                self.sys_ptr,
                binding_index,
                shader_group_name.as_ptr(),
                std::ptr::from_ref(data) as _,
                std::mem::size_of_val(data) as u32,
            )
        }
    }

    pub fn bind_hit_group_for_instance(
        &self,
        tlas: &TopLevelAS,
        instance_name: impl AsRef<str>,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: Option<impl AsRef<str>>,
    ) {
        let instance_name = CString::new(instance_name.as_ref()).unwrap();
        let shader_group_name = shader_group_name.map(|name| CString::new(name.as_ref()).unwrap());
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupForInstance
                .unwrap_unchecked()(
                self.sys_ptr,
                tlas.sys_ptr,
                instance_name.as_ptr(),
                ray_offset_in_hit_group_index,
                shader_group_name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
                std::ptr::null(),
                0,
            )
        }
    }

    pub fn bind_hit_group_for_instance_with_data<T>(
        &self,
        tlas: &TopLevelAS,
        instance_name: impl AsRef<str>,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: impl AsRef<str>,
        data: &T,
    ) {
        let instance_name = CString::new(instance_name.as_ref()).unwrap();
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupForInstance
                .unwrap_unchecked()(
                self.sys_ptr,
                tlas.sys_ptr,
                instance_name.as_ptr(),
                ray_offset_in_hit_group_index,
                shader_group_name.as_ptr(),
                std::ptr::from_ref(data) as _,
                std::mem::size_of_val(data) as u32,
            )
        }
    }

    pub fn bind_hit_group_for_tlas(
        &self,
        tlas: &TopLevelAS,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: Option<impl AsRef<str>>,
    ) {
        let shader_group_name = shader_group_name.map(|name| CString::new(name.as_ref()).unwrap());
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupForTLAS
                .unwrap_unchecked()(
                self.sys_ptr,
                tlas.sys_ptr,
                ray_offset_in_hit_group_index,
                shader_group_name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
                std::ptr::null(),
                0,
            )
        }
    }

    pub fn bind_hit_group_for_tlas_with_data<T>(
        &self,
        tlas: &TopLevelAS,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: impl AsRef<str>,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindHitGroupForTLAS
                .unwrap_unchecked()(
                self.sys_ptr,
                tlas.sys_ptr,
                ray_offset_in_hit_group_index,
                shader_group_name.as_ptr(),
                std::ptr::from_ref(data) as _,
                std::mem::size_of_val(data) as u32,
            )
        }
    }

    pub fn bind_callable_shader(&self, shader_group_name: impl AsRef<str>, callable_index: u32) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindCallableShader
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_group_name.as_ptr(),
                callable_index,
                std::ptr::null(),
                0,
            )
        }
    }

    pub fn bind_callable_shader_with_data<T>(
        &self,
        shader_group_name: impl AsRef<str>,
        callable_index: u32,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe {
            (*self.virtual_functions)
                .ShaderBindingTable
                .BindCallableShader
                .unwrap_unchecked()(
                self.sys_ptr,
                shader_group_name.as_ptr(),
                callable_index,
                std::ptr::from_ref(data) as _,
                std::mem::size_of_val(data) as u32,
            )
        }
    }
}
