//!
//!
//!

use crate::vulkan::api::{PFN_vkVoidFunction, VkInstance};
use std::ops::Deref;
use std::os::raw::c_char;
use std::path::Path;

trait Loader {
    fn load(&self, instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction;
}

struct DynamicLoader {
    #[allow(dead_code)]
    library: libloading::Library,
    get_proc_addr: extern "system" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction,
}

impl DynamicLoader {
    pub fn new(path: &Path) -> Result<DynamicLoader, LoaderError> {
        let library =
            unsafe { libloading::Library::new(path).map_err(|_| LoaderError::LoadFailure)? };

        let get_proc_addr = unsafe {
            let ptr: libloading::Symbol<
                extern "system" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction,
            > = library
                .get(b"vkGetInstanceProcAddr")
                .map_err(|_| LoaderError::MissingEntryPoint("vkGetInstanceProcAddr".to_owned()))?;

            ptr as extern "system" fn(VkInstance, *const c_char) -> PFN_vkVoidFunction
        };

        Ok(DynamicLoader {
            library,
            get_proc_addr,
        })
    }
}

impl Loader for DynamicLoader {
    fn load(&self, instance: VkInstance, name: *const c_char) -> PFN_vkVoidFunction {
        (self.get_proc_addr)(instance, name)
    }
}

pub struct FunctionPointers<L>
where
    L: Loader,
{
    loader: L,
    entry_points: EntryPoints,
}

impl<L> FunctionPointers<L>
where
    L: Loader,
{
    fn new(loader: L) -> FunctionPointers<L> {
        let entry_points = EntryPoints::load_entry(|name| {
            loader.load(0, name.as_ptr()) as *const std::ffi::c_void
        });

        FunctionPointers {
            loader,
            entry_points,
        }
    }

    pub fn entry_points(&self) -> &EntryPoints {
        &self.entry_points
    }

    pub fn get_instance_proc_addr(
        &self,
        instance: VkInstance,
        name: *const c_char,
    ) -> PFN_vkVoidFunction {
        self.loader.load(instance, name)
    }
}

/// Returns a platform-specific path referencing a Vulkan
/// shared library.
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

// Result<&'static FunctionPointers<Box<dyn Loader + Send + Sync>>, LoaderError>
pub(crate) fn vulkan_library_loader() {
    fn loader() -> Result<Box<dyn Loader + Send + Sync>, LoaderError> {
        #[cfg(not(target_os = "ios"))]
        let loader = DynamicLoader::new(get_library_path())?;

        Ok(Box::new(loader))
    }

    lazy_static! {
        static ref LOADER: Result<FunctionPointers<Box<dyn Loader + Send + Sync>>, LoaderError> =
            loader.map(FunctionPointers::new);
    }

    match *LOADER.deref() {
        Ok(ref ptr) => Ok(ptr),
        Err(ref err) => Err(err.clone()),
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
        write!(
            fmt,
            "{}",
            match *self {
                LoaderError::LoadFailure => "Failed to load the Vulkan library",
                LoaderError::MissingEntryPoint(_) =>
                    "One of the entry points required to use Vulkan is missing",
            }
        )
    }
}
