mod application;
mod instance;
mod platform_base;
mod platform_windows;

pub mod application_info;
pub mod instance_create_info;

pub use application::Application;
pub use instance::Instance;

// Platform-specific

#[cfg(target_os = "windows")]
pub use platform_windows::extensions;
