use std::os::raw::c_void;

use crate::bindings;

use super::{
    data_blob::DataBlob, device_context::DeviceContext, object::Object,
    render_device::RenderDevice, swap_chain::SwapChain,
};

pub struct EngineCreateInfo {
    engine_api_version: i32,

    adapter_id: u32,
    graphics_api_version: bindings::Version,

    // TODO
    //immediate_context_info: Option<bindings::ImmediateContextCreateInfo>,
    pub num_immediate_contexts: u32,
    pub num_deferred_contexts: u32,

    features: bindings::DeviceFeatures,

    enable_validation: bool,

    validation_flags: bindings::VALIDATION_FLAGS,

    // TODO
    //struct IMemoryAllocator* pRawMemAllocator       DEFAULT_INITIALIZER(nullptr);
    //IThreadPool* pAsyncShaderCompilationThreadPool DEFAULT_INITIALIZER(nullptr);
    num_async_shader_compilation_threads: u32,

    padding: u32,
    // TODO
    //const OpenXRAttribs *pXRAttribs DEFAULT_INITIALIZER(nullptr);
}

impl Default for EngineCreateInfo {
    fn default() -> Self {
        EngineCreateInfo {
            engine_api_version: bindings::DILIGENT_API_VERSION as i32,
            adapter_id: bindings::DEFAULT_ADAPTER_ID,
            graphics_api_version: bindings::Version { Major: 0, Minor: 0 },
            num_immediate_contexts: 0,
            num_deferred_contexts: 0,

            features: bindings::DeviceFeatures::default(),

            enable_validation: false,

            validation_flags: bindings::VALIDATION_FLAG_NONE,

            num_async_shader_compilation_threads: 0xFFFFFFFF,

            padding: 0,
        }
    }
}

impl Into<bindings::EngineCreateInfo> for EngineCreateInfo {
    fn into(self) -> bindings::EngineCreateInfo {
        bindings::EngineCreateInfo {
            EngineAPIVersion: self.engine_api_version,
            AdapterId: self.adapter_id,
            GraphicsAPIVersion: self.graphics_api_version,
            pImmediateContextInfo: std::ptr::null(),
            NumImmediateContexts: self.num_immediate_contexts,
            NumDeferredContexts: self.num_deferred_contexts,
            Features: self.features,
            EnableValidation: self.enable_validation,
            ValidationFlags: self.validation_flags,
            pRawMemAllocator: std::ptr::null_mut() as *mut bindings::IMemoryAllocator,
            pAsyncShaderCompilationThreadPool: std::ptr::null_mut() as *mut bindings::IThreadPool,
            NumAsyncShaderCompilationThreads: self.num_async_shader_compilation_threads,
            Padding: self.padding,
            pXRAttribs: std::ptr::null() as *const bindings::OpenXRAttribs,
        }
    }
}

pub struct EngineFactory {
    pub(crate) m_engine_factory: *mut bindings::IEngineFactory,
    m_virtual_functions: *mut bindings::IEngineFactoryVtbl,

    m_object: Object,
}

pub trait AsEngineFactory {
    fn as_engine_factory(&self) -> &EngineFactory;
}

pub trait EngineFactoryImplementation {
    type EngineCreateInfo;

    fn get() -> Self;

    fn create_device_and_contexts(
        &self,
        create_info: Self::EngineCreateInfo,
    ) -> Option<(RenderDevice, Vec<DeviceContext>, Vec<DeviceContext>)>;

    fn create_swap_chain(
        &self,
        device: &RenderDevice,
        immediate_context: &DeviceContext,
        swapchain_desc: &bindings::SwapChainDesc,
        window: Option<&bindings::NativeWindow>,
    ) -> Option<SwapChain>;
}

impl EngineFactory {
    pub(crate) fn new(engine_factory_ptr: *mut bindings::IEngineFactory) -> Self {
        EngineFactory {
            m_engine_factory: engine_factory_ptr,
            m_virtual_functions: unsafe { (*engine_factory_ptr).pVtbl },
            m_object: Object::new(engine_factory_ptr as *mut bindings::IObject),
        }
    }

    fn get_api_info(&self) -> &bindings::APIInfo {
        unsafe {
            (*self.m_virtual_functions)
                .EngineFactory
                .GetAPIInfo
                .unwrap_unchecked()(self.m_engine_factory)
            .as_ref()
            .unwrap_unchecked()
        }
    }

    //fn create_default_shader_source_stream_factory(&self, search_directories: Vec<PathBuf>) -> bindings::IShaderSourceInputStreamFactory;

    fn create_data_blob<T>(&self, initial_size: usize, data: *const T) -> Option<DataBlob> {
        let mut data_blob_ptr: *mut bindings::IDataBlob = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .EngineFactory
                .CreateDataBlob
                .unwrap_unchecked()(
                self.m_engine_factory,
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
    fn enumerate_adapters(&self, version: bindings::Version) -> Vec<bindings::GraphicsAdapterInfo> {
        let mut num_adapters: u32 = 0;
        let adapters_ptr = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .EngineFactory
                .EnumerateAdapters
                .unwrap_unchecked()(
                self.m_engine_factory,
                version,
                &mut num_adapters,
                adapters_ptr,
            );

            let num_adapters = num_adapters as usize;

            Vec::from_raw_parts(adapters_ptr, num_adapters, num_adapters)
        }
    }

    //fn create_dearchiver(&self, create_info : &bindings::DearchiverCreateInfo) -> bindings::IDearchiver;

    fn set_message_callback(&self, callback: bindings::DebugMessageCallbackType) {
        unsafe {
            (*self.m_virtual_functions)
                .EngineFactory
                .SetMessageCallback
                .unwrap_unchecked()(self.m_engine_factory, callback)
        }
    }
    fn set_break_on_error(&self, break_on_error: bool) {
        unsafe {
            (*self.m_virtual_functions)
                .EngineFactory
                .SetBreakOnError
                .unwrap_unchecked()(self.m_engine_factory, break_on_error)
        }
    }
}
