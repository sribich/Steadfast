/// A thin wrapper around Vulkan [`Version Numbers`].
///
/// [Version Numbers]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers
#[derive(Clone, Debug)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
    }

    pub fn from_cargo() -> Self {
        Version {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    }

    pub fn from_vulkan(version: u32) -> Self {
        let parsed = format!("{}.{}.{}", version >> 22, (version >> 12) & 0x3ff, version & 0xfff);

        Version {
            major: format!("{}", version >> 22).parse::<u32>().unwrap(),
            minor: format!("{}", (version >> 12) & 0x3ff).parse::<u32>().unwrap(),
            patch: format!("{}", version & 0xfff).parse::<u32>().unwrap(),
        }
    }

    pub fn to_vulkan_u32(&self) -> u32 {
        0
    }
}
