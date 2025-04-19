use std::{
    cmp::{max, min},
    collections::HashSet,
    ffi::c_char,
    ptr::null_mut,
};

use ffi_utils::StringFfi;
use glfw_bindings::{
    self, GLFW_CLIENT_API, GLFW_FALSE, GLFW_NO_API, GLFW_RESIZABLE, GLFWwindow, glfw_create_window,
    glfw_create_window_surface, glfw_destroy_window, glfw_get_framebuffer_size,
    glfw_get_required_instance_extensions, glfw_init, glfw_poll_events, glfw_terminate,
    glfw_window_hint, glfw_window_should_close,
};
use vulkan_bindings::{
    VK_KHR_SWAPCHAIN_EXTENSION_NAME, VkApplicationInfo,
    VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
    VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR, VkDevice, VkDeviceCreateInfo,
    VkDeviceQueueCreateInfo, VkExtent2D, VkFormat, VkFormat_VK_FORMAT_B8G8R8A8_SRGB, VkImage,
    VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT, VkInstance, VkInstanceCreateInfo,
    VkPhysicalDevice, VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU, VkPresentModeKHR,
    VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR, VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR,
    VkQueue, VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT, VkSharingMode_VK_SHARING_MODE_CONCURRENT,
    VkSharingMode_VK_SHARING_MODE_EXCLUSIVE, VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR, VkSurfaceCapabilitiesKHR,
    VkSurfaceFormatKHR, VkSurfaceKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR, vk_create_instance,
    vk_create_logical_device, vk_create_swapchain_khr, vk_destroy_device, vk_destroy_instance,
    vk_destroy_surface_khr, vk_destroy_swapchain_khr, vk_get_available_devices,
    vk_get_available_layer_properties, vk_get_device_extensions_properties, vk_get_device_queue,
    vk_get_physical_device_features, vk_get_physical_device_properties,
    vk_get_physical_device_queue_family_properties,
    vk_get_physical_device_surface_capabilities_khr, vk_get_physical_device_surface_formats_khr,
    vk_get_physical_device_surface_present_modes_khr, vk_get_physical_device_surface_support_khr,
    vk_get_supported_extensions, vk_get_swapchain_images_khr, vk_make_api_version, vk_make_version,
};

const DEBUG_ENABLED: bool = cfg!(debug_assertions);
static VALIDATION_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
static REQUIRED_EXTENSIONS: &[&[u8]] = &[VK_KHR_SWAPCHAIN_EXTENSION_NAME];

#[derive(Default)]
pub struct App {
    window: Option<*mut GLFWwindow>,
    vk_instance: Option<VkInstance>,
    vk_physical_device: Option<VkPhysicalDevice>,
    vk_logical_device: Option<VkDevice>,
    vk_surface_khr: Option<VkSurfaceKHR>,
    vk_graphics_queue: Option<VkQueue>,
    vk_present_queue: Option<VkQueue>,
    vk_swap_chain: Option<VkSwapchainKHR>,
    vk_swap_chain_images: Vec<VkImage>,
    vk_swap_chain_image_format: Option<VkFormat>,
    vk_swap_chain_image_extent: Option<VkExtent2D>,
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
        self.vk_create_swap_chain();
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

        let extensions_supported = self.vk_check_device_extension_support(device);

        let swapchain_support_details = self.vk_query_swapchain_support(device);

        return device_properties.deviceType
            == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
            && device_features.geometryShader == 1
            && queue_family_indices.is_complete()
            && extensions_supported
            && swapchain_support_details.is_swapchain_adequate();
    }

    fn vk_check_device_extension_support(self: &mut Self, device: VkPhysicalDevice) -> bool {
        let extensions = match vk_get_device_extensions_properties(device) {
            Ok(extensions) => extensions,
            Err(_) => todo!(),
        };

        let required_extensions: Vec<StringFfi> = REQUIRED_EXTENSIONS
            .iter()
            .map(|s| StringFfi::from_u8_array(*s))
            .collect();

        let mut required_extensions_set: HashSet<StringFfi> = HashSet::new();

        for extension in required_extensions {
            required_extensions_set.insert(extension);
        }

        for extension in extensions {
            required_extensions_set.remove(&StringFfi::from_i8_array(&extension.extensionName));
        }

        required_extensions_set.is_empty()
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

        let mut queue_create_infos: Vec<VkDeviceQueueCreateInfo> = Vec::new();
        let mut unique_families = HashSet::<u32>::new();
        unique_families.insert(queue_family_indices.graphics_family.unwrap());
        unique_families.insert(queue_family_indices.present_family.unwrap());

        let priority = 1.0;
        for family in unique_families {
            let mut queue_create_info = VkDeviceQueueCreateInfo::default();
            queue_create_info
                .set_s_type(VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO);
            queue_create_info.set_queue_family_index(family);
            queue_create_info.set_queue_count(1);
            queue_create_info.set_p_queue_priorities(&[priority]);
            queue_create_infos.push(queue_create_info);
        }

        let extension_names: Vec<*const u8> =
            REQUIRED_EXTENSIONS.iter().map(|s| s.as_ptr()).collect();

        let mut device_create_info = VkDeviceCreateInfo::default();
        let device_features = vk_get_physical_device_features(self.vk_physical_device.unwrap());
        device_create_info.set_s_type(VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO);
        device_create_info.set_p_queue_create_infos(queue_create_infos.as_ptr());
        device_create_info.set_queue_create_info_count(queue_create_infos.len() as u32);
        device_create_info.set_p_enabled_features(&device_features);
        device_create_info.set_enabled_extension_count(extension_names.len() as u32);
        device_create_info.set_pp_enabled_extension_names(extension_names.as_ptr());

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

        self.vk_graphics_queue = Some(vk_get_device_queue(
            self.vk_logical_device.unwrap(),
            queue_family_indices.graphics_family.unwrap(),
            0,
        ));

        self.vk_present_queue = Some(vk_get_device_queue(
            self.vk_logical_device.unwrap(),
            queue_family_indices.present_family.unwrap(),
            0,
        ));
    }

    fn vk_query_swapchain_support(
        self: &mut Self,
        device: VkPhysicalDevice,
    ) -> SwapChainSupportDetails {
        let surface_capabilities = match vk_get_physical_device_surface_capabilities_khr(
            device,
            self.vk_surface_khr.unwrap(),
        ) {
            Ok(capabilities) => capabilities,
            Err(err) => panic!("Failed to query swapchain support: {:?}", err),
        };
        let mut swapchain_support_details = SwapChainSupportDetails::new(surface_capabilities);

        swapchain_support_details.formats = match vk_get_physical_device_surface_formats_khr(
            device,
            self.vk_surface_khr.unwrap(),
        ) {
            Ok(formats) => formats,
            Err(err) => panic!("Failed to query swapchain formats: {:?}", err),
        };

        swapchain_support_details.present_modes =
            match vk_get_physical_device_surface_present_modes_khr(
                device,
                self.vk_surface_khr.unwrap(),
            ) {
                Ok(present_modes) => present_modes,
                Err(err) => panic!("Failed to query swapchain present modes: {:?}", err),
            };
        swapchain_support_details
    }

    fn vk_create_swap_chain(&mut self) {
        let swapchain_support_details =
            self.vk_query_swapchain_support(self.vk_physical_device.unwrap());

        let surface_format = self.vk_choose_swap_surface_format(swapchain_support_details.formats);
        let present_mode =
            self.vk_choose_swap_present_mode(swapchain_support_details.present_modes);
        let extent = self.vk_choose_swap_extent(swapchain_support_details.capabilities);

        let mut image_count = swapchain_support_details.capabilities.minImageCount + 1;

        if swapchain_support_details.capabilities.maxImageCount > 0
            && image_count > swapchain_support_details.capabilities.maxImageCount
        {
            image_count = swapchain_support_details.capabilities.maxImageCount;
        }

        let mut swapchain_create_info = VkSwapchainCreateInfoKHR::default();
        swapchain_create_info
            .set_s_type(VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR);
        swapchain_create_info.set_surface(self.vk_surface_khr.unwrap());
        swapchain_create_info.set_min_image_count(image_count);
        swapchain_create_info.set_image_color_space(surface_format.colorSpace);
        swapchain_create_info.set_image_extent(extent);
        swapchain_create_info.set_image_array_layers(1);
        swapchain_create_info
            .set_image_usage(VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT);

        let queue_family_indices = self.vk_find_queue_families(self.vk_physical_device.unwrap());

        if queue_family_indices.graphics_family.unwrap()
            != queue_family_indices.present_family.unwrap()
        {
            swapchain_create_info.set_image_sharing_mode(VkSharingMode_VK_SHARING_MODE_CONCURRENT);
            swapchain_create_info.set_queue_family_index_count(2);
            swapchain_create_info.set_p_queue_family_indices(&[
                queue_family_indices.graphics_family.unwrap(),
                queue_family_indices.present_family.unwrap(),
            ]);
        } else {
            swapchain_create_info.set_image_sharing_mode(VkSharingMode_VK_SHARING_MODE_EXCLUSIVE);
            swapchain_create_info.set_queue_family_index_count(0);
            swapchain_create_info.set_p_queue_family_indices(&[]);
        }
        swapchain_create_info
            .set_pre_transform(swapchain_support_details.capabilities.currentTransform);
        swapchain_create_info
            .set_composite_alpha(VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR);
        swapchain_create_info.set_present_mode(present_mode);
        swapchain_create_info.set_clipped(true);
        swapchain_create_info.set_old_swapchain(null_mut());

        self.vk_swap_chain =
            match vk_create_swapchain_khr(self.vk_logical_device.unwrap(), swapchain_create_info) {
                Ok(swap_chain) => Some(swap_chain),
                Err(err) => panic!("Failed to create swap chain: {:?}", err),
            };

        self.vk_swap_chain_images = match vk_get_swapchain_images_khr(
            self.vk_logical_device.unwrap(),
            self.vk_swap_chain.unwrap(),
        ) {
            Ok(images) => images,
            Err(err) => panic!("Failed to get swap chain images: {:?}", err),
        };

        self.vk_swap_chain_image_format = Some(surface_format.format);
        self.vk_swap_chain_image_extent = Some(extent);
    }

    fn vk_choose_swap_surface_format(
        self: &mut Self,
        available_formats: Vec<VkSurfaceFormatKHR>,
    ) -> VkSurfaceFormatKHR {
        for format in &available_formats {
            if format.format == VkFormat_VK_FORMAT_B8G8R8A8_SRGB
                && format.colorSpace == VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
            {
                return format.to_owned();
            }
        }
        available_formats[0]
    }

    fn vk_choose_swap_present_mode(
        self: &mut Self,
        available_present_modes: Vec<VkPresentModeKHR>,
    ) -> VkPresentModeKHR {
        for mode in &available_present_modes {
            if *mode == VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR {
                return *mode;
            }
        }
        VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR
    }

    fn vk_choose_swap_extent(
        self: &mut Self,
        capabilities: VkSurfaceCapabilitiesKHR,
    ) -> VkExtent2D {
        if capabilities.currentExtent.width != u32::MAX {
            return capabilities.currentExtent;
        } else {
            let (width, height) = glfw_get_framebuffer_size(self.window.unwrap());

            let mut actual_extent = VkExtent2D { width, height };

            actual_extent.width = min(
                max(actual_extent.width, capabilities.minImageExtent.width),
                capabilities.maxImageExtent.width,
            );

            actual_extent.height = min(
                max(actual_extent.height, capabilities.minImageExtent.height),
                capabilities.maxImageExtent.height,
            );
            return actual_extent;
        }
    }

    // GLFW FUNCTIONS

    fn glfw_create_surface(self: &mut Self) {
        self.vk_surface_khr =
            match glfw_create_window_surface(self.vk_instance.unwrap(), self.window.unwrap()) {
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
        while !glfw_window_should_close(self.window.unwrap()) {
            glfw_poll_events();
        }
    }

    fn cleanup(&mut self) {
        if let (Some(device), Some(swapchain)) =
            (self.vk_logical_device.take(), self.vk_swap_chain.take())
        {
            vk_destroy_swapchain_khr(device, swapchain);
            vk_destroy_device(device);
        }

        if let (Some(instance), Some(surface)) =
            (self.vk_instance.take(), self.vk_surface_khr.take())
        {
            vk_destroy_surface_khr(instance, surface);
            vk_destroy_instance(instance);
        }

        if let Some(window) = self.window.take() {
            glfw_destroy_window(window);
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
struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

struct SwapChainSupportDetails {
    capabilities: VkSurfaceCapabilitiesKHR,
    formats: Vec<VkSurfaceFormatKHR>,
    present_modes: Vec<VkPresentModeKHR>,
}

impl SwapChainSupportDetails {
    pub fn new(capabilities: VkSurfaceCapabilitiesKHR) -> Self {
        Self {
            capabilities,
            formats: Vec::new(),
            present_modes: Vec::new(),
        }
    }
    pub fn is_swapchain_adequate(&self) -> bool {
        !self.formats.is_empty() && !self.present_modes.is_empty()
    }
}
