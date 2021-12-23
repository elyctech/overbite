use ash::vk;

use std::ptr;

pub fn make(queue_family_index: u32, queue_priorities: &[f32]) -> vk::DeviceQueueCreateInfo {
  vk::DeviceQueueCreateInfo {
    s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
    p_next: ptr::null(),
    flags: vk::DeviceQueueCreateFlags::empty(),
    queue_family_index: queue_family_index,
    p_queue_priorities: queue_priorities.as_ptr(),
    queue_count: queue_priorities.len() as u32,
  }
}
