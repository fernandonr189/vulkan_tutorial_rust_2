use crate::{
    VkComponentMapping, VkFormat, VkImage, VkImageSubresourceRange, VkImageViewCreateInfo,
    VkImageViewType, VkStructureType,
};

impl Default for VkImageViewCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkImageViewCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
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
