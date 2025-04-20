use crate::{
    VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures,
    VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkDeviceCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    set_p_queue_create_infos => pQueueCreateInfos: *const VkDeviceQueueCreateInfo => null,
    set_queue_create_info_count => queueCreateInfoCount: u32,
    set_p_enabled_features => pEnabledFeatures: *const VkPhysicalDeviceFeatures => null,
    set_enabled_extension_count => enabledExtensionCount: u32,
    set_enabled_layer_count => enabledLayerCount: u32,
    set_pp_enabled_layer_names => ppEnabledLayerNames: *const *const i8 => null,
    set_pp_enabled_extension_names => ppEnabledExtensionNames: *const *const i8 => null,
);
