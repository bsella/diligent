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

impl From<&TextureViewType> for diligent_sys::TEXTURE_VIEW_TYPE {
    fn from(value: &TextureViewType) -> Self {
        (match value {
            TextureViewType::ShaderResource => diligent_sys::TEXTURE_VIEW_SHADER_RESOURCE,
            TextureViewType::RenderTarget => diligent_sys::TEXTURE_VIEW_RENDER_TARGET,
            TextureViewType::DepthStencil => diligent_sys::TEXTURE_VIEW_DEPTH_STENCIL,
            TextureViewType::ReadOnlyDepthStencil => {
                diligent_sys::TEXTURE_VIEW_READ_ONLY_DEPTH_STENCIL
            }
            TextureViewType::UnorderedAccess => diligent_sys::TEXTURE_VIEW_UNDEFINED,
            TextureViewType::ShadingRate => diligent_sys::TEXTURE_VIEW_SHADING_RATE,
        }) as diligent_sys::TEXTURE_VIEW_TYPE
    }
}

pub struct TextureView {
    pub(crate) texture_view: *mut diligent_sys::ITextureView,
    virtual_functions: *mut diligent_sys::ITextureViewVtbl,
    texture: *const Texture,

    pub(crate) device_object: DeviceObject,
}

impl AsDeviceObject for TextureView {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl TextureView {
    pub(crate) fn new(
        texture_view: *mut diligent_sys::ITextureView,
        texture: *const Texture,
    ) -> Self {
        TextureView {
            virtual_functions: unsafe { (*texture_view).pVtbl },
            texture_view,
            texture,
            device_object: DeviceObject::new(texture_view as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn get_desc(&self) -> diligent_sys::TextureViewDesc {
        unsafe {
            *((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(
                self.texture_view as *mut diligent_sys::IDeviceObject
            ) as *const diligent_sys::TextureViewDesc)
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
