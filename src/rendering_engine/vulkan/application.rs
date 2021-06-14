use crate::rendering_engine::vulkan;

pub struct Application {
    debug_utils_messenger: Option<vulkan::DebugUtilsMessenger>,
    instance: vulkan::Instance,
    physical_device: vulkan::PhysicalDevice,
}

impl Application {
    // Associated functions

    pub fn new(
        debug_utils_messenger: Option<vulkan::DebugUtilsMessenger>,
        instance: vulkan::Instance,
        physical_device: vulkan::PhysicalDevice,
    ) -> Application {
        Application {
            debug_utils_messenger,
            instance,
            physical_device,
        }
    }
}
