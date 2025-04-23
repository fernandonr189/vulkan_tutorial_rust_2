use crate::{
    VkPresentInfoKHR, VkSemaphore, VkStructureType_VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
    VkSwapchainKHR, create_info_builder,
};

create_info_builder!(
    VkPresentInfoKHR,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
    set_wait_semaphore_count => waitSemaphoreCount: u32,
    set_p_wait_semaphores => pWaitSemaphores: &VkSemaphore,
    set_swapchain_count => swapchainCount: u32,
    set_p_swapchains => pSwapchains: &VkSwapchainKHR,
    set_p_image_indices => pImageIndices: &u32 => null,
);
