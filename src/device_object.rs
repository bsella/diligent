use std::ffi::CStr;

use crate::{Ported, object::Object};

define_ported!(
    DeviceObject,
    diligent_sys::IDeviceObject,
    diligent_sys::IDeviceObjectMethods : 4,
    Object
);

impl DeviceObject {
    pub fn get_unique_id(&self) -> i32 {
        unsafe_member_call!(self, DeviceObject, GetUniqueID)
    }
}

#[repr(transparent)]
pub struct DeviceObjectAttribs(diligent_sys::DeviceObjectAttribs);

impl DeviceObjectAttribs {
    pub fn name(&self) -> Option<&CStr> {
        if self.0.Name.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(self.0.Name)) }
        }
    }
}

pub trait ResourceTransition<'resource, ResourceType: Ported> {
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE;

    fn resource_ref(self) -> *mut ResourceType::SysType;
}

#[must_use = ""]
#[repr(transparent)]
pub struct ResourceStateTransition<'resource, ResourceType>(pub(crate) &'resource mut ResourceType);

impl<'resource, ResourceType: Ported> ResourceTransition<'resource, ResourceType>
    for ResourceStateTransition<'resource, ResourceType>
{
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE =
        diligent_sys::RESOURCE_STATE_TRANSITION_MODE_TRANSITION as _;

    fn resource_ref(self) -> *mut ResourceType::SysType {
        self.0.sys_ptr()
    }
}

#[must_use = ""]
#[repr(transparent)]
pub struct ResourceStateVerify<'resource, ResourceType>(pub(crate) &'resource ResourceType);

impl<'resource, ResourceType: Ported> ResourceTransition<'resource, ResourceType>
    for ResourceStateVerify<'resource, ResourceType>
{
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE =
        diligent_sys::RESOURCE_STATE_TRANSITION_MODE_VERIFY as _;

    fn resource_ref(self) -> *mut ResourceType::SysType {
        self.0.sys_ptr()
    }
}

#[must_use = ""]
#[repr(transparent)]
pub struct ResourceStateNoTransition<'resource, ResourceType>(pub(crate) &'resource ResourceType);

impl<'resource, ResourceType: Ported> ResourceTransition<'resource, ResourceType>
    for ResourceStateNoTransition<'resource, ResourceType>
{
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE =
        diligent_sys::RESOURCE_STATE_TRANSITION_MODE_NONE as _;

    fn resource_ref(self) -> *mut ResourceType::SysType {
        self.0.sys_ptr()
    }
}
