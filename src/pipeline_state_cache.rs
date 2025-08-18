use std::{ffi::CString, ops::Deref};

use bitflags::bitflags;
use bon::Builder;
use static_assertions::{const_assert, const_assert_eq};

use crate::{data_blob::DataBlob, device_object::DeviceObject};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IPipelineStateCacheMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct PipelineStateCache(DeviceObject);

impl Deref for PipelineStateCache {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    pub(crate) fn new(fence_ptr: *mut diligent_sys::IPipelineStateCache) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IPipelineStateCache>()
        );
        Self(DeviceObject::new(
            fence_ptr as *mut diligent_sys::IDeviceObject,
        ))
    }

    pub fn get_data(&self) -> Result<DataBlob, ()> {
        let mut data_blob_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            PipelineStateCache,
            GetData,
            std::ptr::addr_of_mut!(data_blob_ptr)
        );

        if data_blob_ptr.is_null() {
            Err(())
        } else {
            Ok(DataBlob::new(data_blob_ptr))
        }
    }
}
