use crate::{
    VkDynamicState, VkPipelineDynamicStateCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
};

impl Default for VkPipelineDynamicStateCreateInfo {
    fn default() -> Self {
        let mut info: VkPipelineDynamicStateCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
        info
    }
}

impl VkPipelineDynamicStateCreateInfo {
    pub fn set_dynamic_state_count(&mut self, count: u32) {
        self.dynamicStateCount = count;
    }
    pub fn set_p_dynamic_states(&mut self, states: &[VkDynamicState]) {
        self.pDynamicStates = states.as_ptr();
    }
}
