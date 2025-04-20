use crate::{VkApplicationInfo, VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO};

impl Default for VkApplicationInfo {
    fn default() -> Self {
        let mut info: VkApplicationInfo = unsafe { std::mem::zeroed() };
        info.sType = VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO;
        info
    }
}

impl VkApplicationInfo {
    pub fn set_p_application_name(&mut self, name: &str) {
        self.pApplicationName = name.as_ptr() as *const i8;
    }
    pub fn set_p_engine_name(&mut self, name: &str) {
        self.pEngineName = name.as_ptr() as *const i8;
    }
    pub fn set_application_version(&mut self, version: u32) {
        self.applicationVersion = version;
    }
    pub fn set_engine_version(&mut self, version: u32) {
        self.engineVersion = version;
    }
    pub fn set_api_version(&mut self, version: u32) {
        self.apiVersion = version;
    }
}
