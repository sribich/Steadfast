#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "windows")]
mod windows;

pub struct SteadfastAllocator;
