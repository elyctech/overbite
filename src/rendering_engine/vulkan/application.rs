use crate::rendering_engine::vulkan;

pub struct Application {
  instance: vulkan::Instance,
}

impl Application {
  // Associated functions

  pub fn new(
    instance: vulkan::Instance,
  ) -> Application {
    Application {
      instance,
    }
  }
}
