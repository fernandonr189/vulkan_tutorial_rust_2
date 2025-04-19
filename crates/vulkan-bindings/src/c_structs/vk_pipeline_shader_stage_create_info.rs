use crate::{
    VkPipelineShaderStageCreateInfo, VkShaderModule, VkShaderStageFlagBits, VkStructureType,
};

impl Default for VkPipelineShaderStageCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkPipelineShaderStageCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
    pub fn set_stage(&mut self, stage: VkShaderStageFlagBits) {
        self.stage = stage;
    }
    pub fn set_module(&mut self, module: VkShaderModule) {
        self.module = module;
    }
    pub fn set_p_name(&mut self, p_name: *const u8) {
        self.pName = p_name as *const i8;
    }
}
