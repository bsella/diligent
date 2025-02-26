use std::os::raw::c_void;

use crate::bindings;

use super::{
    data_blob::DataBlob,
    graphics_types::{DeviceFeatures, GraphicsAdapterInfo, Version},
    object::Object,
};

pub struct EngineCreateInfo {
    pub engine_api_version: i32,

    pub adapter_index: Option<usize>,
    pub graphics_api_version: Version,

    // TODO
    //immediate_context_info: Option<bindings::ImmediateContextCreateInfo>,
    pub num_immediate_contexts: u32,
    pub num_deferred_contexts: u32,

    pub features: DeviceFeatures,

    pub enable_validation: bool,

    pub validation_flags: bindings::VALIDATION_FLAGS,

    // TODO
    //struct IMemoryAllocator* pRawMemAllocator       DEFAULT_INITIALIZER(nullptr);
    //IThreadPool* pAsyncShaderCompilationThreadPool DEFAULT_INITIALIZER(nullptr);
    pub num_async_shader_compilation_threads: u32,
    // TODO
    //const OpenXRAttribs *pXRAttribs DEFAULT_INITIALIZER(nullptr);
}

impl Default for EngineCreateInfo {
    fn default() -> Self {
        EngineCreateInfo {
            engine_api_version: bindings::DILIGENT_API_VERSION as i32,
            adapter_index: None,
            graphics_api_version: Version { major: 0, minor: 0 },

            num_immediate_contexts: 0,
            num_deferred_contexts: 0,

            features: DeviceFeatures::default(),

            #[cfg(debug_assertions)]
            enable_validation: true,
            #[cfg(not(debug_assertions))]
            enable_validation: false,

            validation_flags: bindings::VALIDATION_FLAG_NONE,

            num_async_shader_compilation_threads: 0xFFFFFFFF,
        }
    }
}

impl From<&EngineCreateInfo> for bindings::EngineCreateInfo {
    fn from(value: &EngineCreateInfo) -> Self {
        bindings::EngineCreateInfo {
            EngineAPIVersion: value.engine_api_version,
            AdapterId: value
                .adapter_index
                .unwrap_or(bindings::DEFAULT_ADAPTER_ID as usize) as u32,
            GraphicsAPIVersion: bindings::Version {
                Major: value.graphics_api_version.minor,
                Minor: value.graphics_api_version.minor,
            },
            pImmediateContextInfo: std::ptr::null(),
            NumImmediateContexts: value.num_immediate_contexts,
            NumDeferredContexts: value.num_deferred_contexts,
            Features: bindings::DeviceFeatures::from(&value.features),
            EnableValidation: value.enable_validation,
            ValidationFlags: value.validation_flags,
            pRawMemAllocator: std::ptr::null_mut() as *mut bindings::IMemoryAllocator,
            pAsyncShaderCompilationThreadPool: std::ptr::null_mut() as *mut bindings::IThreadPool,
            NumAsyncShaderCompilationThreads: value.num_async_shader_compilation_threads,
            Padding: 0,
            pXRAttribs: std::ptr::null() as *const bindings::OpenXRAttribs,
        }
    }
}

pub struct EngineFactory {
    pub(crate) engine_factory: *mut bindings::IEngineFactory,
    virtual_functions: *mut bindings::IEngineFactoryVtbl,

    _object: Object,
}

pub trait AsEngineFactory {
    fn as_engine_factory(&self) -> &EngineFactory;
}

impl EngineFactory {
    pub(crate) fn new(engine_factory: *mut bindings::IEngineFactory) -> Self {
        EngineFactory {
            engine_factory,
            virtual_functions: unsafe { (*engine_factory).pVtbl },

            _object: Object::new(engine_factory as *mut bindings::IObject),
        }
    }

    pub fn get_api_info(&self) -> &bindings::APIInfo {
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .GetAPIInfo
                .unwrap_unchecked()(self.engine_factory)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    //fn create_default_shader_source_stream_factory(&self, search_directories: Vec<PathBuf>) -> bindings::IShaderSourceInputStreamFactory;

    pub fn create_data_blob<T>(&self, initial_size: usize, data: *const T) -> Option<DataBlob> {
        let mut data_blob_ptr: *mut bindings::IDataBlob = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .CreateDataBlob
                .unwrap_unchecked()(
                self.engine_factory,
                initial_size,
                data as *const c_void,
                std::ptr::addr_of_mut!(data_blob_ptr),
            );
        }
        if data_blob_ptr.is_null() {
            None
        } else {
            Some(DataBlob::new(data_blob_ptr))
        }
    }

    pub fn enumerate_adapters(&self, version: &Version) -> Vec<GraphicsAdapterInfo> {
        let mut num_adapters: u32 = 0;
        let version = bindings::Version {
            Major: version.major,
            Minor: version.minor,
        };

        // The first call of EnumerateAdapters with nullptr as the adapters gets the number
        // of adapters
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .EnumerateAdapters
                .unwrap_unchecked()(
                self.engine_factory,
                version,
                std::ptr::addr_of_mut!(num_adapters),
                std::ptr::null_mut(),
            );
        }

        if num_adapters > 0 {
            let mut adapters = Vec::with_capacity(num_adapters as usize);
            // The second call of EnumerateAdapters gets a pointer to the adapters
            unsafe {
                (*self.virtual_functions)
                    .EngineFactory
                    .EnumerateAdapters
                    .unwrap_unchecked()(
                    self.engine_factory,
                    version,
                    std::ptr::addr_of_mut!(num_adapters),
                    adapters.as_mut_ptr(),
                );

                adapters.set_len(num_adapters as usize);
            }

            adapters.iter().map(|&adapter| adapter.into()).collect()
        } else {
            Vec::new()
        }
    }

    //pub fn create_dearchiver(&self, create_info : &bindings::DearchiverCreateInfo) -> bindings::IDearchiver;

    pub fn set_message_callback(&self, callback: bindings::DebugMessageCallbackType) {
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .SetMessageCallback
                .unwrap_unchecked()(self.engine_factory, callback)
        }
    }

    pub fn set_break_on_error(&self, break_on_error: bool) {
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .SetBreakOnError
                .unwrap_unchecked()(self.engine_factory, break_on_error)
        }
    }
}
