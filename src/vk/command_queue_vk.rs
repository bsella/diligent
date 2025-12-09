use std::ops::Deref;

use static_assertions::const_assert_eq;

use crate::command_queue::CommandQueue;

const_assert_eq!(
    std::mem::size_of::<diligent_sys::ICommandQueueVkMethods>(),
    8 * std::mem::size_of::<*const ()>()
);

#[repr(transparent)]
pub struct CommandQueueVk(diligent_sys::ICommandQueueVk);

impl Deref for CommandQueueVk {
    type Target = CommandQueue;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(std::ptr::from_ref(&self.0) as *const diligent_sys::ICommandQueue
                as *const CommandQueue)
        }
    }
}

impl CommandQueueVk {
    /// # Safety
    /// cmd_buffer is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn submit_cmd_buffer(&self, cmd_buffer: diligent_sys::VkCommandBuffer) -> u64 {
        unsafe_member_call!(self, CommandQueueVk, SubmitCmdBuffer, cmd_buffer)
    }

    pub fn submit(&self, submit_info: &diligent_sys::VkSubmitInfo) -> u64 {
        unsafe_member_call!(self, CommandQueueVk, Submit, submit_info)
    }

    pub fn present(&self, present_info: &diligent_sys::VkPresentInfoKHR) -> diligent_sys::VkResult {
        unsafe_member_call!(self, CommandQueueVk, Present, present_info)
    }

    pub fn bind_sparse(&self, bind_sparse_info: &diligent_sys::VkBindSparseInfo) -> u64 {
        unsafe_member_call!(self, CommandQueueVk, BindSparse, bind_sparse_info)
    }

    pub fn get_vk_queue(&self) -> diligent_sys::VkQueue {
        unsafe_member_call!(self, CommandQueueVk, GetVkQueue)
    }

    pub fn get_queue_family_index(&self) -> u32 {
        unsafe_member_call!(self, CommandQueueVk, GetQueueFamilyIndex)
    }

    /// # Safety
    /// vk_fence is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn enqueue_signal_fence(&self, vk_fence: diligent_sys::VkFence) {
        unsafe_member_call!(self, CommandQueueVk, EnqueueSignalFence, vk_fence)
    }

    /// # Safety
    /// vk_timeline_semaphore is a pointer. The user of this function must make sure that it is valid.
    pub unsafe fn enqueue_signal(
        &self,
        vk_timeline_semaphore: diligent_sys::VkSemaphore,
        value: u64,
    ) {
        unsafe_member_call!(
            self,
            CommandQueueVk,
            EnqueueSignal,
            vk_timeline_semaphore,
            value
        )
    }
}
