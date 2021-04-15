//! The swapchain is a link between Vulkan and the screen.
//!
//!

mod surface;
mod swapchain;

pub use surface::*;
pub use swapchain::*;

// Do you want to minimize latency? Use mailbox.
//
// Do you want to minimize stuttering? Use relaxed FIFO.
//
// Do you want to minimize power consumption? Fall back to regular FIFO.
//
// Are you unsure? Leave it up to the user.
