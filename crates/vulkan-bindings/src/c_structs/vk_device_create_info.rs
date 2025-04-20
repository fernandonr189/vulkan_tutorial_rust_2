use crate::{
    VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures,
    VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
};

impl Default for VkDeviceCreateInfo {
    fn default() -> Self {
        let mut info: VkDeviceCreateInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        info
    }
}

impl VkDeviceCreateInfo {
    pub fn set_p_queue_create_infos(
        &mut self,
        p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    ) {
        self.pQueueCreateInfos = p_queue_create_infos;
    }
    pub fn set_queue_create_info_count(&mut self, queue_create_info_count: u32) {
        self.queueCreateInfoCount = queue_create_info_count;
    }
    pub fn set_p_enabled_features(&mut self, device_features: &VkPhysicalDeviceFeatures) {
        self.pEnabledFeatures = device_features;
    }
    pub fn set_enabled_extension_count(&mut self, enabled_extension_count: u32) {
        self.enabledExtensionCount = enabled_extension_count;
    }
    pub fn set_enabled_layer_count(&mut self, enabled_layer_count: u32) {
        self.enabledLayerCount = enabled_layer_count;
    }
    pub fn set_pp_enabled_layer_names(&mut self, enabled_layer_names: *const *const i8) {
        self.ppEnabledLayerNames = enabled_layer_names;
    }
    pub fn set_pp_enabled_extension_names(&mut self, enabled_extension_names: *const *const u8) {
        self.ppEnabledExtensionNames = enabled_extension_names as *const *const i8;
    }
}
