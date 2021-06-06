use ash::{
  version::{
    EntryV1_0,
    InstanceV1_0,
  },
  vk,
};

pub struct Instance {
  instance: ash::Instance,
}

impl Instance {
  // Associated functions

  pub fn new(
    create_info: vk::InstanceCreateInfo,
    entry: &ash::Entry,
  ) -> Instance {
    let instance = unsafe {
      entry
        .create_instance(&create_info, None)
        .expect("failed to create Vulkan instance")
    };

    Instance {
      instance,
    }
  }

  // Methods

  pub fn instance(&self) -> &ash::Instance {
    &self.instance
  }
}

impl Drop for Instance {
  fn drop(&mut self) {
    unsafe {
      self.instance.destroy_instance(None);
    }
  }
}
