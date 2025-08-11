use std::ops::Deref;

use crate::query::{GetSysQueryType, Query};

#[repr(transparent)]
pub struct QueryVk<'a, QueryDataType: GetSysQueryType> {
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
        QueryVk { query: value }
    }
}

impl<QueryDataType: GetSysQueryType> QueryVk<'_, QueryDataType> {}
