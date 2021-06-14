use ash::version::InstanceV1_0;

use crate::rendering_engine::vulkan;

pub struct PhysicalDevice {
    raw: ash::vk::PhysicalDevice,
}

impl PhysicalDevice {
    // Associated functions

    pub fn pick<T>(instance: &vulkan::Instance, selector: T) -> Option<PhysicalDevice>
    where
        T: Fn(&ash::vk::PhysicalDevice) -> bool,
    {
        let physical_devices = unsafe {
            instance
                .raw()
                .enumerate_physical_devices()
                .expect("failed to enumerate physical devices!")
        };

        for physical_device in physical_devices {
            if selector(&physical_device) {
                return Some(PhysicalDevice {
                    raw: physical_device,
                });
            }
        }

        return None;
    }
}
