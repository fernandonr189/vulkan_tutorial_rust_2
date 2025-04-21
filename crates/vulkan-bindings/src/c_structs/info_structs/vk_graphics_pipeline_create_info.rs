use crate::{
    VkGraphicsPipelineCreateInfo, VkPipeline, VkPipelineColorBlendStateCreateInfo,
    VkPipelineDepthStencilStateCreateInfo, VkPipelineDynamicStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo, VkPipelineLayout, VkPipelineMultisampleStateCreateInfo,
    VkPipelineRasterizationStateCreateInfo, VkPipelineShaderStageCreateInfo,
    VkPipelineVertexInputStateCreateInfo, VkPipelineViewportStateCreateInfo, VkRenderPass,
    VkStructureType_VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkGraphicsPipelineCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
    set_stage_count => stageCount: u32,
    set_p_stages => pStages: *const VkPipelineShaderStageCreateInfo => null,
    set_p_vertex_input_state => pVertexInputState: &VkPipelineVertexInputStateCreateInfo => null,
    set_p_input_assembly_state => pInputAssemblyState: &VkPipelineInputAssemblyStateCreateInfo => null,
    set_p_viewport_state => pViewportState: &VkPipelineViewportStateCreateInfo => null,
    set_p_rasterization_state => pRasterizationState: &VkPipelineRasterizationStateCreateInfo => null,
    set_p_multisample_state => pMultisampleState: &VkPipelineMultisampleStateCreateInfo => null,
    set_p_depth_stencil_state => pDepthStencilState: &VkPipelineDepthStencilStateCreateInfo => null,
    set_p_color_blend_state => pColorBlendState: &VkPipelineColorBlendStateCreateInfo => null,
    set_p_dynamic_state => pDynamicState: &VkPipelineDynamicStateCreateInfo => null,
    set_layout => layout: VkPipelineLayout,
    set_render_pass => renderPass: VkRenderPass,
    set_subpass => subpass: u32,
    set_base_pipeline_handle => basePipelineHandle: VkPipeline,
    set_base_pipeline_index => basePipelineIndex: i32,
);
