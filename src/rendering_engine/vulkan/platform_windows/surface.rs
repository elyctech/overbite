use ash::extensions::khr::Win32Surface;
use ash::vk;

use std::os::raw::c_void;
use std::ptr;

// use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;

use winit::platform::windows::WindowExtWindows;

use crate::rendering_engine::vulkan;

use crate::window::Window as OverbiteWindow;

pub fn make(
  entry: &ash::Entry,
  instance: &vulkan::Instance,
  window: &OverbiteWindow,
) -> Result<vk::SurfaceKHR, vk::Result> {
  let hwnd = window.raw().hwnd() as *const c_void;
  let hinstance = unsafe { GetModuleHandleW(ptr::null()) as *const c_void };

  let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
    hinstance,
    hwnd,

    s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
    p_next: ptr::null(),
    flags: Default::default(),
    // hwnd: hwnd as *const c_void,
  };

  let win32_surface_loader = Win32Surface::new(entry, instance.raw());
  unsafe { win32_surface_loader.create_win32_surface(&win32_create_info, None) }
}
