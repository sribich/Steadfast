//! https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#wsi

use crate::vulkan::api::{
    VkSurfaceKHR, VkWin32SurfaceCreateInfoKHR, VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
};
use crate::{Check, Error, Instance, MemoryError, VulkanError};
use anyhow::Result;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::sync::Arc;
use winit::window::Window as WinitWindow;

#[cfg(target_os = "windows")]
pub(crate) fn get_winit_surface(
    instance: &Arc<Instance>,
    window: &WinitWindow,
) -> Result<Arc<Surface>> {
    use winit::platform::windows::WindowExtWindows;

    Surface::from_hwnd(instance, window.hinstance(), window.hwnd())
}

#[cfg(target_os = "macos")]
fn get_winit_surface(
    instance: &Instance,
    window: WinitWindow,
) -> Result<Arc<Surface<WinitWindow>>, SurfaceError> {
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
fn get_winit_surface(
    instance: &Instance,
    window: WinitWindow,
) -> Result<Arc<Surface<WinitWindow>>, SurfaceError> {
    use winit::platform::unix::WindowExtUnix;

    match (
        window.borrow().wayland_display(),
        window.borrow().wayland_surface(),
    ) {
        (Some(display), Some(surface)) => Surface::from_wayland(),
        _ => {
            if instance.extensions().khr_xlib_surface {
                Surface::from_xlib()
            } else {
                Surface::from_xcb()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Surface {
    pub instance: Arc<Instance>,

    surface: VkSurfaceKHR,
}

impl Surface {
    pub fn handle(&self) -> VkSurfaceKHR {
        self.surface
    }
}

impl Surface {
    pub fn from_hwnd<T, U>(
        instance: &Arc<Instance>,
        hinstance: *const T,
        hwnd: *const U,
    ) -> Result<Arc<Surface>> {
        if !instance.extensions().khr_win32_surface {
            return Err(SurfaceError::MissingExtension("VK_KHR_win32_surface").into());
        }

        let surface = {
            let info = VkWin32SurfaceCreateInfoKHR {
                sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                pNext: std::ptr::null(),
                flags: 0,
                hinstance: hinstance as *mut c_void,
                hwnd: hwnd as *mut c_void,
            };

            let mut output = MaybeUninit::uninit();

            instance
                .vulkan()
                .vkCreateWin32SurfaceKHR(
                    instance.handle(),
                    &info,
                    std::ptr::null(),
                    output.as_mut_ptr(),
                )
                .check()?;

            unsafe { output.assume_init() }
        };

        Ok(Arc::new(Surface {
            instance: (*instance).clone(),
            surface,
        }))
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.instance.vulkan().vkDestroySurfaceKHR(
            self.instance.handle(),
            self.handle(),
            std::ptr::null(),
        );
    }
}

#[derive(Debug, Error)]
pub enum SurfaceError {
    #[error(transparent)]
    MemoryError(#[from] MemoryError),
    #[error("Missing extension {0}")]
    MissingExtension(&'static str),
}
