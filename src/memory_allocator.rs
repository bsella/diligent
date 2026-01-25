use std::os::raw::c_void;

define_ported!(
    MemoryAllocator,
    diligent_sys::IMemoryAllocator,
    diligent_sys::IMemoryAllocatorMethods : 4
);

pub struct MemoryAllocation<'allocator> {
    allocator: &'allocator MemoryAllocator,
    sys_ptr: *mut c_void,
}

impl Drop for MemoryAllocation<'_> {
    fn drop(&mut self) {
        unsafe {
            (*self.allocator.0.pVtbl)
                .MemoryAllocator
                .Free
                .unwrap_unchecked()(
                std::ptr::from_ref(&self.allocator.0) as _, self.sys_ptr
            )
        };
    }
}

pub struct AlignedMemoryAllocation<'allocator> {
    allocator: &'allocator MemoryAllocator,
    sys_ptr: *mut c_void,
}

impl Drop for AlignedMemoryAllocation<'_> {
    fn drop(&mut self) {
        unsafe {
            (*self.allocator.0.pVtbl)
                .MemoryAllocator
                .FreeAligned
                .unwrap_unchecked()(
                std::ptr::from_ref(&self.allocator.0) as _, self.sys_ptr
            )
        };
    }
}

impl MemoryAllocator {
    pub fn allocate(&self, size: usize) -> MemoryAllocation<'_> {
        let mem_ptr = unsafe {
            (*self.0.pVtbl).MemoryAllocator.Allocate.unwrap_unchecked()(
                std::ptr::from_ref(&self.0) as _,
                size,
                // TODO
                std::ptr::null(),
                std::ptr::null(),
                line!() as _,
            )
        };

        MemoryAllocation {
            allocator: self,
            sys_ptr: mem_ptr,
        }
    }

    pub fn allocate_aligned(&self, size: usize, alignment: usize) -> MemoryAllocation<'_> {
        let mem_ptr = unsafe {
            (*self.0.pVtbl)
                .MemoryAllocator
                .AllocateAligned
                .unwrap_unchecked()(
                std::ptr::from_ref(&self.0) as _,
                size,
                alignment,
                // TODO
                std::ptr::null(),
                std::ptr::null(),
                line!() as _,
            )
        };

        MemoryAllocation {
            allocator: self,
            sys_ptr: mem_ptr,
        }
    }
}
