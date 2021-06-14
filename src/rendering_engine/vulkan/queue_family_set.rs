use ash::{version::InstanceV1_0, vk};

use crate::rendering_engine::vulkan;

pub struct QueueFamilySet {
    graphics_family: Option<u32>,
}

impl QueueFamilySet {
    // Associated functions

    pub fn empty() -> QueueFamilySet {
        QueueFamilySet {
            graphics_family: None,
        }
    }

    pub fn find(
        instance: &vulkan::Instance,
        physical_device: &vulkan::PhysicalDevice,
    ) -> QueueFamilySet {
        QueueFamilySet::find_with_raw_device(instance, physical_device.raw())
    }

    pub fn find_with_raw_device(
        instance: &vulkan::Instance,
        physical_device: &ash::vk::PhysicalDevice,
    ) -> QueueFamilySet {
        let queue_families = unsafe {
            instance
                .raw()
                .get_physical_device_queue_family_properties(*physical_device)
        };

        let mut queue_family_set = QueueFamilySet {
            graphics_family: None,
        };

        let mut index = 0;
        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                queue_family_set.graphics_family = Some(index);
            }

            if queue_family_set.is_complete() {
                break;
            }

            index += 1;
        }

        queue_family_set
    }

    // Methods

    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}
