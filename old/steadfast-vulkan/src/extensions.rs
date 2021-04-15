// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use crate::vulkan::api::VkPhysicalDevice;
use crate::vulkan_loader;
use crate::Check;
use crate::LoaderError;
use crate::{check_errors, Error};
use crate::{Instance, InstanceError};
use anyhow::Result;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use steadfast_std::{Intersect, Union};

pub trait HasExtensions {
    fn has_extension(&self, extension: &str) -> bool;
}

macro_rules! instance_extensions {
    ($extensions:ident, $($extension:ident => $extension_name:expr,)*) => (
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $extensions {
            pub enabled: Vec<CString>,
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
                            extensions.enabled.push(property.deref().to_owned());
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
                    enabled: vec!(),
                    $($extension: false,)*
                }
            }

            pub fn intersection(&self, other: &$extensions) -> Self {
                $extensions {
                    enabled: self.enabled.intersect(other.enabled.clone()),
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
    khr_device_group_creation => b"VK_KHR_device_group_creation",
}

macro_rules! device_extensions {
    ($($extension:ident,)*) => (
        #[derive(Clone, Debug)]
        pub struct DeviceExtensions {
            pub enabled: Vec<CString>,
            $(pub $extension: bool,)*
        }

        impl DeviceExtensions {
            pub fn from_physical_device(instance: &Instance, physical_device: VkPhysicalDevice) -> Result<DeviceExtensions> {
                let properties = {
                    let mut num = 0;

                    instance.vulkan().vkEnumerateDeviceExtensionProperties(
                        physical_device,
                        std::ptr::null(),
                        &mut num,
                        std::ptr::null_mut()
                    ).check()?;

                    let mut properties = Vec::with_capacity(num as usize);

                    instance.vulkan().vkEnumerateDeviceExtensionProperties(
                        physical_device,
                        std::ptr::null(),
                        &mut num,
                        properties.as_mut_ptr()
                    ).check()?;

                    unsafe {
                        properties.set_len(num as usize);
                    }

                    properties
                };

                let mut extensions = DeviceExtensions {
                    ..DeviceExtensions::none()
                };

                for property in properties.iter() {
                    let extension_name = unsafe { CStr::from_ptr(property.extensionName.as_ptr()).to_owned() };

                    $(
                        if (extension_name.to_bytes() == stringify!($extension)[..].as_bytes()) {
                            extensions.$extension = true;
                            extensions.enabled.push(extension_name.clone());
                        }
                    )+
                }

                Ok(extensions)
            }

            pub fn none() -> Self {
                DeviceExtensions {
                    enabled: vec!(),
                    $($extension: false,)*
                }
            }

            pub fn intersection(&self, other: &DeviceExtensions) -> Self {
                DeviceExtensions {
                    enabled: self.enabled.intersect(other.enabled.clone()),
                    $($extension: self.$extension && other.$extension,)*
                }
            }

            pub fn union(&self, other: &DeviceExtensions) -> Self {
                DeviceExtensions {
                    enabled: self.enabled.union(other.enabled.clone()),
                    $($extension: self.$extension || other.$extension,)*
                }
            }
        }
    );
}

device_extensions! {
    VK_KHR_swapchain,
    VK_KHR_display_swapchain,
    VK_KHR_sampler_mirror_clamp_to_edge,
    VK_KHR_maintenance1,
    VK_KHR_get_memory_requirements2,
    VK_KHR_dedicated_allocation,
    VK_KHR_incremental_present,
    VK_KHR_16bit_storage,
    VK_KHR_8bit_storage,
    VK_KHR_storage_buffer_storage_class,
    VK_EXT_debug_utils,
    VK_KHR_multiview,
    VK_EXT_full_screen_exclusive,
    VK_KHR_external_memory,
    VK_KHR_external_memory_fd,
    VK_EXT_external_memory_dma_buf,
    VK_KHR_portability_subset,
}

impl HasExtensions for InstanceExtensions {
    fn has_extension(&self, extension: &str) -> bool {
        self.enabled
            .iter()
            .any(|x| x.to_str().map_or(false, |str| str == extension))
    }
}

impl Default for InstanceExtensions {
    fn default() -> Self {
        let requested = InstanceExtensions {
            khr_surface: true,
            khr_xlib_surface: true,
            khr_xcb_surface: true,
            khr_wayland_surface: true,
            khr_android_surface: true,
            khr_win32_surface: true,
            mvk_ios_surface: true,
            mvk_macos_surface: true,
            khr_get_physical_device_properties2: true,
            khr_get_surface_capabilities2: true,
            ..InstanceExtensions::supported().unwrap()
        };

        match InstanceExtensions::supported() {
            Ok(supported) => supported.intersection(&requested),
            Err(_) => InstanceExtensions::none(),
        }
    }
}

// impl fmt::Debug for $sname {
//     #[allow(unused_assignments)]
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[")?;
//
//         let mut first = true;
//
//         $(
//             if self.$ext {
//                 if !first { write!(f, ", ")? }
//                 else { first = false; }
//                 f.write_str(str::from_utf8($s).unwrap())?;
//             }
//         )*
//
//         write!(f, "]")
//     }
// }
//
// /// Set of extensions, not restricted to those vulkano knows about.
// ///
// /// This is useful when interacting with external code that has statically-unknown extension
// /// requirements.
// #[derive(Clone, Eq, PartialEq)]
// pub struct $rawname(HashSet<CString>);
//
// impl $rawname {
//     /// Constructs an extension set containing the supplied extensions.
//     pub fn new<I>(extensions: I) -> Self
//         where I: IntoIterator<Item=CString>
//     {
//         $rawname(extensions.into_iter().collect())
//     }
//
//     /// Constructs an empty extension set.
//     pub fn none() -> Self { $rawname(HashSet::new()) }
//
//     /// Adds an extension to the set if it is not already present.
//     pub fn insert(&mut self, extension: CString) {
//         self.0.insert(extension);
//     }
//
//     /// Returns the intersection of this set and another.
//     pub fn intersection(&self, other: &Self) -> Self {
//         $rawname(self.0.intersection(&other.0).cloned().collect())
//     }
//
//     /// Returns the difference of another set from this one.
//     pub fn difference(&self, other: &Self) -> Self {
//         $rawname(self.0.difference(&other.0).cloned().collect())
//     }
//
//     /// Returns the union of both extension sets
//     pub fn union(&self, other: &Self) -> Self {
//         $rawname(self.0.union(&other.0).cloned().collect())
//     }
//
//     // TODO: impl Iterator
//     pub fn iter(&self) -> ::std::collections::hash_set::Iter<CString> { self.0.iter() }
// }
//
// impl fmt::Debug for $rawname {
//     #[allow(unused_assignments)]
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.0.fmt(f)
//     }
// }
//
// impl FromIterator<CString> for $rawname {
//     fn from_iter<T>(iter: T) -> Self
//         where T: IntoIterator<Item = CString>
//     {
//         $rawname(iter.into_iter().collect())
//     }
// }
//
// impl<'a> From<&'a $sname> for $rawname {
//     fn from(x: &'a $sname) -> Self {
//         let mut data = HashSet::new();
//         $(if x.$ext { data.insert(CString::new(&$s[..]).unwrap()); })*
//         $rawname(data)
//     }
// }
//
// impl<'a> From<&'a $rawname> for $sname {
//     fn from(x: &'a $rawname) -> Self {
//         let mut extensions = $sname::none();
//         $(
//             if x.0.iter().any(|x| x.as_bytes() == &$s[..]) {
//                 extensions.$ext = true;
//             }
//         )*
//         extensions
//     }
// }

#[derive(Clone, Debug, Error)]
pub enum SupportedExtensionsError {
    #[error(transparent)]
    LoadingError(#[from] LoaderError),
}
