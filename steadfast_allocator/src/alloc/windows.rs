use crate::alloc::SteadfastAllocator;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

unsafe impl GlobalAlloc for SteadfastAllocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        _aligned_malloc(layout.size(), layout.align()) as *mut u8
    }

    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        _aligned_free(ptr as *mut c_void)
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
        _aligned_realloc(ptr as *mut c_void, new_size, layout.align()) as *mut u8
    }
}

extern "C" {
    pub fn _aligned_malloc(size: usize, align: usize) -> *mut c_void;
    pub fn _aligned_realloc(p: *mut c_void, size: usize, align: usize) -> *mut c_void;
    pub fn _aligned_free(p: *mut c_void);
}
