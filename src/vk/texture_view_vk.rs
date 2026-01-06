use crate::texture_view::TextureView;

define_ported!(
    TextureViewVk,
    diligent_sys::ITextureViewVk,
    diligent_sys::ITextureViewVkMethods : 1,
    TextureView
);

impl TextureViewVk {
    pub fn get_vulkan_image_view(&self) -> diligent_sys::VkImageView {
        unsafe_member_call!(self, TextureViewVk, GetVulkanImageView)
    }
}
