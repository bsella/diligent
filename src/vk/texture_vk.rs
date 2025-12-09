use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::texture::Texture;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureVkMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TextureVk(diligent_sys::ITextureVk);

impl Deref for TextureVk {
    type Target = Texture;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ITexture as *const Texture)
        }
    }
}

impl TextureVk {
    pub fn get_vk_image(&self) -> diligent_sys::VkImage {
        unsafe_member_call!(self, TextureVk, GetVkImage)
    }

    pub fn set_layout(&self, layout: diligent_sys::VkImageLayout) {
        unsafe_member_call!(self, TextureVk, SetLayout, layout)
    }

    pub fn get_layout(&self) -> diligent_sys::VkImageLayout {
        unsafe_member_call!(self, TextureVk, GetLayout)
    }
}
