use std::ops::Deref;

use static_assertions::const_assert;

use crate::device_object::DeviceObject;

const_assert!(diligent_sys::QUERY_TYPE_NUM_TYPES == 6);

#[derive(Default)]
pub struct QueryDataOcclusion {
    pub num_samples: u64,
}

#[derive(Default)]
pub struct QueryDataBinaryOcclusion {
    pub any_sample_passed: bool,
}

#[derive(Default)]
pub struct QueryDataTimestamp {
    pub counter: u64,
    pub frequency: u64,
}

#[derive(Default)]
pub struct QueryDataPipelineStatistics {
    pub input_vertices: u64,
    pub input_primitives: u64,
    pub gs_primitives: u64,
    pub clipping_invocations: u64,
    pub clipping_primitives: u64,
    pub vs_invocations: u64,
    pub gs_invocations: u64,
    pub ps_invocations: u64,
    pub hs_invocations: u64,
    pub ds_invocations: u64,
    pub cs_invocations: u64,
}

#[derive(Default)]
pub struct QueryDataDuration {
    pub duration: u64,
    pub frequency: u64,
}

pub trait GetSysQueryType {
    const QUERY_TYPE: diligent_sys::QUERY_TYPE;
}

impl GetSysQueryType for QueryDataOcclusion {
    const QUERY_TYPE: diligent_sys::QUERY_TYPE = diligent_sys::QUERY_TYPE_OCCLUSION;
}

impl GetSysQueryType for QueryDataBinaryOcclusion {
    const QUERY_TYPE: diligent_sys::QUERY_TYPE = diligent_sys::QUERY_TYPE_BINARY_OCCLUSION;
}

impl GetSysQueryType for QueryDataTimestamp {
    const QUERY_TYPE: diligent_sys::QUERY_TYPE = diligent_sys::QUERY_TYPE_TIMESTAMP;
}

impl GetSysQueryType for QueryDataPipelineStatistics {
    const QUERY_TYPE: diligent_sys::QUERY_TYPE = diligent_sys::QUERY_TYPE_PIPELINE_STATISTICS;
}

impl GetSysQueryType for QueryDataDuration {
    const QUERY_TYPE: diligent_sys::QUERY_TYPE = diligent_sys::QUERY_TYPE_DURATION;
}

pub struct Query<QueryDataType: GetSysQueryType> {
    pub(crate) sys_ptr: *mut diligent_sys::IQuery,
    virtual_functions: *mut diligent_sys::IQueryVtbl,

    data: QueryDataType,

    device_object: DeviceObject,
}

impl<QueryDataType: GetSysQueryType> Deref for Query<QueryDataType> {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.device_object
    }
}

impl<QueryDataType: GetSysQueryType + Default> Query<QueryDataType> {
    pub(crate) fn new(query_ptr: *mut diligent_sys::IQuery) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IDeviceObject>()
                == std::mem::size_of::<diligent_sys::IQuery>()
        );
        Query::<QueryDataType> {
            sys_ptr: query_ptr,
            data: QueryDataType::default(),
            virtual_functions: unsafe { (*query_ptr).pVtbl },
            device_object: DeviceObject::new(query_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn invalidate(&self) {
        unsafe_member_call!(self, Query, Invalidate,)
    }

    pub fn get_data(&self, autoinvalidate: bool) -> &QueryDataType {
        unsafe_member_call!(
            self,
            Query,
            GetData,
            std::ptr::from_ref(&self.data) as _,
            std::mem::size_of::<QueryDataType>() as u32,
            autoinvalidate
        );

        &self.data
    }
}
