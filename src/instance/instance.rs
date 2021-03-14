use crate::vulkan::loader;
use std::mem::MaybeUninit;
use crate::vulkan;
use crate::vulkan::loader::{LoaderError, vulkan_loader};
use crate::vulkan::api::{VkInstance, VkInstanceCreateInfo, VkStructureType};
use crate::{check_errors, Error};
use std::sync::Arc;
use crate::instance::extensions::InstanceExtensions;

/// A thin wrapper around the Vulkan context (instance), which contains
/// application state information like the Vulkan API version you're using,
/// your application's name, and which extensions and layers you want to
/// enable.
///
/// [`blah`]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#initialization
pub struct Instance {
    extensions: InstanceExtensions,
    instance:   VkInstance,
    vulkan:     vulkan::api::InstancePointers,
}

impl Instance {
    /// Instantiates a new instance of the Vulkan wrapper
    ///
    /// # Todo
    ///
    ///   - VkApplicationInfo
    ///
    /// # Example
    ///
    /// ```
    /// use steadfast_vulkan::instance::instance::Instance;
    /// use steadfast_vulkan::instance::instance::InstanceError;
    /// use steadfast_vulkan::instance::extensions::InstanceExtensions;
    ///
    /// fn main() -> Result<(), InstanceError> {
    ///     match Instance::new(InstanceExtensions::supported()?) {
    ///         Ok(_)    => Ok(()),
    ///         Err(err) => panic!("Failed to create Vulkan wrapper: {:?}", err)
    ///     }
    /// }
    /// ```
    pub fn new(extensions: InstanceExtensions) -> Result<Arc<Instance>, InstanceError> {
        let mut output = MaybeUninit::uninit();

        let instance = unsafe {
            let info = VkInstanceCreateInfo {
                sType: VkStructureType::STRUCTURE_TYPE_INSTANCE_CREATE_INFO as u32,
                pNext: std::ptr::null(),
                flags: 0,
                pApplicationInfo: std::ptr::null(),
                enabledLayerCount: 0,
                ppEnabledLayerNames: std::ptr::null(),
                enabledExtensionCount: 0,
                ppEnabledExtensionNames: std::ptr::null(),
            };

            loader::vulkan_loader()?.entry_points().vkCreateInstance(&info, std::ptr::null(), output.as_mut_ptr());
            output.assume_init()
        };

        let vulkan = {
            let loader = vulkan_loader()?;

            vulkan::api::InstancePointers::load(|name| {
                loader.get_instance_proc_addr(instance, name.as_ptr()) as *const std::ffi::c_void
            })
        };

        Ok(Arc::new(Instance {
            extensions,
            instance,
            vulkan,
        }))
    }

    /// Returns the Vulkan instance version formatted as defined by
    /// the [`spec`].
    ///
    /// [`spec`]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers
    ///
    pub fn version() -> Result<String, InstanceError> {
        let mut version = 0;

        let result = vulkan_loader()?.entry_points().vkEnumerateInstanceVersion(&mut version);
        check_errors(result)?;

        Ok(format!("{}.{}.{}", version >> 22, (version >> 12) & 0x3ff, version & 0xfff))
    }

    pub fn extensions(&self) -> InstanceExtensions {
        self.extensions
    }

    pub fn vulkan(&self) -> &vulkan::api::InstancePointers {
        &self.vulkan
    }

    pub fn handle(&self) -> VkInstance {
        self.instance
    }
}

#[derive(Debug)]
pub enum InstanceError {
    LibraryError(LoaderError),
    VulkanError(Error),
}

impl From<Error> for InstanceError {
    fn from(err: Error) -> Self {
        InstanceError::VulkanError(err)
    }
}

impl From<LoaderError> for InstanceError {
    fn from(err: LoaderError) -> Self {
        InstanceError::LibraryError(err)
    }
}