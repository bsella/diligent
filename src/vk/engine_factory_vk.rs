use std::ffi::CString;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;

use static_assertions::const_assert;

use crate::{
    device_context::DeferredDeviceContext, device_context::ImmediateDeviceContext,
    engine_factory::EngineCreateInfo, engine_factory::EngineFactory,
    graphics_types::DeviceFeatureState, platforms::native_window::NativeWindow,
    render_device::RenderDevice, swap_chain::SwapChain, swap_chain::SwapChainDesc,
};

pub struct DeviceFeaturesVk {
    dynamic_rendering: DeviceFeatureState,
    host_image_copy: DeviceFeatureState,
}

impl DeviceFeaturesVk {
    pub fn new(dynamic_rendering: DeviceFeatureState, host_image_copy: DeviceFeatureState) -> Self {
        DeviceFeaturesVk {
            dynamic_rendering,
            host_image_copy,
        }
    }
}

pub struct EngineVkCreateInfo {
    engine_create_info: EngineCreateInfo,

    pub features_vk: DeviceFeaturesVk,

    pub instance_layer_names: Vec<CString>,
    pub instance_extension_names: Vec<CString>,
    pub device_extension_names: Vec<CString>,
    pub ignore_debug_message_names: Vec<CString>,
    // TODO
    //pub device_extension_features: *mut c_void,
    //pub vk_allocator: *mut c_void,
    pub main_descriptor_pool_size: diligent_sys::VulkanDescriptorPoolSize,
    pub dynamic_descriptor_pool_size: diligent_sys::VulkanDescriptorPoolSize,

    pub device_local_memory_page_size: u32,
    pub host_visible_memory_page_size: u32,
    pub device_local_memory_reserve_size: u32,
    pub host_visible_memory_reserve_size: u32,
    pub upload_heap_page_size: u32,
    pub dynamic_heap_size: u32,
    pub dynamic_heap_page_size: u32,

    pub query_pool_sizes: [u32; 6],
    pub dx_compiler_path: Option<PathBuf>,
}

impl Deref for EngineVkCreateInfo {
    type Target = EngineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.engine_create_info
    }
}

impl DerefMut for EngineVkCreateInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine_create_info
    }
}

impl EngineVkCreateInfo {
    pub fn new(engine_create_info: EngineCreateInfo) -> Self {
        EngineVkCreateInfo {
            engine_create_info,

            features_vk: DeviceFeaturesVk {
                dynamic_rendering: DeviceFeatureState::Optional,
                host_image_copy: DeviceFeatureState::Optional,
            },

            instance_layer_names: Vec::new(),
            instance_extension_names: Vec::new(),
            device_extension_names: Vec::new(),
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
    sys_ptr: *mut diligent_sys::IEngineFactoryVk,
    virtual_functions: *mut diligent_sys::IEngineFactoryVkVtbl,

    engine_factory: EngineFactory,
}

impl Deref for EngineFactoryVk {
    type Target = EngineFactory;
    fn deref(&self) -> &Self::Target {
        &self.engine_factory
    }
}

pub fn get_engine_factory_vk() -> EngineFactoryVk {
    let engine_factory_vk = unsafe { diligent_sys::Diligent_GetEngineFactoryVk() };

    // Both base and derived classes have exactly the same size.
    // This means that we can up-cast to the base class without worrying about layout offset between both classes
    const_assert!(
        std::mem::size_of::<diligent_sys::IEngineFactory>()
            == std::mem::size_of::<diligent_sys::IEngineFactoryVk>()
    );

    EngineFactoryVk {
        sys_ptr: engine_factory_vk,
        virtual_functions: unsafe { (*engine_factory_vk).pVtbl },

        engine_factory: EngineFactory::new(engine_factory_vk as *mut diligent_sys::IEngineFactory),
    }
}

impl EngineFactoryVk {
    pub fn create_device_and_contexts(
        &self,
        create_info: &EngineVkCreateInfo,
    ) -> Result<
        (
            RenderDevice,
            Vec<ImmediateDeviceContext>,
            Vec<DeferredDeviceContext>,
        ),
        (),
    > {
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
            let instance_layer_names = create_info
                .instance_layer_names
                .iter()
                .map(|s| s.as_ptr())
                .collect::<Vec<_>>();

            let instance_extension_names = create_info
                .instance_extension_names
                .iter()
                .map(|s| s.as_ptr())
                .collect::<Vec<_>>();

            let device_extension_names = create_info
                .device_extension_names
                .iter()
                .map(|s| s.as_ptr())
                .collect::<Vec<_>>();

            let ignore_debug_message_names = create_info
                .ignore_debug_message_names
                .iter()
                .map(|s| s.as_ptr())
                .collect::<Vec<_>>();

            let create_info = diligent_sys::EngineVkCreateInfo {
                _EngineCreateInfo: (&create_info.engine_create_info).into(),
                FeaturesVk: diligent_sys::DeviceFeaturesVk {
                    DynamicRendering: create_info.features_vk.dynamic_rendering.into(),
                    HostImageCopy: create_info.features_vk.host_image_copy.into(),
                },
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
                pDeviceExtensionFeatures: std::ptr::null_mut(),

                pVkAllocator: std::ptr::null_mut(),

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
                    self.sys_ptr,
                    std::ptr::addr_of!(create_info),
                    std::ptr::addr_of_mut!(render_device_ptr),
                    device_context_ptrs.as_mut_ptr(),
                )
            }
        }

        if render_device_ptr.is_null() {
            Err(())
        } else {
            Ok((
                RenderDevice::new(render_device_ptr),
                Vec::from_iter(
                    device_context_ptrs
                        .iter()
                        .take(num_immediate_contexts)
                        .map(|dc_ptr| ImmediateDeviceContext::new(*dc_ptr)),
                ),
                Vec::from_iter(
                    device_context_ptrs
                        .iter()
                        .rev()
                        .take(num_deferred_contexts)
                        .map(|dc_ptr| DeferredDeviceContext::new(*dc_ptr)),
                ),
            ))
        }
    }

    pub fn create_swap_chain(
        &self,
        device: &RenderDevice,
        immediate_context: &ImmediateDeviceContext,
        swapchain_desc: &SwapChainDesc,
        window: Option<&NativeWindow>,
    ) -> Result<SwapChain, ()> {
        let swapchain_desc = swapchain_desc.into();
        let mut swap_chain_ptr = std::ptr::null_mut();

        let window = window.map(|window| window.into());

        unsafe {
            (*self.virtual_functions)
                .EngineFactoryVk
                .CreateSwapChainVk
                .unwrap_unchecked()(
                self.sys_ptr,
                device.sys_ptr,
                immediate_context.sys_ptr,
                std::ptr::addr_of!(swapchain_desc),
                window
                    .as_ref()
                    .map_or(std::ptr::null(), |window| std::ptr::from_ref(window)),
                std::ptr::addr_of_mut!(swap_chain_ptr),
            );
        }
        if swap_chain_ptr.is_null() {
            Err(())
        } else {
            Ok(SwapChain::new(swap_chain_ptr))
        }
    }

    pub fn enable_device_simulation(&self) {
        unsafe {
            (*self.virtual_functions)
                .EngineFactoryVk
                .EnableDeviceSimulation
                .unwrap_unchecked()(self.sys_ptr);
        }
    }
}
