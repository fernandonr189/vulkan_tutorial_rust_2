use crate::{VkAccessFlags, VkPipelineStageFlags, VkSubpassDependency, state_struct_builder};

state_struct_builder!(
    VkSubpassDependency,
    set_src_stage_mask => srcStageMask: VkPipelineStageFlags,
    set_dst_stage_mask => dstStageMask: VkPipelineStageFlags,
    set_src_access_mask => srcAccessMask: VkAccessFlags,
    set_dst_access_mask => dstAccessMask: VkAccessFlags,
    set_src_subpass => srcSubpass: u32,
    set_dst_subpass => dstSubpass: u32,
);
