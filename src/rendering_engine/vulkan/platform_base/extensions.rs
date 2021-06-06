use ash::extensions::khr::Surface;

#[cfg(debug_assertions)]
use ash::extensions::ext::DebugUtils;

use std::os::raw::c_char;

pub fn required_extension_names() -> Vec<*const c_char> {
  vec![
    Surface::name().as_ptr(),
    #[cfg(debug_assertions)]
    DebugUtils::name().as_ptr(),
  ]
}
