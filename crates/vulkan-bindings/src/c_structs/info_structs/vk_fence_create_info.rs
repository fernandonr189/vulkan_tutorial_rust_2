use std::ffi::c_void;

use crate::{
    VkFenceCreateInfo, VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkFenceCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
    p_next => pNext: *const c_void => null,
);
