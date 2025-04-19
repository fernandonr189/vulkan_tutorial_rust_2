use crate::{VkDynamicState, VkPipelineDynamicStateCreateInfo, VkStructureType};

impl Default for VkPipelineDynamicStateCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkPipelineDynamicStateCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
    pub fn set_dynamic_state_count(&mut self, count: u32) {
        self.dynamicStateCount = count;
    }
    pub fn set_p_dynamic_states(&mut self, states: &[VkDynamicState]) {
        self.pDynamicStates = states.as_ptr();
    }
}
