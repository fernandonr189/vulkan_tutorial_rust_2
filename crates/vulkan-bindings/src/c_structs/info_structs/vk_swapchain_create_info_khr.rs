use crate::{
    VkColorSpaceKHR, VkCompositeAlphaFlagBitsKHR, VkExtent2D, VkFormat, VkImageUsageFlags,
    VkPresentModeKHR, VkSharingMode, VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
    VkSurfaceKHR, VkSurfaceTransformFlagBitsKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR,
    create_vk_builder,
};

create_vk_builder!(
    VkSwapchainCreateInfoKHR,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
    set_surface => surface: VkSurfaceKHR,
    set_min_image_count => minImageCount: u32,
    set_image_format => imageFormat: VkFormat,
    set_image_color_space => imageColorSpace: VkColorSpaceKHR,
    set_image_extent => imageExtent: VkExtent2D,
    set_image_array_layers => imageArrayLayers: u32,
    set_image_usage => imageUsage: VkImageUsageFlags,
    set_image_sharing_mode => imageSharingMode: VkSharingMode,
    set_queue_family_index_count => queueFamilyIndexCount: u32,
    set_p_queue_family_indices => pQueueFamilyIndices: *const u32 => null,
    set_pre_transform => preTransform: VkSurfaceTransformFlagBitsKHR,
    set_composite_alpha => compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    set_present_mode => presentMode: VkPresentModeKHR,
    set_clipped => clipped: u32,
    set_old_swapchain => oldSwapchain: VkSwapchainKHR,
);
