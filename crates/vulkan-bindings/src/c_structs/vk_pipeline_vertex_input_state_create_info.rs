use crate::{
    VkPipelineVertexInputStateCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    VkVertexInputBindingDescription,
};

impl Default for VkPipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        let mut info: VkPipelineVertexInputStateCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
        info
    }
}

impl VkPipelineVertexInputStateCreateInfo {
    pub fn set_vertex_binding_description_count(&mut self, vertex_binding_description_count: u32) {
        self.vertexBindingDescriptionCount = vertex_binding_description_count;
    }
    pub fn set_vertex_attribute_description_count(
        &mut self,
        vertex_attribute_description_count: u32,
    ) {
        self.vertexAttributeDescriptionCount = vertex_attribute_description_count;
    }
    pub fn set_p_vertex_binding_descriptions(
        &mut self,
        p_vertex_binding_descriptions: &VkVertexInputBindingDescription,
    ) {
        self.pVertexBindingDescriptions = p_vertex_binding_descriptions;
    }
}
