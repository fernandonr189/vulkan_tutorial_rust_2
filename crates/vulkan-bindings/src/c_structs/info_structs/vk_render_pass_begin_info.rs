use crate::{
    VkClearValue, VkFramebuffer, VkRect2D, VkRenderPass, VkRenderPassBeginInfo,
    VkStructureType_VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO, create_info_builder,
};

create_info_builder!(
    VkRenderPassBeginInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
    set_render_pass => renderPass: VkRenderPass,
    set_framebuffer => framebuffer: VkFramebuffer,
    set_render_area => renderArea: VkRect2D,
    set_clear_values => pClearValues: *const VkClearValue,
    set_clear_value_count => clearValueCount: u32,
);
