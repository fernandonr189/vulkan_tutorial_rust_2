use crate::{
    VkDeviceQueueCreateInfo, VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    create_vk_builder,
};

create_vk_builder!(
    VkDeviceQueueCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    set_queue_family_index => queueFamilyIndex: u32,
    set_queue_count => queueCount: u32,
    set_p_queue_priorities => pQueuePriorities: *const f32 => null,
);
