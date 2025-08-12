use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::texture::Texture;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureVkMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TextureVk<'a> {
    texture: &'a Texture,
}

impl Deref for TextureVk<'_> {
    type Target = Texture;
    fn deref(&self) -> &Self::Target {
        self.texture
    }
}

impl<'a> From<&'a Texture> for TextureVk<'a> {
    fn from(value: &'a Texture) -> Self {
        TextureVk { texture: value }
    }
}

impl TextureVk<'_> {
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
