use crate::{
    VkComponentMapping, VkFormat, VkImage, VkImageSubresourceRange, VkImageViewCreateInfo,
    VkImageViewType, VkStructureType_VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
};

impl Default for VkImageViewCreateInfo {
    fn default() -> Self {
        let mut info: VkImageViewCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        info
    }
}

impl VkImageViewCreateInfo {
    pub fn set_image(&mut self, image: VkImage) {
        self.image = image;
    }
    pub fn set_view_type(&mut self, view_type: VkImageViewType) {
        self.viewType = view_type;
    }
    pub fn set_format(&mut self, format: VkFormat) {
        self.format = format;
    }
    pub fn set_components(&mut self, r: u32, g: u32, b: u32, a: u32) {
        let components = VkComponentMapping { r, g, b, a };
        self.components = components;
    }
    pub fn set_subresource_range(
        &mut self,
        aspect_mask: u32,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) {
        let subresource_range = VkImageSubresourceRange {
            aspectMask: aspect_mask,
            baseMipLevel: base_mip_level,
            levelCount: level_count,
            baseArrayLayer: base_array_layer,
            layerCount: layer_count,
        };
        self.subresourceRange = subresource_range;
    }
}
