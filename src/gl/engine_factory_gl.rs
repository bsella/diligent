use std::ops::{Deref, DerefMut};

use crate::{
    Boxed,
    device_context::ImmediateDeviceContext,
    engine_factory::{EngineCreateInfo, EngineFactory},
    graphics_types::AdapterType,
    platforms::native_window::NativeWindow,
    render_device::RenderDevice,
    swap_chain::{SwapChain, SwapChainCreateInfo},
};

pub struct EngineGLCreateInfo {
    engine_create_info: EngineCreateInfo,

    window: NativeWindow,
    zero_to_one_ndz: bool,
    preferred_adapter_type: AdapterType,
}

impl Deref for EngineGLCreateInfo {
    type Target = EngineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.engine_create_info
    }
}

impl DerefMut for EngineGLCreateInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine_create_info
    }
}

impl EngineGLCreateInfo {
    pub fn new(window: NativeWindow, engine_create_info: EngineCreateInfo) -> Self {
        EngineGLCreateInfo {
            engine_create_info,
            preferred_adapter_type: AdapterType::Unknown,
            zero_to_one_ndz: false,
            window,
        }
    }
}

impl From<&EngineGLCreateInfo> for diligent_sys::EngineGLCreateInfo {
    fn from(value: &EngineGLCreateInfo) -> Self {
        diligent_sys::EngineGLCreateInfo {
            _EngineCreateInfo: (&value.engine_create_info).into(),
            PreferredAdapterType: value.preferred_adapter_type.into(),
            ZeroToOneNDZ: value.zero_to_one_ndz,
            Window: (&value.window).into(),
        }
    }
}

#[repr(transparent)]
pub struct EngineFactoryOpenGL(diligent_sys::IEngineFactoryOpenGL);

impl Deref for EngineFactoryOpenGL {
    type Target = EngineFactory;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IEngineFactory
                as *const EngineFactory)
        }
    }
}

impl EngineFactoryOpenGL {
    pub fn create_device_and_swap_chain_gl(
        &self,
        engine_ci: &EngineGLCreateInfo,
        sc_desc: &SwapChainCreateInfo,
    ) -> Result<
        (
            Boxed<RenderDevice>,
            Boxed<ImmediateDeviceContext>,
            Boxed<SwapChain>,
        ),
        (),
    > {
        let engine_ci = engine_ci.into();

        let mut render_device_ptr: *mut diligent_sys::IRenderDevice = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();
        let mut swap_chain_ptr = std::ptr::null_mut();

        let swap_chain_desc = sc_desc.into();

        unsafe_member_call!(
            self,
            EngineFactoryOpenGL,
            CreateDeviceAndSwapChainGL,
            std::ptr::from_ref(&engine_ci),
            std::ptr::addr_of_mut!(render_device_ptr),
            std::ptr::addr_of_mut!(device_context_ptr),
            std::ptr::from_ref(&swap_chain_desc),
            std::ptr::addr_of_mut!(swap_chain_ptr)
        );

        if render_device_ptr.is_null() {
            Err(())
        } else {
            Ok((
                Boxed::<RenderDevice>::new(render_device_ptr as _),
                Boxed::<ImmediateDeviceContext>::new(device_context_ptr as _),
                Boxed::<SwapChain>::new(swap_chain_ptr as _),
            ))
        }
    }

    //TODO pub fn create_hlsl2glsl_converter(&self) -> Result<HLSL2GLSLConverter, ()>{}

    pub fn attach_to_active_gl_context(
        &self,
        engine_ci: &EngineGLCreateInfo,
    ) -> Result<(Boxed<RenderDevice>, Boxed<ImmediateDeviceContext>), ()> {
        let engine_ci = engine_ci.into();

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            EngineFactoryOpenGL,
            AttachToActiveGLContext,
            std::ptr::from_ref(&engine_ci),
            std::ptr::addr_of_mut!(render_device_ptr),
            std::ptr::addr_of_mut!(device_context_ptr)
        );

        if render_device_ptr.is_null() {
            Err(())
        } else {
            Ok((
                Boxed::<RenderDevice>::new(render_device_ptr as _),
                Boxed::<ImmediateDeviceContext>::new(device_context_ptr as _),
            ))
        }
    }
}

pub fn get_engine_factory_gl() -> Boxed<EngineFactoryOpenGL> {
    let engine_factory_gl = unsafe { diligent_sys::Diligent_GetEngineFactoryOpenGL() };

    Boxed::new(engine_factory_gl as _)
}
