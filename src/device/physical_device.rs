use crate::vulkan::api::{VkInstance, VkPhysicalDevice, VkPhysicalDeviceProperties};
use std::sync::Arc;
use crate::instance::instance::{Instance, InstanceError};
use crate::{check_errors, VulkanError};
use std::slice::Iter;
use std::ffi::CString;
use std::ops::Deref;
use std::mem::MaybeUninit;

pub struct PhysicalDevice<'a> {
    instance: &'a Arc<VkInstance>,
    device: u32,
}

pub struct PhysicalDeviceInfo {}

impl<'a> PhysicalDevice<'a> {
    /// Retrieves the list of physical device handles representing the physical
    /// devices installed in the system.
    ///
    /// # Vulkan Reference
    ///
    ///  [Physical Devices]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#devsandqueues-physical-device-enumeration
    ///
    /// # Example
    ///
    /// ```
    /// use steadfast_vulkan::device::physical_device::PhysicalDevice;
    /// use steadfast_vulkan::instance::extensions::InstanceExtensions;
    /// use steadfast_vulkan::instance::instance::Instance;
    /// use steadfast_vulkan::instance::instance::InstanceError;
    /// use steadfast_vulkan::vulkan::api::VkPhysicalDevice;
    ///
    /// fn main() -> Result<(), InstanceError> {
    ///     let instance       = Instance::new(InstanceExtensions::supported()?)?;
    ///     let device_handles = PhysicalDevice::handles(&instance)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn handles(instance: &'a Arc<Instance>) -> Result<Vec<VkPhysicalDevice>, VulkanError> {
        let mut num = 0;

        let result = instance.vulkan().vkEnumeratePhysicalDevices(instance.handle(), &mut num, std::ptr::null_mut());
        check_errors(result)?;

        let mut devices = Vec::with_capacity(num as usize);
        let result = instance.vulkan().vkEnumeratePhysicalDevices(instance.handle(), &mut num, devices.as_mut_ptr());
        check_errors(result)?;

        unsafe {
            devices.set_len(num as usize);
        }

        Ok(devices)
    }

    ///
    ///
    pub fn init_physical_devices(instance: &'a Arc<Instance>, physical_devices: Vec<VkPhysicalDevice>) {
        let mut output = Vec::with_capacity(physical_devices.len());

        for device in physical_devices {
            let properties = unsafe {
                let mut output = MaybeUninit::uninit();
                instance.vulkan().vkGetPhysicalDeviceProperties(device, output.as_mut_ptr());
                output.assume_init()
            };

            output.push(properties)
        }
    }

    // pub fn test() {
    //     let vk_khr_get_physical_device_properties2 = CString::new(b"VK_KHR_get_physical_device_properties2".to_vec()).unwrap();
    //
    //     let physical_devices = if instance.extensions().strings()?.iter().any(|v| (**v) == vk_khr_get_physical_device_properties2) {
    //
    //     } else {
    //
    //     };
    //
    //     // instance.extensions()
    //
    //     // Ok(devices.iter())
    //     Ok(())
    // }

    pub fn highest_score(instance: &'a Arc<Instance>, scorer: fn() -> u32) {
        instance.vulkan();
        ()
    }

    fn init_physical_devices_2(physical_devices: Vec<VkPhysicalDevice>) -> Vec<PhysicalDeviceInfo> {
        let mut output = Vec::with_capacity(physical_devices.len());

        for device in physical_devices {
            // let mut extended_properties =


        }

        output
    }
}
