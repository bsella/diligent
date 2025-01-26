use crate::bindings;

use super::sampler::Sampler;
use super::texture::Texture;

use super::device_object::{AsDeviceObject, DeviceObject};

pub struct TextureView {
    pub(crate) texture_view: *mut bindings::ITextureView,
    virtual_functions: *mut bindings::ITextureViewVtbl,
    texture: *const Texture,

    pub(crate) device_object: DeviceObject,
}

impl AsDeviceObject for TextureView {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl TextureView {
    pub(crate) fn new(texture_view: *mut bindings::ITextureView, texture: *const Texture) -> Self {
        TextureView {
            virtual_functions: unsafe { (*texture_view).pVtbl },
            texture_view: texture_view,
            texture: texture,
            device_object: DeviceObject::new(texture_view as *mut bindings::IDeviceObject),
        }
    }

    fn get_desc(&self) -> bindings::TextureViewDesc {
        unsafe {
            *((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.texture_view as *mut bindings::IDeviceObject
            ) as *const bindings::TextureViewDesc)
        }
    }

    fn set_sampler(&mut self, sampler: &Sampler) {
        unsafe {
            (*self.virtual_functions)
                .TextureView
                .SetSampler
                .unwrap_unchecked()(self.texture_view, sampler.sampler);
        }
    }

    fn get_sampler(&self) -> Option<&Sampler> {
        todo!()
    }

    #[inline]
    fn get_texture(&self) -> &Texture {
        unsafe { self.texture.as_ref().unwrap_unchecked() }
    }
}
