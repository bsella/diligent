use std::{marker::PhantomData, ops::Deref};

use static_assertions::const_assert_eq;

use crate::{GetSysQueryType, query::Query};

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IQueryGLMethods>(),
    std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct QueryGL<QueryDataType: GetSysQueryType>(
    diligent_sys::IQueryGL,
    PhantomData<QueryDataType>,
);

impl<QueryDataType: GetSysQueryType> Deref for QueryGL<QueryDataType> {
    type Target = Query<QueryDataType>;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IQuery
                as *const Query<QueryDataType>)
        }
    }
}

impl<QueryDataType: GetSysQueryType> QueryGL<QueryDataType> {
    pub fn get_query_handle(&self) -> diligent_sys::GLuint {
        unsafe_member_call!(self, QueryGL, GetGlQueryHandle)
    }
}
