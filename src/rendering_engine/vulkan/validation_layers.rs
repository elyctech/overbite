use ash::{
  version::EntryV1_0,
};

use std::{
  ffi::{
    CStr,
    CString,
  },
  os::raw::c_char,
};

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;

#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

// String conversion helper
fn c_chars_to_string(c_char_string: &[c_char]) -> String {
  let raw_string = unsafe {
    let pointer = c_char_string.as_ptr();
    CStr::from_ptr(pointer)
  };

  raw_string
    .to_str()
    .expect("failed to convert C string!")
    .to_owned()
}

// Desired validation layers
const DESIRED_VALIDATION_LAYERS: [&str; 1] = [
  "VK_LAYER_KHRONOS_validation",
];

// TODO check if raw c_char comparison works (eliminates the need for the above function)
pub fn check_validation_layer_support(entry: &ash::Entry) -> bool {
  let available_layers = entry
    .enumerate_instance_layer_properties()
    .expect("failed to enumerate instance layer properties!");

  for layer_name in DESIRED_VALIDATION_LAYERS.iter() {
    let mut layer_found = false;

    for layer_property in available_layers.iter() {
      let layer_property_name = c_chars_to_string(&layer_property.layer_name);

      if (*layer_name) == layer_property_name {
        layer_found = true;
        break;
      }
    }

    if !layer_found {
      return false;
    }
  }

  true
}

// Desired validation layers

pub struct ValidationLayers {
  _enabled_layer_c_strings: Vec<CString>,
  enabled_layer_names: Vec<*const c_char>,
}

impl ValidationLayers {
  pub fn count(&self) -> u32 {
    self.enabled_layer_names.len() as u32
  }

  pub fn names(&self) -> *const *const i8 {
    self.enabled_layer_names.as_ptr()
  }
}

pub fn get_desired_validation_layers() -> ValidationLayers {
  if ENABLE_VALIDATION_LAYERS {
    let _enabled_layer_c_strings: Vec<CString> = DESIRED_VALIDATION_LAYERS.iter()
      .map(|layer_name| CString::new(*layer_name).unwrap())
      .collect();

    let enabled_layer_names: Vec<*const c_char> = _enabled_layer_c_strings.iter()
      .map(|layer_name| layer_name.as_ptr())
      .collect();

    ValidationLayers {
      _enabled_layer_c_strings,
      enabled_layer_names,
    }
  } else {
    ValidationLayers {
      _enabled_layer_c_strings: vec![],
      enabled_layer_names: vec![],
    }
  }
}
