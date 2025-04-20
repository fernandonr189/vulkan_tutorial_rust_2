use crate::{
    VkShaderModuleCreateInfo, VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
};

impl Default for VkShaderModuleCreateInfo {
    fn default() -> Self {
        let mut info: VkShaderModuleCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
        info
    }
}

impl VkShaderModuleCreateInfo {
    pub fn set_code_size(&mut self, code_size: usize) {
        self.codeSize = code_size;
    }
    pub fn set_p_code(&mut self, p_code: *const u32) {
        self.pCode = p_code;
    }
}
