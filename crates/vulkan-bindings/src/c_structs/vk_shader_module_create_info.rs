use crate::{VkShaderModuleCreateInfo, VkStructureType};

impl Default for VkShaderModuleCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkShaderModuleCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
    pub fn set_code_size(&mut self, code_size: usize) {
        self.codeSize = code_size;
    }
    pub fn set_p_code(&mut self, p_code: *const u32) {
        self.pCode = p_code;
    }
}
