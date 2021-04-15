use crate::api::VkPhysicalDevice;
use crate::device::{DevicePicker, DeviceRequirements, LogicalDevice, PhysicalDevice};
use crate::extensions::{DeviceExtensions, HasExtensions, InstanceExtensions};
use crate::{check_errors, get_winit_surface, Error, Instance, Surface, VulkanError};
use anyhow::Result;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ffi::CStr;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::{Window as WinitWindow, WindowBuilder};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Window {
    instance: Arc<Instance>,
    window: WinitWindow,
    device: Rc<RefCell<Option<LogicalDevice>>>,
    device_requirements: Rc<RefCell<DeviceRequirements>>,
    pub surface: Arc<Surface>,
}

impl Window {
    fn recreate_device(&self) {
        let mut device = self.device.borrow_mut();
        let physical_device = self.select_device().unwrap().unwrap();

        // *device = Some(LogicalDevice::new())
    }

    fn adjust_requirements(&self, func: fn(&mut DeviceRequirements) -> ()) {
        let mut requirements = self.device_requirements.borrow_mut();

        func(&mut requirements)
    }

    pub fn create_swapchain(&self) {
        self.adjust_requirements(|mut requirements| {
            requirements.device_extensions = DeviceExtensions {
                VK_KHR_swapchain: true,
                ..requirements.device_extensions.clone()
            };
        });

        self.recreate_device();
    }

    pub fn get_logical_device(&self) -> Result<LogicalDevice> {
        Err(VulkanError::ExtensionNotPresent.into())
    }

    pub fn select_device(&self) -> Result<Option<VkPhysicalDevice>> {
        let physical_devices = PhysicalDevice::enumerate(&self.instance)?;
        let picker = DevicePicker::default();

        let mut result: (Option<&PhysicalDevice>, u32) = (None, 0);

        for device in physical_devices.iter() {
            let score = picker.score(device, &DeviceRequirements::default());

            if score > result.1 {
                result = (Some(device), score);
            }

            println!(
                "Device {} has a score of {}",
                &device.properties.device_name, score
            );
        }

        if let (Some(device), _) = result {
            Ok(Some(device.device))
        } else {
            Ok(None)
        }
    }
}

// pub device: Option<LogicalDevice>,
// Arc<Surface<WinitWindow>>,

impl Window {
    pub fn new(instance: &Arc<Instance>) -> Self {
        let built = WindowBuilder::new()
            .build(&instance.event_loop.clone().borrow_mut())
            .unwrap();

        let surface = get_winit_surface(instance, &built).unwrap();

        let window = Window {
            instance: instance.clone(),
            window: built,
            surface,
            device: Rc::new(RefCell::new(None)),
            device_requirements: Rc::new(RefCell::new(DeviceRequirements::default())),
        };

        window
    }

    pub fn surface(&self) -> Arc<Surface> {
        get_winit_surface(&self.instance, &self.window).unwrap()
    }

    // let window = Window {
    //     build_context: WindowBuilder::new(),
    //     // device: None,
    //     // window: window.build_surface(self, &self.event_loop.borrow())?,
    // };

    fn present(&self) -> Result<()> {
        // let submit = VkSubmitInfo {
        //     sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
        //     pNext: std::ptr::null(),
        //     waitSemaphoreCount: 1,
        //     pWaitSemaphores: self.present_semaphore.get_ptr(),
        //     pWaitDstStageMask: VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        //     commandBufferCount: 1,
        //     pCommandBuffers: cmd.get_ptr(),
        //     signalSemaphoreCount: 1,
        //     pSignalSemaphores: self.render_semaphore.get_ptr(),
        // };
        //
        // let result = self
        //     .device
        //     .api
        //     .vkQueueSubmit(self.device.graphics_queue(), 1, &submit, self.render_fence);
        // check_errors(result)?;
        //
        // let present_info = VkPresentInfoKHR {
        //     sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
        //     pNext: std::ptr::null(),
        //     waitSemaphoreCount: 1,
        //     pWaitSemaphores: self.render_semaphore.get_ptr(),
        //     swapchainCount: 1,
        //     pSwapchains: &self.swapchain,
        //     pImageIndices: &swapchain_image_index,
        //     pResults: std::ptr::null_mut(),
        // };
        //
        // let result = self.device.api.vkQueuePresentKHR(self.device.graphics_queue(), &present_info);
        // check_errors(result)?;

        Ok(())
    }
}

pub trait WindowHandler {
    fn close(&self) -> bool;

    fn event(&self, event: WindowEvent);

    fn update(&self);

    fn render(&self) -> Result<()>;
}

impl WindowHandler for Window {
    fn close(&self) -> bool {
        true
    }

    fn event(&self, event: WindowEvent) {}

    fn update(&self) {}

    fn render(&self) -> Result<()> {
        Ok(())
    }
}
