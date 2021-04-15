#![rustfmt::skip]

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;

pub type VkDispatchableHandle = usize;
pub type VkNonDispatchableHandle = u64;

pub type VkInstance = VkDispatchableHandle;

pub type PFN_vkVoidFunction = *const c_void;
