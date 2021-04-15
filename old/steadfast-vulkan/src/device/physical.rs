use crate::api::{
    VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceFeatures2, VkPhysicalDeviceLimits,
    VkPhysicalDeviceProperties, VkPhysicalDeviceProperties2, VkPhysicalDeviceSparseProperties,
    VkQueueFamilyProperties2, VK_INCOMPLETE, VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2, VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2,
};
use crate::{Check, DeviceExtensions, Instance, QueueFamily, Version, VulkanError};
use anyhow::Result;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PhysicalDevice {
    pub instance: Arc<Instance>,
    pub device: VkPhysicalDevice,
    pub properties: PhysicalDeviceProperties,
    pub features: VkPhysicalDeviceFeatures,
    pub queue_families: Vec<QueueFamily>,
    pub extensions: DeviceExtensions,
}

impl PhysicalDevice {
    /// Retrieves a list of `PhysicalDevice` objects
    ///
    /// # Example
    ///
    /// ```
    /// use steadfast_vulkan::{Instance, InstanceError, InstanceExtensions, PhysicalDevice};
    /// use steadfast_vulkan::api::VkPhysicalDevice;
    ///
    /// fn main() -> Result<(), InstanceError> {
    ///     let instance = Instance::new(None, InstanceExtensions::default()?)?;
    ///     let devices  = PhysicalDevice::enumerate(&instance)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn enumerate(instance: &Arc<Instance>) -> Result<Vec<PhysicalDevice>, VulkanError> {
        PhysicalDevice::handles(&instance)?
            .iter()
            .map(|device| PhysicalDevice::new(instance, *device))
            .collect()
    }

    /// Retrieves the list of [`Physical Device`] handles representing the
    /// discrete physical devices present in the system.
    ///
    /// # Panics
    ///
    /// This function panics when vkEnumeratePhysicalDevices does not fully
    /// populate the `devices` vector. We are unable to guarantee memory
    /// safety in this case and must panic.
    ///
    /// [`Physical Devices`]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#devsandqueues-physical-device-enumeration
    fn handles(instance: &Arc<Instance>) -> Result<Vec<VkPhysicalDevice>, VulkanError> {
        let mut num = 0;

        instance
            .vulkan()
            .vkEnumeratePhysicalDevices(instance.handle(), &mut num, std::ptr::null_mut())
            .check()?;

        let mut devices = Vec::with_capacity(num as usize);

        let result = instance
            .vulkan()
            .vkEnumeratePhysicalDevices(instance.handle(), &mut num, devices.as_mut_ptr())
            .check()?;

        if result == VK_INCOMPLETE || devices.capacity() != num as usize {
            panic!(
                "Received fewer ({}) than expected ({}) devices from vkEnumeratePhysicalDevices",
                num,
                devices.capacity()
            );
        }

        unsafe {
            devices.set_len(num as usize);
        }

        Ok(devices)
    }
}

impl PhysicalDevice {
    pub fn new(
        instance: &Arc<Instance>,
        physical_device: VkPhysicalDevice,
    ) -> Result<PhysicalDevice> {
        if instance
            .vulkan()
            .has_function("vkGetPhysicalDeviceProperties2")
        {
            PhysicalDevice::new_extended(instance, physical_device)
        } else {
            PhysicalDevice::new_legacy(instance, physical_device)
        }
    }

    pub fn new_legacy(
        instance: &Arc<Instance>,
        physical_device: VkPhysicalDevice,
    ) -> Result<PhysicalDevice> {
        let properties = PhysicalDeviceProperties::new({
            let mut output = MaybeUninit::uninit();

            instance
                .vulkan()
                .vkGetPhysicalDeviceProperties(physical_device, output.as_mut_ptr());

            unsafe { output.assume_init() }
        });

        let queue_families: Vec<QueueFamily> = {
            let mut num = 0;

            instance.vulkan().vkGetPhysicalDeviceQueueFamilyProperties(
                physical_device,
                &mut num,
                std::ptr::null_mut(),
            );

            let mut queue_families = Vec::with_capacity(num as usize);

            instance.vulkan().vkGetPhysicalDeviceQueueFamilyProperties(
                instance.handle(),
                &mut num,
                queue_families.as_mut_ptr(),
            );

            unsafe {
                queue_families.set_len(num as usize);
            }

            queue_families
                .iter()
                .map(|q| QueueFamily::new((*q).clone()))
                .collect()
        };

        let features = {
            let mut output = MaybeUninit::uninit();

            instance
                .vulkan()
                .vkGetPhysicalDeviceFeatures(physical_device, output.as_mut_ptr());

            unsafe { output.assume_init() }
        };

        Ok(PhysicalDevice {
            instance: (*instance).clone(),
            device: physical_device,
            properties,
            features,
            queue_families,
            extensions: DeviceExtensions::from_physical_device(instance, physical_device)?,
        })
    }

    pub fn new_extended(
        instance: &Arc<Instance>,
        physical_device: VkPhysicalDevice,
    ) -> Result<PhysicalDevice> {
        let properties = PhysicalDeviceProperties::new({
            let mut output = VkPhysicalDeviceProperties2 {
                sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2,
                pNext: std::ptr::null_mut(),
                properties: unsafe { std::mem::zeroed() },
            };

            instance
                .vulkan()
                .vkGetPhysicalDeviceProperties2(physical_device, &mut output);

            output.properties
        });

        let features = unsafe {
            let mut output = VkPhysicalDeviceFeatures2 {
                sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
                pNext: std::ptr::null_mut(),
                features: std::mem::zeroed(),
            };

            instance
                .vulkan()
                .vkGetPhysicalDeviceFeatures2(physical_device, &mut output);

            output.features
        };

        let queue_families = unsafe {
            let mut num = 0;

            instance.vulkan().vkGetPhysicalDeviceQueueFamilyProperties2(
                physical_device,
                &mut num,
                std::ptr::null_mut(),
            );

            let mut queue_families = (0..num)
                .map(|_| VkQueueFamilyProperties2 {
                    sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2,
                    pNext: std::ptr::null_mut(),
                    queueFamilyProperties: std::mem::zeroed(),
                })
                .collect::<Vec<_>>();

            instance.vulkan().vkGetPhysicalDeviceQueueFamilyProperties2(
                physical_device,
                &mut num,
                queue_families.as_mut_ptr(),
            );

            queue_families
                .into_iter()
                .map(|family| family.queueFamilyProperties)
                .map(QueueFamily::new)
                .collect::<Vec<QueueFamily>>()
        };

        Ok(PhysicalDevice {
            instance: (*instance).clone(),
            device: physical_device,
            properties,
            features,
            queue_families,
            extensions: DeviceExtensions::from_physical_device(instance, physical_device)?,
        })
    }

    /*
    ///
    ///
    pub fn get_device_propertiess(
        instance: Arc<Instance>,
        physical_device: VkPhysicalDevice,
    ) -> Result<(), InstanceError> {
        let vk_khr_get_physical_device_properties2 =
            CString::new("VK_KHR_get_physical_device_properties2").unwrap();

        let physical_devices = if instance
            .extensions()
            .strings()?
            .iter()
            .any(|v| (**v) == vk_khr_get_physical_device_properties2)
        {
            // let x = PhysicalDevice::init_physical_device(instance, physical_device);

            // println!("{}", x.apiVersion);
            // println!("{}", x.vendorID);
            // println!("{}", 0x1003);
        } else {
        };

        Ok(())
    }*/

    pub fn get_device_properties(
        instance: Arc<Instance>,
        physical_device: VkPhysicalDevice,
    ) -> VkPhysicalDeviceProperties {
        let mut output = MaybeUninit::uninit();

        instance
            .vulkan()
            .vkGetPhysicalDeviceProperties(physical_device, output.as_mut_ptr());

        // let queue_families = {
        //     let mut num = 0;
        //
        //     let result = instance.vulkan().vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut num, std::ptr::null_mut());
        //     check_errors(result)?;
        //
        //     let mut queue_families = Vec::with_capacity(num as usize);
        //     let result = instance.vulkan().vkGetPhysicalDeviceQueueFamilyProperties(instance.handle(), &mut num, queue_families.as_mut_ptr());
        //     check_errors(result)?;
        //
        //     unsafe {
        //         queue_families.set_len(num as usize);
        //     }
        //
        //     queue_families
        // };

        unsafe { output.assume_init() }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicalDeviceProperties {
    pub api_version: Version,
    pub device_type: PhysicalDeviceType,
    pub device_name: String,
    // pub pipelineCacheUUID: [u8; VK_UUID_SIZE as usize],
    pub limits: VkPhysicalDeviceLimits,
    pub sparse_properties: VkPhysicalDeviceSparseProperties,

    raw: VkPhysicalDeviceProperties,
}

#[derive(Clone, Debug)]
pub enum PhysicalDeviceType {
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
}

impl PhysicalDeviceType {
    fn from_u32(value: u32) -> PhysicalDeviceType {
        match value {
            1 => PhysicalDeviceType::IntegratedGpu,
            2 => PhysicalDeviceType::DiscreteGpu,
            3 => PhysicalDeviceType::VirtualGpu,
            4 => PhysicalDeviceType::Cpu,
            _ => PhysicalDeviceType::Other,
        }
    }
}

impl PhysicalDeviceProperties {
    fn new(properties: VkPhysicalDeviceProperties) -> Self {
        Self {
            api_version: Version::from_vulkan(properties.apiVersion),
            device_type: PhysicalDeviceType::from_u32(properties.deviceType),
            device_name: unsafe {
                CStr::from_ptr(properties.deviceName.as_ptr())
                    .to_str()
                    .unwrap()
                    .to_owned()
            },
            limits: properties.limits.clone(),
            sparse_properties: properties.sparseProperties.clone(),
            raw: properties,
        }
    }
}
