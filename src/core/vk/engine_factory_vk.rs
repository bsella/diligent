use std::ffi::c_void;

use std::path::PathBuf;

use crate::core::device_context::DeviceContext;
use crate::core::engine_factory::EngineCreateInfo;
use crate::core::engine_factory::EngineFactory;

use crate::core::engine_factory::AsEngineFactory;
use crate::core::render_device::RenderDevice;
use crate::core::swap_chain::SwapChain;
use crate::core::swap_chain::SwapChainDesc;
use crate::tools::native_app::NativeWindow;

pub struct EngineVkCreateInfo {
    engine_create_info: EngineCreateInfo,

    // TODO : find a way to replace all of the following Vec<String> with Vec<&str>
    instance_layer_names: Vec<String>,
    instance_extension_names: Vec<String>,
    device_extension_names: Vec<String>,
    device_extension_features: *mut c_void,
    vk_allocator: *mut c_void,
    ignore_debug_message_names: Vec<String>,

    main_descriptor_pool_size: diligent_sys::VulkanDescriptorPoolSize,
    dynamic_descriptor_pool_size: diligent_sys::VulkanDescriptorPoolSize,

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

impl EngineVkCreateInfo {
    pub fn new(engine_create_info: EngineCreateInfo) -> Self {
        EngineVkCreateInfo {
            engine_create_info,

            instance_layer_names: Vec::new(),
            instance_extension_names: Vec::new(),
            device_extension_names: Vec::new(),
            device_extension_features: std::ptr::null_mut(),
            vk_allocator: std::ptr::null_mut(),
            ignore_debug_message_names: Vec::new(),
            main_descriptor_pool_size: diligent_sys::VulkanDescriptorPoolSize {
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

            dynamic_descriptor_pool_size: diligent_sys::VulkanDescriptorPoolSize {
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

impl Default for EngineVkCreateInfo {
    fn default() -> Self {
        EngineVkCreateInfo::new(EngineCreateInfo::default())
    }
}

pub struct EngineFactoryVk {
    engine_factory_vk: *mut diligent_sys::IEngineFactoryVk,
    virtual_functions: *mut diligent_sys::IEngineFactoryVkVtbl,

    engine_factory: EngineFactory,
}

impl AsEngineFactory for EngineFactoryVk {
    #[inline]
    fn as_engine_factory(&self) -> &EngineFactory {
        &self.engine_factory
    }
}

pub fn get_engine_factory_vk() -> EngineFactoryVk {
    let engine_factory_vk = unsafe { diligent_sys::Diligent_GetEngineFactoryVk() };

    EngineFactoryVk {
        engine_factory_vk,
        virtual_functions: unsafe { (*engine_factory_vk).pVtbl },

        engine_factory: EngineFactory::new(engine_factory_vk as *mut diligent_sys::IEngineFactory),
    }
}

impl EngineFactoryVk {
    pub fn create_device_and_contexts(
        &self,
        create_info: &EngineVkCreateInfo,
    ) -> Option<(RenderDevice, Vec<DeviceContext>, Vec<DeviceContext>)> {
        let num_immediate_contexts = create_info
            .engine_create_info
            .immediate_context_info
            .len()
            .max(1);

        let num_deferred_contexts = create_info.engine_create_info.num_deferred_contexts as usize;

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptrs = Vec::from_iter(
            std::iter::repeat(std::ptr::null_mut())
                .take(num_immediate_contexts + num_deferred_contexts),
        );

        {
            fn vec_string_to_vec_cstring_ptr(
                strings: &Vec<String>,
            ) -> (Vec<std::ffi::CString>, Vec<*const i8>) {
                let cstrings = strings
                    .iter()
                    .map(|s| std::ffi::CString::new(s.as_str()).unwrap())
                    .collect::<Vec<std::ffi::CString>>();
                let ptrs = cstrings
                    .iter()
                    .map(|s| s.as_ptr())
                    .collect::<Vec<*const i8>>();
                (cstrings, ptrs)
            }

            let (_a, instance_layer_names) =
                vec_string_to_vec_cstring_ptr(&create_info.instance_layer_names);
            let (_a, instance_extension_names) =
                vec_string_to_vec_cstring_ptr(&create_info.instance_extension_names);
            let (_a, device_extension_names) =
                vec_string_to_vec_cstring_ptr(&create_info.device_extension_names);
            let (_a, ignore_debug_message_names) =
                vec_string_to_vec_cstring_ptr(&create_info.ignore_debug_message_names);

            let create_info = diligent_sys::EngineVkCreateInfo {
                _EngineCreateInfo: diligent_sys::EngineCreateInfo::from(
                    &create_info.engine_create_info,
                ),
                InstanceLayerCount: create_info.instance_layer_names.len() as u32,
                ppInstanceLayerNames: if instance_layer_names.is_empty() {
                    std::ptr::null()
                } else {
                    instance_layer_names.as_ptr()
                },
                InstanceExtensionCount: create_info.instance_extension_names.len() as u32,
                ppInstanceExtensionNames: if instance_extension_names.is_empty() {
                    std::ptr::null()
                } else {
                    instance_extension_names.as_ptr()
                },

                DeviceExtensionCount: create_info.device_extension_names.len() as u32,
                ppDeviceExtensionNames: if device_extension_names.is_empty() {
                    std::ptr::null()
                } else {
                    device_extension_names.as_ptr()
                },
                pDeviceExtensionFeatures: create_info.device_extension_features,

                pVkAllocator: create_info.vk_allocator,

                IgnoreDebugMessageCount: create_info.ignore_debug_message_names.len() as u32,
                ppIgnoreDebugMessageNames: if ignore_debug_message_names.is_empty() {
                    std::ptr::null()
                } else {
                    ignore_debug_message_names.as_ptr()
                },

                MainDescriptorPoolSize: create_info.main_descriptor_pool_size,

                DynamicDescriptorPoolSize: create_info.dynamic_descriptor_pool_size,

                DeviceLocalMemoryPageSize: create_info.device_local_memory_page_size,
                HostVisibleMemoryPageSize: create_info.host_visible_memory_page_size,
                DeviceLocalMemoryReserveSize: create_info.device_local_memory_reserve_size,
                HostVisibleMemoryReserveSize: create_info.host_visible_memory_reserve_size,
                UploadHeapPageSize: create_info.upload_heap_page_size,
                DynamicHeapSize: create_info.dynamic_heap_size,
                DynamicHeapPageSize: create_info.dynamic_heap_page_size,

                QueryPoolSizes: create_info.query_pool_sizes,

                pDxCompilerPath: if let Some(path) = &create_info.dx_compiler_path {
                    path.as_os_str().as_encoded_bytes().as_ptr() as *const i8
                } else {
                    std::ptr::null()
                },
            };
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

    pub fn create_swap_chain(
        &self,
        device: &RenderDevice,
        immediate_context: &DeviceContext,
        swapchain_desc: &SwapChainDesc,
        window: Option<&NativeWindow>,
    ) -> Option<SwapChain> {
        let swapchain_desc = diligent_sys::SwapChainDesc::from(swapchain_desc);
        let mut swap_chain_ptr = std::ptr::null_mut();

        let window = window.map(|window| diligent_sys::NativeWindow::from(window));

        unsafe {
            (*self.virtual_functions)
                .EngineFactoryVk
                .CreateSwapChainVk
                .unwrap_unchecked()(
                self.engine_factory_vk,
                device.render_device,
                immediate_context.device_context,
                std::ptr::addr_of!(swapchain_desc),
                window
                    .as_ref()
                    .map_or(std::ptr::null(), |window| std::ptr::from_ref(window)),
                std::ptr::addr_of_mut!(swap_chain_ptr),
            );
        }
        if swap_chain_ptr.is_null() {
            None
        } else {
            Some(SwapChain::new(swap_chain_ptr))
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
