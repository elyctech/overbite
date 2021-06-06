use ash::vk;

use std::{
  ffi::CStr,
  os::raw::c_void,
  ptr,
};

unsafe extern "system" fn vulkan_debug_utils_callback(
  _message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
  p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _p_user_data: *mut c_void,
) -> vk::Bool32 {
  let message = CStr::from_ptr((*p_callback_data).p_message);
  println!("validation layer: {:?}", message);

  vk::FALSE
}

pub fn make() -> vk::DebugUtilsMessengerCreateInfoEXT {
  vk::DebugUtilsMessengerCreateInfoEXT {
    s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
      | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
      | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
      | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
      | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
    pfn_user_callback: Some(vulkan_debug_utils_callback),
    flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
    p_user_data: ptr::null_mut(),
    p_next: ptr::null(),
  }
}