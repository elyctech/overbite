use crate::rendering_engine::vulkan;

// In general, this struct is used to keep the lifetimes of critical Vulkan objects
// alive and drop them in the appropriate order at the appropriate time. As such,
// objects that do not need the extra lifetime or explicit drop functionality need
// not be part of this struct; saving them just wastes memory.
pub struct Application {
    // ORDER MATTERS FOR DROPPING
    debug_utils_messenger: Option<vulkan::DebugUtilsMessenger>,
    logical_device: vulkan::LogicalDevice,
    // TODO Does this need saved?
    physical_device: vulkan::PhysicalDevice,
    surface: vulkan::Surface,
    instance: vulkan::Instance,
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
        surface: vulkan::Surface,
    ) -> Application {
        Application {
            debug_utils_messenger,
            instance,
            logical_device,
            physical_device,
            queue_families,
            surface,
        }
    }
}
