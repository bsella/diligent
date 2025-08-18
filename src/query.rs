use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::device_object::DeviceObject;

const_assert_eq!(diligent_sys::QUERY_TYPE_NUM_TYPES, 6);

#[repr(C)]
pub struct QueryDataOcclusion {
    query_type: diligent_sys::QUERY_TYPE,
    pub num_samples: u64,
}

impl Default for QueryDataOcclusion {
    fn default() -> Self {
        Self {
            query_type: diligent_sys::QUERY_TYPE_OCCLUSION,
            num_samples: 0,
        }
    }
}

#[repr(C)]
pub struct QueryDataBinaryOcclusion {
    query_type: diligent_sys::QUERY_TYPE,
    pub any_sample_passed: bool,
}

impl Default for QueryDataBinaryOcclusion {
    fn default() -> Self {
        Self {
            query_type: diligent_sys::QUERY_TYPE_BINARY_OCCLUSION,
            any_sample_passed: false,
        }
    }
}

#[repr(C)]
pub struct QueryDataTimestamp {
    query_type: diligent_sys::QUERY_TYPE,
    pub counter: u64,
    pub frequency: u64,
}

impl Default for QueryDataTimestamp {
    fn default() -> Self {
        Self {
            query_type: diligent_sys::QUERY_TYPE_TIMESTAMP,
            counter: 0,
            frequency: 0,
        }
    }
}

#[repr(C)]
pub struct QueryDataPipelineStatistics {
    query_type: diligent_sys::QUERY_TYPE,
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

impl Default for QueryDataPipelineStatistics {
    fn default() -> Self {
        Self {
            query_type: diligent_sys::QUERY_TYPE_PIPELINE_STATISTICS,
            input_vertices: 0,
            input_primitives: 0,
            gs_primitives: 0,
            clipping_invocations: 0,
            clipping_primitives: 0,
            vs_invocations: 0,
            gs_invocations: 0,
            ps_invocations: 0,
            hs_invocations: 0,
            ds_invocations: 0,
            cs_invocations: 0,
        }
    }
}

#[repr(C)]
pub struct QueryDataDuration {
    query_type: diligent_sys::QUERY_TYPE,
    pub duration: u64,
    pub frequency: u64,
}

impl Default for QueryDataDuration {
    fn default() -> Self {
        Self {
            query_type: diligent_sys::QUERY_TYPE_DURATION,
            duration: 0,
            frequency: 0,
        }
    }
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

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IQueryMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

pub struct Query<QueryDataType: GetSysQueryType> {
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
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IDeviceObject>(),
            std::mem::size_of::<diligent_sys::IQuery>()
        );
        Query::<QueryDataType> {
            data: QueryDataType::default(),
            device_object: DeviceObject::new(query_ptr as *mut diligent_sys::IDeviceObject),
        }
    }

    pub fn invalidate(&self) {
        unsafe_member_call!(self, Query, Invalidate)
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
