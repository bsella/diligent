use crate::core::bindings;

use crate::core::texture_view::TextureView;

use super::device_object::{AsDeviceObject, DeviceObject};
use super::object::AsObject;

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

pub struct Texture {
    pub(crate) m_texture: *mut bindings::ITexture,
    pub(crate) m_virtual_functions: *mut bindings::ITextureVtbl,

    m_default_view: Option<TextureView>,

    m_device_object: DeviceObject,
}

impl AsDeviceObject for Texture {
    fn as_device_object(&self) -> &DeviceObject {
        &self.m_device_object
    }
}

impl Texture {
    pub(crate) fn create(
        texture_ptr: *mut bindings::ITexture,
        texture_desc: &bindings::TextureDesc,
    ) -> Self {
        let mut texture = Texture {
            m_device_object: DeviceObject::create(texture_ptr as *mut bindings::IDeviceObject),
            m_texture: texture_ptr,
            m_virtual_functions: unsafe { (*texture_ptr).pVtbl },
            m_default_view: None,
        };

        let texture_view_type = bind_flags_to_texture_view_type(texture_desc.BindFlags);

        if texture_view_type != bindings::BUFFER_VIEW_UNDEFINED {
            let texture_view = TextureView::create(
                unsafe {
                    (*(*texture_ptr).pVtbl)
                        .Texture
                        .GetDefaultView
                        .unwrap_unchecked()(texture_ptr, texture_view_type as u8)
                },
                std::ptr::addr_of!(texture),
            );
            texture_view.as_device_object().as_object().add_ref();
            texture.m_default_view = Some(texture_view);
        }

        texture
    }
}

pub trait TextureImpl {
    fn get_desc(&self) -> &bindings::TextureDesc;
    fn create_view(&mut self, texture_view_desc: &bindings::TextureViewDesc)
        -> Option<TextureView>;
    fn get_default_view(
        &self,
        texture_view_type: bindings::TEXTURE_VIEW_TYPE,
    ) -> Option<&TextureView>;
    fn get_native_handle(&self) -> u64;
    fn set_state(&mut self, state: bindings::RESOURCE_STATE);
    fn get_state(&self) -> bindings::RESOURCE_STATE;
    fn get_sparse_properties(&self) -> &bindings::SparseTextureProperties;
}

impl TextureImpl for Texture {
    fn get_desc(&self) -> &bindings::TextureDesc {
        unsafe {
            ((*self.m_virtual_functions)
                .DeviceObject
                .GetDesc
                .unwrap_unchecked()(self.m_texture as *mut bindings::IDeviceObject)
                as *const bindings::TextureDesc)
                .as_ref()
                .unwrap_unchecked()
        }
    }

    fn create_view(
        &mut self,
        texture_view_desc: &bindings::TextureViewDesc,
    ) -> Option<TextureView> {
        let mut texture_view_ptr: *mut bindings::ITextureView = std::ptr::null_mut();
        unsafe {
            (*self.m_virtual_functions)
                .Texture
                .CreateView
                .unwrap_unchecked()(
                self.m_texture,
                std::ptr::addr_of!(texture_view_desc) as *const bindings::TextureViewDesc,
                std::ptr::addr_of_mut!(texture_view_ptr),
            );
        }

        if texture_view_ptr.is_null() {
            None
        } else {
            Some(TextureView::create(texture_view_ptr, self as *const Self))
        }
    }

    fn get_default_view(
        &self,
        texture_view_type: bindings::TEXTURE_VIEW_TYPE,
    ) -> Option<&TextureView> {
        if unsafe {
            (*self.m_virtual_functions)
                .Texture
                .GetDefaultView
                .unwrap_unchecked()(self.m_texture, texture_view_type)
        }
        .is_null()
        {
            None
        } else {
            self.m_default_view.as_ref()
        }
    }

    fn get_native_handle(&self) -> u64 {
        unsafe {
            (*self.m_virtual_functions)
                .Texture
                .GetNativeHandle
                .unwrap_unchecked()(self.m_texture)
        }
    }

    fn set_state(&mut self, state: bindings::RESOURCE_STATE) {
        unsafe {
            (*self.m_virtual_functions)
                .Texture
                .SetState
                .unwrap_unchecked()(self.m_texture, state);
        }
    }

    fn get_state(&self) -> bindings::RESOURCE_STATE {
        unsafe {
            (*self.m_virtual_functions)
                .Texture
                .GetState
                .unwrap_unchecked()(self.m_texture)
        }
    }

    fn get_sparse_properties(&self) -> &bindings::SparseTextureProperties {
        unsafe {
            (*self.m_virtual_functions)
                .Texture
                .GetSparseProperties
                .unwrap_unchecked()(self.m_texture)
            .as_ref()
            .unwrap_unchecked()
        }
    }
}
