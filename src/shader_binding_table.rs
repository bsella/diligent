use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ops::Deref,
};

use bitflags::bitflags;

use crate::{
    Ported, RayTracingPipelineState,
    device_object::{DeviceObject, DeviceObjectAttribs},
    pipeline_state::PipelineState,
    tlas::TopLevelAS,
};

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

#[repr(transparent)]
#[derive(Clone)]
pub struct ShaderBindingTableDesc<'name, 'pipeline_state>(
    pub(crate) diligent_sys::ShaderBindingTableDesc,
    PhantomData<(&'name (), &'pipeline_state ())>,
);

impl Deref for ShaderBindingTableDesc<'_, '_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name, 'pipeline_state> ShaderBindingTableDesc<'name, 'pipeline_state> {
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,
        raytracing_pso: &'pipeline_state RayTracingPipelineState,
    ) -> Self {
        Self(
            diligent_sys::ShaderBindingTableDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
                },
                pPSO: raytracing_pso.sys_ptr(),
            },
            PhantomData,
        )
    }
}

define_ported!(
    ShaderBindingTable,
    diligent_sys::IShaderBindingTable,
    diligent_sys::IShaderBindingTableMethods : 10,
    DeviceObject
);

impl ShaderBindingTable {
    pub fn desc(&self) -> &ShaderBindingTableDesc<'_, '_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const ShaderBindingTableDesc) }
    }

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
