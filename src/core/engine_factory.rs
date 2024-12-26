use std::os::raw::c_void;

use crate::core::bindings;

use crate::core::data_blob::DataBlob;

pub struct EngineFactory {
    pub(crate) m_engine_factory: *mut bindings::IEngineFactory,
    pub(crate) m_virtual_functions: *mut bindings::IEngineFactoryVtbl,
}

pub trait AsEngineFactory {
    fn as_engine_factory(&self) -> &EngineFactory;
}

impl EngineFactory {
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
