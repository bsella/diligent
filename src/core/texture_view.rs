use crate::bindings;

use super::sampler::Sampler;
use super::texture::Texture;

use super::device_object::{AsDeviceObject, DeviceObject};

pub enum TextureViewType {
    ShaderResource,
    RenderTarget,
    DepthStencil,
    ReadOnlyDepthStencil,
    UnorderedAccess,
    ShadingRate,
}

impl From<&TextureViewType> for bindings::TEXTURE_VIEW_TYPE {
    fn from(value: &TextureViewType) -> Self {
        (match value {
            TextureViewType::ShaderResource => bindings::TEXTURE_VIEW_SHADER_RESOURCE,
            TextureViewType::RenderTarget => bindings::TEXTURE_VIEW_RENDER_TARGET,
            TextureViewType::DepthStencil => bindings::TEXTURE_VIEW_DEPTH_STENCIL,
            TextureViewType::ReadOnlyDepthStencil => bindings::TEXTURE_VIEW_READ_ONLY_DEPTH_STENCIL,
            TextureViewType::UnorderedAccess => bindings::TEXTURE_VIEW_UNDEFINED,
            TextureViewType::ShadingRate => bindings::TEXTURE_VIEW_SHADING_RATE,
        }) as bindings::TEXTURE_VIEW_TYPE
    }
}

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
            texture_view,
            texture,
            device_object: DeviceObject::new(texture_view as *mut bindings::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> bindings::TextureViewDesc {
        unsafe {
            *((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.texture_view as *mut bindings::IDeviceObject)
                as *const bindings::TextureViewDesc)
        }
    }

    pub fn set_sampler(&mut self, sampler: &Sampler) {
        unsafe {
            (*self.virtual_functions)
                .TextureView
                .SetSampler
                .unwrap_unchecked()(self.texture_view, sampler.sampler);
        }
    }

    pub fn get_sampler(&self) -> Option<&Sampler> {
        todo!()
    }

    #[inline]
    pub fn get_texture(&self) -> &Texture {
        unsafe { self.texture.as_ref().unwrap_unchecked() }
    }
}
