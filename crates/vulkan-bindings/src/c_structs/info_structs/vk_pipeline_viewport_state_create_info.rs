use crate::{
    VkPipelineViewportStateCreateInfo, VkRect2D,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO, VkViewport,
    create_info_builder,
};

create_info_builder!(
    VkPipelineViewportStateCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
    set_viewport_count => viewportCount: u32,
    set_p_viewports => pViewports: &VkViewport => null,
    set_scissor_count => scissorCount: u32,
    set_p_scissors => pScissors: &VkRect2D => null,
);
