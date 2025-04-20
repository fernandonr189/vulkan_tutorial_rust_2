use crate::{
    VkApplicationInfo, VkInstanceCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkInstanceCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    set_p_application_info => pApplicationInfo: &VkApplicationInfo => null,
    set_enabled_extension_count => enabledExtensionCount: u32,
    set_pp_enabled_extension_names => ppEnabledExtensionNames: *const *const i8 => null,
    set_enabled_layer_count => enabledLayerCount: u32,
);
