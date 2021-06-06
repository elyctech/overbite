use ash::vk;

use overbite::rendering_engine::vulkan;
use overbite::window::Window;

fn main() {
	// Ash boilerplate
	let entry = unsafe {
		ash::Entry::new()
			.expect("failed to create Ash entry!")
	};

	// Application Info
	let app_info = vulkan::application_info::make(
		"Hello Triangle",
		vk::make_version(1, 0, 0)
	);

	// Extensions
	let required_extension_names = vulkan::extensions::required_extension_names();

	// Instance create info
	let instance_create_info = vulkan::instance_create_info::make(
		&app_info,
		&required_extension_names,
	);

	// Overbite vulkan instance
	let instance = vulkan::Instance::new(
		&instance_create_info,
		&entry,
	);

	// Overbite vulkan application
	let application = vulkan::Application::new(
		instance,
	);

	// Window
	let window = Window::new("Overbite", 800, 600, false);

	window.run(
		application,
	);
}
