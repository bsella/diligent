use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::shader_binding_table::ShaderBindingTable;

#[repr(transparent)]
pub struct BindingTableVk(diligent_sys::BindingTableVk);

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IShaderBindingTableVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ShaderBindingTableVk(diligent_sys::IShaderBindingTableVk);

impl Deref for ShaderBindingTableVk {
    type Target = ShaderBindingTable;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IShaderBindingTable
                as *const ShaderBindingTable)
        }
    }
}

impl ShaderBindingTableVk {
    pub fn get_vk_binding_table(&self) -> Option<&BindingTableVk> {
        let binding_table_ptr = unsafe_member_call!(self, ShaderBindingTableVk, GetVkBindingTable);
        if binding_table_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*(binding_table_ptr as *const BindingTableVk) })
        }
    }
}
