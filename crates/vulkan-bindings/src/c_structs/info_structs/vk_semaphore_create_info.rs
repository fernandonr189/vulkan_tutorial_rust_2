use std::ffi::c_void;

use crate::{
    VkSemaphoreCreateInfo, VkStructureType_VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
    create_info_builder,
};

create_info_builder!(
    VkSemaphoreCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
    p_next => pNext: *const c_void => null,
);
