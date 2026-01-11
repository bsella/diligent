use std::ops::{Deref, DerefMut};

use crate::{
    Boxed, BoxedFromNulError,
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
            Window: value.window.0,
        }
    }
}

define_ported!(
    EngineFactoryOpenGL,
    diligent_sys::IEngineFactoryOpenGL,
    diligent_sys::IEngineFactoryOpenGLMethods : 3,
    EngineFactory
);

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
        BoxedFromNulError,
    > {
        let engine_ci = engine_ci.into();

        let mut render_device_ptr: *mut diligent_sys::IRenderDevice = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();
        let mut swap_chain_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            EngineFactoryOpenGL,
            CreateDeviceAndSwapChainGL,
            &engine_ci,
            &mut render_device_ptr,
            &mut device_context_ptr,
            &sc_desc.0.0,
            &mut swap_chain_ptr
        );

        Boxed::new(render_device_ptr).and_then(|render_device| {
            Boxed::new(device_context_ptr).and_then(|device_context| {
                Boxed::new(swap_chain_ptr)
                    .map(|swap_chain| (render_device, device_context, swap_chain))
            })
        })
    }

    //TODO pub fn create_hlsl2glsl_converter(&self) -> Result<HLSL2GLSLConverter, ()>{}

    pub fn attach_to_active_gl_context(
        &self,
        engine_ci: &EngineGLCreateInfo,
    ) -> Result<(Boxed<RenderDevice>, Boxed<ImmediateDeviceContext>), BoxedFromNulError> {
        let engine_ci = engine_ci.into();

        let mut render_device_ptr = std::ptr::null_mut();
        let mut device_context_ptr = std::ptr::null_mut();

        unsafe_member_call!(
            self,
            EngineFactoryOpenGL,
            AttachToActiveGLContext,
            &engine_ci,
            &mut render_device_ptr,
            &mut device_context_ptr
        );

        Boxed::new(render_device_ptr).and_then(|render_device| {
            Boxed::new(device_context_ptr).map(|device_context| (render_device, device_context))
        })
    }
}

pub fn get_engine_factory_gl() -> Boxed<EngineFactoryOpenGL> {
    Boxed::new(unsafe { diligent_sys::Diligent_GetEngineFactoryOpenGL() }).unwrap()
}
