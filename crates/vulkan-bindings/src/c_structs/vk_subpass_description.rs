use crate::{
    VkAttachmentReference, VkPipelineBindPoint, VkSubpassDescription, state_struct_builder,
};

state_struct_builder!(
    VkSubpassDescription,
    set_pipeline_bind_point => pipelineBindPoint: VkPipelineBindPoint,
    set_color_attachment_count => colorAttachmentCount: u32,
    set_p_color_attachments => pColorAttachments: &VkAttachmentReference,
);
