mod application;
mod debug_utils_messenger;
mod instance;
mod logical_device;
mod physical_device;
mod platform_base;
mod queue_family_set;
mod surface;

pub mod application_info;
pub mod debug_utils_messenger_create_info;
pub mod device_create_info;
pub mod device_extensions;
pub mod device_queue_create_info;
pub mod instance_create_info;
pub mod physical_device_features;
pub mod utility;
pub mod validation_layers;

pub use application::Application;
pub use debug_utils_messenger::DebugUtilsMessenger;
pub use instance::Instance;
pub use logical_device::LogicalDevice;
pub use physical_device::PhysicalDevice;
pub use queue_family_set::QueueFamilySet;
pub use surface::Surface;

// Platform-specific

// Windows

#[cfg(target_os = "windows")]
mod platform_windows;

#[cfg(target_os = "windows")]
pub use platform_windows::extensions;

// Unix

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
mod platform_unix;

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub use platform_unix::extensions;
