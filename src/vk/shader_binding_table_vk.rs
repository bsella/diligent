use std::ops::Deref;

use crate::shader_binding_table::ShaderBindingTable;

#[repr(transparent)]
pub struct ShaderBindingTableVk<'a> {
    sbt: &'a ShaderBindingTable,
}

impl Deref for ShaderBindingTableVk<'_> {
    type Target = ShaderBindingTable;
    fn deref(&self) -> &Self::Target {
        self.sbt
    }
}

impl<'a> From<&'a ShaderBindingTable> for ShaderBindingTableVk<'a> {
    fn from(value: &'a ShaderBindingTable) -> Self {
        ShaderBindingTableVk { sbt: value }
    }
}

impl ShaderBindingTableVk<'_> {
    pub fn get_vk_binding_table(&self) -> &diligent_sys::BindingTableVk {
        let bt = unsafe_member_call!(self, ShaderBindingTableVk, GetVkBindingTable,);
        unsafe { bt.as_ref().unwrap_unchecked() }
    }
}
