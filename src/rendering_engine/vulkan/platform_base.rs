use ash::extensions::khr::Surface;

#[cfg(debug_assertions)]
use ash::extensions::ext::DebugUtils;

pub fn required_extension_names() -> Vec<*const i8> {
  vec![
    Surface::name().as_ptr(),
    #[cfg(debug_assertions)]
    DebugUtils::name().as_ptr(),
  ]
}
