use std::ffi::CString;

use bitflags::bitflags;
use bon::Builder;

use crate::{data_blob::DataBlob, device_object::DeviceObject};

define_ported!(
    PipelineStateCache,
    diligent_sys::IPipelineStateCache,
    diligent_sys::IPipelineStateCacheMethods : 1,
    DeviceObject
);

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

#[derive(Builder)]
pub struct PipelineStateCacheCreateInfo<T> {
    #[builder(with =|name : impl AsRef<str>| CString::new(name.as_ref()).unwrap())]
    name: Option<CString>,
    mode: PsoCacheMode,
    flags: PsoCacheFlags,
    cache_data: Vec<T>,
}

impl<T> From<&PipelineStateCacheCreateInfo<T>> for diligent_sys::PipelineStateCacheCreateInfo {
    fn from(value: &PipelineStateCacheCreateInfo<T>) -> Self {
        Self {
            Desc: diligent_sys::PipelineStateCacheDesc {
                _DeviceObjectAttribs: {
                    diligent_sys::DeviceObjectAttribs {
                        Name: value
                            .name
                            .as_ref()
                            .map_or(std::ptr::null(), |name| name.as_ptr()),
                    }
                },
                Mode: match value.mode {
                    PsoCacheMode::Load => diligent_sys::PSO_CACHE_MODE_LOAD,
                    PsoCacheMode::Store => diligent_sys::PSO_CACHE_MODE_STORE,
                    PsoCacheMode::LoadStore => diligent_sys::PSO_CACHE_MODE_LOAD_STORE,
                } as _,
                Flags: value.flags.bits(),
            },
            pCacheData: value.cache_data.as_ptr() as _,
            CacheDataSize: (value.cache_data.len() * std::mem::size_of::<T>()) as u32,
        }
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
