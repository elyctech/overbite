use std::{ffi::CStr, os::raw::c_char};

// TODO This might be useful beyond Vulkan, but that needs confirmed before abstraction
// String conversion helper
pub fn c_chars_to_string(c_char_string: &[c_char]) -> String {
    let raw_string = unsafe {
        let pointer = c_char_string.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string
        .to_str()
        .expect("failed to convert C string!")
        .to_owned()
}
