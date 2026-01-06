use crate::texture::Texture;

define_ported!(
    TextureVk,
    diligent_sys::ITextureVk,
    diligent_sys::ITextureVkMethods : 3,
    Texture
);

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
