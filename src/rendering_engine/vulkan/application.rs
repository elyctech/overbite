use crate::rendering_engine::vulkan;

pub struct Application {
    // ORDER MATTERS FOR DROPPING
    debug_utils_messenger: Option<vulkan::DebugUtilsMessenger>,
    logical_device: vulkan::LogicalDevice,
    instance: vulkan::Instance,
    physical_device: vulkan::PhysicalDevice,
    // TODO Does this need saved?
    queue_families: vulkan::QueueFamilySet,
}

impl Application {
    // Associated functions

    pub fn new(
        debug_utils_messenger: Option<vulkan::DebugUtilsMessenger>,
        instance: vulkan::Instance,
        logical_device: vulkan::LogicalDevice,
        physical_device: vulkan::PhysicalDevice,
        queue_families: vulkan::QueueFamilySet,
    ) -> Application {
        Application {
            debug_utils_messenger,
            instance,
            logical_device,
            physical_device,
            queue_families,
        }
    }
}
