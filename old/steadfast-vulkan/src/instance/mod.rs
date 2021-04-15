//! Vulkan does not maintain any global state for an application, opting instead
//! to store this state within a `VkInstance` object maintained by the application
//! itself.
//!
//! This module acts as a wrapper around the `VkInstance` object.
//!
//! ```
//! use steadfast_vulkan::Instance;
//!
//! let instance = Instance::new(None)
//! ```
//!

mod instance;
mod layers;
mod version;

pub use instance::*;
pub use layers::*;
pub use version::*;
