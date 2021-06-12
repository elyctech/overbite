mod application;
mod debug_utils_messenger;
mod instance;
mod platform_base;

pub mod application_info;
pub mod debug_utils_messenger_create_info;
pub mod instance_create_info;
pub mod validation_layers;

pub use application::Application;
pub use debug_utils_messenger::DebugUtilsMessenger;
pub use instance::Instance;

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
