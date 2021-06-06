use ash::extensions::khr::Win32Surface;

use crate::rendering_engine::vulkan::platform_base;

pub fn required_extension_names() -> Vec<*const i8> {
  let mut extensions = platform_base::required_extension_names().clone();
  extensions.push(Win32Surface::name().as_ptr());

  extensions
}