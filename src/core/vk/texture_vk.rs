use crate::core::{
    device_object::{AsDeviceObject, DeviceObject},
    texture::Texture,
};

pub struct TextureVk<'a> {
    texture_ptr: *mut diligent_sys::ITextureVk,
    virtual_functions: *mut diligent_sys::ITextureVkVtbl,

    texture: &'a Texture,
}

impl AsDeviceObject for TextureVk<'_> {
    fn as_device_object(&self) -> &DeviceObject {
        &self.texture.as_device_object()
    }
}

impl<'a> From<&'a Texture> for TextureVk<'a> {
    fn from(value: &'a Texture) -> Self {
        TextureVk {
            texture: value,
            texture_ptr: value.texture as *mut diligent_sys::ITextureVk,
            virtual_functions: unsafe { (*(value.texture as *mut diligent_sys::ITextureVk)).pVtbl },
        }
    }
}

impl TextureVk<'_> {
    pub fn get_vk_image(&self) -> diligent_sys::VkImage {
        unsafe {
            (*self.virtual_functions)
                .TextureVk
                .GetVkImage
                .unwrap_unchecked()(self.texture_ptr)
        }
    }

    pub fn set_layout(&self, layout: diligent_sys::VkImageLayout) {
        unsafe {
            (*self.virtual_functions)
                .TextureVk
                .SetLayout
                .unwrap_unchecked()(self.texture_ptr, layout)
        }
    }

    pub fn get_layout(&self) -> diligent_sys::VkImageLayout {
        unsafe {
            (*self.virtual_functions)
                .TextureVk
                .GetLayout
                .unwrap_unchecked()(self.texture_ptr)
        }
    }
}
