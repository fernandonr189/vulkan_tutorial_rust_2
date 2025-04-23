use crate::{
    VkAttachmentDescription, VkRenderPassCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO, VkSubpassDependency,
    VkSubpassDescription, create_info_builder,
};

create_info_builder!(
    VkRenderPassCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
    set_attachment_count => attachmentCount: u32,
    set_p_attachments => pAttachments: &VkAttachmentDescription,
    set_p_subpasses => pSubpasses: &VkSubpassDescription,
    set_subpass_count => subpassCount: u32,
    set_dependency_count => dependencyCount: u32,
    set_p_dependencies => pDependencies: &VkSubpassDependency,
);
