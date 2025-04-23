use crate::{
    VkCommandBuffer, VkPipelineStageFlags, VkSemaphore,
    VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO, VkSubmitInfo, create_info_builder,
};

create_info_builder!(
    VkSubmitInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_SUBMIT_INFO,
    set_wait_semaphore_count => waitSemaphoreCount: u32,
    set_p_wait_semaphores => pWaitSemaphores: &VkSemaphore => null,
    set_p_wait_dst_stage_mask => pWaitDstStageMask: &VkPipelineStageFlags => null,
    set_command_buffer_count => commandBufferCount: u32,
    set_p_command_buffers => pCommandBuffers: &VkCommandBuffer => null,
    set_signal_semaphore_count => signalSemaphoreCount: u32,
    set_p_signal_semaphores => pSignalSemaphores: &VkSemaphore => null,
);
