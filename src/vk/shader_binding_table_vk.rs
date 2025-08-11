use std::ops::Deref;

use crate::shader_binding_table::ShaderBindingTable;

pub struct ShaderBindingTableVk<'a> {
    sys_ptr: *mut diligent_sys::IShaderBindingTableVk,
    virtual_functions: *mut diligent_sys::IShaderBindingTableVkVtbl,

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
        ShaderBindingTableVk {
            sbt: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IShaderBindingTableVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::IShaderBindingTableVk)).pVtbl
            },
        }
    }
}

impl ShaderBindingTableVk<'_> {
    pub fn get_vk_binding_table(&self) -> &diligent_sys::BindingTableVk {
        let bt = unsafe_member_call!(self, ShaderBindingTableVk, GetVkBindingTable,);
        unsafe { bt.as_ref().unwrap_unchecked() }
    }
}
