use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::vk;

use crate::rendering_engine::vulkan;

pub struct LogicalDevice {
    raw: ash::Device,
}

impl LogicalDevice {
    // Associated functions

    pub fn new(
        physical_device: &vulkan::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
        instance: &vulkan::Instance,
    ) -> LogicalDevice {
        let raw = unsafe {
            instance
                .raw()
                .create_device(*physical_device.raw(), create_info, None)
                .expect("failed to create Vulkan logical device!")
        };

        LogicalDevice { raw }
    }

    // Methods

    pub fn get_queue(&self, queue_family: u32, index: u32) -> vk::Queue {
        unsafe { self.raw.get_device_queue(queue_family, index) }
    }

    pub fn raw(&self) -> &ash::Device {
        &self.raw
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.raw.destroy_device(None);
        }
    }
}
