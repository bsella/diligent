use crate::object::Object;

define_ported!(
    CommandQueue,
    diligent_sys::ICommandQueue,
    diligent_sys::ICommandQueueMethods : 3,
    Object
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
