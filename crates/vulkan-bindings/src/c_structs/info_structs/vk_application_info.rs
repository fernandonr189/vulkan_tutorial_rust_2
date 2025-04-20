use crate::create_vk_builder;
use crate::{VkApplicationInfo, VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO};

create_vk_builder!(
    VkApplicationInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
    set_p_application_name => pApplicationName: *const i8 => null,
    set_p_engine_name => pEngineName: *const i8 => null,
    set_application_version => applicationVersion: u32,
    set_engine_version => engineVersion: u32,
    set_api_version => apiVersion: u32,
);
