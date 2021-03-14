//! Provides a loader responsible for obtaining Vulkan function pointers.
//!
//! https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html

use std::os::raw::c_char;
use lazy_static::lazy_static;
use std::path::Path;
use std::ops::Deref;
use std::ffi::{CString};
use crate::vulkan::api;
use crate::vulkan::api::{VkInstance, PFN_vkVoidFunction};

pub trait Loader {
    /// Calls the `vkGetInstanceProcAddr` function.
    fn get_instance_proc_addr(&self, instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction;
}

impl<T> Loader for T
where
    T: Deref,
    T::Target: Loader,
{
    fn get_instance_proc_addr(&self, instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction {
        (**self).get_instance_proc_addr(instance, name)
    }
}

struct DynamicLoader {
    vk_lib: libloading::Library,
    get_proc_addr: extern "system" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction,
}

impl DynamicLoader {
    pub unsafe fn new(path: &Path) -> Result<DynamicLoader, LoaderError>
    {
        let vk_lib = libloading::Library::new(path).map_err(|_| LoaderError::LoadFailure)?;

        let get_proc_addr: extern "system" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction = {
            let ptr: libloading::Symbol<extern "system" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction> = vk_lib
                .get(b"vkGetInstanceProcAddr")
                .map_err(|_| LoaderError::MissingEntryPoint("vkGetInstanceProcAddr".to_owned()))?;

            std::mem::transmute(ptr)
        };

        Ok(DynamicLoader {
            vk_lib,
            get_proc_addr
        })
    }
}

impl Loader for DynamicLoader {
    fn get_instance_proc_addr(&self, instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction {
        (self.get_proc_addr)(instance, name)
    }
}

#[cfg(target_os = "ios")]
extern "C" {
    fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> vk::PFN_vkVoidFunction;
}

#[cfg(target_os = "ios")]
struct StaticLoader;

#[cfg(target_os = "ios")]
impl Loader for StaticLoader {
    fn get_instance_proc_addr(&self, instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction {
        unsafe { vkGetInstanceProcAddr(instance, name) }
    }
}

pub struct FunctionPointers<L> {
    loader: L,
    entry_points: api::EntryPoints,
}

impl<L> FunctionPointers<L> {
    /// Loads some global function pointer from the loader.
    fn new(loader: L) -> FunctionPointers<L>
        where
            L: Loader,
    {
        let entry_points = api::EntryPoints::load(|name| unsafe {
            name.as_ptr();
            std::mem::transmute(loader.get_instance_proc_addr(0, name.as_ptr()))
        });

        FunctionPointers {
            loader,
            entry_points,
        }
    }

    pub fn entry_points(&self) -> &api::EntryPoints {
        &self.entry_points
    }

    pub fn get_instance_proc_addr(&self, instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction
    where
        L: Loader
    {
        self.loader.get_instance_proc_addr(instance, name)
    }
}

fn get_library_path() -> &'static Path {
    #[cfg(windows)]
    let path: &'static Path = Path::new("vulkan-1.dll");
    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    let path: &'static Path = Path::new("libvulkan.so.1");
    #[cfg(target_os = "macos")]
    let path: &'static Path = Path::new("libvulkan.1.dylib");
    #[cfg(target_os = "android")]
    let path: &'static Path = Path::new("libvulkan.so");

    path
}

pub fn vulkan_loader() -> Result<&'static FunctionPointers<Box<dyn Loader + Send + Sync>>, LoaderError> {
    fn loader_impl() -> Result<Box<dyn Loader + Send + Sync>, LoaderError> {
        #[cfg(target_os = "ios")]
        let loader = StaticLoader {};

        #[cfg(not(target_os = "ios"))]
        let loader = unsafe { DynamicLoader::new(get_library_path())? };

        Ok(Box::new(loader))
    }

    lazy_static! {
        static ref LOADER: Result<FunctionPointers<Box<dyn Loader + Send + Sync>>, LoaderError>
            = loader_impl().map(FunctionPointers::new);
    }

    match *LOADER.deref() {
        Ok(ref ptr)  => Ok(ptr),
        Err(ref err) => Err(err.clone())
    }
}

#[derive(Debug, Clone)]
pub enum LoaderError {
    LoadFailure,
    MissingEntryPoint(String),
}

impl std::error::Error for LoaderError {}

impl std::fmt::Display for LoaderError {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", match *self {
            LoaderError::LoadFailure          => "Failed to load the Vulkan library",
            LoaderError::MissingEntryPoint(_) => "One of the entry points required to use Vulkan is missing",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::vulkan::loader::{DynamicLoader, LoaderError};
    use std::path::Path;

    #[test]
    fn dl_open_error() {
        unsafe {
            match DynamicLoader::new(Path::new("nonexistent")) {
                Err(LoaderError::LoadFailure) => (),
                _ => panic!(),
            }
        }
    }
}
