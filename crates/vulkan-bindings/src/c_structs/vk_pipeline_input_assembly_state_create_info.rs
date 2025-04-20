use crate::{
    VkPipelineInputAssemblyStateCreateInfo, VkPrimitiveTopology,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
};

impl Default for VkPipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        let mut info: VkPipelineInputAssemblyStateCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
        info
    }
}

impl VkPipelineInputAssemblyStateCreateInfo {
    pub fn set_topology(&mut self, topology: VkPrimitiveTopology) {
        self.topology = topology;
    }
    pub fn set_primitive_restart_enable(&mut self, primitive_restart_enable: bool) {
        self.primitiveRestartEnable = primitive_restart_enable as u32;
    }
}
