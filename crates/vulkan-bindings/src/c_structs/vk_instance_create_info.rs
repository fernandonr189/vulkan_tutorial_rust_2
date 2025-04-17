use crate::{VkApplicationInfo, VkInstanceCreateInfo, VkStructureType};

impl Default for VkInstanceCreateInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkInstanceCreateInfo {
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
    }
    pub fn set_p_application_info(&mut self, application_info: &VkApplicationInfo) {
        self.pApplicationInfo = application_info as *const VkApplicationInfo;
    }
    pub fn set_enabled_extension_count(&mut self, extension_count: u32) {
        self.enabledExtensionCount = extension_count;
    }
    pub fn set_pp_enabled_extension_names(&mut self, extension_names_pp: *const *const i8) {
        self.ppEnabledExtensionNames = extension_names_pp;
    }
    pub fn set_enabled_layer_count(&mut self, count: u32) {
        self.enabledLayerCount = count;
    }
}
