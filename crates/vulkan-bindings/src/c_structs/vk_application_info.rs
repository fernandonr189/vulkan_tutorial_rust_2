use crate::{VkApplicationInfo, VkStructureType};

impl Default for VkApplicationInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl VkApplicationInfo {
    pub fn set_p_application_name(&mut self, name: &str) {
        self.pApplicationName = name.as_ptr() as *const i8;
    }
    pub fn set_s_type(&mut self, s_type: VkStructureType) {
        self.sType = s_type;
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
