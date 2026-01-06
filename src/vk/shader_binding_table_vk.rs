use crate::shader_binding_table::ShaderBindingTable;

#[repr(transparent)]
pub struct BindingTableVk(diligent_sys::BindingTableVk);

define_ported!(
    ShaderBindingTableVk,
    diligent_sys::IShaderBindingTableVk,
    diligent_sys::IShaderBindingTableVkMethods : 1,
    ShaderBindingTable
);

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
