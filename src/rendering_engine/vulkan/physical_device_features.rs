use ash::vk;

pub fn make() -> vk::PhysicalDeviceFeatures {
  vk::PhysicalDeviceFeatures {
    ..Default::default()
  }
}
