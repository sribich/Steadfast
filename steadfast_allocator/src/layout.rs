use crate::align::ALIGNMENT;
use core::alloc::Layout;

pub unsafe fn get_alignment_layout(size: usize) -> Layout {
    Layout::from_size_align_unchecked(size, ALIGNMENT)
}
