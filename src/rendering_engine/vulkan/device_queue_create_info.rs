use ash::vk;

use std::ptr;

use crate::rendering_engine::vulkan;

pub fn make(queue_familys: &vulkan::QueueFamilySet) -> vk::DeviceQueueCreateInfo {
  let queue_priorities = [1.0_f32];

  vk::DeviceQueueCreateInfo {
    s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
    p_next: ptr::null(),
    flags: vk::DeviceQueueCreateFlags::empty(),
    queue_family_index: queue_familys
      .get_graphics_family()
      .expect("graphics family not found!"),
    p_queue_priorities: queue_priorities.as_ptr(),
    queue_count: queue_priorities.len() as u32,
  }
}
