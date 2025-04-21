use crate::{
    VkAttachmentDescription, VkAttachmentLoadOp, VkAttachmentStoreOp, VkFormat, VkImageLayout,
    VkSampleCountFlags, state_struct_builder,
};

state_struct_builder!(
    VkAttachmentDescription,
    set_format => format: VkFormat,
    set_samples => samples: VkSampleCountFlags,
    set_load_op => loadOp: VkAttachmentLoadOp,
    set_store_op => storeOp: VkAttachmentStoreOp,
    set_initial_layout => initialLayout: VkImageLayout,
    set_final_layout => finalLayout: VkImageLayout,
    set_stencil_load_op => stencilLoadOp: VkAttachmentLoadOp,
    set_stencil_store_op => stencilStoreOp: VkAttachmentStoreOp,
);
