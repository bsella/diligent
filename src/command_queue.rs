use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::object::Object;

#[repr(transparent)]
pub struct CommandQueue(diligent_sys::ICommandQueue);

impl Deref for CommandQueue {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(std::ptr::addr_of!(self.0) as *const diligent_sys::IObject as *const Object) }
    }
}

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ICommandQueueMethods>(),
    3 * std::mem::size_of::<*const ()>()
);

impl CommandQueue {
    pub fn get_next_fence_value(&self) -> u64 {
        unsafe_member_call!(self, CommandQueue, GetNextFenceValue)
    }

    pub fn get_completed_fence_value(&self) -> u64 {
        unsafe_member_call!(self, CommandQueue, GetCompletedFenceValue)
    }

    pub fn wait_for_idle(&self) -> u64 {
        unsafe_member_call!(self, CommandQueue, WaitForIdle)
    }
}
