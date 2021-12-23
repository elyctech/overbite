use ash::{version::InstanceV1_0, vk};

use std::collections::HashSet;
use std::{ffi::CString, os::raw::c_char};

use crate::rendering_engine::vulkan::utility;

// Desired device extensions
const DESIRED_DEVICE_EXTENSIONS: [&str; 1] = ["VK_KHR_swapchain"];

// Check for support on device
pub fn check_device_extension_support(
    device: &vk::PhysicalDevice,
    instance: &ash::Instance,
) -> bool {
    // Enumerate available extension names

    let available_extensions = unsafe {
        instance
            .enumerate_device_extension_properties(*device)
            .expect("failed to get device extension properties!")
    };

    let mut available_extension_names = vec![];

    for &extension in available_extensions.iter() {
        available_extension_names.push(utility::c_chars_to_string(&extension.extension_name));
    }

    // Enumerate desired extension names

    let mut desired_extension_names = HashSet::new();

    for &extension in DESIRED_DEVICE_EXTENSIONS.iter() {
        desired_extension_names.insert(extension.to_string());
    }

    // Check off available extensions from desired extensions

    for available_extension_name in available_extension_names.iter() {
        desired_extension_names.remove(available_extension_name);
    }

    // If all have been removed, then all desired extensions are available
    desired_extension_names.is_empty()
}

// Desired device extensions

pub struct DeviceExtensions {
    // Held so it is not freed prematurely
    _enabled_extension_c_strings: Vec<CString>,
    enabled_extension_names: Vec<*const c_char>,
}

impl DeviceExtensions {
    pub fn count(&self) -> u32 {
        self.enabled_extension_names.len() as u32
    }

    pub fn names(&self) -> *const *const i8 {
        self.enabled_extension_names.as_ptr()
    }
}

// TODO Find a way to not duplicate this from validation layers
pub fn get_desired_device_extensions() -> DeviceExtensions {
    let _enabled_extension_c_strings: Vec<CString> = DESIRED_DEVICE_EXTENSIONS
        .iter()
        .map(|extension_name| CString::new(*extension_name).unwrap())
        .collect();

    let enabled_extension_names: Vec<*const c_char> = _enabled_extension_c_strings
        .iter()
        .map(|extension_name| extension_name.as_ptr())
        .collect();

    DeviceExtensions {
        _enabled_extension_c_strings,
        enabled_extension_names,
    }
}
