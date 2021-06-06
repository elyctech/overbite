use ash::vk;

use std::ptr;

pub fn make(
  app_info: &vk::ApplicationInfo,
  required_extension_names: &Vec<*const i8>,
) -> vk::InstanceCreateInfo {
  vk::InstanceCreateInfo {
		s_type: vk::StructureType::INSTANCE_CREATE_INFO,
		p_application_info: app_info,
		enabled_extension_count: required_extension_names.len() as u32,
		pp_enabled_extension_names: required_extension_names.as_ptr(),
		enabled_layer_count: 0,
		pp_enabled_layer_names: ptr::null(),
		flags: vk::InstanceCreateFlags::empty(),
		p_next: ptr::null(),
	}
}