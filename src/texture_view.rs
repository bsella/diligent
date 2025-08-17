use std::ops::Deref;

use static_assertions::{const_assert, const_assert_eq};

use crate::{device_object::DeviceObject, sampler::Sampler, texture::Texture};

#[derive(Clone, Copy)]
pub enum TextureViewType {
    ShaderResource,
    RenderTarget,
    DepthStencil,
    ReadOnlyDepthStencil,
    UnorderedAccess,
    ShadingRate,
}

impl From<TextureViewType> for diligent_sys::TEXTURE_VIEW_TYPE {
    fn from(value: TextureViewType) -> Self {
        (match value {
            TextureViewType::ShaderResource => diligent_sys::TEXTURE_VIEW_SHADER_RESOURCE,
            TextureViewType::RenderTarget => diligent_sys::TEXTURE_VIEW_RENDER_TARGET,
            TextureViewType::DepthStencil => diligent_sys::TEXTURE_VIEW_DEPTH_STENCIL,
            TextureViewType::ReadOnlyDepthStencil => {
                diligent_sys::TEXTURE_VIEW_READ_ONLY_DEPTH_STENCIL
            }
            TextureViewType::UnorderedAccess => diligent_sys::TEXTURE_VIEW_UNORDERED_ACCESS,
            TextureViewType::ShadingRate => diligent_sys::TEXTURE_VIEW_SHADING_RATE,
        }) as _
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureViewMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TextureView(DeviceObject);

impl Deref for TextureView {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TextureView {
    pub(crate) fn new(texture_view_ptr: *mut diligent_sys::ITextureView) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::ITextureView>()
        );

        Self(DeviceObject::new(
            texture_view_ptr as *mut diligent_sys::IDeviceObject,
        ))
    }

    pub fn set_sampler(&mut self, sampler: &Sampler) {
        unsafe_member_call!(self, TextureView, SetSampler, sampler.sys_ptr as _);
    }

    pub fn get_sampler(&self) -> Result<&Sampler, ()> {
        todo!()
    }

    #[inline]
    pub fn get_texture(&self) -> Texture {
        let texture = Texture::new(unsafe_member_call!(self, TextureView, GetTexture));
        texture.add_ref();
        texture
    }
}
