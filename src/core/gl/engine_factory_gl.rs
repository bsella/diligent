use crate::{
    core::{
        device_context::ImmediateDeviceContext,
        engine_factory::{AsEngineFactory, EngineCreateInfo, EngineFactory},
        graphics_types::AdapterType,
        render_device::RenderDevice,
        swap_chain::{SwapChain, SwapChainDesc},
    },
    tools::native_app::NativeWindow,
};

pub struct EngineGLCreateInfo<'a> {
    engine_create_info: EngineCreateInfo,

    window: &'a NativeWindow,
    zero_to_one_ndz: bool,
    preferred_adapter_type: AdapterType,
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
            _EngineCreateInfo: diligent_sys::EngineCreateInfo::from(&value.engine_create_info),
            PreferredAdapterType: diligent_sys::ADAPTER_TYPE::from(&value.preferred_adapter_type),
            ZeroToOneNDZ: value.zero_to_one_ndz,
            Window: diligent_sys::NativeWindow::from(value.window),
        }
    }
}

pub struct EngineFactoryOpenGL {
    engine_factory_gl: *mut diligent_sys::IEngineFactoryOpenGL,
    virtual_functions: *mut diligent_sys::IEngineFactoryOpenGLVtbl,

    engine_factory: EngineFactory,
}

impl AsEngineFactory for EngineFactoryOpenGL {
    #[inline]
    fn as_engine_factory(&self) -> &EngineFactory {
        &self.engine_factory
    }
}

impl EngineFactoryOpenGL {
    pub fn create_device_and_swap_chain_gl(
        &self,
        engine_ci: &EngineGLCreateInfo,
        sc_desc: &SwapChainDesc,
    ) -> Option<(RenderDevice, ImmediateDeviceContext, SwapChain)> {
        let engine_ci = diligent_sys::EngineGLCreateInfo::from(engine_ci);

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();
        let mut swap_chain_ptr = std::ptr::null_mut();

        let swap_chain_desc = diligent_sys::SwapChainDesc::from(sc_desc);

        unsafe {
            (*self.virtual_functions)
                .EngineFactoryOpenGL
                .CreateDeviceAndSwapChainGL
                .unwrap_unchecked()(
                self.engine_factory_gl,
                std::ptr::from_ref(&engine_ci),
                std::ptr::addr_of_mut!(render_device_ptr),
                std::ptr::addr_of_mut!(device_context_ptr),
                std::ptr::from_ref(&swap_chain_desc),
                std::ptr::addr_of_mut!(swap_chain_ptr),
            );
        }

        if render_device_ptr.is_null() {
            None
        } else {
            Some((
                RenderDevice::new(render_device_ptr),
                ImmediateDeviceContext::new(device_context_ptr),
                SwapChain::new(swap_chain_ptr),
            ))
        }
    }

    //pub fn create_hlsl2glsl_converter(&self) -> Option<HLSL2GLSLConverter>{}

    pub fn attach_to_active_gl_context(
        &self,
        engine_ci: &EngineGLCreateInfo,
    ) -> Option<(RenderDevice, ImmediateDeviceContext)> {
        let engine_ci = diligent_sys::EngineGLCreateInfo::from(engine_ci);

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();

        unsafe {
            (*self.virtual_functions)
                .EngineFactoryOpenGL
                .AttachToActiveGLContext
                .unwrap_unchecked()(
                self.engine_factory_gl,
                std::ptr::from_ref(&engine_ci),
                std::ptr::addr_of_mut!(render_device_ptr),
                std::ptr::addr_of_mut!(device_context_ptr),
            );
        }

        if render_device_ptr.is_null() {
            None
        } else {
            Some((
                RenderDevice::new(render_device_ptr),
                ImmediateDeviceContext::new(device_context_ptr),
            ))
        }
    }
}

pub fn get_engine_factory_gl() -> EngineFactoryOpenGL {
    let engine_factory_gl = unsafe { diligent_sys::Diligent_GetEngineFactoryOpenGL() };

    EngineFactoryOpenGL {
        engine_factory_gl,
        virtual_functions: unsafe { (*engine_factory_gl).pVtbl },

        engine_factory: EngineFactory::new(engine_factory_gl as _),
    }
}
