use ash::vk;

use std::{ffi::CString, ptr};

pub fn make(application_name: &str, application_version: u32) -> vk::ApplicationInfo {
    let c_application_name = CString::new(application_name).unwrap();
    let p_application_name = c_application_name.as_ptr();

    let c_engine_name = CString::new("Overbite").unwrap();
    let p_engine_name = c_engine_name.as_ptr();

    vk::ApplicationInfo {
        s_type: vk::StructureType::APPLICATION_INFO,
        p_application_name,
        application_version,
        p_engine_name,
        engine_version: vk::make_version(1, 0, 0),
        api_version: vk::API_VERSION_1_0,
        p_next: ptr::null(),
    }
}
