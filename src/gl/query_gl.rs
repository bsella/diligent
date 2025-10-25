use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::{GetSysQueryType, query::Query};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IQueryGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct QueryGL<'a, QueryDataType: GetSysQueryType>(&'a Query<QueryDataType>);

impl<'a, QueryDataType: GetSysQueryType> Deref for QueryGL<'a, QueryDataType> {
    type Target = Query<QueryDataType>;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, QueryDataType: GetSysQueryType> From<&'a Query<QueryDataType>>
    for QueryGL<'a, QueryDataType>
{
    fn from(value: &'a Query<QueryDataType>) -> Self {
        QueryGL(value)
    }
}

impl<QueryDataType: GetSysQueryType> QueryGL<'_, QueryDataType> {
    pub fn get_query_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, QueryGL, GetGlQueryHandle)
    }
}
