use ash::{
  version::{
    EntryV1_0,
    InstanceV1_0,
  },
  vk,
};

pub struct Instance {
  raw: ash::Instance,
}

impl Instance {
  // Associated functions

  pub fn new(
    create_info: vk::InstanceCreateInfo,
    entry: &ash::Entry,
  ) -> Instance {
    let raw = unsafe {
      entry
        .create_instance(&create_info, None)
        .expect("failed to create Vulkan instance")
    };

    Instance {
      raw,
    }
  }

  // Methods

  pub fn raw(&self) -> &ash::Instance {
    &self.raw
  }
}

impl Drop for Instance {
  fn drop(&mut self) {
    unsafe {
      self.raw.destroy_instance(None);
    }
  }
}
