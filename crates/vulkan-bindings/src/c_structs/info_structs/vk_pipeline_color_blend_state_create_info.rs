use crate::{
    VkBool32, VkLogicOp, VkPipelineColorBlendAttachmentState, VkPipelineColorBlendStateCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkPipelineColorBlendStateCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
    set_logic_op_enable => logicOpEnable: VkBool32,
    set_logic_op => logicOp: VkLogicOp,
    set_blend_constants => blendConstants: [f32; 4],
    set_p_attachments => pAttachments: &VkPipelineColorBlendAttachmentState => null,
    set_attachment_count => attachmentCount: u32,
);
