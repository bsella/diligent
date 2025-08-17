use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::shader_binding_table::ShaderBindingTable;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IShaderBindingTableVkMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct ShaderBindingTableVk<'a>(&'a ShaderBindingTable);

impl Deref for ShaderBindingTableVk<'_> {
    type Target = ShaderBindingTable;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a ShaderBindingTable> for ShaderBindingTableVk<'a> {
    fn from(value: &'a ShaderBindingTable) -> Self {
        ShaderBindingTableVk(value)
    }
}

impl ShaderBindingTableVk<'_> {
    pub fn get_vk_binding_table(&self) -> &diligent_sys::BindingTableVk {
        let bt = unsafe_member_call!(self, ShaderBindingTableVk, GetVkBindingTable);
        unsafe { bt.as_ref().unwrap_unchecked() }
    }
}
