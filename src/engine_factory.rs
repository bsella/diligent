use std::{os::raw::c_void, path::Path};

use crate::{
    APIInfo, Boxed, BoxedFromNulError, Dearchiver, ImmediateContextCreateInfo, ValidationFlags,
    data_blob::DataBlob,
    graphics_types::{DeviceFeatures, GraphicsAdapterInfo, Version},
    memory_allocator::MemoryAllocator,
    object::Object,
    shader::ShaderSourceInputStreamFactory,
};

pub struct DearchiverCreateInfo(diligent_sys::DearchiverCreateInfo);

#[bon::bon]
impl DearchiverCreateInfo {
    #[builder]
    pub fn new() -> Self {
        Self(diligent_sys::DearchiverCreateInfo {
            pDummy: std::ptr::null_mut(),
        })
    }
}

pub struct EngineCreateInfo {
    pub engine_api_version: i32,

    pub adapter_index: Option<usize>,
    pub graphics_api_version: Version,

    pub immediate_context_info: Vec<ImmediateContextCreateInfo>,

    pub num_deferred_contexts: usize,

    pub features: DeviceFeatures,

    pub enable_validation: bool,

    pub validation_flags: ValidationFlags,

    // TODO
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

            validation_flags: ValidationFlags::None,

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
            pImmediateContextInfo: if value.immediate_context_info.is_empty() {
                std::ptr::null()
            } else {
                value.immediate_context_info.as_ptr() as _
            },
            NumImmediateContexts: value.immediate_context_info.len() as u32,
            NumDeferredContexts: value.num_deferred_contexts as u32,
            Features: value.features.0,
            EnableValidation: value.enable_validation,
            ValidationFlags: value.validation_flags.bits(),
            pAsyncShaderCompilationThreadPool: std::ptr::null_mut(),
            NumAsyncShaderCompilationThreads: value.num_async_shader_compilation_threads,
            Padding: 0,
            pXRAttribs: std::ptr::null(),
        }
    }
}

define_ported!(
    EngineFactory,
    diligent_sys::IEngineFactory,
    diligent_sys::IEngineFactoryMethods : 8,
    Object
);

impl EngineFactory {
    pub fn get_api_info(&self) -> &APIInfo {
        let api_info_ptr = unsafe_member_call!(self, EngineFactory, GetAPIInfo) as *const APIInfo;
        unsafe { api_info_ptr.as_ref().unwrap_unchecked() }
    }

    pub fn create_default_shader_source_stream_factory(
        &self,
        search_directories: &[&Path],
    ) -> Result<Boxed<ShaderSourceInputStreamFactory>, BoxedFromNulError> {
        let mut search = String::new();

        search_directories.iter().for_each(|&dir| {
            search += dir.to_str().to_owned().unwrap();
            search += ";"
        });

        let search = std::ffi::CString::new(search).unwrap();

        let mut stream_factory_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            EngineFactory,
            CreateDefaultShaderSourceStreamFactory,
            search.as_ptr(),
            &mut stream_factory_ptr
        );

        Boxed::new(stream_factory_ptr)
    }

    pub fn create_empty_data_blob(
        &self,
        initial_size: usize,
    ) -> Result<Boxed<DataBlob>, BoxedFromNulError> {
        let mut data_blob_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            EngineFactory,
            CreateDataBlob,
            initial_size,
            std::ptr::null(),
            &mut data_blob_ptr
        );

        Boxed::new(data_blob_ptr)
    }

    pub fn create_data_blob<T>(&self, data: &T) -> Result<Boxed<DataBlob>, BoxedFromNulError> {
        let mut data_blob_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            EngineFactory,
            CreateDataBlob,
            std::mem::size_of_val(data),
            std::ptr::from_ref(data) as *const c_void,
            &mut data_blob_ptr
        );

        Boxed::new(data_blob_ptr)
    }

    pub fn enumerate_adapters(&self, version: Version) -> Vec<GraphicsAdapterInfo> {
        let mut num_adapters: u32 = 0;
        let version = diligent_sys::Version {
            Major: version.major,
            Minor: version.minor,
        };

        // The first call of EnumerateAdapters with nullptr as the adapters gets the number
        // of adapters
        unsafe_member_call!(
            self,
            EngineFactory,
            EnumerateAdapters,
            version,
            &mut num_adapters,
            std::ptr::null_mut()
        );

        if num_adapters > 0 {
            let mut adapters: Vec<GraphicsAdapterInfo> = Vec::with_capacity(num_adapters as usize);
            // The second call of EnumerateAdapters gets a pointer to the adapters
            unsafe_member_call!(
                self,
                EngineFactory,
                EnumerateAdapters,
                version,
                &mut num_adapters,
                adapters.as_mut_ptr() as _
            );

            unsafe {
                adapters.set_len(num_adapters as usize);
            };

            adapters
        } else {
            Vec::new()
        }
    }

    pub fn create_dearchiver(
        &self,
        create_info: &DearchiverCreateInfo,
    ) -> Result<Boxed<Dearchiver>, BoxedFromNulError> {
        let mut dearchiver_ptr = std::ptr::null_mut();
        unsafe_member_call!(
            self,
            EngineFactory,
            CreateDearchiver,
            &create_info.0,
            &mut dearchiver_ptr
        );
        Boxed::new(dearchiver_ptr)
    }

    pub fn set_message_callback(&self, callback: diligent_sys::DebugMessageCallbackType) {
        unsafe_member_call!(self, EngineFactory, SetMessageCallback, callback)
    }

    pub fn set_break_on_error(&self, break_on_error: bool) {
        unsafe_member_call!(self, EngineFactory, SetBreakOnError, break_on_error)
    }

    pub fn set_memory_allocator(&self, allocator: &MemoryAllocator) {
        unsafe_member_call!(
            &self,
            EngineFactory,
            SetMemoryAllocator,
            std::ptr::from_ref(&allocator.0) as *mut _
        );
    }
}
