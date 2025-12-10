use std::{ffi::CStr, ops::Deref};

use static_assertions::const_assert_eq;

use crate::device_object::DeviceObject;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::IFenceMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct Fence(pub(crate) diligent_sys::IFence);

impl Deref for Fence {
    type Target = DeviceObject;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::IDeviceObject
                as *const DeviceObject)
        }
    }
}

#[derive(Clone, Copy)]
pub enum FenceType {
    CpuWaitOnly,
    General,
}

const_assert_eq!(diligent_sys::FENCE_TYPE_LAST, 1);

#[repr(transparent)]
pub struct FenceDesc(pub(crate) diligent_sys::FenceDesc);

#[bon::bon]
impl FenceDesc {
    #[builder]
    pub fn new(
        name: Option<&CStr>,

        #[builder(default = FenceType::CpuWaitOnly)] fence_type: FenceType,
    ) -> Self {
        Self(diligent_sys::FenceDesc {
            _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
            },
            Type: match fence_type {
                FenceType::CpuWaitOnly => diligent_sys::FENCE_TYPE_CPU_WAIT_ONLY,
                FenceType::General => diligent_sys::FENCE_TYPE_GENERAL,
            } as diligent_sys::FENCE_TYPE,
        })
    }
}

impl Fence {
    pub(crate) fn sys_ptr(&self) -> *mut diligent_sys::IFence {
        std::ptr::from_ref(&self.0) as _
    }

    pub fn get_completed_value(&self) -> u64 {
        unsafe_member_call!(self, Fence, GetCompletedValue)
    }

    pub fn signal(&self, value: u64) {
        unsafe_member_call!(self, Fence, Signal, value)
    }

    pub fn wait(&self, value: u64) {
        unsafe_member_call!(self, Fence, Wait, value)
    }
}
