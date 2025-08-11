use std::ops::Deref;

use crate::query::{GetSysQueryType, Query};

pub struct QueryVk<'a, QueryDataType: GetSysQueryType> {
    #[allow(dead_code)]
    sys_ptr: *mut diligent_sys::IQueryVk,
    #[allow(dead_code)]
    virtual_functions: *mut diligent_sys::IQueryVkVtbl,

    query: &'a Query<QueryDataType>,
}

impl<QueryDataType: GetSysQueryType> Deref for QueryVk<'_, QueryDataType> {
    type Target = Query<QueryDataType>;
    fn deref(&self) -> &Self::Target {
        self.query
    }
}

impl<'a, QueryDataType: GetSysQueryType> From<&'a Query<QueryDataType>>
    for QueryVk<'a, QueryDataType>
{
    fn from(value: &'a Query<QueryDataType>) -> Self {
        QueryVk {
            query: value,
            sys_ptr: value.sys_ptr as *mut diligent_sys::IQueryVk,
            virtual_functions: unsafe { (*(value.sys_ptr as *mut diligent_sys::IQueryVk)).pVtbl },
        }
    }
}

impl<QueryDataType: GetSysQueryType> QueryVk<'_, QueryDataType> {}
