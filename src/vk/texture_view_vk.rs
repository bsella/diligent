use std::ops::Deref;

use crate::texture_view::TextureView;

#[repr(transparent)]
pub struct TextureViewVk<'a> {
    texture_view: &'a TextureView,
}

impl Deref for TextureViewVk<'_> {
    type Target = TextureView;
    fn deref(&self) -> &Self::Target {
        self.texture_view
    }
}

impl<'a> From<&'a TextureView> for TextureViewVk<'a> {
    fn from(value: &'a TextureView) -> Self {
        TextureViewVk {
            texture_view: value,
        }
    }
}

impl TextureViewVk<'_> {
    pub fn get_vulkan_image_view(&self) -> diligent_sys::VkImageView {
        unsafe_member_call!(self, TextureViewVk, GetVulkanImageView,)
    }
}
