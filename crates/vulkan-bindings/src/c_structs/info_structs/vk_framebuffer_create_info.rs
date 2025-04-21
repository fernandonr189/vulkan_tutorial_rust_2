use crate::{
    VkFramebufferCreateInfo, VkImageView, VkRenderPass,
    VkStructureType_VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO, create_info_builder,
};

create_info_builder!(
    VkFramebufferCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
    set_render_pass => renderPass: VkRenderPass,
    set_p_attachments => pAttachments: *const VkImageView => null,
    set_attachment_count => attachmentCount: u32,
    set_width => width: u32,
    set_height => height: u32,
    set_layers => layers: u32,
);
