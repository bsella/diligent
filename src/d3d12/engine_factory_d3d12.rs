use std::{
    ffi::CString,
    ops::{Deref, DerefMut},
    path::Path,
};

use bitflags::bitflags;

use crate::{
    Boxed, BoxedFromNulError,
    device_context::{DeferredDeviceContext, ImmediateDeviceContext},
    engine_factory::{EngineCreateInfo, EngineFactory},
    graphics_types::{DisplayModeAttribs, FullScreenModeDesc, TextureFormat, Version},
    platforms::native_window::NativeWindow,
    render_device::RenderDevice,
    swap_chain::{SwapChain, SwapChainCreateInfo},
};

define_ported!(
    EngineFactoryD3D12,
    diligent_sys::IEngineFactoryD3D12,
    diligent_sys::IEngineFactoryD3D12Methods : 6,
    EngineFactory
);

pub fn get_engine_factory_d3d12() -> Boxed<EngineFactoryD3D12> {
    Boxed::new(unsafe { diligent_sys::Diligent_GetEngineFactoryD3D12() }).unwrap()
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct D3D12ValidationFlags: diligent_sys::D3D12_VALIDATION_FLAGS {
        const None                     = diligent_sys::D3D12_VALIDATION_FLAG_NONE as diligent_sys::D3D12_VALIDATION_FLAGS;
        const BreakOnError             = diligent_sys::D3D12_VALIDATION_FLAG_BREAK_ON_ERROR as diligent_sys::D3D12_VALIDATION_FLAGS;
        const BreakOnCorruption        = diligent_sys::D3D12_VALIDATION_FLAG_BREAK_ON_CORRUPTION as diligent_sys::D3D12_VALIDATION_FLAGS;
        const EnableGpuBasedValidation = diligent_sys::D3D12_VALIDATION_FLAG_ENABLE_GPU_BASED_VALIDATION as diligent_sys::D3D12_VALIDATION_FLAGS;
    }
}

pub struct EngineD3D12CreateInfo {
    engine_create_info: EngineCreateInfo,

    d3d12_dll_name: CString,
    d3d12_validation_flags: D3D12ValidationFlags,
    cpu_descriptor_heap_allocation_size: [u32; 4usize],
    gpu_descriptor_heap_size: [u32; 2usize],
    gpu_descriptor_heap_dynamic_size: [u32; 2usize],
    dynamic_descriptor_allocation_chunk_size: [u32; 2usize],
    dynamic_heap_page_size: u32,
    num_dynamic_heap_pages_to_reserve: u32,
    query_pool_sizes: [u32; 6usize],
    p_dx_compiler_path: CString,
}

impl Deref for EngineD3D12CreateInfo {
    type Target = EngineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.engine_create_info
    }
}

impl DerefMut for EngineD3D12CreateInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine_create_info
    }
}

impl EngineD3D12CreateInfo {
    pub fn new(engine_create_info: EngineCreateInfo) -> Self {
        EngineD3D12CreateInfo {
            engine_create_info,
            cpu_descriptor_heap_allocation_size: [
                8192, // D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV
                2048, // D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER
                1024, // D3D12_DESCRIPTOR_HEAP_TYPE_RTV
                1024, // D3D12_DESCRIPTOR_HEAP_TYPE_DSV
            ],
            d3d12_dll_name: CString::new("d3d12.dll").unwrap(),
            d3d12_validation_flags: D3D12ValidationFlags::BreakOnCorruption,
            gpu_descriptor_heap_size: [
                16384, // D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV
                1024,  // D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER
            ],
            gpu_descriptor_heap_dynamic_size: [
                8192, // D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV
                1024, // D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER
            ],
            dynamic_descriptor_allocation_chunk_size: [
                256, // D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV
                32,  // D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER
            ],
            dynamic_heap_page_size: 1 << 20,
            num_dynamic_heap_pages_to_reserve: 1,
            query_pool_sizes: [
                0,   // Ignored
                128, // QUERY_TYPE_OCCLUSION
                128, // QUERY_TYPE_BINARY_OCCLUSION
                512, // QUERY_TYPE_TIMESTAMP
                128, // QUERY_TYPE_PIPELINE_STATISTICS
                256, // QUERY_TYPE_DURATION
            ],
            p_dx_compiler_path: CString::default(),
        }
    }
}

impl From<&EngineD3D12CreateInfo> for diligent_sys::EngineD3D12CreateInfo {
    fn from(value: &EngineD3D12CreateInfo) -> Self {
        diligent_sys::EngineD3D12CreateInfo {
            _EngineCreateInfo: (&value.engine_create_info).into(),
            D3D12ValidationFlags: value.d3d12_validation_flags.bits(),
            CPUDescriptorHeapAllocationSize: value.cpu_descriptor_heap_allocation_size,
            D3D12DllName: value.d3d12_dll_name.as_ptr(),
            DynamicDescriptorAllocationChunkSize: value.dynamic_descriptor_allocation_chunk_size,
            DynamicHeapPageSize: value.dynamic_heap_page_size,
            GPUDescriptorHeapDynamicSize: value.gpu_descriptor_heap_dynamic_size,
            GPUDescriptorHeapSize: value.gpu_descriptor_heap_size,
            NumDynamicHeapPagesToReserve: value.num_dynamic_heap_pages_to_reserve,
            QueryPoolSizes: value.query_pool_sizes,
            pDxCompilerPath: value.p_dx_compiler_path.as_ptr(),
        }
    }
}

impl EngineFactoryD3D12 {
    pub fn load_d3d12(&self) -> bool {
        unsafe_member_call!(self, EngineFactoryD3D12, LoadD3D12, c"d3d12.dll".as_ptr())
    }

    pub fn load_d3d12_from_path(&self, path: impl AsRef<Path>) -> bool {
        let path_str = CString::new(path.as_ref().to_string_lossy().as_bytes()).unwrap();
        unsafe_member_call!(self, EngineFactoryD3D12, LoadD3D12, path_str.as_ptr())
    }

    pub fn create_device_and_contexts(
        &self,
        engine_ci: &EngineD3D12CreateInfo,
    ) -> Result<
        (
            Boxed<RenderDevice>,
            Vec<Boxed<ImmediateDeviceContext>>,
            Vec<Boxed<DeferredDeviceContext>>,
        ),
        BoxedFromNulError,
    > {
        let num_immediate_contexts = engine_ci
            .engine_create_info
            .immediate_context_info
            .len()
            .max(1);

        let num_deferred_contexts = engine_ci.engine_create_info.num_deferred_contexts;

        let engine_ci = engine_ci.into();

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptrs: Vec<_> = std::iter::repeat_n(
            std::ptr::null_mut(),
            num_immediate_contexts + num_deferred_contexts,
        )
        .collect();

        unsafe_member_call!(
            self,
            EngineFactoryD3D12,
            CreateDeviceAndContextsD3D12,
            &engine_ci,
            &mut render_device_ptr,
            device_context_ptrs.as_mut_ptr()
        );

        Boxed::new(render_device_ptr).and_then(|render_device| {
            device_context_ptrs
                .iter()
                .take(num_immediate_contexts)
                .map(|&dc_ptr| Boxed::new(dc_ptr))
                .collect::<Result<Vec<_>, _>>()
                .and_then(|immediate_devices| {
                    device_context_ptrs
                        .iter()
                        .rev()
                        .take(num_deferred_contexts)
                        .map(|&dc_ptr| Boxed::new(dc_ptr))
                        .collect::<Result<Vec<_>, _>>()
                        .map(|deferred_devices| {
                            (render_device, immediate_devices, deferred_devices)
                        })
                })
        })
    }

    // TODO
    //pub fn create_command_queue_d3d12(&self,
    //    pd3d12NativeDevice : *mut c_void,
    //    pd3d12NativeCommandQueue : *mut c_void,
    //    struct IMemoryAllocator*    pRawMemAllocator) -> Result<CommandQueueD3D12, ()>
    //    {
    //
    //    }

    // TODO
    //pub fn attach_to_d3d12_device(
    //    &self,
    //    native_device: *mut c_void,
    //    command_queues: impl AsRef<[CommandQueueD3D12]>,
    //    engine_ci: &EngineD3D12CreateInfo,
    //) -> Result<(
    //    RenderDevice,
    //    Vec<ImmediateDeviceContext>,
    //    Vec<DeferredDeviceContext>,
    //),()> {
    //
    //}

    pub fn create_swap_chain(
        &self,
        device: &RenderDevice,
        context: &ImmediateDeviceContext,
        swapchain_desc: &SwapChainCreateInfo,
        fs_desc: &FullScreenModeDesc,
        window: &NativeWindow,
    ) -> Result<Boxed<SwapChain>, BoxedFromNulError> {
        let mut swap_chain_ptr = std::ptr::null_mut();

        let fs_desc = fs_desc.into();
        unsafe_member_call!(
            self,
            EngineFactoryD3D12,
            CreateSwapChainD3D12,
            device.sys_ptr(),
            context.sys_ptr(),
            &swapchain_desc.0.0,
            &fs_desc,
            &window.0,
            &mut swap_chain_ptr
        );

        Boxed::new(swap_chain_ptr)
    }

    pub fn enumerate_display_modes(
        &self,
        min_feature_level: Version,
        adapter_id: u32,
        output_id: u32,
        format: TextureFormat,
    ) -> Vec<DisplayModeAttribs> {
        let mut num_display_modes = 0;

        unsafe_member_call!(
            self,
            EngineFactoryD3D12,
            EnumerateDisplayModes,
            diligent_sys::Version {
                Major: min_feature_level.major,
                Minor: min_feature_level.minor,
            },
            adapter_id,
            output_id,
            format.into(),
            &mut num_display_modes,
            std::ptr::null_mut()
        );

        let mut display_modes: Vec<DisplayModeAttribs> =
            Vec::with_capacity(num_display_modes as usize);

        unsafe_member_call!(
            self,
            EngineFactoryD3D12,
            EnumerateDisplayModes,
            diligent_sys::Version {
                Major: min_feature_level.major,
                Minor: min_feature_level.minor,
            },
            adapter_id,
            output_id,
            format.into(),
            std::ptr::from_mut(&mut num_display_modes),
            display_modes.as_mut_ptr() as *mut diligent_sys::DisplayModeAttribs
        );

        unsafe {
            display_modes.set_len(num_display_modes as usize);
        }

        display_modes
    }
}
