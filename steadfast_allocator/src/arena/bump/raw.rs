use crate::layout::get_alignment_layout;
use crate::SteadfastAllocator;
use core::alloc::{GlobalAlloc, Layout};

#[derive(Debug)]
pub struct RawBumpArena {
    name: &'static str,

    ptr: *mut u8,
    end: *mut u8,

    layout: Layout,
}

impl RawBumpArena {
    pub fn new(name: &'static str, capacity: usize) -> Self {
        unsafe {
            let layout = get_alignment_layout(capacity);
            let ptr = SteadfastAllocator.alloc(layout);

            if ptr.is_null() {
                panic!("Allocation Failed");
            }

            let end = ptr.offset(layout.size() as isize);

            Self {
                name,
                ptr,
                end,
                layout,
            }
        }
    }

    // pub fn alloc<T>(&self, item: T) -> &mut T {}
}

impl Drop for RawBumpArena {
    fn drop(&mut self) {
        unsafe {
            SteadfastAllocator.dealloc(
                self.ptr,
                Layout::from_size_align_unchecked(
                    self.end as usize - self.ptr as usize,
                    self.layout.align(),
                ),
            )
        }
    }
}
