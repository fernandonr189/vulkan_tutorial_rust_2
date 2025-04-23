use crate::{VkViewport, state_struct_builder};

state_struct_builder!(
    VkViewport,
    set_x => x: f32,
    set_y => y: f32,
    set_width => width: f32,
    set_height => height: f32,
    set_min_depth => minDepth: f32,
    set_max_depth => maxDepth: f32,
);
