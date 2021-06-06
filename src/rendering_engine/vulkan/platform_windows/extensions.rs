use ash::extensions::khr::Win32Surface;

use std::os::raw::c_char;

use crate::rendering_engine::vulkan::platform_base;

pub fn required_extension_names() -> Vec<*const c_char> {
  let mut extensions = platform_base::extensions::required_extension_names().clone();
  extensions.push(Win32Surface::name().as_ptr());

  extensions
}