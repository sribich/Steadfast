use crate::SteadfastAllocator;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

unsafe impl GlobalAlloc for SteadfastAllocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = core::ptr::null_mut();

        let result = posix_memalign(
            &mut ptr,
            layout.align().max(core::mem::size_of::<usize>()),
            layout.size(),
        );

        if result == 0 {
            ptr as *mut u8
        } else {
            core::ptr::null_mut()
        }
    }

    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr as *mut c_void)
    }

    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = self.alloc(layout);

        if !ptr.is_null() {
            core::ptr::write_bytes(ptr, 0, layout.size());
        }

        ptr
    }

    #[inline(always)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        realloc(ptr as *mut c_void, new_size) as *mut u8
    }
}

extern "C" {
    pub fn free(p: *mut c_void);
    pub fn posix_memalign(memptr: *mut *mut c_void, align: usize, size: usize) -> u32;
    pub fn realloc(p: *mut c_void, size: usize) -> *mut c_void;
}
