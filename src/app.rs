use std::ffi::c_char;

use ffi_utils::StringFfi;
use glfw_bindings::{
    self, GLFW_CLIENT_API, GLFW_FALSE, GLFW_NO_API, GLFW_RESIZABLE, GLFWwindow, glfw_create_window,
    glfw_create_window_surface, glfw_destroy_window, glfw_get_required_instance_extensions,
    glfw_init, glfw_poll_events, glfw_terminate, glfw_window_hint, glfw_window_should_close,
};
use vulkan_bindings::{
    VkApplicationInfo, VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateInfo, VkInstance,
    VkInstanceCreateInfo, VkPhysicalDevice,
    VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU, VkQueue,
    VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT, VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VkSurfaceKHR, vk_create_instance,
    vk_create_logical_device, vk_destroy_device, vk_destroy_instance, vk_destroy_surface_khr,
    vk_get_available_devices, vk_get_available_layer_properties, vk_get_device_queue,
    vk_get_physical_device_features, vk_get_physical_device_properties,
    vk_get_physical_device_queue_family_properties, vk_get_physical_device_surface_support_khr,
    vk_get_supported_extensions, vk_make_api_version, vk_make_version,
};

const DEBUG_ENABLED: bool = cfg!(debug_assertions);
static VALIDATION_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];

#[derive(Default)]
pub struct App {
    window: Option<GLFWwindow>,
    vk_instance: Option<VkInstance>,
    vk_physical_device: Option<VkPhysicalDevice>,
    vk_logical_device: Option<VkDevice>,
    vk_queue: Option<VkQueue>,
    vk_surface_khr: Option<VkSurfaceKHR>,
}

impl App {
    pub fn run(self: &mut Self) {
        self.init_window();
        self.init_vulkan();
        self.main_loop();
    }

    // VULKAN FUNCTIONS

    fn init_vulkan(self: &mut Self) {
        self.vk_create_instance();
        self.glfw_create_surface();
        self.vk_pick_physical_device();
        self.vk_create_logical_device();
    }

    fn vk_create_instance(self: &mut Self) {
        if DEBUG_ENABLED && !self.vk_check_validation_layer_support() {
            panic!("Validation layers not available");
        }
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

        // TODO check supported extensions against required extensions
        let (_extension_count, _supported_extensions) = match vk_get_supported_extensions() {
            Ok(extensions) => extensions,
            Err(err) => panic!("Could not obtain supported extensions: {:?}", err),
        };

        self.vk_instance = match vk_create_instance(&instance_create_info) {
            Ok(instance) => Some(instance),
            Err(err) => panic!("Could not create instance: {:?}", err),
        };

        self.vk_check_validation_layer_support();
    }

    fn vk_check_validation_layer_support(self: &mut Self) -> bool {
        let (_layer_count, available_layers) = match vk_get_available_layer_properties() {
            Ok(extensions) => extensions,
            Err(err) => panic!("Could not obtain supported extensions: {:?}", err),
        };

        for validation_layer in VALIDATION_LAYERS {
            let mut layer_found = false;

            for layer_properties in &available_layers {
                if validation_layer.to_string()
                    == StringFfi::from_i8_array(&layer_properties.layerName).to_string()
                {
                    layer_found = true;
                    break;
                }
            }

            if !layer_found {
                return false;
            }
        }
        return true;
    }

    fn vk_pick_physical_device(self: &mut Self) {
        let (_device_count, physical_devices) =
            match vk_get_available_devices(self.vk_instance.unwrap()) {
                Ok(devices) => devices,
                Err(err) => panic!("Could not obtain avaliable devices: {:?}", err),
            };

        for device in &physical_devices {
            if self.vk_is_device_suitable(*device) {
                self.vk_physical_device = Some(*device);
                break;
            }
        }

        if self.vk_physical_device.is_none() {
            panic!("No suitable devices found!")
        }
    }

    fn vk_is_device_suitable(self: &mut Self, device: VkPhysicalDevice) -> bool {
        let device_properties = vk_get_physical_device_properties(device);
        let device_features = vk_get_physical_device_features(device);

        let queue_family_indices = self.vk_find_queue_families(device);

        return device_properties.deviceType
            == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
            && device_features.geometryShader == 1
            && queue_family_indices.graphics_family.is_some();
    }

    fn vk_find_queue_families(self: &mut Self, device: VkPhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::default();

        let queue_families = match vk_get_physical_device_queue_family_properties(device) {
            Ok(families) => families,
            Err(err) => panic!("Failed to get queue families: {:?}", err),
        };

        for (i, queue_family) in queue_families.iter().enumerate() {
            if queue_family.queueFlags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT != 0 {
                indices.graphics_family = Some(i as u32);
            }

            let surface_support = match vk_get_physical_device_surface_support_khr(
                device,
                i as u32,
                self.vk_surface_khr.unwrap(),
            ) {
                Ok(supported) => supported,
                Err(err) => panic!("Failed to get surface support: {:?}", err),
            };

            if surface_support {
                indices.present_family = Some(i as u32);
            }

            if indices.is_complete() {
                break;
            }
        }
        indices
    }

    fn vk_create_logical_device(self: &mut Self) {
        let queue_family_indices = self.vk_find_queue_families(self.vk_physical_device.unwrap());

        let mut queue_create_info = VkDeviceQueueCreateInfo::default();
        queue_create_info.set_s_type(VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO);
        queue_create_info.set_queue_family_index(queue_family_indices.graphics_family.unwrap());
        queue_create_info.set_queue_count(1);
        queue_create_info.set_p_queue_priorities(&[1.0]);

        let mut device_create_info = VkDeviceCreateInfo::default();
        let device_features = vk_get_physical_device_features(self.vk_physical_device.unwrap());
        device_create_info.set_s_type(VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO);
        device_create_info.set_p_queue_create_infos(&queue_create_info);
        device_create_info.set_queue_create_info_count(1);
        device_create_info.set_p_enabled_features(&device_features);
        device_create_info.set_enabled_extension_count(0);

        if DEBUG_ENABLED {
            let strings_ffi: Vec<StringFfi> = VALIDATION_LAYERS
                .iter()
                .map(|&s| StringFfi::from_string(s))
                .collect();
            let layer_ptrs: Vec<*const c_char> = strings_ffi.iter().map(|cs| cs.as_ptr()).collect();
            device_create_info.set_enabled_layer_count(VALIDATION_LAYERS.len() as u32);
            device_create_info.set_pp_enabled_layer_names(layer_ptrs.as_ptr());
        } else {
            device_create_info.set_enabled_layer_count(0);
        }

        self.vk_logical_device =
            match vk_create_logical_device(self.vk_physical_device.unwrap(), &device_create_info) {
                Ok(device) => Some(device),
                Err(err) => panic!("Failed to create logical device: {:?}", err),
            };

        self.vk_queue = Some(vk_get_device_queue(
            self.vk_logical_device.unwrap(),
            queue_family_indices.graphics_family.unwrap(),
            0,
        ));
    }
    // GLFW FUNCTIONS

    fn glfw_create_surface(self: &mut Self) {
        self.vk_surface_khr =
            match glfw_create_window_surface(self.vk_instance.unwrap(), &mut self.window.unwrap()) {
                Ok(surface) => Some(surface),
                Err(err) => panic!("Failed to create surface: {:?}", err),
            }
    }

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
        if self.vk_instance.is_some() && self.vk_surface_khr.is_some() {
            vk_destroy_surface_khr(self.vk_instance.unwrap(), self.vk_surface_khr.unwrap());
            vk_destroy_instance(self.vk_instance.unwrap());
        }
        if self.vk_logical_device.is_some() {
            vk_destroy_device(self.vk_logical_device.unwrap());
        }
        if self.window.is_some() {
            glfw_destroy_window(&mut self.window.unwrap());
        }
        glfw_terminate();
    }
}

impl Drop for App {
    fn drop(self: &mut Self) {
        self.cleanup();
    }
}

#[derive(Default)]
pub struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}
