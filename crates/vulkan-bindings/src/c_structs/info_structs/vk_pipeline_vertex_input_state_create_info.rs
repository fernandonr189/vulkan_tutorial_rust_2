use crate::{
    VkPipelineVertexInputStateCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    VkVertexInputBindingDescription, create_info_builder,
};

create_info_builder!(
    VkPipelineVertexInputStateCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    set_vertex_binding_description_count => vertexBindingDescriptionCount: u32,
    set_vertex_attribute_description_count => vertexAttributeDescriptionCount: u32,
    set_p_vertex_binding_descriptions => pVertexBindingDescriptions: &VkVertexInputBindingDescription => null,
);
