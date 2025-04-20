use crate::{
    VkDescriptorSetLayout, VkPipelineLayoutCreateInfo, VkPushConstantRange,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkPipelineLayoutCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    set_layout_count => setLayoutCount: u32,
    set_p_set_layouts => pSetLayouts: &VkDescriptorSetLayout => null,
    set_push_constant_range_count => pushConstantRangeCount: u32,
    set_p_push_constant_ranges => pPushConstantRanges: &VkPushConstantRange => null,
);
