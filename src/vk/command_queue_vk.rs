use std::ops::Deref;

use crate::command_queue::CommandQueue;

pub struct CommandQueueVk<'a> {
    sys_ptr: *mut diligent_sys::ICommandQueueVk,
    virtual_functions: *mut diligent_sys::ICommandQueueVkVtbl,

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
            sys_ptr: value.sys_ptr as *mut diligent_sys::ICommandQueueVk,
            virtual_functions: unsafe {
                (*(value.sys_ptr as *mut diligent_sys::ICommandQueueVk)).pVtbl
            },
        }
    }
}

impl CommandQueueVk<'_> {
    pub fn submit_cmd_buffer(&self, cmd_buffer: diligent_sys::VkCommandBuffer) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .SubmitCmdBuffer
                .unwrap_unchecked()(self.sys_ptr, cmd_buffer)
        }
    }

    pub fn submit(&self, submit_info: &diligent_sys::VkSubmitInfo) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .Submit
                .unwrap_unchecked()(self.sys_ptr, submit_info)
        }
    }

    pub fn present(&self, present_info: &diligent_sys::VkPresentInfoKHR) -> diligent_sys::VkResult {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .Present
                .unwrap_unchecked()(self.sys_ptr, present_info)
        }
    }

    pub fn bind_sparse(&self, bind_sparse_info: &diligent_sys::VkBindSparseInfo) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .BindSparse
                .unwrap_unchecked()(self.sys_ptr, bind_sparse_info)
        }
    }

    pub fn get_vk_queue(&self) -> diligent_sys::VkQueue {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .GetVkQueue
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_queue_family_index(&self) -> u32 {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .GetQueueFamilyIndex
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn enqueue_signal_fence(&self, vk_fence: diligent_sys::VkFence) {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .EnqueueSignalFence
                .unwrap_unchecked()(self.sys_ptr, vk_fence)
        }
    }

    pub fn enqueue_signal(&self, vk_timeline_semaphore: diligent_sys::VkSemaphore, value: u64) {
        unsafe {
            (*self.virtual_functions)
                .CommandQueueVk
                .EnqueueSignal
                .unwrap_unchecked()(self.sys_ptr, vk_timeline_semaphore, value)
        }
    }
}
