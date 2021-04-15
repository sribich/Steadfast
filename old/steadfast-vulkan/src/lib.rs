#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate steadfast_vulkan_shaders;
#[macro_use]
extern crate thiserror;

#[macro_use]
mod extensions;

mod debug;
mod device;
mod instance;
mod swapchain;
mod vulkan;
mod window;

use crate::api::*;
use anyhow::Result;
use thiserror::Error;

pub use crate::device::*;
pub use crate::extensions::*;
pub use crate::instance::*;
pub use crate::swapchain::*;
pub use crate::vulkan::*;
pub use crate::window::*;

///
///
trait VulkanHandle {
    fn handle(&self) -> VkHandle;
}

#[derive(Debug)]
#[repr(u32)]
enum Success {
    Success = VK_SUCCESS,
    NotReady = VK_NOT_READY,
    Timeout = VK_TIMEOUT,
    EventSet = VK_EVENT_SET,
    EventReset = VK_EVENT_RESET,
    Incomplete = VK_INCOMPLETE,
    Suboptimal = VK_SUBOPTIMAL_KHR,
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Out of device memory")]
    OutOfDeviceMemory,
    #[error("Out of host memory")]
    OutOfHostMemory,
}

#[derive(Debug, Error)]
#[repr(u32)]
pub enum VulkanError {
    #[error("Out of host memory")]
    OutOfHostMemory = VK_ERROR_OUT_OF_HOST_MEMORY,
    #[error("")]
    OutOfDeviceMemory = VK_ERROR_OUT_OF_DEVICE_MEMORY,
    #[error("")]
    InitializationFailed = VK_ERROR_INITIALIZATION_FAILED,
    #[error("")]
    DeviceLost = VK_ERROR_DEVICE_LOST,
    #[error("")]
    MemoryMapFailed = VK_ERROR_MEMORY_MAP_FAILED,
    #[error("")]
    LayerNotPresent = VK_ERROR_LAYER_NOT_PRESENT,
    #[error("")]
    ExtensionNotPresent = VK_ERROR_EXTENSION_NOT_PRESENT,
    #[error("")]
    FeatureNotPresent = VK_ERROR_FEATURE_NOT_PRESENT,
    #[error("")]
    IncompatibleDriver = VK_ERROR_INCOMPATIBLE_DRIVER,
    #[error("")]
    TooManyObjects = VK_ERROR_TOO_MANY_OBJECTS,
    #[error("")]
    FormatNotSupported = VK_ERROR_FORMAT_NOT_SUPPORTED,
    #[error("")]
    SurfaceLost = VK_ERROR_SURFACE_LOST_KHR,
    #[error("")]
    NativeWindowInUse = VK_ERROR_NATIVE_WINDOW_IN_USE_KHR,
    #[error("")]
    OutOfDate = VK_ERROR_OUT_OF_DATE_KHR,
    #[error("")]
    IncompatibleDisplay = VK_ERROR_INCOMPATIBLE_DISPLAY_KHR,
    #[error("")]
    ValidationFailed = VK_ERROR_VALIDATION_FAILED_EXT,
    #[error("")]
    OutOfPoolMemory = VK_ERROR_OUT_OF_POOL_MEMORY_KHR,
    #[error("")]
    FullscreenExclusiveLost = VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
}

fn check_errors(result: VkResult) -> Result<Success, VulkanError> {
    match result {
        VK_SUCCESS => Ok(Success::Success),
        VK_NOT_READY => Ok(Success::NotReady),
        VK_TIMEOUT => Ok(Success::Timeout),
        VK_EVENT_SET => Ok(Success::EventSet),
        VK_EVENT_RESET => Ok(Success::EventReset),
        VK_INCOMPLETE => Ok(Success::Incomplete),
        VK_ERROR_OUT_OF_HOST_MEMORY => Err(VulkanError::OutOfHostMemory),
        VK_ERROR_OUT_OF_DEVICE_MEMORY => Err(VulkanError::OutOfDeviceMemory),
        VK_ERROR_INITIALIZATION_FAILED => Err(VulkanError::InitializationFailed),
        VK_ERROR_DEVICE_LOST => Err(VulkanError::DeviceLost),
        VK_ERROR_MEMORY_MAP_FAILED => Err(VulkanError::MemoryMapFailed),
        VK_ERROR_LAYER_NOT_PRESENT => Err(VulkanError::LayerNotPresent),
        VK_ERROR_EXTENSION_NOT_PRESENT => Err(VulkanError::ExtensionNotPresent),
        VK_ERROR_FEATURE_NOT_PRESENT => Err(VulkanError::FeatureNotPresent),
        VK_ERROR_INCOMPATIBLE_DRIVER => Err(VulkanError::IncompatibleDriver),
        VK_ERROR_TOO_MANY_OBJECTS => Err(VulkanError::TooManyObjects),
        VK_ERROR_FORMAT_NOT_SUPPORTED => Err(VulkanError::FormatNotSupported),
        VK_ERROR_SURFACE_LOST_KHR => Err(VulkanError::SurfaceLost),
        VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(VulkanError::NativeWindowInUse),
        VK_SUBOPTIMAL_KHR => Ok(Success::Suboptimal),
        VK_ERROR_OUT_OF_DATE_KHR => Err(VulkanError::OutOfDate),
        VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => Err(VulkanError::IncompatibleDisplay),
        VK_ERROR_VALIDATION_FAILED_EXT => Err(VulkanError::ValidationFailed),
        VK_ERROR_OUT_OF_POOL_MEMORY_KHR => Err(VulkanError::OutOfPoolMemory),
        VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => Err(VulkanError::FullscreenExclusiveLost),
        VK_ERROR_INVALID_SHADER_NV => panic!(
            "Vulkan function returned \
                                               VK_ERROR_INVALID_SHADER_NV"
        ),
        c => unreachable!("Unexpected error code returned by Vulkan: {:?}", c),
    }
}

trait Check {
    fn check(self) -> Result<Self, VulkanError>
    where
        Self: Sized;
}

impl Check for VkResult {
    fn check(self) -> Result<Self, VulkanError>
    where
        Self: Sized,
    {
        check_errors(self)?;

        Ok(self)
    }
}
