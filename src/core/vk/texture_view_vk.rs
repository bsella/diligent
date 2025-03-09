use crate::core::{
    device_object::{AsDeviceObject, DeviceObject},
    texture_view::TextureView,
};

pub struct TextureViewVk<'a> {
    texture_view_ptr: *mut diligent_sys::ITextureViewVk,
    virtual_functions: *mut diligent_sys::ITextureViewVkVtbl,

    texture_view: &'a TextureView,
}

impl AsDeviceObject for TextureViewVk<'_> {
    fn as_device_object(&self) -> &DeviceObject {
        &self.texture_view.as_device_object()
    }
}

impl<'a> From<&'a TextureView> for TextureViewVk<'a> {
    fn from(value: &'a TextureView) -> Self {
        TextureViewVk {
            texture_view: value,
            texture_view_ptr: value.texture_view as *mut diligent_sys::ITextureViewVk,
            virtual_functions: unsafe {
                (*(value.texture_view as *mut diligent_sys::ITextureViewVk)).pVtbl
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
                .unwrap_unchecked()(self.texture_view_ptr)
        }
    }
}
