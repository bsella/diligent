use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use bitflags::bitflags;

use crate::{
    data_blob::DataBlob,
    device_object::{DeviceObject, DeviceObjectAttribs},
};

define_ported!(
    PipelineStateCache,
    diligent_sys::IPipelineStateCache,
    diligent_sys::IPipelineStateCacheMethods : 1,
    DeviceObject
);

#[derive(Clone, Copy)]
pub enum PsoCacheMode {
    Load,
    Store,
    LoadStore,
}

bitflags! {
    #[derive(Clone,Copy)]
    pub struct PsoCacheFlags: diligent_sys::PSO_CACHE_FLAGS {
        const None    = diligent_sys::PSO_CACHE_FLAG_NONE as diligent_sys::PSO_CACHE_FLAGS;
        const Verbose = diligent_sys::PSO_CACHE_FLAG_VERBOSE as diligent_sys::PSO_CACHE_FLAGS;
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct PipelineStateCacheCreateInfo<'name, 'data, T>(
    pub(crate) diligent_sys::PipelineStateCacheCreateInfo,
    PhantomData<(&'name (), &'data (), T)>,
);

impl<T> Deref for PipelineStateCacheCreateInfo<'_, '_, T> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name, 'data, T> PipelineStateCacheCreateInfo<'name, 'data, T> {
    #[builder(derive(Clone))]
    pub fn new(
        name: Option<&'name CStr>,
        mode: PsoCacheMode,
        flags: PsoCacheFlags,
        cache_data: &'data [T],
    ) -> Self {
        PipelineStateCacheCreateInfo(
            diligent_sys::PipelineStateCacheCreateInfo {
                Desc: diligent_sys::PipelineStateCacheDesc {
                    _DeviceObjectAttribs: {
                        diligent_sys::DeviceObjectAttribs {
                            Name: name.as_ref().map_or(std::ptr::null(), |name| name.as_ptr()),
                        }
                    },
                    Mode: match mode {
                        PsoCacheMode::Load => diligent_sys::PSO_CACHE_MODE_LOAD,
                        PsoCacheMode::Store => diligent_sys::PSO_CACHE_MODE_STORE,
                        PsoCacheMode::LoadStore => diligent_sys::PSO_CACHE_MODE_LOAD_STORE,
                    } as _,
                    Flags: flags.bits(),
                },
                pCacheData: cache_data.as_ptr() as _,
                CacheDataSize: std::mem::size_of_val(cache_data) as u32,
            },
            PhantomData,
        )
    }
}

impl PipelineStateCache {
    pub fn get_data(&self) -> Option<&DataBlob> {
        let mut data_blob_ptr = std::ptr::null_mut();
        unsafe_member_call!(self, PipelineStateCache, GetData, &mut data_blob_ptr);

        if data_blob_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(data_blob_ptr as *const DataBlob) })
        }
    }
}
