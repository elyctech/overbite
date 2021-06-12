use ash::{
  extensions::ext::DebugUtils,
  vk,
};

use crate::rendering_engine::vulkan;

pub struct DebugUtilsMessenger {
  debug_utils: DebugUtils,
  debug_utils_messenger: vk::DebugUtilsMessengerEXT
}

impl DebugUtilsMessenger {
  // Associated functions

  pub fn new(
    create_info: vk::DebugUtilsMessengerCreateInfoEXT,
    entry: &ash::Entry,
    instance: &vulkan::Instance,
  ) -> DebugUtilsMessenger {
    let debug_utils = DebugUtils::new(entry, instance.raw());
    let debug_utils_messenger = unsafe {
      debug_utils
        .create_debug_utils_messenger(&create_info, None)
        .expect("failed to set up debug utils messenger!")
    };

    DebugUtilsMessenger {
      debug_utils,
      debug_utils_messenger
    }
  }
}

impl Drop for DebugUtilsMessenger {
  fn drop(&mut self) {
    unsafe {
      self.debug_utils.destroy_debug_utils_messenger(
        self.debug_utils_messenger,
        None
      );
    }
  }
}
