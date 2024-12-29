use crate::bindings;
use crate::core::engine_factory::EngineFactory;

use crate::core::engine_factory::AsEngineFactory;

pub struct EngineFactoryVk {
    m_engine_factory_vk: *mut bindings::IEngineFactoryVk,
    m_virtual_functions: *mut bindings::IEngineFactoryVkVtbl,

    m_engine_factory: EngineFactory,
}

impl EngineFactoryVk {
    //fn create_device_and_contexts_vk(
    //    &mut self,
    //    create_info: &bindings::EngineVkCreateInfo,
    //) -> Option<(IRenderDevice, Vec<IDeviceContext>)>
    //{}

    //fn create_swap_chain_vk(
    //    &mut self,
    //    device: &IRenderDevice,
    //    immediate_context: &IDeviceContext,
    //    swapchain_desc: &bindings::SwapChainDesc,
    //    window: &bindings::NativeWindow,
    //) -> Option<ISwapChain>;

    fn enable_device_simulation(&mut self) {
        unsafe {
            (*self.m_virtual_functions)
                .EngineFactoryVk
                .EnableDeviceSimulation
                .unwrap_unchecked()(self.m_engine_factory_vk);
        }
    }
}

impl AsEngineFactory for EngineFactoryVk {
    #[inline]
    fn as_engine_factory(&self) -> &EngineFactory {
        &self.m_engine_factory
    }
}

pub fn get_engine_factory_vk() -> EngineFactoryVk {
    unsafe {
        let engine_factory_vk = bindings::Diligent_GetEngineFactoryVk();
        let engine_factory = engine_factory_vk as *mut bindings::IEngineFactory;

        EngineFactoryVk {
            m_engine_factory: EngineFactory {
                m_virtual_functions: (*engine_factory).pVtbl,
                m_engine_factory: engine_factory,
            },
            m_virtual_functions: (*engine_factory_vk).pVtbl,
            m_engine_factory_vk: engine_factory_vk,
        }
    }
}
