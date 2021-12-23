use ash::{version::InstanceV1_0, vk};

use crate::rendering_engine::vulkan;

pub struct QueueFamilySet {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilySet {
    // Associated functions

    pub fn empty() -> QueueFamilySet {
        QueueFamilySet {
            graphics_family: None,
            present_family: None,
        }
    }

    pub fn find(
        instance: &vulkan::Instance,
        physical_device: &vulkan::PhysicalDevice,
        surface: &vulkan::Surface,
    ) -> QueueFamilySet {
        QueueFamilySet::find_with_raw_device(instance, physical_device.raw(), surface)
    }

    pub fn find_with_raw_device(
        instance: &vulkan::Instance,
        physical_device: &ash::vk::PhysicalDevice,
        surface: &vulkan::Surface,
    ) -> QueueFamilySet {
        let queue_families = unsafe {
            instance
                .raw()
                .get_physical_device_queue_family_properties(*physical_device)
        };

        let mut queue_family_set = QueueFamilySet::empty();

        let mut index = 0;
        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                queue_family_set.graphics_family = Some(index);
            }

            let is_present_supported = unsafe {
                surface
                    .loader()
                    .get_physical_device_surface_support(
                        *physical_device,
                        index as u32,
                        *surface.raw(),
                    )
                    // TODO is a panic here necessary?
                    .expect("unable to detect device surface support!")
            };

            if queue_family.queue_count > 0 && is_present_supported {
                queue_family_set.present_family = Some(index);
            }

            if queue_family_set.is_complete() {
                break;
            }

            index += 1;
        }

        queue_family_set
    }

    // Methods

    pub fn graphics_family(&self) -> &Option<u32> {
        &self.graphics_family
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    pub fn present_family(&self) -> &Option<u32> {
        &self.present_family
    }
}
