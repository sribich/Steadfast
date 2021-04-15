use crate::api::{VkSurfaceKHR, VK_QUEUE_GRAPHICS_BIT};
use crate::device::PhysicalDevice;
use crate::extensions::DeviceExtensions;
use crate::Check;
use std::ffi::CStr;
use std::mem::MaybeUninit;

type DeviceDiscriminator = fn(&PhysicalDevice, &DeviceRequirements) -> u32;

///
///
pub struct DevicePicker {
    pub discriminator: DeviceDiscriminator,
}

impl DevicePicker {
    pub fn new(discriminator: DeviceDiscriminator) -> Self {
        Self { discriminator }
    }

    pub fn pick(&self, physical_devices: Vec<PhysicalDevice>, device_requirements: &DeviceRequirements) -> Option<PhysicalDevice> {
        let mut best_device = (0, None);

        for device in physical_devices.iter() {
            let score = self.score(device, device_requirements);

            if score > best_device.0 {
                best_device = (score, Some(device.clone()));
            }
        }

        best_device.1
    }

    pub fn score(&self, physical_device: &PhysicalDevice, device_requirements: &DeviceRequirements) -> u32 {
        (self.discriminator)(physical_device, device_requirements)
    }

    pub fn queue_requirements(physical_device: &PhysicalDevice, device_requirements: &DeviceRequirements) -> bool {
        let mut meets_requirements = true;

        if device_requirements.queues.has_graphics {
            meets_requirements = physical_device
                .queue_families
                .iter()
                .any(|family| family.properties.queueFlags & VK_QUEUE_GRAPHICS_BIT == 0);
        }

        if let Some(surface) = device_requirements.queues.can_present {
            meets_requirements = physical_device.queue_families.iter().enumerate().any(|(i, family)| {
                let mut output = MaybeUninit::uninit();

                println!("{}", surface);

                let result = physical_device
                    .instance
                    .vulkan()
                    .vkGetPhysicalDeviceSurfaceSupportKHR(physical_device.device, i as u32, surface, output.as_mut_ptr())
                    .check();

                if let Err(e) = result {
                    false
                } else {
                    unsafe { output.assume_init() != 0 }
                }
            });
        }

        meets_requirements
    }
}

impl Default for DevicePicker {
    fn default() -> Self {
        Self {
            discriminator: |physical_device, device_requirements| {
                let (mut score, mut multiplier) = (0, 1);

                if device_requirements
                    .device_extensions
                    .intersection(&physical_device.extensions)
                    .enabled
                    .len()
                    != device_requirements.device_extensions.enabled.len()
                {
                    multiplier = 0;
                } else {
                    score += 1;
                }

                if !DevicePicker::queue_requirements(physical_device, device_requirements) {
                    multiplier = 0;
                }

                //
                score * multiplier
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueueRequirements {
    pub has_graphics: bool,

    pub can_present: Option<VkSurfaceKHR>,
}

impl Default for QueueRequirements {
    fn default() -> Self {
        Self {
            has_graphics: false,
            can_present: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicalDeviceTypeMultiplier {
    pub other: (u32, f32),
    pub integrated_gpu: (u32, f32),
    pub discrete_gpu: (u32, f32),
    pub virtual_gpu: (u32, f32),
    pub cpu: (u32, f32),
}

impl Default for PhysicalDeviceTypeMultiplier {
    fn default() -> Self {
        Self {
            other: (1, 1.0),
            integrated_gpu: (1, 1.0),
            discrete_gpu: (1, 1.0),
            virtual_gpu: (1, 1.0),
            cpu: (1, 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceRequirements {
    pub physical_device_type_multiplier: PhysicalDeviceTypeMultiplier,

    pub queues: QueueRequirements,
    pub device_extensions: DeviceExtensions,
}

impl DeviceRequirements {}

impl Default for DeviceRequirements {
    fn default() -> Self {
        Self {
            physical_device_type_multiplier: PhysicalDeviceTypeMultiplier::default(),
            queues: QueueRequirements::default(),
            device_extensions: DeviceExtensions::none(),
        }
    }
}
