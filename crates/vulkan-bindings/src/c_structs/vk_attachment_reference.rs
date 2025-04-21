use crate::{VkAttachmentReference, VkImageLayout, state_struct_builder};

state_struct_builder!(
    VkAttachmentReference,
    set_layout => layout: VkImageLayout,
    set_attachment => attachment: u32,
);
