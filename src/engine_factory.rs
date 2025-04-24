use std::{os::raw::c_void, path::PathBuf};

use static_assertions::const_assert;

use super::{
    data_blob::DataBlob,
    graphics_types::{DeviceFeatures, GraphicsAdapterInfo, Version},
    object::Object,
    shader::ShaderSourceInputStreamFactory,
};

pub struct EngineCreateInfo {
    pub engine_api_version: i32,

    pub adapter_index: Option<usize>,
    pub graphics_api_version: Version,

    pub immediate_context_info: Vec<diligent_sys::ImmediateContextCreateInfo>,

    pub num_deferred_contexts: usize,

    pub features: DeviceFeatures,

    pub enable_validation: bool,

    pub validation_flags: diligent_sys::VALIDATION_FLAGS,

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
            engine_api_version: diligent_sys::DILIGENT_API_VERSION as i32,
            adapter_index: None,
            graphics_api_version: Version { major: 0, minor: 0 },

            immediate_context_info: Vec::new(),
            num_deferred_contexts: 0,

            features: DeviceFeatures::default(),

            #[cfg(debug_assertions)]
            enable_validation: true,
            #[cfg(not(debug_assertions))]
            enable_validation: false,

            validation_flags: diligent_sys::VALIDATION_FLAG_NONE as diligent_sys::VALIDATION_FLAGS,

            num_async_shader_compilation_threads: 0xFFFFFFFF,
        }
    }
}

impl From<&EngineCreateInfo> for diligent_sys::EngineCreateInfo {
    fn from(value: &EngineCreateInfo) -> Self {
        diligent_sys::EngineCreateInfo {
            EngineAPIVersion: value.engine_api_version,
            AdapterId: value
                .adapter_index
                .unwrap_or(diligent_sys::DEFAULT_ADAPTER_ID as usize) as u32,
            GraphicsAPIVersion: diligent_sys::Version {
                Major: value.graphics_api_version.minor,
                Minor: value.graphics_api_version.minor,
            },
            pImmediateContextInfo: std::ptr::null(),
            NumImmediateContexts: value.immediate_context_info.len() as u32,
            NumDeferredContexts: value.num_deferred_contexts as u32,
            Features: diligent_sys::DeviceFeatures::from(&value.features),
            EnableValidation: value.enable_validation,
            ValidationFlags: value.validation_flags,
            pRawMemAllocator: std::ptr::null_mut() as *mut diligent_sys::IMemoryAllocator,
            pAsyncShaderCompilationThreadPool: std::ptr::null_mut()
                as *mut diligent_sys::IThreadPool,
            NumAsyncShaderCompilationThreads: value.num_async_shader_compilation_threads,
            Padding: 0,
            pXRAttribs: std::ptr::null() as *const diligent_sys::OpenXRAttribs,
        }
    }
}

pub struct EngineFactory {
    pub(crate) sys_ptr: *mut diligent_sys::IEngineFactory,
    virtual_functions: *mut diligent_sys::IEngineFactoryVtbl,

    _object: Object,
}

impl EngineFactory {
    pub(crate) fn new(engine_factory_ptr: *mut diligent_sys::IEngineFactory) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::IEngineFactory>()
        );

        EngineFactory {
            sys_ptr: engine_factory_ptr,
            virtual_functions: unsafe { (*engine_factory_ptr).pVtbl },

            _object: Object::new(engine_factory_ptr as *mut diligent_sys::IObject),
        }
    }

    pub fn get_api_info(&self) -> &diligent_sys::APIInfo {
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .GetAPIInfo
                .unwrap_unchecked()(self.sys_ptr)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    pub fn create_default_shader_source_stream_factory(
        &self,
        search_directories: &[&PathBuf],
    ) -> Result<ShaderSourceInputStreamFactory, ()> {
        let mut search = String::new();

        search_directories.iter().for_each(|&dir| {
            search += dir.to_str().to_owned().unwrap();
            search += ";"
        });

        let search = std::ffi::CString::new(search).unwrap();

        let mut stream_factory_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .CreateDefaultShaderSourceStreamFactory
                .unwrap_unchecked()(
                self.sys_ptr,
                search.as_ptr(),
                std::ptr::addr_of_mut!(stream_factory_ptr),
            );
        }
        if stream_factory_ptr.is_null() {
            Err(())
        } else {
            Ok(ShaderSourceInputStreamFactory::new(stream_factory_ptr))
        }
    }

    pub fn create_data_blob<T>(&self, initial_size: usize, data: *const T) -> Result<DataBlob, ()> {
        let mut data_blob_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .CreateDataBlob
                .unwrap_unchecked()(
                self.sys_ptr,
                initial_size,
                data as *const c_void,
                std::ptr::addr_of_mut!(data_blob_ptr),
            );
        }
        if data_blob_ptr.is_null() {
            Err(())
        } else {
            Ok(DataBlob::new(data_blob_ptr))
        }
    }

    pub fn enumerate_adapters(&self, version: &Version) -> Vec<GraphicsAdapterInfo> {
        let mut num_adapters: u32 = 0;
        let version = diligent_sys::Version {
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
                self.sys_ptr,
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
                    self.sys_ptr,
                    version,
                    std::ptr::addr_of_mut!(num_adapters),
                    adapters.as_mut_ptr(),
                );

                adapters.set_len(num_adapters as usize);
            }

            adapters
                .iter()
                .map(|adapter| GraphicsAdapterInfo::from(adapter))
                .collect()
        } else {
            Vec::new()
        }
    }

    //pub fn create_dearchiver(&self, create_info : &diligent_sys::DearchiverCreateInfo) -> diligent_sys::IDearchiver;

    pub fn set_message_callback(&self, callback: diligent_sys::DebugMessageCallbackType) {
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .SetMessageCallback
                .unwrap_unchecked()(self.sys_ptr, callback)
        }
    }

    pub fn set_break_on_error(&self, break_on_error: bool) {
        unsafe {
            (*self.virtual_functions)
                .EngineFactory
                .SetBreakOnError
                .unwrap_unchecked()(self.sys_ptr, break_on_error)
        }
    }
}
