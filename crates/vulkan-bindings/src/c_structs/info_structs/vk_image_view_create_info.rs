use crate::{
    VkComponentMapping, VkFormat, VkImage, VkImageSubresourceRange, VkImageViewCreateInfo,
    VkImageViewType, VkStructureType_VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO, create_vk_builder,
};

create_vk_builder!(
    VkImageViewCreateInfo,
    s_type: VkStructureType_VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
    set_image => image: VkImage,
    set_view_type => viewType: VkImageViewType,
    set_format => format: VkFormat,
    set_components => components: VkComponentMapping,
    set_subresource_range => subresourceRange: VkImageSubresourceRange,
);
