use ash::vk;

use std::ffi::c_void;

use overbite::rendering_engine::vulkan;
use overbite::window::Window;

use vulkan::validation_layers::{check_validation_layer_support, ENABLE_VALIDATION_LAYERS};

fn main() {
    // Ash boilerplate
    let entry = unsafe { ash::Entry::new().expect("failed to create Ash entry!") };

    // Check on validation layers
    if ENABLE_VALIDATION_LAYERS && !check_validation_layer_support(&entry) {
        panic!("validation layers requested but not available!");
    }

    // Application Info
    let app_info = vulkan::application_info::make("Hello Triangle", vk::make_version(1, 0, 0));

    // Extensions
    let required_extension_names = vulkan::extensions::required_extension_names();

    // Validation layers
    let validation_layers = vulkan::validation_layers::get_desired_validation_layers();

    // Instance create info
    let mut instance_create_info =
        vulkan::instance_create_info::make(app_info, &required_extension_names, &validation_layers);

    // Instance debugging
    let instance_debug_create_info = vulkan::debug_utils_messenger_create_info::make();

    if ENABLE_VALIDATION_LAYERS {
        instance_create_info.p_next = &instance_debug_create_info
            as *const vk::DebugUtilsMessengerCreateInfoEXT
            as *const c_void;
    }

    // Overbite vulkan instance
    let instance = vulkan::Instance::new(&instance_create_info, &entry);

    // Application debugging
    let mut debug_utils_messenger: Option<vulkan::DebugUtilsMessenger> = None;

    if ENABLE_VALIDATION_LAYERS {
        let application_debug_create_info = vulkan::debug_utils_messenger_create_info::make();

        debug_utils_messenger = Some(vulkan::DebugUtilsMessenger::new(
            application_debug_create_info,
            &entry,
            &instance,
        ));
    }

    // Overbite vulkan physical device
    let physical_device = vulkan::PhysicalDevice::pick(&instance, |raw_physical_device| {
        vulkan::QueueFamilySet::find_with_raw_device(&instance, raw_physical_device).is_complete()
    })
    .expect("failed to find suitable physical device!");

    // Selected queue families
    let queue_families = vulkan::QueueFamilySet::find(&instance, &physical_device);

    if !queue_families.is_complete() {
        panic!("failed to find complete queue family set!");
    }

    // Device queue create info
    let device_queue_create_info = vulkan::device_queue_create_info::make(&queue_families);

    // Device features
    let physical_device_features = vulkan::physical_device_features::make();

    // Device create info
    let device_create_info = vulkan::device_create_info::make(
        &device_queue_create_info,
        &physical_device_features,
        &validation_layers,
    );

    // Overbite vulkan logical device
    let logical_device =
        vulkan::LogicalDevice::new(&physical_device, &device_create_info, &instance);

    // Overbite vulkan application
    let application = vulkan::Application::new(
        debug_utils_messenger,
        instance,
        logical_device,
        physical_device,
        // TODO This might not be necessary
        queue_families,
    );

    // Window
    let window = Window::new("Overbite", 800, 600, false);

    window.run(application);
}
