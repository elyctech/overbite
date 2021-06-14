use ash::vk;

use std::{os::raw::c_char, ptr};

use crate::rendering_engine::vulkan;

pub fn make(
    app_info: vk::ApplicationInfo,
    required_extension_names: &Vec<*const c_char>,
    validation_layers: &vulkan::validation_layers::ValidationLayers,
) -> vk::InstanceCreateInfo {
    vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_application_info: &app_info,
        enabled_extension_count: required_extension_names.len() as u32,
        pp_enabled_extension_names: required_extension_names.as_ptr(),
        enabled_layer_count: validation_layers.count(),
        pp_enabled_layer_names: validation_layers.names(),
        flags: vk::InstanceCreateFlags::empty(),
        p_next: ptr::null(),
    }
}
