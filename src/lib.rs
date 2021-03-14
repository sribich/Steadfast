use crate::vulkan::api::{_VkResult, VkResult};
use crate::vulkan::loader::LoaderError;

pub mod device;
#[macro_use]
pub mod extensions;
pub mod instance;
pub mod vulkan;
pub mod window;

#[derive(Debug)]
#[repr(i32)]
enum Success {
    Success    = VkResult::SUCCESS as i32,
    NotReady   = VkResult::NOT_READY as i32,
    Timeout    = VkResult::TIMEOUT as i32,
    EventSet   = VkResult::EVENT_SET as i32,
    EventReset = VkResult::EVENT_RESET as i32,
    Incomplete = VkResult::INCOMPLETE as i32,
    Suboptimal = VkResult::SUBOPTIMAL_KHR as i32,
}

#[derive(Debug)]
#[repr(i32)]
pub enum Error {
    OutOfHostMemory         = VkResult::ERROR_OUT_OF_HOST_MEMORY as i32,
    OutOfDeviceMemory       = VkResult::ERROR_OUT_OF_DEVICE_MEMORY as i32,
    InitializationFailed    = VkResult::ERROR_INITIALIZATION_FAILED as i32,
    DeviceLost              = VkResult::ERROR_DEVICE_LOST as i32,
    MemoryMapFailed         = VkResult::ERROR_MEMORY_MAP_FAILED as i32,
    LayerNotPresent         = VkResult::ERROR_LAYER_NOT_PRESENT as i32,
    ExtensionNotPresent     = VkResult::ERROR_EXTENSION_NOT_PRESENT as i32,
    FeatureNotPresent       = VkResult::ERROR_FEATURE_NOT_PRESENT as i32,
    IncompatibleDriver      = VkResult::ERROR_INCOMPATIBLE_DRIVER as i32,
    TooManyObjects          = VkResult::ERROR_TOO_MANY_OBJECTS as i32,
    FormatNotSupported      = VkResult::ERROR_FORMAT_NOT_SUPPORTED as i32,
    SurfaceLost             = VkResult::ERROR_SURFACE_LOST_KHR as i32,
    NativeWindowInUse       = VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR as i32,
    OutOfDate               = VkResult::ERROR_OUT_OF_DATE_KHR as i32,
    IncompatibleDisplay     = VkResult::ERROR_INCOMPATIBLE_DISPLAY_KHR as i32,
    ValidationFailed        = VkResult::ERROR_VALIDATION_FAILED_EXT as i32,
    OutOfPoolMemory         = VkResult::ERROR_OUT_OF_POOL_MEMORY_KHR as i32,
    FullscreenExclusiveLost = VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT as i32,
}
type VulkanError = Error;

impl std::error::Error for Error {
    /*#[inline]
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LoadingError::LibraryLoadFailure(ref err) => Some(err),
            _ => None
        }
    }*/
}

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            match *self {
                _ => ""
            }
        )
    }
}

fn check_errors(result: _VkResult) -> Result<Success, Error> {
    match VkResult::from_raw(result) {
        VkResult::SUCCESS => Ok(Success::Success),
        VkResult::NOT_READY => Ok(Success::NotReady),
        VkResult::TIMEOUT => Ok(Success::Timeout),
        VkResult::EVENT_SET => Ok(Success::EventSet),
        VkResult::EVENT_RESET => Ok(Success::EventReset),
        VkResult::INCOMPLETE => Ok(Success::Incomplete),
        VkResult::ERROR_OUT_OF_HOST_MEMORY => Err(Error::OutOfHostMemory),
        VkResult::ERROR_OUT_OF_DEVICE_MEMORY => Err(Error::OutOfDeviceMemory),
        VkResult::ERROR_INITIALIZATION_FAILED => Err(Error::InitializationFailed),
        VkResult::ERROR_DEVICE_LOST => Err(Error::DeviceLost),
        VkResult::ERROR_MEMORY_MAP_FAILED => Err(Error::MemoryMapFailed),
        VkResult::ERROR_LAYER_NOT_PRESENT => Err(Error::LayerNotPresent),
        VkResult::ERROR_EXTENSION_NOT_PRESENT => Err(Error::ExtensionNotPresent),
        VkResult::ERROR_FEATURE_NOT_PRESENT => Err(Error::FeatureNotPresent),
        VkResult::ERROR_INCOMPATIBLE_DRIVER => Err(Error::IncompatibleDriver),
        VkResult::ERROR_TOO_MANY_OBJECTS => Err(Error::TooManyObjects),
        VkResult::ERROR_FORMAT_NOT_SUPPORTED => Err(Error::FormatNotSupported),
        VkResult::ERROR_SURFACE_LOST_KHR => Err(Error::SurfaceLost),
        VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(Error::NativeWindowInUse),
        VkResult::SUBOPTIMAL_KHR => Ok(Success::Suboptimal),
        VkResult::ERROR_OUT_OF_DATE_KHR => Err(Error::OutOfDate),
        VkResult::ERROR_INCOMPATIBLE_DISPLAY_KHR => Err(Error::IncompatibleDisplay),
        VkResult::ERROR_VALIDATION_FAILED_EXT => Err(Error::ValidationFailed),
        VkResult::ERROR_OUT_OF_POOL_MEMORY_KHR => Err(Error::OutOfPoolMemory),
        VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => Err(Error::FullscreenExclusiveLost),
        VkResult::ERROR_INVALID_SHADER_NV => panic!(
            "Vulkan function returned \
                                               VK_ERROR_INVALID_SHADER_NV"
        ),
        c => unreachable!("Unexpected error code returned by Vulkan: {:?}", c),
    }
}
