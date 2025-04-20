use crate::{VkDeviceQueueCreateInfo, VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO};

impl Default for VkDeviceQueueCreateInfo {
    fn default() -> Self {
        let mut info: VkDeviceQueueCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        info
    }
}

impl VkDeviceQueueCreateInfo {
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
