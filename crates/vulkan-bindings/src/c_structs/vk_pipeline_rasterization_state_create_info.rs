use crate::{
    VkBool32, VkCullModeFlags, VkFrontFace, VkPipelineRasterizationStateCreateInfo, VkPolygonMode,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO, create_vk_builder,
};

create_vk_builder!(
    VkPipelineRasterizationStateCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    set_polygon_mode => polygonMode: VkPolygonMode,
    set_cull_mode => cullMode: VkCullModeFlags,
    set_front_face => frontFace: VkFrontFace,
    set_depth_clamp_enable => depthClampEnable: VkBool32,
    set_rasterizer_discard_enable => rasterizerDiscardEnable: VkBool32,
    set_depth_bias_enable => depthBiasEnable: VkBool32,
    set_depth_bias_constant_factor => depthBiasConstantFactor: f32,
    set_depth_bias_clamp => depthBiasClamp: f32,
    set_depth_bias_slope_factor => depthBiasSlopeFactor: f32,
    set_line_width => lineWidth: f32,
);
