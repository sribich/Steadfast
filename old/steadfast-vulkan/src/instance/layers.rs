use crate::check_errors;
use crate::extensions::SupportedExtensionsError;
use crate::instance::instance::InstanceError;
use crate::{vulkan_loader, LoaderError};
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::fmt;
use std::iter::FromIterator;
use std::ops::Deref;
use std::ptr;
use std::str;
use steadfast_std::Intersect;

macro_rules! instance_layers {
    ($extensions:ident, $($extension:ident => $extension_name:expr,)*) => (
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $extensions {
            pub enabled: Vec<CString>,
            $(pub $extension: bool,)*
        }

        impl $extensions {
            fn raw() -> Result<Vec<Box<CString>>, InstanceError> {
                let entry_points = vulkan_loader()?.entry_points();

                let layers = unsafe {
                    let mut num = 0;

                    let result = entry_points.vkEnumerateInstanceLayerProperties(&mut num, std::ptr::null_mut());
                    check_errors(result)?;

                    let mut layers = Vec::with_capacity(num as usize);
                    let result = entry_points.vkEnumerateInstanceLayerProperties(&mut num, layers.as_mut_ptr());
                    check_errors(result)?;

                    layers.set_len(num as usize);
                    layers
                };

                Ok(layers.iter().map(|layer| unsafe {
                    Box::new(CStr::from_ptr(layer.layerName.as_ptr()).to_owned())
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
                            extensions.enabled.push(property.clone().deref().to_owned());
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

instance_layers! {
    InstanceLayers,
    khronos_validation => b"VK_LAYER_KHRONOS_validation",
    api_dump => b"VK_LAYER_LUNARG_api_dump",
    monitor => b"VK_LAYER_LUNARG_monitor",
}

impl Default for InstanceLayers {
    fn default() -> Self {
        let requested = InstanceLayers {
            khronos_validation: true,
            api_dump: true,
            monitor: true,
            ..InstanceLayers::supported().unwrap()
        };

        match InstanceLayers::supported() {
            Ok(supported) => supported.intersection(&requested),
            Err(_) => InstanceLayers::none(),
        }
    }
}
