use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::texture_view::TextureView;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ITextureViewVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct TextureViewVk(diligent_sys::ITextureViewVk);

impl Deref for TextureViewVk {
    type Target = TextureView;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ITextureView
                as *const TextureView)
        }
    }
}

impl TextureViewVk {
    pub fn get_vulkan_image_view(&self) -> diligent_sys::VkImageView {
        unsafe_member_call!(self, TextureViewVk, GetVulkanImageView)
    }
}
