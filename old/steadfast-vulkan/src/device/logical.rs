use crate::device::{DeviceRequirements, PhysicalDevice};
use crate::{DevicePicker, Instance, Window};
use anyhow::Result;
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct LogicalDevice {
    instance: Arc<Instance>,
    requirements: RefCell<DeviceRequirements>,
    // pub api: DevicePointers,
}

impl LogicalDevice {
    pub fn new(instance: Arc<Instance>) -> Self {
        LogicalDevice {
            instance,
            requirements: RefCell::new(DeviceRequirements::default()),
        }
    }
}

impl LogicalDevice {
    pub fn recreate_device(&self) {
        let devices = PhysicalDevice::enumerate(&self.instance).unwrap();

        for device in devices {
            let score = DevicePicker::default().score(&device, &self.requirements.borrow_mut());

            println!("{}", score);
        }

        // let mut device = self.device.borrow_mut();
        // let physical_device = self.select_device().unwrap().unwrap();

        // *device = Some(LogicalDevice::new())
    }

    fn adjust_requirements(&self, func: impl Fn(&mut DeviceRequirements) -> ()) {
        let mut requirements = self.requirements.borrow_mut();

        func(&mut requirements)
    }

    pub fn window(&self) -> Result<Box<Window>> {
        let window = Window::new(&self.instance);

        self.adjust_requirements(|requirements| {
            requirements.queues.has_graphics = true;
            requirements.queues.can_present = Some(window.surface.handle());
        });

        // window.select_device(Some(DevicePicker::default()));

        // let id = context.window.window.id();
        // let mut windows = self.windows.borrow_mut();

        // windows.insert(id, Rc::new(context));

        // Ok(windows.get(&id).unwrap().clone())

        Ok(Box::new(window))
    }

    //

    //
    // pub fn create_swapchain(&self) {
    //     self.adjust_requirements(|mut requirements| {
    //         requirements.device_extensions = DeviceExtensions {
    //             VK_KHR_swapchain: true,
    //             ..requirements.device_extensions.clone()
    //         };
    //     });
    //
    //     self.recreate_device();
    // }
}

// struct DeviceRequirements {}
//
// trait DeviceSelector {
//     fn create_logical_device(&self) -> LogicalDevice;
//     fn select_physical_device(&self) -> PhysicalDevice;
//
//     fn set_device_requirements(device_requirements: DeviceRequirements);
// }
//
// impl DeviceSelector for Instance {
//     fn create_logical_device(&self) -> LogicalDevice {
//         // self.vulkan().vkCreateDevice
//
//         LogicalDevice {}
//     }
//
//     fn select_physical_device(&self) -> PhysicalDevice {
//         PhysicalDevice::enumerate(self).unwrap().first().unwrap()
//     }
//
//     fn set_device_requirements(device_requirements: DeviceRequirements) {}
// }

// let device_discriminator = |device: PhysicalDevice| -> u32 {
// let mut score = 0;
// let mut multiplier = 1;
//
// score = match device.properties.deviceType {
// VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => 1000,
// VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => 500,
// };
//
// // device.properties.deviceType == VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
// score = 1;
//
// score * multiplier
// };
