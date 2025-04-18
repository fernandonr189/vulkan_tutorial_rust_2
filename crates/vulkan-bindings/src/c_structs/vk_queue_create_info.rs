use crate::{VkDeviceQueueCreateInfo, VkStructureType};

impl Default for VkDeviceQueueCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkDeviceQueueCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
    pub fn set_queue_family_index(&mut self, index: u32) {
        self.queueFamilyIndex = index;
    }
    pub fn set_queue_count(&mut self, count: u32) {
        self.queueCount = count;
    }
    pub fn set_p_queue_priorities(&mut self, priorities: &[f32]) {
        self.pQueuePriorities = priorities.as_ptr();
    }
}
