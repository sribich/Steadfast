use crate::api::VkQueueFamilyProperties;

#[derive(Clone, Debug)]
pub struct QueueFamily {
    pub properties: VkQueueFamilyProperties,
}

impl QueueFamily {
    pub fn new(properties: VkQueueFamilyProperties) -> Self {
        Self { properties }
    }
}
