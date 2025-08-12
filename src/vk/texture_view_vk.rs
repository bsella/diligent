use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::texture_view::TextureView;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureViewVkMethods>(),
    std::mem::size_of::<*const ()>()
);

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
        unsafe_member_call!(self, TextureViewVk, GetVulkanImageView)
    }
}
