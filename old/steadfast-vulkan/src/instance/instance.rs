use crate::api::{
    VkApplicationInfo, VkInstance, VkInstanceCreateInfo, VK_STRUCTURE_TYPE_APPLICATION_INFO,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
};
use crate::device::{DevicePicker, LogicalDevice, PhysicalDevice};
use crate::extensions::{HasExtensions, InstanceExtensions};
use crate::instance::{InstanceLayers, Version};
use crate::{check_errors, Error, VulkanError, VulkanHandle, Window};
use crate::{vulkan, WindowHandler};
use crate::{vulkan_loader, Check, LoaderError};
use anyhow::Result;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::sync::Arc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{WindowBuilder, WindowId};

lazy_static! {
    static ref API_VERSION: Version = {
        let loader = vulkan_loader();

        if let Err(_) = loader {
            panic!("Unable to find an instance of Vulkan on the system")
        }

        let mut version = 0;

        if let Ok(loader) = loader {
            loader
                .entry_points()
                .vkEnumerateInstanceVersion(&mut version);

            Version::from_vulkan(version)
        } else {
            panic!("Unable to determine Vulkan instance version");
        }
    };
}

/// A thin wrapper around the Vulkan context (instance), which contains
/// application state information like the Vulkan API version you're using,
/// your application's name, and which extensions and layers you want to
/// enable.
///
/// [`blah`]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#initialization
#[derive(Debug)]
pub struct Instance {
    extensions: InstanceExtensions,
    instance: VkInstance,
    vulkan: vulkan::api::InstancePointers,

    pub event_loop: Arc<RefCell<EventLoop<()>>>,
    windows: Rc<RefCell<HashMap<WindowId, Rc<Window>>>>,
}

impl VulkanHandle for Instance {
    fn handle(&self) -> VkInstance {
        self.instance
    }
}

pub struct ApplicationInfo<'a> {
    pub application_name: Cow<'a, str>,
    pub application_version: Version,
    pub engine_name: Cow<'a, str>,
    pub engine_version: Version,
    pub api_version: Version,
}

impl Instance {
    pub fn recreate_presentation_layer() {}
}

impl HasExtensions for Instance {
    fn has_extension(&self, extension: &str) -> bool {
        self.extensions
            .enabled
            .iter()
            .any(|x| x.to_str().map_or(false, |str| str == extension))
    }
}

impl Instance {
    /// Instantiates a new instance of the Vulkan wrapper
    ///
    /// # Todo
    ///
    ///   - VkApplicationInfo
    ///   - Custom Allocator (https://github.com/gwihlidal/vk-mem-rs)
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
    pub fn new(
        app_info: Option<ApplicationInfo>,
        extensions: InstanceExtensions,
    ) -> Result<Arc<Instance>> {
        // debug!("Initializing instance with extensions: {:#?}", extensions);

        let mut output = MaybeUninit::uninit();

        let layers = InstanceLayers::default();

        let instance = unsafe {
            let app_info = if let Some(app_info) = app_info {
                &Instance::app_info(app_info) as *const _
            } else {
                std::ptr::null()
            };

            let info = VkInstanceCreateInfo {
                sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                pApplicationInfo: app_info,
                enabledLayerCount: layers.enabled.len() as u32,
                ppEnabledLayerNames: layers
                    .enabled
                    .iter()
                    .map(|ext| ext.as_ptr())
                    .collect::<SmallVec<[_; 32]>>()
                    .as_ptr(),
                enabledExtensionCount: extensions.enabled.len() as u32,
                ppEnabledExtensionNames: extensions
                    .enabled
                    .iter()
                    .map(|ext| ext.as_ptr())
                    .collect::<SmallVec<[_; 32]>>()
                    .as_ptr(),
            };

            vulkan_loader()?
                .entry_points()
                .vkCreateInstance(&info, std::ptr::null(), output.as_mut_ptr())
                .check()?;

            output.assume_init()
        };

        let vulkan = {
            let loader = vulkan_loader()?;

            vulkan::api::InstancePointers::load(extensions.clone(), |name| {
                loader.get_instance_proc_addr(instance, name.as_ptr()) as *const std::ffi::c_void
            })
        };

        Ok(Arc::new(Instance {
            extensions,
            instance,
            vulkan,
            event_loop: Arc::new(RefCell::new(EventLoop::new())),
            windows: Rc::new(RefCell::new(HashMap::new())),
        }))
    }

    pub fn run(&self) {
        self.event_loop
            .borrow_mut()
            .run_return(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                match event {
                    Event::WindowEvent {
                        event, window_id, ..
                    } => {
                        match event {
                            WindowEvent::CloseRequested => {
                                let mut windows = self.windows.borrow_mut();
                                let mut window = windows.get_mut(&window_id);

                                if let Some(context) = window {
                                    let closed = context.close();

                                    if closed {
                                        {
                                            windows.remove(&window_id);
                                        }

                                        // {
                                        //     std::mem::drop(context.window)
                                        // }
                                    };
                                };

                                if windows.len() == 0 {
                                    *control_flow = ControlFlow::Exit;
                                }
                            }
                            WindowEvent::Resized(size) => (),
                            _ => (),
                        };
                    }
                    Event::MainEventsCleared => {
                        let windows = self.windows.borrow();

                        for window in windows.values() {
                            window.update();
                        }
                    }
                    Event::RedrawRequested(_) => {
                        let windows = self.windows.borrow();

                        for window in windows.values() {
                            window.render();
                        }
                    }
                    _ => (),
                };
            });
    }

    pub fn device_groups(&self) {}

    /// Returns the Vulkan instance version formatted as defined by
    /// the [`spec`].
    ///
    /// [`spec`]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers
    ///
    pub fn version() -> Result<String, InstanceError> {
        let mut version = 0;

        let result = vulkan_loader()?
            .entry_points()
            .vkEnumerateInstanceVersion(&mut version);
        check_errors(result)?;

        Ok(format!(
            "{}.{}.{}",
            version >> 22,
            (version >> 12) & 0x3ff,
            version & 0xfff
        ))
    }

    pub fn extensions(&self) -> &InstanceExtensions {
        &self.extensions
    }

    pub fn vulkan(&self) -> &vulkan::api::InstancePointers {
        &self.vulkan
    }

    pub fn handle(&self) -> VkInstance {
        self.instance
    }

    fn app_info(app_info: ApplicationInfo) -> VkApplicationInfo {
        VkApplicationInfo {
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: std::ptr::null(),
            pApplicationName: CString::new(app_info.application_name.as_bytes().to_owned())
                .map(|x| x.as_ptr())
                .unwrap_or(std::ptr::null()),
            applicationVersion: app_info.application_version.to_vulkan_u32(),
            pEngineName: CString::new(app_info.engine_name.as_bytes().to_owned())
                .map(|x| x.as_ptr())
                .unwrap_or(std::ptr::null()),
            engineVersion: app_info.engine_version.to_vulkan_u32(),
            apiVersion: app_info.api_version.to_vulkan_u32(),
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.vulkan
            .vkDestroyInstance(self.handle(), std::ptr::null());
    }
}

#[derive(Debug, Error)]
pub enum InstanceError {
    #[error(transparent)]
    LibraryError(#[from] LoaderError),
    #[error(transparent)]
    VulkanError(#[from] VulkanError),
}
