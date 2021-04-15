use crate::api::VkSwapchainKHR;
use crate::device::LogicalDevice;
use std::sync::Arc;

pub struct Swapchain {
    device: Arc<LogicalDevice>,

    swapchain: VkSwapchainKHR,
    // images: Vec<VkImage>,
}

impl Swapchain {
    // fn new() -> Self {}
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        // self.device
        //     .api
        //     .vkDestroySwapchainKHR(self.device.handle(), self.swapchain, std::ptr::null());
    }
}
