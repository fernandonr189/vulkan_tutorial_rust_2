use crate::{
    VkPipelineShaderStageCreateInfo, VkShaderModule, VkShaderStageFlagBits,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
};

impl Default for VkPipelineShaderStageCreateInfo {
    fn default() -> Self {
        let mut info: VkPipelineShaderStageCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
        info
    }
}

impl VkPipelineShaderStageCreateInfo {
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
