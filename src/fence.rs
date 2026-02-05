use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use static_assertions::const_assert_eq;

use crate::device_object::{DeviceObject, DeviceObjectAttribs};

define_ported!(
    Fence,
    diligent_sys::IFence,
    diligent_sys::IFenceMethods : 3,
    DeviceObject
);

#[derive(Clone, Copy)]
pub enum FenceType {
    CpuWaitOnly,
    General,
}

const_assert_eq!(diligent_sys::FENCE_TYPE_LAST, 1);

#[repr(transparent)]
pub struct FenceDesc<'name>(pub(crate) diligent_sys::FenceDesc, PhantomData<&'name ()>);

impl Deref for FenceDesc<'_> {
    type Target = DeviceObjectAttribs;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::from_ref(&self.0) as *const _) }
    }
}

#[bon::bon]
impl<'name> FenceDesc<'name> {
    #[builder]
    pub fn new(
        name: Option<&'name CStr>,

        #[builder(default = FenceType::CpuWaitOnly)] fence_type: FenceType,
    ) -> Self {
        Self(
            diligent_sys::FenceDesc {
                _DeviceObjectAttribs: diligent_sys::DeviceObjectAttribs {
                    Name: name.map_or(std::ptr::null(), |name| name.as_ptr()),
                },
                Type: match fence_type {
                    FenceType::CpuWaitOnly => diligent_sys::FENCE_TYPE_CPU_WAIT_ONLY,
                    FenceType::General => diligent_sys::FENCE_TYPE_GENERAL,
                } as diligent_sys::FENCE_TYPE,
            },
            PhantomData,
        )
    }
}

impl Fence {
    pub fn desc(&self) -> &FenceDesc<'_> {
        let desc_ptr = unsafe_member_call!(self, DeviceObject, GetDesc);
        unsafe { &*(desc_ptr as *const FenceDesc) }
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
