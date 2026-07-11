use std::{marker::PhantomData, os::raw::c_void, path::Path};

use crate::{
    APIInfo, Boxed, BoxedFromNulError, Dearchiver, ImmediateContextCreateInfo, ValidationFlags,
    data_blob::DataBlob,
    graphics_types::{DeviceFeatures, GraphicsAdapterInfo, Version},
    memory_allocator::MemoryAllocator,
    object::Object,
    shader::ShaderSourceInputStreamFactory,
};

#[repr(transparent)]
#[derive(Clone)]
pub struct DearchiverCreateInfo(diligent_sys::DearchiverCreateInfo);

#[bon::bon]
impl DearchiverCreateInfo {
    #[builder(derive(Clone))]
    pub fn new() -> Self {
        Self(diligent_sys::DearchiverCreateInfo {
            pDummy: std::ptr::null_mut(),
        })
    }
}

#[repr(transparent)]
pub struct EngineCreateInfo<'immediate_context_info>(
    pub(crate) diligent_sys::EngineCreateInfo,
    PhantomData<&'immediate_context_info ()>,
);

#[bon::bon]
impl<'immediate_context_info> EngineCreateInfo<'immediate_context_info> {
    #[builder]
    pub fn new(
        #[builder(default = diligent_sys::DILIGENT_API_VERSION)] engine_api_version: u32,
        adapter_index: Option<usize>,
        #[builder(default = Version { major: 0, minor: 0 })] graphics_api_version: Version,
        #[builder(default = &[])]
        immediate_context_info: &'immediate_context_info [ImmediateContextCreateInfo],
        #[builder(default = 0)] num_deferred_contexts: usize,
        #[builder(default = DeviceFeatures::default())] features: DeviceFeatures,
        #[builder(default = cfg!(debug_assertions))] enable_validation: bool,
        #[builder(default = ValidationFlags::None)] validation_flags: ValidationFlags,
        #[builder(default = 0xFFFFFFFF)] num_async_shader_compilation_threads: u32,
        // TODO
        //IThreadPool* pAsyncShaderCompilationThreadPool DEFAULT_INITIALIZER(nullptr);
        // TODO
        //const OpenXRAttribs *pXRAttribs DEFAULT_INITIALIZER(nullptr);
    ) -> Self {
        EngineCreateInfo(
            diligent_sys::EngineCreateInfo {
                EngineAPIVersion: engine_api_version as i32,
                AdapterId: adapter_index.unwrap_or(diligent_sys::DEFAULT_ADAPTER_ID as usize)
                    as u32,
                GraphicsAPIVersion: diligent_sys::Version {
                    Major: graphics_api_version.minor,
                    Minor: graphics_api_version.minor,
                },
                pImmediateContextInfo: if immediate_context_info.is_empty() {
                    std::ptr::null()
                } else {
                    immediate_context_info.as_ptr() as _
                },
                NumImmediateContexts: immediate_context_info.len() as u32,
                NumDeferredContexts: num_deferred_contexts as u32,
                Features: features.0,
                EnableValidation: enable_validation,
                ValidationFlags: validation_flags.bits(),
                pAsyncShaderCompilationThreadPool: std::ptr::null_mut(),
                NumAsyncShaderCompilationThreads: num_async_shader_compilation_threads,
                Padding: 0,
                pXRAttribs: std::ptr::null(),
            },
            PhantomData,
        )
    }
}

impl EngineCreateInfo<'_> {
    pub fn engine_api_version(&self) -> u32 {
        self.0.EngineAPIVersion as u32
    }
    pub fn adapter_index(&self) -> usize {
        self.0.AdapterId as usize
    }
    pub fn graphics_api_version(&self) -> Version {
        Version {
            major: self.0.GraphicsAPIVersion.Major,
            minor: self.0.GraphicsAPIVersion.Minor,
        }
    }
    pub fn immediate_context_info(&self) -> &[ImmediateContextCreateInfo] {
        if self.0.pImmediateContextInfo.is_null() {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    self.0.pImmediateContextInfo as _,
                    self.0.NumImmediateContexts as usize,
                )
            }
        }
    }
    pub fn num_deferred_contexts(&self) -> usize {
        self.0.NumDeferredContexts as usize
    }
    pub fn features(&self) -> &DeviceFeatures {
        let ptr = std::ptr::from_ref(&self.0.Features);
        unsafe { &*(ptr as *const DeviceFeatures) }
    }
    pub fn enable_validation(&self) -> bool {
        self.0.EnableValidation
    }
    pub fn validation_flags(&self) -> ValidationFlags {
        ValidationFlags::from_bits(self.0.ValidationFlags).unwrap()
    }
    pub fn num_async_shader_compilation_threads(&self) -> u32 {
        self.0.NumAsyncShaderCompilationThreads
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

    pub fn set_message_callback(&mut self, callback: diligent_sys::DebugMessageCallbackType) {
        unsafe_member_call!(self, EngineFactory, SetMessageCallback, callback)
    }

    pub fn set_break_on_error(&mut self, break_on_error: bool) {
        unsafe_member_call!(self, EngineFactory, SetBreakOnError, break_on_error)
    }

    pub fn set_memory_allocator(&mut self, allocator: &MemoryAllocator) {
        unsafe_member_call!(
            &self,
            EngineFactory,
            SetMemoryAllocator,
            std::ptr::from_ref(&allocator.0) as *mut _
        );
    }
}

// # Safety : Access to EngineFactory can be thread safe
unsafe impl Sync for EngineFactory {}
