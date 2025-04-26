use std::ops::{Deref, DerefMut};

use static_assertions::const_assert;

use crate::{
    device_context::ImmediateDeviceContext,
    engine_factory::{EngineCreateInfo, EngineFactory},
    graphics_types::AdapterType,
    platforms::native_window::NativeWindow,
    render_device::RenderDevice,
    swap_chain::{SwapChain, SwapChainDesc},
};

pub struct EngineGLCreateInfo<'a> {
    engine_create_info: EngineCreateInfo,

    window: &'a NativeWindow,
    zero_to_one_ndz: bool,
    preferred_adapter_type: AdapterType,
}

impl Deref for EngineGLCreateInfo<'_> {
    type Target = EngineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.engine_create_info
    }
}

impl DerefMut for EngineGLCreateInfo<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine_create_info
    }
}

impl<'a> EngineGLCreateInfo<'a> {
    pub fn new(window: &'a NativeWindow, engine_create_info: EngineCreateInfo) -> Self {
        EngineGLCreateInfo {
            engine_create_info,
            preferred_adapter_type: AdapterType::Unknown,
            zero_to_one_ndz: false,
            window,
        }
    }
}

impl From<&EngineGLCreateInfo<'_>> for diligent_sys::EngineGLCreateInfo {
    fn from(value: &EngineGLCreateInfo) -> Self {
        diligent_sys::EngineGLCreateInfo {
            _EngineCreateInfo: (&value.engine_create_info).into(),
            PreferredAdapterType: (&value.preferred_adapter_type).into(),
            ZeroToOneNDZ: value.zero_to_one_ndz,
            Window: value.window.into(),
        }
    }
}

pub struct EngineFactoryOpenGL {
    sys_ptr: *mut diligent_sys::IEngineFactoryOpenGL,
    virtual_functions: *mut diligent_sys::IEngineFactoryOpenGLVtbl,

    engine_factory: EngineFactory,
}

impl Deref for EngineFactoryOpenGL {
    type Target = EngineFactory;
    fn deref(&self) -> &Self::Target {
        &self.engine_factory
    }
}

impl EngineFactoryOpenGL {
    pub fn create_device_and_swap_chain_gl(
        &self,
        engine_ci: &EngineGLCreateInfo,
        sc_desc: &SwapChainDesc,
    ) -> Result<(RenderDevice, ImmediateDeviceContext, SwapChain), ()> {
        let engine_ci = engine_ci.into();

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();
        let mut swap_chain_ptr = std::ptr::null_mut();

        let swap_chain_desc = sc_desc.into();

        unsafe {
            (*self.virtual_functions)
                .EngineFactoryOpenGL
                .CreateDeviceAndSwapChainGL
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&engine_ci),
                std::ptr::addr_of_mut!(render_device_ptr),
                std::ptr::addr_of_mut!(device_context_ptr),
                std::ptr::from_ref(&swap_chain_desc),
                std::ptr::addr_of_mut!(swap_chain_ptr),
            );
        }

        if render_device_ptr.is_null() {
            Err(())
        } else {
            Ok((
                RenderDevice::new(render_device_ptr),
                ImmediateDeviceContext::new(device_context_ptr),
                SwapChain::new(swap_chain_ptr),
            ))
        }
    }

    //pub fn create_hlsl2glsl_converter(&self) -> Result<HLSL2GLSLConverter, ()>{}

    pub fn attach_to_active_gl_context(
        &self,
        engine_ci: &EngineGLCreateInfo,
    ) -> Result<(RenderDevice, ImmediateDeviceContext), ()> {
        let engine_ci = engine_ci.into();

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();

        unsafe {
            (*self.virtual_functions)
                .EngineFactoryOpenGL
                .AttachToActiveGLContext
                .unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&engine_ci),
                std::ptr::addr_of_mut!(render_device_ptr),
                std::ptr::addr_of_mut!(device_context_ptr),
            );
        }

        if render_device_ptr.is_null() {
            Err(())
        } else {
            Ok((
                RenderDevice::new(render_device_ptr),
                ImmediateDeviceContext::new(device_context_ptr),
            ))
        }
    }
}

pub fn get_engine_factory_gl() -> EngineFactoryOpenGL {
    let engine_factory_gl = unsafe { diligent_sys::Diligent_GetEngineFactoryOpenGL() };

    // Both base and derived classes have exactly the same size.
    // This means that we can up-cast to the base class without worrying about layout offset between both classes
    const_assert!(
        std::mem::size_of::<diligent_sys::IEngineFactory>()
            == std::mem::size_of::<diligent_sys::IEngineFactoryOpenGL>()
    );

    EngineFactoryOpenGL {
        sys_ptr: engine_factory_gl,
        virtual_functions: unsafe { (*engine_factory_gl).pVtbl },

        engine_factory: EngineFactory::new(engine_factory_gl as _),
    }
}
