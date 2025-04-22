use crate::{
    VkCommandPoolCreateFlags, VkCommandPoolCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkCommandPoolCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    set_flags => flags: VkCommandPoolCreateFlags,
    set_queue_family_index => queueFamilyIndex: u32,
);
