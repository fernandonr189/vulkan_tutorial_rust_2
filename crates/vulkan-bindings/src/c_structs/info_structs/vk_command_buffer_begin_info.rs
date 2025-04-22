use crate::{
    VkCommandBufferBeginInfo, VkCommandBufferInheritanceInfo, VkCommandBufferUsageFlags,
    VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO, create_info_builder,
};

create_info_builder!(
    VkCommandBufferBeginInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
    set_flags => flags: VkCommandBufferUsageFlags,
    set_p_inheritance_info => pInheritanceInfo: *const VkCommandBufferInheritanceInfo => null,
);
