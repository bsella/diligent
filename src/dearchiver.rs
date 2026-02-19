use std::{ffi::CStr, marker::PhantomData};

use crate::{
    Boxed, BoxedFromNulError, DataBlob, PipelineResourceSignature, PipelineState,
    PipelineStateCache, PipelineStateCreateInfo, PipelineType, Ported, RenderDevice, RenderPass,
    RenderPassDesc, Shader, ShaderDesc, object::Object,
};

#[repr(transparent)]
pub struct ShaderUnpackInfo<'device, 'user_data, 'name, UserData>(
    diligent_sys::ShaderUnpackInfo,
    PhantomData<(&'user_data UserData, &'device (), &'name ())>,
);

#[bon::bon]
impl<'device, 'user_data, 'name, UserData> ShaderUnpackInfo<'device, 'user_data, 'name, UserData> {
    #[builder]
    pub fn new(
        device: &'device RenderDevice,
        name: &'name CStr,
        modify_shader_desc: Option<(fn(&mut ShaderDesc, &mut UserData), &'user_data mut UserData)>,
    ) -> Self {
        Self(
            diligent_sys::ShaderUnpackInfo {
                pDevice: device.sys_ptr(),
                Name: name.as_ptr(),
                ModifyShaderDesc: modify_shader_desc.as_ref().map(|desc| unsafe {
                    std::mem::transmute::<
                        fn(&mut ShaderDesc, &mut UserData),
                        unsafe extern "C" fn(*mut diligent_sys::ShaderDesc, *mut std::ffi::c_void),
                    >(desc.0)
                }),
                pUserData: modify_shader_desc
                    .map_or(std::ptr::null_mut(), |desc| std::ptr::from_mut(desc.1) as _),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct PipelineStateUnpackInfo<'user_data, 'device, 'name, UserData>(
    diligent_sys::PipelineStateUnpackInfo,
    PhantomData<(&'user_data UserData, &'device (), &'name ())>,
);

#[bon::bon]
impl<'user_data, 'device, 'name, UserData>
    PipelineStateUnpackInfo<'user_data, 'device, 'name, UserData>
{
    #[builder]
    pub fn new(
        device: &'device RenderDevice,
        name: &'name CStr,
        pipeline_type: PipelineType,
        srb_allocation_granularity: u32,
        immediate_context_mask: u64,
        cache: &PipelineStateCache,
        modify_pipeline_state_create_info: Option<(
            fn(&mut PipelineStateCreateInfo, &mut UserData),
            &'user_data mut UserData,
        )>,
    ) -> Self {
        Self(
            diligent_sys::PipelineStateUnpackInfo {
                pDevice: device.sys_ptr(),
                Name: name.as_ptr(),
                PipelineType: pipeline_type.into(),
                SRBAllocationGranularity: srb_allocation_granularity,
                ImmediateContextMask: immediate_context_mask,
                pCache: std::ptr::from_ref(&cache.0) as _,
                ModifyPipelineStateCreateInfo: modify_pipeline_state_create_info.as_ref().map(
                    |desc| unsafe {
                        std::mem::transmute::<
                            fn(&mut PipelineStateCreateInfo, &mut UserData),
                            unsafe extern "C" fn(
                                *mut diligent_sys::PipelineStateCreateInfo,
                                *mut std::ffi::c_void,
                            ),
                        >(desc.0)
                    },
                ),
                pUserData: modify_pipeline_state_create_info
                    .map_or(std::ptr::null_mut(), |desc| std::ptr::from_mut(desc.1) as _),
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct ResourceSignatureUnpackInfo<'device, 'name>(
    diligent_sys::ResourceSignatureUnpackInfo,
    PhantomData<(&'device (), &'name ())>,
);

#[bon::bon]
impl<'device, 'name> ResourceSignatureUnpackInfo<'device, 'name> {
    #[builder]
    pub fn new(
        device: &'device RenderDevice,
        name: &'name CStr,
        srb_allocation_granularity: u32,
    ) -> Self {
        Self(
            diligent_sys::ResourceSignatureUnpackInfo {
                pDevice: std::ptr::from_ref(&device.0) as _,
                Name: name.as_ptr(),
                SRBAllocationGranularity: srb_allocation_granularity,
            },
            PhantomData,
        )
    }
}

#[repr(transparent)]
pub struct RenderPassUnpackInfo<'device, 'user_data, 'name, UserData>(
    diligent_sys::RenderPassUnpackInfo,
    PhantomData<(&'user_data UserData, &'device (), &'name ())>,
);

#[bon::bon]
impl<'device, 'user_data, 'name, UserData>
    RenderPassUnpackInfo<'device, 'user_data, 'name, UserData>
{
    #[builder]
    pub fn new(
        device: &'device RenderDevice,
        name: &'name CStr,
        modify_render_pass_desc: Option<(
            fn(&mut RenderPassDesc, &mut UserData),
            &'user_data mut UserData,
        )>,
    ) -> Self {
        Self(
            diligent_sys::RenderPassUnpackInfo {
                pDevice: device.sys_ptr(),
                Name: name.as_ptr(),
                ModifyRenderPassDesc: modify_render_pass_desc.as_ref().map(|desc| unsafe {
                    std::mem::transmute::<
                        fn(&mut RenderPassDesc, &mut UserData),
                        unsafe extern "C" fn(
                            *mut diligent_sys::RenderPassDesc,
                            *mut std::ffi::c_void,
                        ),
                    >(desc.0)
                }),
                pUserData: modify_render_pass_desc
                    .map_or(std::ptr::null_mut(), |desc| std::ptr::from_mut(desc.1) as _),
            },
            PhantomData,
        )
    }
}

define_ported!(
    Dearchiver,
    diligent_sys::IDearchiver,
    diligent_sys::IDearchiverMethods : 8,
    Object
);

impl Dearchiver {
    pub fn load_archive(
        &mut self,
        archive: &DataBlob,
        content_version: u32,
        make_copy: bool,
    ) -> bool {
        unsafe_member_call!(
            self,
            Dearchiver,
            LoadArchive,
            &archive.0,
            content_version,
            make_copy
        )
    }

    pub fn unpack_shader<'device, 'user_data, 'name, UserData>(
        &self,
        unpack_info: &ShaderUnpackInfo<'device, 'user_data, 'name, UserData>,
    ) -> Result<Boxed<Shader>, BoxedFromNulError> {
        let mut shader_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            Dearchiver,
            UnpackShader,
            &unpack_info.0,
            &mut shader_ptr
        );
        Boxed::new(shader_ptr)
    }

    pub fn unpack_pipeline_state<'user_data, 'device, 'name, UserData>(
        &self,
        unpack_info: &PipelineStateUnpackInfo<'user_data, 'device, 'name, UserData>,
    ) -> Result<Boxed<PipelineState>, BoxedFromNulError> {
        let mut pipeline_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            Dearchiver,
            UnpackPipelineState,
            &unpack_info.0,
            &mut pipeline_ptr
        );
        Boxed::new(pipeline_ptr)
    }

    pub fn unpack_resource_signature(
        &self,
        unpack_info: &ResourceSignatureUnpackInfo,
    ) -> Result<Boxed<PipelineResourceSignature>, BoxedFromNulError> {
        let mut rs_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            Dearchiver,
            UnpackResourceSignature,
            &unpack_info.0,
            &mut rs_ptr
        );
        Boxed::new(rs_ptr)
    }

    pub fn unpack_render_pass<'device, 'user_data, 'name, UserData>(
        &self,
        unpack_info: &RenderPassUnpackInfo<'device, 'user_data, 'name, UserData>,
    ) -> Result<Boxed<RenderPass>, BoxedFromNulError> {
        let mut render_pass_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            Dearchiver,
            UnpackRenderPass,
            &unpack_info.0,
            &mut render_pass_ptr
        );
        Boxed::new(render_pass_ptr)
    }

    pub fn store(&mut self) -> Result<Boxed<DataBlob>, BoxedFromNulError> {
        let mut data_blob_ptr = std::ptr::null_mut();
        let success = unsafe_member_call!(self, Dearchiver, Store, &mut data_blob_ptr);
        if success {
            Boxed::new(data_blob_ptr)
        } else {
            Err(BoxedFromNulError)
        }
    }

    pub fn reset(&mut self) {
        unsafe_member_call!(self, Dearchiver, Reset)
    }
    pub fn get_content_version(&self) -> u32 {
        unsafe_member_call!(self, Dearchiver, GetContentVersion)
    }
}
