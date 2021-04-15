//! Devices and Queues are the primary objects used to interface with
//! the Vulkan API.
//!
//! Vulkan separates the concept of _physical_ and _logical_ devices.
//!
//! A physical device represents a device in the system that is able
//! to perform work. Each of these devices have their own features
//! and capabilities, of which may be optionally enabled.
//!
//! Physical devices also contain discrete queue families, which are
//! dedicated pipelines in the GPU used to provide graphics, compute,
//! and transfer capabilities. [`command buffers`] are submitted to
//! queues in order to perform work.
//!
//! A logical device is an active instance of a physical device bound
//! to the Vulkan implementation. A logical device contains enabled
//! layers, features, and extensions along with the enabled queues.
//!
//! # Examples
//!
//!

mod logical;
mod physical;
mod queue;
mod requirements;

pub use logical::*;
pub use physical::*;
pub use queue::*;
pub use requirements::*;
