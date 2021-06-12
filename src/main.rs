use ash::vk;

use std::ffi::c_void;

use overbite::rendering_engine::vulkan;
use overbite::window::Window;

use vulkan::validation_layers::{
	ENABLE_VALIDATION_LAYERS,
	check_validation_layer_support
};

fn main() {
	// Ash boilerplate
	let entry = unsafe {
		ash::Entry::new()
			.expect("failed to create Ash entry!")
	};

	// Check on validation layers
	if ENABLE_VALIDATION_LAYERS && !check_validation_layer_support(&entry) {
		panic!("validation layers requested but not available!");
	}

	// Application Info
	let app_info = vulkan::application_info::make(
		"Hello Triangle",
		vk::make_version(1, 0, 0)
	);

	// Extensions
	let required_extension_names = vulkan::extensions::required_extension_names();

	// Validation layers
	let validation_layers = vulkan::validation_layers::get_desired_validation_layers();

	// Instance create info
	let mut instance_create_info = vulkan::instance_create_info::make(
		app_info,
		&required_extension_names,
		&validation_layers,
	);

	// Instance debugging
	let instance_debug_create_info = vulkan::debug_utils_messenger_create_info::make();

	if ENABLE_VALIDATION_LAYERS {
		instance_create_info.p_next = &instance_debug_create_info
			as *const vk::DebugUtilsMessengerCreateInfoEXT
			as *const c_void;
	}

	// Overbite vulkan instance
	let instance = vulkan::Instance::new(
		instance_create_info,
		&entry,
	);

	// Application debugging
	let mut debug_utils_messenger: Option<vulkan::DebugUtilsMessenger> = None;

	if ENABLE_VALIDATION_LAYERS {
		let application_debug_create_info = vulkan::debug_utils_messenger_create_info::make();

		debug_utils_messenger = Some(
			vulkan::DebugUtilsMessenger::new(
				application_debug_create_info,
				&entry,
				&instance
			)
		);
	}

	// Overbite vulkan application
	let application = vulkan::Application::new(
		debug_utils_messenger,
		instance,
	);

	// Window
	let window = Window::new("Overbite", 800, 600, false);

	window.run(
		application,
	);
}
