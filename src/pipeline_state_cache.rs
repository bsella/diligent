use std::{ffi::CString, ops::Deref, str::FromStr};

use bitflags::bitflags;
use static_assertions::const_assert;

use crate::{data_blob::DataBlob, device_object::DeviceObject};

pub struct PipelineStateCache {
    pub(crate) sys_ptr: *mut diligent_sys::IPipelineStateCache,
    virtual_functions: *mut diligent_sys::IPipelineStateCacheVtbl,

    device_object: DeviceObject,
}

impl Deref for PipelineStateCache {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.device_object
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

pub struct PipelineStateCacheCreateInfo<T> {
    name: CString,
    mode: PsoCacheMode,
    flags: PsoCacheFlags,
    cache_data: Vec<T>,
}

impl<T> PipelineStateCacheCreateInfo<T> {
    pub fn new(name: impl AsRef<str>) -> Self {
        let name = CString::from_str(name.as_ref()).unwrap();
        Self {
            name,
            mode: PsoCacheMode::LoadStore,
            flags: PsoCacheFlags::None,
            cache_data: Vec::new(),
        }
    }
    pub fn mode(mut self, mode: PsoCacheMode) -> Self {
        self.mode = mode;
        self
    }
    pub fn flags(mut self, flags: PsoCacheFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn cache_data(mut self, cache_data: impl Into<Vec<T>>) -> Self {
        self.cache_data = cache_data.into();
        self
    }
}

impl<T> From<&PipelineStateCacheCreateInfo<T>> for diligent_sys::PipelineStateCacheCreateInfo {
    fn from(value: &PipelineStateCacheCreateInfo<T>) -> Self {
        Self {
            Desc: diligent_sys::PipelineStateCacheDesc {
                _DeviceObjectAttribs: {
                    diligent_sys::DeviceObjectAttribs {
                        Name: value.name.as_ptr(),
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
        PipelineStateCache {
            sys_ptr: fence_ptr,
            virtual_functions: unsafe { (*fence_ptr).pVtbl },
            device_object: DeviceObject::new(fence_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_data(&self) -> Result<DataBlob, ()> {
        let mut data_blob_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .PipelineStateCache
                .GetData
                .unwrap_unchecked()(self.sys_ptr, std::ptr::addr_of_mut!(data_blob_ptr));
        }

        if data_blob_ptr.is_null() {
            Err(())
        } else {
            Ok(DataBlob::new(data_blob_ptr))
        }
    }
}
