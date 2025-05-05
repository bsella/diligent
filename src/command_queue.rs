use static_assertions::const_assert;

use crate::{device_context::DeviceContext, object::Object};

pub struct CommandQueue<'a> {
    pub(crate) sys_ptr: *mut diligent_sys::ICommandQueue,
    virtual_functions: *mut diligent_sys::ICommandQueueVtbl,

    context: &'a DeviceContext,

    object: Object,
}

impl AsRef<Object> for CommandQueue<'_> {
    fn as_ref(&self) -> &Object {
        &self.object
    }
}

impl Drop for CommandQueue<'_> {
    fn drop(&mut self) {
        unsafe {
            (*self.context.virtual_functions)
                .DeviceContext
                .UnlockCommandQueue
                .unwrap_unchecked()(self.context.sys_ptr)
        };
    }
}

impl<'a> CommandQueue<'a> {
    pub(crate) fn new(context: &'a DeviceContext) -> Result<Self, ()> {
        let command_queue_ptr = unsafe {
            (*context.virtual_functions)
                .DeviceContext
                .LockCommandQueue
                .unwrap_unchecked()(context.sys_ptr)
        };

        // Both base and derived classes have exactly the same size.
        // This means that we can up-cast to the base class without worrying about layout offset between both classes
        const_assert!(
            std::mem::size_of::<diligent_sys::IObject>()
                == std::mem::size_of::<diligent_sys::ICommandQueue>()
        );

        if command_queue_ptr.is_null() {
            Err(())
        } else {
            Ok(CommandQueue {
                sys_ptr: command_queue_ptr,
                virtual_functions: unsafe { (*command_queue_ptr).pVtbl },
                context,
                object: Object::new(command_queue_ptr as *mut diligent_sys::IObject),
            })
        }
    }

    pub fn get_next_fence_value(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .CommandQueue
                .GetNextFenceValue
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn get_completed_fence_value(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .CommandQueue
                .GetCompletedFenceValue
                .unwrap_unchecked()(self.sys_ptr)
        }
    }

    pub fn wait_for_idle(&self) -> u64 {
        unsafe {
            (*self.virtual_functions)
                .CommandQueue
                .WaitForIdle
                .unwrap_unchecked()(self.sys_ptr)
        }
    }
}
