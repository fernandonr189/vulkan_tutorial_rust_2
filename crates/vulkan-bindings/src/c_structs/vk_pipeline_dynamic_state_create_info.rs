use crate::{
    VkDynamicState, VkPipelineDynamicStateCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO, create_vk_builder,
};

create_vk_builder!(
    VkPipelineDynamicStateCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    set_dynamic_state_count => dynamicStateCount: u32,
    set_p_dynamic_states => pDynamicStates: *const VkDynamicState => null,
);
