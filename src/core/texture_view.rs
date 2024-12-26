use crate::core::bindings;

use crate::core::sampler::Sampler;
use crate::core::texture::Texture;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct TextureView {
    pub(crate) m_texture_view: *mut bindings::ITextureView,
    m_virtual_functions: *mut bindings::ITextureViewVtbl,
    pub(crate) m_texture: *const Texture,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for TextureView {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl TextureView {
    pub(crate) fn new(texture_view: *mut bindings::ITextureView, texture: *const Texture) -> Self {
        TextureView {
            m_virtual_functions: unsafe { (*texture_view).pVtbl },
            m_texture_view: texture_view,
            m_texture: texture,
            m_device_object: DeviceObject::new(texture_view as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> bindings::TextureViewDesc {
        unsafe {
            *((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.m_texture_view as *mut bindings::IDeviceObject
            ) as *const bindings::TextureViewDesc)
        }
    }

    fn set_sampler(&mut self, sampler: &Sampler) {
        unsafe {
            (*self.m_virtual_functions)
                .TextureView
                .SetSampler
                .unwrap_unchecked()(self.m_texture_view, sampler.m_sampler);
        }
    }

    //fn get_sampler(&self) -> Option<&Sampler> {
    //    // TODO
    //}

    #[inline]
    fn get_texture(&self) -> &Texture {
        unsafe { self.m_texture.as_ref().unwrap_unchecked() }
    }
}
