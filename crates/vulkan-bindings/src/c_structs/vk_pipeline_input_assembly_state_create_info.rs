use crate::{VkPipelineInputAssemblyStateCreateInfo, VkPrimitiveTopology, VkStructureType};

impl Default for VkPipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkPipelineInputAssemblyStateCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
    pub fn set_topology(&mut self, topology: VkPrimitiveTopology) {
        self.topology = topology;
    }
    pub fn set_primitive_restart_enable(&mut self, primitive_restart_enable: bool) {
        self.primitiveRestartEnable = primitive_restart_enable as u32;
    }
}
