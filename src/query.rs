use std::{cell::RefCell, marker::PhantomData, mem::MaybeUninit, ops::Deref};

use static_assertions::const_assert_eq;

use crate::{
    device_context::DeviceContext, device_object::DeviceObject, render_device::RenderDevice,
};

const_assert_eq!(diligent_sys::QUERY_TYPE_NUM_TYPES, 6);

#[repr(C)]
pub struct QueryDataOcclusion {
    query_type: diligent_sys::QUERY_TYPE,
    pub num_samples: u64,
}

#[repr(C)]
pub struct QueryDataBinaryOcclusion {
    query_type: diligent_sys::QUERY_TYPE,
    pub any_sample_passed: bool,
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

#[repr(C)]
pub struct QueryDataDuration {
    query_type: diligent_sys::QUERY_TYPE,
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

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IQueryMethods>(),
    2 * std::mem::size_of::<*const ()>()
);

pub struct Query<QueryDataType: GetSysQueryType>(DeviceObject, PhantomData<QueryDataType>);

impl<QueryDataType: GetSysQueryType> Deref for Query<QueryDataType> {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<QueryDataType: GetSysQueryType> Query<QueryDataType> {
    pub(crate) fn new(query_ptr: *mut diligent_sys::IQuery) -> Self {
        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert_eq!(
            std::mem::size_of::<diligent_sys::IDeviceObject>(),
            std::mem::size_of::<diligent_sys::IQuery>()
        );
        Query::<QueryDataType>(
            DeviceObject::new(query_ptr as *mut diligent_sys::IDeviceObject),
            PhantomData,
        )
    }

    pub fn invalidate(&self) {
        unsafe_member_call!(self, Query, Invalidate)
    }

    pub(crate) fn get_data_sys(&self, data: &mut QueryDataType, autoinvalidate: bool) -> bool {
        unsafe_member_call!(
            self,
            Query,
            GetData,
            std::ptr::from_mut(data) as _,
            std::mem::size_of::<QueryDataType>() as u32,
            autoinvalidate
        )
    }
}

pub struct ScopedQueryToken<'a, QueryDataType: GetSysQueryType> {
    query: &'a Query<QueryDataType>,
    context: &'a DeviceContext,
}

impl<'a, QueryDataType: GetSysQueryType> ScopedQueryToken<'a, QueryDataType> {
    pub(crate) fn new(context: &'a DeviceContext, query: &'a Query<QueryDataType>) -> Self {
        unsafe_member_call!(context, DeviceContext, BeginQuery, query.sys_ptr as _);

        Self { query, context }
    }

    pub fn data(self, invalidate: bool) -> Option<QueryDataType> {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            EndQuery,
            self.query.sys_ptr as _
        );

        let mut data = MaybeUninit::<QueryDataType>::uninit();

        unsafe {
            let query_type: &mut diligent_sys::QUERY_TYPE = std::mem::transmute(&mut data);
            *query_type = QueryDataType::QUERY_TYPE;
        }

        if unsafe_member_call!(
            self.query,
            Query,
            GetData,
            std::ptr::from_ref(&data) as _,
            std::mem::size_of::<QueryDataType>() as u32,
            invalidate
        ) {
            Some(unsafe { data.assume_init() })
        } else {
            None
        }
    }
}

pub struct DurationQueryHelper {
    start: Query<QueryDataTimestamp>,
    start_timestamp: RefCell<QueryDataTimestamp>,
    end: Query<QueryDataTimestamp>,
    end_timestamp: RefCell<QueryDataTimestamp>,
}

pub struct TimeStampQueryToken<'a> {
    query: &'a DurationQueryHelper,
    context: &'a DeviceContext,
}

impl TimeStampQueryToken<'_> {
    pub(crate) fn new<'a>(
        helper: &'a DurationQueryHelper,
        context: &'a DeviceContext,
    ) -> TimeStampQueryToken<'a> {
        unsafe_member_call!(context, DeviceContext, EndQuery, helper.start.sys_ptr as _);

        TimeStampQueryToken {
            context,
            query: helper,
        }
    }

    pub fn duration(self) -> Option<f64> {
        unsafe_member_call!(
            self.context,
            DeviceContext,
            EndQuery,
            self.query.end.sys_ptr as _
        );

        let mut start_timestamp = self.query.start_timestamp.borrow_mut();
        // Do not invalidate the query until we also get end timestamp
        if !self.query.start.get_data_sys(&mut *start_timestamp, false) {
            return None;
        }

        let mut end_timestamp = self.query.end_timestamp.borrow_mut();
        if !self.query.end.get_data_sys(&mut *end_timestamp, true) {
            return None;
        }

        self.query.start.invalidate();

        Some(
            end_timestamp.counter as f64 / end_timestamp.frequency as f64
                - start_timestamp.counter as f64 / start_timestamp.frequency as f64,
        )
    }
}

impl DurationQueryHelper {
    pub fn new(device: &RenderDevice) -> Result<DurationQueryHelper, ()> {
        Ok(DurationQueryHelper {
            start: device.create_query_timestamp(Some(c"Duration start timestamp query"))?,
            end: device.create_query_timestamp(Some(c"Duration end timestamp query"))?,
            start_timestamp: RefCell::new(QueryDataTimestamp::default()),
            end_timestamp: RefCell::new(QueryDataTimestamp::default()),
        })
    }
}
