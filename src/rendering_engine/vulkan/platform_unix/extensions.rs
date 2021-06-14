use ash::extensions::khr::XlibSurface;

use std::os::raw::c_char;

use crate::rendering_engine::vulkan::platform_base;

pub fn required_extension_names() -> Vec<*const c_char> {
    let mut extensions = platform_base::extensions::required_extension_names().clone();
    extensions.push(XlibSurface::name().as_ptr());

    extensions
}
