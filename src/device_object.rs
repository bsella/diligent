use std::{ffi::CStr, marker::PhantomData};

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

    fn sys_ptr(self) -> *mut ResourceType::SysType;
}

#[must_use = "The resource transition functions don't transition the state of the resource immediately.
They act as a marker for the resource transition which occurs asynchronously and ensures the exclusivity
of resources with transitioning states"]
#[repr(transparent)]
pub struct ResourceStateTransitionMode<
    'resource,
    ResourceType: Ported,
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE,
>(
    *mut ResourceType::SysType,
    PhantomData<&'resource mut ResourceType>,
);

impl<
    'resource,
    ResourceType: Ported,
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE,
> Clone for ResourceStateTransitionMode<'resource, ResourceType, TRANSITION_MODE>
{
    fn clone(&self) -> Self {
        ResourceStateTransitionMode(self.0, PhantomData)
    }
}

impl<
    'resource,
    ResourceType: Ported,
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE,
> ResourceTransition<'resource, ResourceType>
    for ResourceStateTransitionMode<'resource, ResourceType, TRANSITION_MODE>
{
    const TRANSITION_MODE: diligent_sys::RESOURCE_STATE_TRANSITION_MODE = TRANSITION_MODE;

    fn sys_ptr(self) -> *mut ResourceType::SysType {
        self.0
    }
}

pub type ResourceStateTransition<'resource, ResourceType> = ResourceStateTransitionMode<
    'resource,
    ResourceType,
    { diligent_sys::RESOURCE_STATE_TRANSITION_MODE_TRANSITION as _ },
>;

impl<'resource, ResourceType: Ported> ResourceStateTransition<'resource, ResourceType> {
    pub(crate) fn new(resource: &mut ResourceType) -> Self {
        Self(resource.sys_ptr(), PhantomData)
    }
}

pub type ResourceStateVerify<'resource, ResourceType> = ResourceStateTransitionMode<
    'resource,
    ResourceType,
    { diligent_sys::RESOURCE_STATE_TRANSITION_MODE_VERIFY as _ },
>;

impl<'resource, ResourceType: Ported> ResourceStateVerify<'resource, ResourceType> {
    pub(crate) fn new(resource: &ResourceType) -> Self {
        Self(resource.sys_ptr(), PhantomData)
    }
}

pub type ResourceStateNoTransition<'resource, ResourceType> = ResourceStateTransitionMode<
    'resource,
    ResourceType,
    { diligent_sys::RESOURCE_STATE_TRANSITION_MODE_NONE as _ },
>;

impl<'resource, ResourceType: Ported> ResourceStateNoTransition<'resource, ResourceType> {
    pub(crate) fn new(resource: &ResourceType) -> Self {
        Self(resource.sys_ptr(), PhantomData)
    }
}
