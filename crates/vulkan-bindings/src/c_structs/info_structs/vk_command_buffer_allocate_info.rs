use crate::{
    VkCommandBufferAllocateInfo, VkCommandBufferLevel, VkCommandPool,
    VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO, create_info_builder,
};

create_info_builder!(
    VkCommandBufferAllocateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    set_command_pool => commandPool: VkCommandPool,
    set_level => level: VkCommandBufferLevel,
    set_command_buffer_count => commandBufferCount: u32,
);
