use ash::vk;

use std::ptr;

use crate::rendering_engine::vulkan;

pub fn make(
  device_queue_create_info: &vk::DeviceQueueCreateInfo,
  physical_device_features: &vk::PhysicalDeviceFeatures,
  validation_layers: &vulkan::validation_layers::ValidationLayers,
) -> vk::DeviceCreateInfo {
  vk::DeviceCreateInfo {
    s_type: vk::StructureType::DEVICE_CREATE_INFO,
    p_next: ptr::null(),
    flags: vk::DeviceCreateFlags::empty(),
    queue_create_info_count: 1,
    p_queue_create_infos: device_queue_create_info,
    enabled_layer_count: validation_layers.count(),
    pp_enabled_layer_names: validation_layers.names(),
    enabled_extension_count: 0,
    pp_enabled_extension_names: ptr::null(),
    p_enabled_features: physical_device_features,
  }
}
