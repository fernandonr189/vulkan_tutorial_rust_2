use crate::{
    VK_FALSE, VK_TRUE, VkColorSpaceKHR, VkCompositeAlphaFlagBitsKHR, VkExtent2D, VkFormat,
    VkImageUsageFlags, VkPresentModeKHR, VkSharingMode, VkStructureType, VkSurfaceKHR,
    VkSurfaceTransformFlagBitsKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR,
};

impl Default for VkSwapchainCreateInfoKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkSwapchainCreateInfoKHR {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
    pub fn set_surface(&mut self, surface: VkSurfaceKHR) {
        self.surface = surface;
    }
    pub fn set_min_image_count(&mut self, min_image_count: u32) {
        self.minImageCount = min_image_count;
    }
    pub fn set_image_format(&mut self, image_format: VkFormat) {
        self.imageFormat = image_format;
    }
    pub fn set_image_color_space(&mut self, image_color_space: VkColorSpaceKHR) {
        self.imageColorSpace = image_color_space;
    }
    pub fn set_image_extent(&mut self, image_extent: VkExtent2D) {
        self.imageExtent = image_extent;
    }
    pub fn set_image_array_layers(&mut self, image_array_layers: u32) {
        self.imageArrayLayers = image_array_layers;
    }
    pub fn set_image_usage(&mut self, image_usage: VkImageUsageFlags) {
        self.imageUsage = image_usage;
    }
    pub fn set_image_sharing_mode(&mut self, image_sharing_mode: VkSharingMode) {
        self.imageSharingMode = image_sharing_mode;
    }
    pub fn set_queue_family_index_count(&mut self, queue_family_index_count: u32) {
        self.queueFamilyIndexCount = queue_family_index_count;
    }
    pub fn set_p_queue_family_indices(&mut self, queue_family_indices: &[u32]) {
        self.pQueueFamilyIndices = if queue_family_indices.is_empty() {
            std::ptr::null()
        } else {
            queue_family_indices.as_ptr()
        };
    }
    pub fn set_pre_transform(&mut self, pre_transform: VkSurfaceTransformFlagBitsKHR) {
        self.preTransform = pre_transform;
    }
    pub fn set_composite_alpha(&mut self, composite_alpha: VkCompositeAlphaFlagBitsKHR) {
        self.compositeAlpha = composite_alpha;
    }
    pub fn set_present_mode(&mut self, present_mode: VkPresentModeKHR) {
        self.presentMode = present_mode;
    }
    pub fn set_clipped(&mut self, clipped: bool) {
        self.clipped = if clipped { VK_TRUE } else { VK_FALSE };
    }
    pub fn set_old_swapchain(&mut self, old_swapchain: VkSwapchainKHR) {
        self.oldSwapchain = old_swapchain;
    }
}
