use crate::{
    VkPipelineVertexInputStateCreateInfo, VkStructureType, VkVertexInputBindingDescription,
};

impl Default for VkPipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkPipelineVertexInputStateCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
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
