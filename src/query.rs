use static_assertions::const_assert;

use crate::device_object::DeviceObject;

const_assert!(diligent_sys::QUERY_TYPE_NUM_TYPES == 6);

pub struct QueryDataOcclusion {
    pub num_samples: u64,
}

pub struct QueryDataBinaryOcclusion {
    pub any_sample_passed: bool,
}

pub struct QueryDataTimestamp {
    pub counter: u64,
    pub frequency: u64,
}

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

pub struct QueryDataDuration {
    pub duration: u64,
    pub frequency: u64,
}

impl Default for QueryDataOcclusion {
    fn default() -> Self {
        Self { num_samples: 0 }
    }
}

impl Default for QueryDataBinaryOcclusion {
    fn default() -> Self {
        Self {
            any_sample_passed: false,
        }
    }
}

impl Default for QueryDataTimestamp {
    fn default() -> Self {
        Self {
            counter: 0,
            frequency: 0,
        }
    }
}

impl Default for QueryDataPipelineStatistics {
    fn default() -> Self {
        Self {
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

impl Default for QueryDataDuration {
    fn default() -> Self {
        Self {
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

pub struct Query<QueryDataType: GetSysQueryType> {
    pub(crate) sys_ptr: *mut diligent_sys::IQuery,
    virtual_functions: *mut diligent_sys::IQueryVtbl,

    data: QueryDataType,

    device_object: DeviceObject,
}

impl<QueryDataType: GetSysQueryType> AsRef<DeviceObject> for Query<QueryDataType> {
    fn as_ref(&self) -> &DeviceObject {
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
        unsafe {
            (*self.virtual_functions)
                .Query
                .Invalidate
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_data(&self, autoinvalidate: bool) -> &QueryDataType {
        unsafe {
            (*self.virtual_functions).Query.GetData.unwrap_unchecked()(
                self.sys_ptr,
                std::ptr::from_ref(&self.data) as _,
                std::mem::size_of::<QueryDataType>() as u32,
                autoinvalidate,
            );
        }

        &self.data
    }
}
