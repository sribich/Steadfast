#![no_std]

pub mod arena;

pub(crate) mod align;
pub(crate) mod layout;

mod alloc;

pub use crate::alloc::SteadfastAllocator;

#[global_allocator]
static ALLOCATOR: SteadfastAllocator = SteadfastAllocator;
