use crate::{
    VkPipelineShaderStageCreateInfo, VkShaderModule, VkShaderStageFlagBits,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkPipelineShaderStageCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    set_stage => stage: VkShaderStageFlagBits,
    set_module => module: VkShaderModule,
    set_p_name => pName: *const i8 => null,
);
