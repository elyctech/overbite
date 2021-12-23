use ash::extensions::khr::Surface as VkSurface;
use ash::vk;

#[cfg(target_os = "windows")]
use crate::rendering_engine::vulkan::platform_windows::surface;

use crate::rendering_engine::vulkan;

use crate::window::Window;

pub struct Surface {
  loader: VkSurface,
  raw: vk::SurfaceKHR,
}

impl Surface {
  // Associated functions

  pub fn new(entry: &ash::Entry, instance: &vulkan::Instance, window: &Window) -> Self {
    let loader = VkSurface::new(entry, instance.raw());
    let raw = surface::make(entry, instance, window).expect("failed to create surface!");

    Surface { loader, raw }
  }

  // Methods

  pub fn loader(&self) -> &VkSurface {
    &self.loader
  }

  pub fn raw(&self) -> &vk::SurfaceKHR {
    &self.raw
  }
}

impl Drop for Surface {
  fn drop(&mut self) {
    unsafe {
      self.loader.destroy_surface(self.raw, None);
    };
  }
}
