use std::ops::Deref;

use crate::command_queue::CommandQueue;

#[repr(transparent)]
pub struct CommandQueueVk<'a> {
    command_queue: &'a CommandQueue<'a>,
}

impl<'a> Deref for CommandQueueVk<'a> {
    type Target = CommandQueue<'a>;
    fn deref(&self) -> &Self::Target {
        self.command_queue
    }
}

impl<'a> From<&'a CommandQueue<'a>> for CommandQueueVk<'a> {
    fn from(value: &'a CommandQueue) -> Self {
        CommandQueueVk {
            command_queue: value,
        }
    }
}

impl CommandQueueVk<'_> {
    pub fn submit_cmd_buffer(&self, cmd_buffer: diligent_sys::VkCommandBuffer) -> u64 {
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
        unsafe_member_call!(self, CommandQueueVk, GetVkQueue,)
    }

    pub fn get_queue_family_index(&self) -> u32 {
        unsafe_member_call!(self, CommandQueueVk, GetQueueFamilyIndex,)
    }

    pub fn enqueue_signal_fence(&self, vk_fence: diligent_sys::VkFence) {
        unsafe_member_call!(self, CommandQueueVk, EnqueueSignalFence, vk_fence)
    }

    pub fn enqueue_signal(&self, vk_timeline_semaphore: diligent_sys::VkSemaphore, value: u64) {
        unsafe_member_call!(
            self,
            CommandQueueVk,
            EnqueueSignal,
            vk_timeline_semaphore,
            value
        )
    }
}
