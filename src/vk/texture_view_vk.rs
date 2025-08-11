use std::ops::Deref;

use crate::texture_view::TextureView;

pub struct TextureViewVk<'a> {
    sys_ptr: *mut diligent_sys::ITextureViewVk,
    virtual_functions: *mut diligent_sys::ITextureViewVkVtbl,

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
            sys_ptr: value.sys_ptr as *mut diligent_sys::ITextureViewVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::ITextureViewVk)).pVtbl
            },
        }
    }
}

impl TextureViewVk<'_> {
    pub fn get_vulkan_image_view(&self) -> diligent_sys::VkImageView {
        unsafe {
            (*self.virtual_functions)
                .TextureViewVk
                .GetVulkanImageView
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
