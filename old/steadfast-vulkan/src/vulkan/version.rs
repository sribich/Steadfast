/// A thin wrapper around Vulkan [`Version Numbers`].
///
/// [Version Numbers]: https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers
#[derive(Clone)]
pub struct VulkanVersion {
    pub raw: u32,
    pub formatted: String,
}

impl VulkanVersion {
    fn new(raw: u32) -> Self {
        VulkanVersion {
            raw,
            formatted: format!("{}.{}.{}", raw >> 22, (raw >> 12) & 0x3ff, raw & 0xfff)
        }
    }
}
