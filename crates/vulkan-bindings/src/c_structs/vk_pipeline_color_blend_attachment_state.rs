use crate::{
    VkBlendFactor, VkBlendOp, VkBool32, VkColorComponentFlagBits,
    VkPipelineColorBlendAttachmentState, state_struct_builder,
};

state_struct_builder!(
    VkPipelineColorBlendAttachmentState,
    set_color_blend_op => colorBlendOp: VkBlendOp,
    set_src_blend_color_factor => srcColorBlendFactor: VkBlendFactor,
    set_dst_blend_color_factor => dstColorBlendFactor: VkBlendFactor,
    set_src_blend_alpha_factor => srcAlphaBlendFactor: VkBlendFactor,
    set_dst_blend_alpha_factor => dstAlphaBlendFactor: VkBlendFactor,
    set_alpha_blend_op => alphaBlendOp: VkBlendOp,
    set_blend_enable => blendEnable: VkBool32,
    set_color_write_mask => colorWriteMask: VkColorComponentFlagBits,
);
