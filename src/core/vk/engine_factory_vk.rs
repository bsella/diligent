use std::ffi::c_void;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::bindings;
use crate::core::device_context::DeviceContext;
use crate::core::engine_factory::EngineCreateInfo;
use crate::core::engine_factory::EngineFactory;

use crate::core::engine_factory::AsEngineFactory;
use crate::core::engine_factory::EngineFactoryImplementation;
use crate::core::render_device::RenderDevice;
use crate::core::swap_chain::SwapChain;

pub struct EngineVkCreateInfo {
    engine_create_info: EngineCreateInfo,

    instance_layer_names: Vec<String>,
    instance_extension_names: Vec<String>,
    device_extension_names: Vec<String>,
    device_extension_features: *mut c_void,
    vk_allocator: *mut c_void,
    ignore_debug_message_names: Vec<String>,

    main_descriptor_pool_size: bindings::VulkanDescriptorPoolSize,
    dynamic_descriptor_pool_size: bindings::VulkanDescriptorPoolSize,

    device_local_memory_page_size: u32,
    host_visible_memory_page_size: u32,
    device_local_memory_reserve_size: u32,
    host_visible_memory_reserve_size: u32,
    upload_heap_page_size: u32,
    dynamic_heap_size: u32,
    dynamic_heap_page_size: u32,

    query_pool_sizes: [u32; 6],
    dx_compiler_path: Option<PathBuf>,
}

impl Default for EngineVkCreateInfo {
    fn default() -> Self {
        EngineVkCreateInfo {
            engine_create_info: EngineCreateInfo::default(),

            instance_layer_names: Vec::new(),
            instance_extension_names: Vec::new(),
            device_extension_names: Vec::new(),
            device_extension_features: std::ptr::null_mut(),
            vk_allocator: std::ptr::null_mut(),
            ignore_debug_message_names: Vec::new(),
            main_descriptor_pool_size: bindings::VulkanDescriptorPoolSize {
                MaxDescriptorSets: 8192,
                NumSeparateSamplerDescriptors: 1024,
                NumCombinedSamplerDescriptors: 8192,
                NumSampledImageDescriptors: 8192,
                NumStorageImageDescriptors: 1024,
                NumUniformBufferDescriptors: 4096,
                NumStorageBufferDescriptors: 4096,
                NumUniformTexelBufferDescriptors: 1024,
                NumStorageTexelBufferDescriptors: 1024,
                NumInputAttachmentDescriptors: 256,
                NumAccelStructDescriptors: 256,
            },

            dynamic_descriptor_pool_size: bindings::VulkanDescriptorPoolSize {
                MaxDescriptorSets: 2048,
                NumSeparateSamplerDescriptors: 256,
                NumCombinedSamplerDescriptors: 2048,
                NumSampledImageDescriptors: 2048,
                NumStorageImageDescriptors: 256,
                NumUniformBufferDescriptors: 1024,
                NumStorageBufferDescriptors: 1024,
                NumUniformTexelBufferDescriptors: 256,
                NumStorageTexelBufferDescriptors: 256,
                NumInputAttachmentDescriptors: 64,
                NumAccelStructDescriptors: 64,
            },

            device_local_memory_page_size: 16 << 20,
            host_visible_memory_page_size: 16 << 20,
            device_local_memory_reserve_size: 256 << 20,
            host_visible_memory_reserve_size: 256 << 20,
            upload_heap_page_size: 1 << 20,
            dynamic_heap_size: 8 << 20,
            dynamic_heap_page_size: 256 << 10,

            query_pool_sizes: [
                0,   // Ignored
                128, // QUERY_TYPE_OCCLUSION
                128, // QUERY_TYPE_BINARY_OCCLUSION
                512, // QUERY_TYPE_TIMESTAMP
                128, // QUERY_TYPE_PIPELINE_STATISTICS
                256, // QUERY_TYPE_DURATION
            ],

            dx_compiler_path: None,
        }
    }
}

impl Into<bindings::EngineVkCreateInfo> for EngineVkCreateInfo {
    fn into(self) -> bindings::EngineVkCreateInfo {
        bindings::EngineVkCreateInfo {
            _EngineCreateInfo: self.engine_create_info.into(),
            InstanceLayerCount: self.instance_layer_names.len() as u32,
            ppInstanceLayerNames: self
                .instance_layer_names
                .iter()
                .map(|s| s.as_ptr() as *const i8)
                .collect::<Vec<*const i8>>()
                .as_ptr(),
            InstanceExtensionCount: self.instance_extension_names.len() as u32,
            ppInstanceExtensionNames: self
                .instance_extension_names
                .iter()
                .map(|s| s.as_ptr() as *const i8)
                .collect::<Vec<*const i8>>()
                .as_ptr(),

            DeviceExtensionCount: self.device_extension_names.len() as u32,
            ppDeviceExtensionNames: self
                .device_extension_names
                .iter()
                .map(|s| s.as_ptr() as *const i8)
                .collect::<Vec<*const i8>>()
                .as_ptr(),

            pDeviceExtensionFeatures: self.device_extension_features,

            pVkAllocator: self.vk_allocator,

            IgnoreDebugMessageCount: self.ignore_debug_message_names.len() as u32,
            ppIgnoreDebugMessageNames: self
                .ignore_debug_message_names
                .iter()
                .map(|s| s.as_ptr() as *const i8)
                .collect::<Vec<*const i8>>()
                .as_ptr(),

            MainDescriptorPoolSize: self.main_descriptor_pool_size,

            DynamicDescriptorPoolSize: self.dynamic_descriptor_pool_size,

            DeviceLocalMemoryPageSize: self.device_local_memory_page_size,
            HostVisibleMemoryPageSize: self.host_visible_memory_page_size,
            DeviceLocalMemoryReserveSize: self.device_local_memory_reserve_size,
            HostVisibleMemoryReserveSize: self.host_visible_memory_reserve_size,
            UploadHeapPageSize: self.upload_heap_page_size,
            DynamicHeapSize: self.dynamic_heap_size,
            DynamicHeapPageSize: self.dynamic_heap_page_size,

            QueryPoolSizes: self.query_pool_sizes,

            pDxCompilerPath: if let Some(path) = self.dx_compiler_path {
                path.as_os_str().as_bytes().as_ptr() as *const i8
            } else {
                std::ptr::null()
            },
        }
    }
}

pub struct EngineFactoryVk {
    engine_factory_vk: *mut bindings::IEngineFactoryVk,
    virtual_functions: *mut bindings::IEngineFactoryVkVtbl,

    engine_factory: EngineFactory,
}

impl EngineFactoryVk {
    pub(crate) fn new(engine_factory_ptr: *mut bindings::IEngineFactoryVk) -> Self {
        EngineFactoryVk {
            engine_factory_vk: engine_factory_ptr,
            virtual_functions: unsafe { (*engine_factory_ptr).pVtbl },

            engine_factory: EngineFactory::new(engine_factory_ptr as *mut bindings::IEngineFactory),
        }
    }

    pub fn enable_device_simulation(&self) {
        unsafe {
            (*self.virtual_functions)
                .EngineFactoryVk
                .EnableDeviceSimulation
                .unwrap_unchecked()(self.engine_factory_vk);
        }
    }
}

impl AsEngineFactory for EngineFactoryVk {
    #[inline]
    fn as_engine_factory(&self) -> &EngineFactory {
        &self.engine_factory
    }
}

impl EngineFactoryImplementation for EngineFactoryVk {
    type EngineCreateInfo = EngineVkCreateInfo;

    fn get() -> Self {
        let engine_factory_vk = unsafe { bindings::Diligent_GetEngineFactoryVk() };
        let engine_factory_ptr = engine_factory_vk as *mut bindings::IEngineFactory;

        EngineFactoryVk {
            engine_factory: EngineFactory::new(engine_factory_ptr),
            virtual_functions: unsafe { (*engine_factory_vk).pVtbl },
            engine_factory_vk: engine_factory_vk,
        }
    }

    fn create_device_and_contexts(
        &self,
        create_info: Self::EngineCreateInfo,
    ) -> Option<(RenderDevice, Vec<DeviceContext>, Vec<DeviceContext>)> {
        let num_immediate_contexts =
            std::cmp::max(create_info.engine_create_info.num_immediate_contexts, 1) as usize;
        let num_deferred_contexts = create_info.engine_create_info.num_deferred_contexts as usize;

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptrs = Vec::from_iter(
            std::iter::repeat(std::ptr::null_mut())
                .take(num_immediate_contexts + num_deferred_contexts),
        );

        {
            let create_info: bindings::EngineVkCreateInfo = create_info.into();
            unsafe {
                (*self.virtual_functions)
                    .EngineFactoryVk
                    .CreateDeviceAndContextsVk
                    .unwrap_unchecked()(
                    self.engine_factory_vk,
                    std::ptr::addr_of!(create_info),
                    std::ptr::addr_of_mut!(render_device_ptr),
                    device_context_ptrs.as_mut_ptr(),
                )
            }
        }

        if render_device_ptr.is_null() {
            None
        } else {
            Some((
                RenderDevice::new(render_device_ptr),
                Vec::from_iter(
                    device_context_ptrs
                        .iter()
                        .take(num_immediate_contexts)
                        .map(|dc_ptr| DeviceContext::new(*dc_ptr)),
                ),
                Vec::from_iter(
                    device_context_ptrs
                        .iter()
                        .rev()
                        .take(num_deferred_contexts)
                        .map(|dc_ptr| DeviceContext::new(*dc_ptr)),
                ),
            ))
        }
    }

    fn create_swap_chain(
        &self,
        device: &RenderDevice,
        immediate_context: &DeviceContext,
        swapchain_desc: &bindings::SwapChainDesc,
        window: Option<&bindings::NativeWindow>,
    ) -> Option<SwapChain> {
        let mut swap_chain_ptr = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .EngineFactoryVk
                .CreateSwapChainVk
                .unwrap_unchecked()(
                self.engine_factory_vk,
                device.render_device,
                immediate_context.device_context,
                std::ptr::from_ref(swapchain_desc),
                if let Some(window) = window {
                    std::ptr::from_ref(window)
                } else {
                    std::ptr::null()
                },
                std::ptr::addr_of_mut!(swap_chain_ptr),
            );
        }
        if swap_chain_ptr.is_null() {
            None
        } else {
            Some(SwapChain::new(swap_chain_ptr))
        }
    }
}
