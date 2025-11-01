use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::const_assert_eq;

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

#[derive(Builder)]
pub struct ShaderBindingTableDesc<'a> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: Option<CString>,
    raytracing_pso: &'a PipelineState,
}

impl From<&ShaderBindingTableDesc<'_>> for diligent_sys::ShaderBindingTableDesc {
    fn from(value: &ShaderBindingTableDesc<'_>) -> Self {
        Self {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: value
                    .name
                    .as_ref()
                    .map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            pPSO: value.raytracing_pso.sys_ptr(),
        }
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IShaderBindingTableMethods>(),
    10 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ShaderBindingTable(diligent_sys::IShaderBindingTable);

impl Deref for ShaderBindingTable {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IDeviceObject
                as *const DeviceObject)
        }
    }
}

impl ShaderBindingTable {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IShaderBindingTable {
        std::ptr::addr_of!(self.0) as _
    }

    //TODO pub fn GetDesc() -> ShaderBindingTableDesc{}

    pub fn verify(&self, flags: VerifySBTFlags) -> bool {
        unsafe_member_call!(self, ShaderBindingTable, Verify, flags.bits())
    }
    pub fn reset(&self, pso: &PipelineState) {
        unsafe_member_call!(self, ShaderBindingTable, Reset, pso.sys_ptr())
    }
    pub fn reset_hit_groups(&self) {
        unsafe_member_call!(self, ShaderBindingTable, ResetHitGroups)
    }

    pub fn bind_ray_gen_shader(&self, shader_group_name: impl AsRef<str>) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindRayGenShader,
            shader_group_name.as_ptr(),
            std::ptr::null(),
            0
        )
    }

    pub fn bind_ray_gen_shader_with_data<T>(&self, shader_group_name: impl AsRef<str>, data: &T) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindRayGenShader,
            shader_group_name.as_ptr(),
            std::ptr::from_ref(data) as _,
            std::mem::size_of_val(data) as u32
        )
    }

    pub fn bind_miss_shader(&self, shader_group_name: impl AsRef<str>, miss_index: u32) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindMissShader,
            shader_group_name.as_ptr(),
            miss_index,
            std::ptr::null(),
            0
        )
    }

    pub fn bind_miss_shader_with_data<T>(
        &self,
        shader_group_name: impl AsRef<str>,
        miss_index: u32,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindMissShader,
            shader_group_name.as_ptr(),
            miss_index,
            std::ptr::from_ref(data) as _,
            std::mem::size_of_val(data) as u32
        )
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
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupForGeometry,
            tlas.sys_ptr(),
            instance_name.as_ptr(),
            geometry_name.as_ptr(),
            ray_offset_in_hit_group_index,
            shader_group_name.as_ptr(),
            std::ptr::null(),
            0
        )
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
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupForGeometry,
            tlas.sys_ptr(),
            instance_name.as_ptr(),
            geometry_name.as_ptr(),
            ray_offset_in_hit_group_index,
            shader_group_name.as_ptr(),
            std::ptr::from_ref(data) as _,
            std::mem::size_of_val(data) as u32
        )
    }

    pub fn bind_hit_group_by_index(&self, binding_index: u32, shader_group_name: impl AsRef<str>) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupByIndex,
            binding_index,
            shader_group_name.as_ptr(),
            std::ptr::null(),
            0
        )
    }

    pub fn bind_hit_group_by_index_with_data<T>(
        &self,
        binding_index: u32,
        shader_group_name: impl AsRef<str>,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupByIndex,
            binding_index,
            shader_group_name.as_ptr(),
            std::ptr::from_ref(data) as _,
            std::mem::size_of_val(data) as u32
        )
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
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupForInstance,
            tlas.sys_ptr(),
            instance_name.as_ptr(),
            ray_offset_in_hit_group_index,
            shader_group_name
                .as_ref()
                .map_or(std::ptr::null(), |name| name.as_ptr()),
            std::ptr::null(),
            0
        )
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
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupForInstance,
            tlas.sys_ptr(),
            instance_name.as_ptr(),
            ray_offset_in_hit_group_index,
            shader_group_name.as_ptr(),
            std::ptr::from_ref(data) as _,
            std::mem::size_of_val(data) as u32
        )
    }

    pub fn bind_hit_group_for_tlas(
        &self,
        tlas: &TopLevelAS,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: Option<impl AsRef<str>>,
    ) {
        let shader_group_name = shader_group_name.map(|name| CString::new(name.as_ref()).unwrap());
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupForTLAS,
            tlas.sys_ptr(),
            ray_offset_in_hit_group_index,
            shader_group_name
                .as_ref()
                .map_or(std::ptr::null(), |name| name.as_ptr()),
            std::ptr::null(),
            0
        )
    }

    pub fn bind_hit_group_for_tlas_with_data<T>(
        &self,
        tlas: &TopLevelAS,
        ray_offset_in_hit_group_index: u32,
        shader_group_name: impl AsRef<str>,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindHitGroupForTLAS,
            tlas.sys_ptr(),
            ray_offset_in_hit_group_index,
            shader_group_name.as_ptr(),
            std::ptr::from_ref(data) as _,
            std::mem::size_of_val(data) as u32
        )
    }

    pub fn bind_callable_shader(&self, shader_group_name: impl AsRef<str>, callable_index: u32) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindCallableShader,
            shader_group_name.as_ptr(),
            callable_index,
            std::ptr::null(),
            0
        )
    }

    pub fn bind_callable_shader_with_data<T>(
        &self,
        shader_group_name: impl AsRef<str>,
        callable_index: u32,
        data: &T,
    ) {
        let shader_group_name = CString::new(shader_group_name.as_ref()).unwrap();
        unsafe_member_call!(
            self,
            ShaderBindingTable,
            BindCallableShader,
            shader_group_name.as_ptr(),
            callable_index,
            std::ptr::from_ref(data) as _,
            std::mem::size_of_val(data) as u32
        )
    }
}
