use crate::{
    VkFenceCreateFlags, VkFenceCreateInfo, VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
    create_info_builder,
};

create_info_builder!(
    VkFenceCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
    set_flags => flags: VkFenceCreateFlags,
);
