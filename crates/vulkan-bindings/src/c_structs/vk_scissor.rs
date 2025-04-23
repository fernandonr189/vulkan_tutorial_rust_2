use crate::{VkExtent2D, VkOffset2D, VkRect2D, state_struct_builder};

state_struct_builder!(
    VkRect2D,
    set_offset => offset: VkOffset2D,
    set_extent => extent: VkExtent2D,
);
