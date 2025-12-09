use std::{marker::PhantomData, ops::Deref};

use crate::query::{GetSysQueryType, Query};

#[repr(transparent)]
pub struct QueryVk<QueryDataType: GetSysQueryType>(
    diligent_sys::IQueryVk,
    PhantomData<QueryDataType>,
);

impl<QueryDataType: GetSysQueryType> Deref for QueryVk<QueryDataType> {
    type Target = Query<QueryDataType>;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IQuery
                as *const Query<QueryDataType>)
        }
    }
}

impl<QueryDataType: GetSysQueryType> QueryVk<QueryDataType> {}
