use crate::bindings;

use super::texture_view::TextureView;

use super::device_object::{AsDeviceObject, DeviceObject};
use super::object::AsObject;

pub struct Texture {
    pub(crate) texture: *mut bindings::ITexture,
    virtual_functions: *mut bindings::ITextureVtbl,

    default_view: Option<TextureView>,

    device_object: DeviceObject,
}

impl AsDeviceObject for Texture {
    fn as_device_object(&self) -> &DeviceObject {
        &self.device_object
    }
}

impl Texture {
    pub(crate) fn new(
        texture_ptr: *mut bindings::ITexture,
        texture_desc: &bindings::TextureDesc,
    ) -> Self {
        let mut texture = Texture {
            device_object: DeviceObject::new(texture_ptr as *mut bindings::IDeviceObject),
            texture: texture_ptr,
            virtual_functions: unsafe { (*texture_ptr).pVtbl },
            default_view: None,
        };

        fn bind_flags_to_texture_view_type(
            bind_flag: bindings::BIND_FLAGS,
        ) -> bindings::_TEXTURE_VIEW_TYPE {
            if bind_flag & bindings::BIND_SHADER_RESOURCE != 0 {
                bindings::TEXTURE_VIEW_SHADER_RESOURCE
            } else if bind_flag & bindings::BIND_RENDER_TARGET != 0 {
                bindings::BUFFER_VIEW_SHADER_RESOURCE
            } else if bind_flag & bindings::BIND_DEPTH_STENCIL != 0 {
                bindings::TEXTURE_VIEW_DEPTH_STENCIL
            } else if bind_flag & bindings::BIND_UNORDERED_ACCESS != 0 {
                bindings::TEXTURE_VIEW_UNORDERED_ACCESS
            } else if bind_flag & bindings::BIND_SHADING_RATE != 0 {
                bindings::TEXTURE_VIEW_SHADING_RATE
            } else {
                bindings::TEXTURE_VIEW_UNDEFINED
            }
        }

        let texture_view_type = bind_flags_to_texture_view_type(texture_desc.BindFlags);

        if texture_view_type != bindings::BUFFER_VIEW_UNDEFINED {
            let texture_view = TextureView::new(
                unsafe {
                    (*(*texture_ptr).pVtbl)
                        .Texture
                        .GetDefaultView
                        .unwrap_unchecked()(texture_ptr, texture_view_type as u8)
                },
                std::ptr::addr_of!(texture),
            );
            texture_view.as_device_object().as_object().add_ref();
            texture.default_view = Some(texture_view);
        }

        texture
    }

    pub fn get_desc(&self) -> &bindings::TextureDesc {
        unsafe {
            ((*self.virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.texture as *mut bindings::IDeviceObject)
                as *const bindings::TextureDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    pub fn create_view(
        &mut self,
        texture_view_desc: &bindings::TextureViewDesc,
    ) -> Option<TextureView> {
        let mut texture_view_ptr: *mut bindings::ITextureView = std::ptr::null_mut();
        unsafe {
            (*self.virtual_functions)
                .Texture
                .CreateView
                .unwrap_unchecked()(
                self.texture,
                std::ptr::addr_of!(texture_view_desc) as *const bindings::TextureViewDesc,
                std::ptr::addr_of_mut!(texture_view_ptr),
            );
        }

        if texture_view_ptr.is_null() {
            None
        } else {
            Some(TextureView::new(texture_view_ptr, self as *const Self))
        }
    }

    pub fn get_default_view(
        &self,
        texture_view_type: bindings::TEXTURE_VIEW_TYPE,
    ) -> Option<&TextureView> {
        if unsafe {
            (*self.virtual_functions)
                .Texture
                .GetDefaultView
                .unwrap_unchecked()(self.texture, texture_view_type)
        }
        .is_null()
        {
            None
        } else {
            self.default_view.as_ref()
        }
    }

    pub fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetNativeHandle
                .unwrap_unchecked()(self.texture)
        }
    }

    pub fn set_state(&mut self, state: bindings::RESOURCE_STATE) {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .SetState
                .unwrap_unchecked()(self.texture, state);
        }
    }

    pub fn get_state(&self) -> bindings::RESOURCE_STATE {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetState
                .unwrap_unchecked()(self.texture)
        }
    }

    pub fn get_sparse_properties(&self) -> &bindings::SparseTextureProperties {
        unsafe {
            (*self.virtual_functions)
                .Texture
                .GetSparseProperties
                .unwrap_unchecked()(self.texture)
            .as_ref()
            .unwrap_unchecked()
        }
    }
}
