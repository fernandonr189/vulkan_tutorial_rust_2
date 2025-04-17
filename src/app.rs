use ffi_utils::StringFfi;
use glfw_bindings::{
    self, GLFW_CLIENT_API, GLFW_FALSE, GLFW_NO_API, GLFW_RESIZABLE, GLFWwindow, glfw_create_window,
    glfw_destroy_window, glfw_get_required_instance_extensions, glfw_init, glfw_poll_events,
    glfw_terminate, glfw_window_hint, glfw_window_should_close,
};
use vulkan_bindings::{
    VkApplicationInfo, VkInstance, VkInstanceCreateInfo,
    VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, vk_create_instance,
    vk_destroy_instance, vk_get_supported_extensions, vk_make_api_version, vk_make_version,
};

#[derive(Default)]
pub struct App {
    window: Option<GLFWwindow>,
    vk_instance: Option<VkInstance>,
}

impl App {
    pub fn run(self: &mut Self) {
        self.init_window();
        self.vk_create_instance();
        self.main_loop();
    }

    // VULKAN FUNCTIONS

    fn vk_create_instance(self: &mut Self) {
        let mut app_info = VkApplicationInfo::default();
        app_info.set_s_type(VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO);
        app_info.set_p_application_name("Hello Triangle");
        app_info.set_application_version(vk_make_version(1, 0, 0));
        app_info.set_p_engine_name("No Engine");
        app_info.set_engine_version(vk_make_version(1, 0, 0));
        app_info.set_api_version(vk_make_api_version(0, 1, 0, 0));

        let (extension_count, extensions) = glfw_get_required_instance_extensions()
            .expect("Could not get glfw required extensions");
        let mut instance_create_info = VkInstanceCreateInfo::default();
        instance_create_info.set_s_type(VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO);
        instance_create_info.set_p_application_info(&app_info);
        instance_create_info.set_enabled_extension_count(extension_count);
        instance_create_info.set_pp_enabled_extension_names(extensions);
        instance_create_info.set_enabled_layer_count(0);

        self.vk_instance = match vk_create_instance(&instance_create_info) {
            Ok(instance) => Some(instance),
            Err(err) => panic!("Could not create instance: {:?}", err),
        };

        let (extension_count, supported_extensions) = match vk_get_supported_extensions() {
            Ok(extensions) => extensions,
            Err(err) => panic!("Could not obtain supported extensions: {:?}", err),
        };

        for extension in supported_extensions {
            println!(
                "Supported: '{}'",
                StringFfi::from_i8_array(&extension.extensionName)
            )
        }
    }

    // GLFW FUNCTIONS

    fn init_window(self: &mut Self) {
        glfw_init();
        glfw_window_hint(GLFW_CLIENT_API, GLFW_NO_API);
        glfw_window_hint(GLFW_RESIZABLE, GLFW_FALSE);
        self.window = match glfw_create_window(600, 800, "Vulkan") {
            Ok(window) => Some(window),
            Err(err) => panic!("Failed to create window: {:?}", err),
        };
    }

    fn main_loop(self: &mut Self) {
        while !glfw_window_should_close(&mut self.window.unwrap()) {
            glfw_poll_events();
        }
    }

    fn cleanup(self: &mut Self) {
        vk_destroy_instance(self.vk_instance.unwrap());
        glfw_destroy_window(&mut self.window.unwrap());
        glfw_terminate();
    }
}

impl Drop for App {
    fn drop(self: &mut Self) {
        self.cleanup();
    }
}
