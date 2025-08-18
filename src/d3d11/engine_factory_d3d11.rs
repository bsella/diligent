use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use bitflags::bitflags;
use static_assertions::const_assert_eq;

use crate::{
    device_context::{DeferredDeviceContext, ImmediateDeviceContext},
    engine_factory::{EngineCreateInfo, EngineFactory},
    graphics_types::{DisplayModeAttribs, FullScreenModeDesc, TextureFormat, Version},
    platforms::native_window::NativeWindow,
    render_device::RenderDevice,
    swap_chain::{SwapChain, SwapChainDesc},
};

pub struct EngineFactoryD3D11 {
    engine_factory: EngineFactory,
}

impl Deref for EngineFactoryD3D11 {
    type Target = EngineFactory;
    fn deref(&self) -> &Self::Target {
        &self.engine_factory
    }
}

pub fn get_engine_factory_d3d11() -> EngineFactoryD3D11 {
    let engine_factory_d3d11 = unsafe { diligent_sys::Diligent_GetEngineFactoryD3D11() };

    // Both base and derived classes have exactly the same size.
    // This means that we can up-cast to the base class without worrying about layout offset between both classes
    const_assert_eq!(
        std::mem::size_of::<diligent_sys::IEngineFactory>(),
        std::mem::size_of::<diligent_sys::IEngineFactoryD3D11>()
    );

    EngineFactoryD3D11 {
        engine_factory: EngineFactory::new(
            engine_factory_d3d11 as *mut diligent_sys::IEngineFactory,
        ),
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct D3D11ValidationFlags: diligent_sys::D3D11_VALIDATION_FLAGS {
        const None                             = diligent_sys::D3D11_VALIDATION_FLAG_NONE as diligent_sys::D3D11_VALIDATION_FLAGS;
        const VerifyCommittedResourceRelevance = diligent_sys::D3D11_VALIDATION_FLAG_VERIFY_COMMITTED_RESOURCE_RELEVANCE as diligent_sys::D3D11_VALIDATION_FLAGS;
    }
}

pub struct EngineD3D11CreateInfo {
    engine_create_info: EngineCreateInfo,

    d3d11_validation_flags: D3D11ValidationFlags,
}

impl Deref for EngineD3D11CreateInfo {
    type Target = EngineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.engine_create_info
    }
}

impl DerefMut for EngineD3D11CreateInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine_create_info
    }
}

impl EngineD3D11CreateInfo {
    pub fn new(
        d3d11_validation_flags: D3D11ValidationFlags,
        engine_create_info: EngineCreateInfo,
    ) -> Self {
        EngineD3D11CreateInfo {
            engine_create_info,
            d3d11_validation_flags,
        }
    }
}

impl From<&EngineD3D11CreateInfo> for diligent_sys::EngineD3D11CreateInfo {
    fn from(value: &EngineD3D11CreateInfo) -> Self {
        diligent_sys::EngineD3D11CreateInfo {
            _EngineCreateInfo: (&value.engine_create_info).into(),
            D3D11ValidationFlags: value.d3d11_validation_flags.bits(),
        }
    }
}

impl EngineFactoryD3D11 {
    pub fn create_device_and_contexts(
        &self,
        engine_ci: &EngineD3D11CreateInfo,
    ) -> Result<
        (
            RenderDevice,
            Vec<ImmediateDeviceContext>,
            Vec<DeferredDeviceContext>,
        ),
        (),
    > {
        let num_immediate_contexts = engine_ci
            .engine_create_info
            .immediate_context_info
            .len()
            .max(1);

        let num_deferred_contexts = engine_ci.engine_create_info.num_deferred_contexts as usize;

        let engine_ci = engine_ci.into();

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptrs = Vec::from_iter(
            std::iter::repeat(std::ptr::null_mut())
                .take(num_immediate_contexts + num_deferred_contexts),
        );

        unsafe_member_call!(
            self,
            EngineFactoryD3D11,
            CreateDeviceAndContextsD3D11,
            std::ptr::from_ref(&engine_ci),
            std::ptr::addr_of_mut!(render_device_ptr),
            device_context_ptrs.as_mut_ptr()
        );

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
        context: &ImmediateDeviceContext,
        swapchain_desc: &SwapChainCreateInfo,
        fs_desc: &FullScreenModeDesc,
        window: Option<&NativeWindow>,
    ) -> Result<SwapChain, ()> {
        let swapchain_desc = swapchain_desc.into();
        let window = window.map(|window| window.into());
        let mut swap_chain_ptr = std::ptr::null_mut();

        let fs_desc = fs_desc.into();
        unsafe_member_call!(
            self,
            EngineFactoryD3D11,
            CreateSwapChainD3D11,
            device.sys_ptr,
            context.sys_ptr,
            std::ptr::from_ref(&swapchain_desc),
            std::ptr::from_ref(&fs_desc),
            window
                .as_ref()
                .map_or(std::ptr::null(), |window| std::ptr::from_ref(window)),
            std::ptr::addr_of_mut!(swap_chain_ptr)
        );

        if swap_chain_ptr.is_null() {
            Err(())
        } else {
            Ok(SwapChain::new(swap_chain_ptr))
        }
    }

    pub fn attach_to_d3d11_device(
        &self,
        native_device: *mut c_void,
        immediate_context: *mut c_void,
        engine_ci: &EngineD3D11CreateInfo,
    ) -> Result<
        (
            RenderDevice,
            Vec<ImmediateDeviceContext>,
            Vec<DeferredDeviceContext>,
        ),
        (),
    > {
        let num_immediate_contexts = engine_ci
            .engine_create_info
            .immediate_context_info
            .len()
            .max(1);

        let num_deferred_contexts = engine_ci.engine_create_info.num_deferred_contexts as usize;

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptrs = Vec::from_iter(
            std::iter::repeat(std::ptr::null_mut())
                .take(num_immediate_contexts + num_deferred_contexts),
        );

        let engine_ci = engine_ci.into();
        unsafe_member_call!(
            self,
            EngineFactoryD3D11,
            AttachToD3D11Device,
            native_device,
            immediate_context,
            std::ptr::from_ref(&engine_ci),
            std::ptr::addr_of_mut!(render_device_ptr),
            device_context_ptrs.as_mut_ptr()
        );

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
            EngineFactoryD3D11,
            EnumerateDisplayModes,
            diligent_sys::Version {
                Major: min_feature_level.major,
                Minor: min_feature_level.minor,
            },
            adapter_id,
            output_id,
            format.into(),
            std::ptr::from_mut(&mut num_display_modes),
            std::ptr::null_mut()
        );

        let mut display_modes = Vec::with_capacity(num_display_modes as usize);

        unsafe_member_call!(
            self,
            EngineFactoryD3D11,
            EnumerateDisplayModes,
            diligent_sys::Version {
                Major: min_feature_level.major,
                Minor: min_feature_level.minor,
            },
            adapter_id,
            output_id,
            format.into(),
            std::ptr::from_mut(&mut num_display_modes),
            display_modes.as_mut_ptr()
        );

        unsafe {
            display_modes.set_len(num_display_modes as usize);
        }

        Vec::from_iter(display_modes.iter().map(|display_mode| display_mode.into()))
    }
}
