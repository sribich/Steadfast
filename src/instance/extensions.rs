// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::fmt;
use std::iter::FromIterator;
use std::ptr;
use std::str;
use crate::check_errors;
use crate::vulkan::loader::{vulkan_loader, LoaderError};
use crate::instance::instance::InstanceError;
use crate::extensions::SupportedExtensionsError;

macro_rules! instance_extensions {
    ($extensions:ident, $($extension:ident => $extension_name:expr,)*) => (
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub struct $extensions {
            $(pub $extension: bool,)*
        }

        impl $extensions {
            fn raw() -> Result<Vec<Box<CString>>, InstanceError> {
                let entry_points = vulkan_loader()?.entry_points();

                let properties = unsafe {
                    let mut num = 0;

                    let result = entry_points.vkEnumerateInstanceExtensionProperties(std::ptr::null(), &mut num, std::ptr::null_mut());
                    check_errors(result)?;

                    let mut properties = Vec::with_capacity(num as usize);
                    let result = entry_points.vkEnumerateInstanceExtensionProperties(std::ptr::null(), &mut num, properties.as_mut_ptr());
                    check_errors(result)?;

                    properties.set_len(num as usize);
                    properties
                };

                Ok(properties.iter().map(|property| unsafe {
                    Box::new(CStr::from_ptr(property.extensionName.as_ptr()).to_owned())
                }.to_owned()).collect())
            }

            pub fn supported() -> Result<Self, InstanceError> {
                let mut extensions = $extensions {
                    ..$extensions::none()
                };

                for property in $extensions::raw()? {
                    $(
                        if property.to_bytes() == &$extension_name[..] {
                            extensions.$extension = true;
                        }
                    )*
                }

                Ok(extensions)
            }

            pub fn strings(&self) -> Result<Vec<Box<CString>>, InstanceError> {
                $extensions::raw()
            }

            pub fn none() -> Self {
                $extensions {
                    $($extension: false,)*
                }
            }

            pub fn intersection(&self, other: &$extensions) -> Self {
                $extensions {
                    $($extension: self.$extension && other.$extension,)*
                }
            }

            // /// Returns the union of this list and another list.
            // #[inline]
            // pub fn union(&self, other: &$sname) -> $sname {
            //     $sname {
            //         $(
            //             $ext: self.$ext || other.$ext,
            //         )*
            //         _unbuildable: Unbuildable(())
            //     }
            // }
            //
            // /// Returns the difference of another list from this list.
            // #[inline]
            // pub fn difference(&self, other: &$sname) -> $sname {
            //     $sname {
            //         $(
            //             $ext: self.$ext && !other.$ext,
            //         )*
            //         _unbuildable: Unbuildable(())
            //     }
            // }
        }
    );
}

instance_extensions! {
    InstanceExtensions,
    khr_surface => b"VK_KHR_surface",
    khr_display => b"VK_KHR_display",
    khr_xlib_surface => b"VK_KHR_xlib_surface",
    khr_xcb_surface => b"VK_KHR_xcb_surface",
    khr_wayland_surface => b"VK_KHR_wayland_surface",
    khr_android_surface => b"VK_KHR_android_surface",
    khr_win32_surface => b"VK_KHR_win32_surface",
    ext_debug_utils => b"VK_EXT_debug_utils",
    mvk_ios_surface => b"VK_MVK_ios_surface",
    mvk_macos_surface => b"VK_MVK_macos_surface",
    mvk_moltenvk => b"VK_MVK_moltenvk",
    nn_vi_surface => b"VK_NN_vi_surface",
    ext_swapchain_colorspace => b"VK_EXT_swapchain_colorspace",
    khr_get_physical_device_properties2 => b"VK_KHR_get_physical_device_properties2",
    khr_get_surface_capabilities2 => b"VK_KHR_get_surface_capabilities2",
}

// impl $raw_extensions {
//             /// See the docs of supported_by_core().
//             pub fn supported_by_core_raw() -> Result<Self, SupportedExtensionsError> {
//                 $rawname::supported_by_core_raw_with_loader(loader::auto_loader()?)
//             }
//
//             /// Same as `supported_by_core_raw()`, but allows specifying a loader.
//             pub fn supported_by_core_raw_with_loader<L>(ptrs: &loader::FunctionPointers<L>)
//                         -> Result<Self, SupportedExtensionsError>
//                 where L: loader::Loader
//             {
//                 let entry_points = ptrs.entry_points();
//
//                 let properties: Vec<vk::ExtensionProperties> = unsafe {
//                     let mut num = 0;
//                     check_errors(entry_points.EnumerateInstanceExtensionProperties(
//                         ptr::null(), &mut num, ptr::null_mut()
//                     ))?;
//
//                     let mut properties = Vec::with_capacity(num as usize);
//                     check_errors(entry_points.EnumerateInstanceExtensionProperties(
//                         ptr::null(), &mut num, properties.as_mut_ptr()
//                     ))?;
//                     properties.set_len(num as usize);
//                     properties
//                 };
//                 Ok($rawname(properties.iter().map(|x| unsafe { CStr::from_ptr(x.extensionName.as_ptr()) }.to_owned()).collect()))
//             }
//
//             /// Returns a `RawExtensions` object with extensions supported by the core driver.
//             pub fn supported_by_core() -> Result<Self, LoadingError> {
//                 match $rawname::supported_by_core_raw() {
//                     Ok(l) => Ok(l),
//                     Err(SupportedExtensionsError::LoadingError(e)) => Err(e),
//                     Err(SupportedExtensionsError::OomError(e)) => panic!("{:?}", e),
//                 }
//             }
//
//             /// Same as `supported_by_core`, but allows specifying a loader.
//             pub fn supported_by_core_with_loader<L>(ptrs: &loader::FunctionPointers<L>)
//                         -> Result<Self, LoadingError>
//                 where L: loader::Loader
//             {
//                 match $rawname::supported_by_core_raw_with_loader(ptrs) {
//                     Ok(l) => Ok(l),
//                     Err(SupportedExtensionsError::LoadingError(e)) => Err(e),
//                     Err(SupportedExtensionsError::OomError(e)) => panic!("{:?}", e),
//                 }
//             }
//         }