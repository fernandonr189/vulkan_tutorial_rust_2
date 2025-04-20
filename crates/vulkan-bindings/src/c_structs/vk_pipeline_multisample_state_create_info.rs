use crate::{
    VkPipelineMultisampleStateCreateInfo, VkSampleCountFlagBits,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO, create_vk_builder,
};

create_vk_builder!(
    VkPipelineMultisampleStateCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    set_sample_shading_enable => sampleShadingEnable: u32,
    set_rasterization_samples => rasterizationSamples: VkSampleCountFlagBits,
    set_min_sample_shading => minSampleShading: f32,
    set_p_sample_mask => pSampleMask: *const u32 => null,
    set_alpha_to_coverage_enable => alphaToCoverageEnable: u32,
    set_alpha_to_one_enable => alphaToOneEnable: u32,
);
