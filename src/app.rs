use std::{
    cmp::{max, min},
    collections::HashSet,
    ffi::c_char,
    ptr::null_mut,
    thread::sleep,
    time::Duration,
};

use ffi_utils::StringFfi;
use glfw_bindings::{
    self, GLFW_CLIENT_API, GLFW_FALSE, GLFW_NO_API, GLFW_RESIZABLE, GLFWwindow, glfw_create_window,
    glfw_create_window_surface, glfw_destroy_window, glfw_get_framebuffer_size,
    glfw_get_required_instance_extensions, glfw_init, glfw_poll_events, glfw_terminate,
    glfw_window_hint, glfw_window_should_close,
};
use vulkan_bindings::{
    UINT32_MAX, VK_KHR_SWAPCHAIN_EXTENSION_NAME, VkApplicationInfo, VkAttachmentDescription,
    VkAttachmentLoadOp_VK_ATTACHMENT_LOAD_OP_CLEAR,
    VkAttachmentLoadOp_VK_ATTACHMENT_LOAD_OP_DONT_CARE, VkAttachmentReference,
    VkAttachmentStoreOp_VK_ATTACHMENT_STORE_OP_DONT_CARE,
    VkAttachmentStoreOp_VK_ATTACHMENT_STORE_OP_STORE, VkBlendFactor_VK_BLEND_FACTOR_ONE,
    VkBlendFactor_VK_BLEND_FACTOR_ZERO, VkBlendOp_VK_BLEND_OP_ADD, VkClearColorValue, VkClearValue,
    VkColorComponentFlagBits_VK_COLOR_COMPONENT_A_BIT,
    VkColorComponentFlagBits_VK_COLOR_COMPONENT_B_BIT,
    VkColorComponentFlagBits_VK_COLOR_COMPONENT_G_BIT,
    VkColorComponentFlagBits_VK_COLOR_COMPONENT_R_BIT,
    VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR, VkCommandBuffer,
    VkCommandBufferAllocateInfo, VkCommandBufferBeginInfo,
    VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_PRIMARY, VkCommandPool,
    VkCommandPoolCreateFlagBits_VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
    VkCommandPoolCreateFlags, VkCommandPoolCreateInfo,
    VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
    VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
    VkCullModeFlagBits_VK_CULL_MODE_BACK_BIT, VkDevice, VkDeviceCreateInfo,
    VkDeviceQueueCreateInfo, VkDynamicState, VkDynamicState_VK_DYNAMIC_STATE_SCISSOR,
    VkDynamicState_VK_DYNAMIC_STATE_VIEWPORT, VkExtent2D, VkFence,
    VkFenceCreateFlagBits_VK_FENCE_CREATE_SIGNALED_BIT, VkFenceCreateInfo, VkFormat,
    VkFormat_VK_FORMAT_B8G8R8A8_SRGB, VkFramebuffer, VkFramebufferCreateInfo,
    VkFrontFace_VK_FRONT_FACE_CLOCKWISE, VkGraphicsPipelineCreateInfo, VkImage,
    VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT,
    VkImageLayout_VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
    VkImageLayout_VK_IMAGE_LAYOUT_PRESENT_SRC_KHR, VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED,
    VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT, VkImageView, VkImageViewCreateInfo,
    VkImageViewType_VK_IMAGE_VIEW_TYPE_2D, VkInstance, VkInstanceCreateInfo,
    VkLogicOp_VK_LOGIC_OP_COPY, VkOffset2D, VkPhysicalDevice,
    VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU, VkPipeline,
    VkPipelineBindPoint_VK_PIPELINE_BIND_POINT_GRAPHICS, VkPipelineColorBlendAttachmentState,
    VkPipelineColorBlendStateCreateInfo, VkPipelineDynamicStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo, VkPipelineLayout, VkPipelineLayoutCreateInfo,
    VkPipelineMultisampleStateCreateInfo, VkPipelineRasterizationStateCreateInfo,
    VkPipelineShaderStageCreateInfo, VkPipelineVertexInputStateCreateInfo,
    VkPipelineViewportStateCreateInfo, VkPolygonMode_VK_POLYGON_MODE_FILL, VkPresentModeKHR,
    VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR, VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR,
    VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST, VkQueue,
    VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT, VkRect2D, VkRenderPass, VkRenderPassBeginInfo,
    VkRenderPassCreateInfo, VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT, VkSemaphore,
    VkSemaphoreCreateInfo, VkShaderModule, VkShaderModuleCreateInfo,
    VkShaderStageFlagBits_VK_SHADER_STAGE_FRAGMENT_BIT,
    VkShaderStageFlagBits_VK_SHADER_STAGE_VERTEX_BIT, VkSharingMode_VK_SHARING_MODE_CONCURRENT,
    VkSharingMode_VK_SHARING_MODE_EXCLUSIVE, VkSubpassContents_VK_SUBPASS_CONTENTS_INLINE,
    VkSubpassDescription, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR, VkSurfaceKHR,
    VkSwapchainCreateInfoKHR, VkSwapchainKHR, VkViewport, vk_acquire_next_image_khr,
    vk_allocate_command_buffers, vk_begin_command_buffer, vk_cmd_begin_render_pass,
    vk_cmd_bind_pipeline, vk_cmd_draw, vk_cmd_end_render_pass, vk_cmd_set_scissor,
    vk_cmd_set_viewport, vk_create_command_pool, vk_create_fence, vk_create_framebuffer,
    vk_create_graphics_pipeline, vk_create_image_view, vk_create_instance,
    vk_create_logical_device, vk_create_pipeline_layout, vk_create_render_pass,
    vk_create_semaphore, vk_create_shader_module, vk_create_swapchain_khr, vk_destroy_command_pool,
    vk_destroy_device, vk_destroy_fence, vk_destroy_framebuffer, vk_destroy_graphics_pipeline,
    vk_destroy_image_view, vk_destroy_instance, vk_destroy_pipeline_layout, vk_destroy_render_pass,
    vk_destroy_semaphore, vk_destroy_shader_module, vk_destroy_surface_khr,
    vk_destroy_swapchain_khr, vk_end_command_buffer, vk_get_available_devices,
    vk_get_available_layer_properties, vk_get_device_extensions_properties, vk_get_device_queue,
    vk_get_physical_device_features, vk_get_physical_device_properties,
    vk_get_physical_device_queue_family_properties,
    vk_get_physical_device_surface_capabilities_khr, vk_get_physical_device_surface_formats_khr,
    vk_get_physical_device_surface_present_modes_khr, vk_get_physical_device_surface_support_khr,
    vk_get_supported_extensions, vk_get_swapchain_images_khr, vk_make_api_version, vk_make_version,
    vk_reset_fences, vk_wait_for_fences,
};

const DEBUG_ENABLED: bool = cfg!(debug_assertions);
static VALIDATION_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
static REQUIRED_EXTENSIONS: &[&[u8]] = &[VK_KHR_SWAPCHAIN_EXTENSION_NAME];
const VERT_SHADER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shader.vert.spv"));
const FRAG_SHADER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shader.frag.spv"));

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
    vk_swap_chain_image_views: Vec<VkImageView>,
    vk_swap_chain_image_format: Option<VkFormat>,
    vk_swap_chain_image_extent: Option<VkExtent2D>,
    vk_render_pass: Option<VkRenderPass>,
    vk_pipeline_layout: Option<VkPipelineLayout>,
    vk_graphics_pipeline: Option<VkPipeline>,
    vk_swap_chain_framebuffers: Vec<VkFramebuffer>,
    vk_command_pool: Option<VkCommandPool>,
    vk_command_buffer: Option<VkCommandBuffer>,
    vk_image_available_semaphore: Option<VkSemaphore>,
    vk_render_finished_semaphore: Option<VkSemaphore>,
    vk_in_flight_fence: Option<VkFence>,
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
        println!("Instance created");
        self.glfw_create_surface();
        println!("Surface created");
        self.vk_pick_physical_device();
        println!("Physical device picked");
        self.vk_create_logical_device();
        println!("Logical device created");
        self.vk_create_swap_chain();
        println!("Swap chain created");
        self.vk_create_image_views();
        println!("Image views created");
        self.vk_create_render_pass();
        println!("Render pass created");
        self.vk_create_graphics_pipeline();
        println!("Graphics pipeline created");
        self.vk_create_framebuffers();
        println!("Framebuffers created");
        self.vk_create_command_pool();
        println!("Command pool created");
        self.vk_create_command_buffers();
        println!("Command buffers created");
        self.vk_create_sync_objects();
        println!("Sync objects created");
    }

    fn vk_create_instance(self: &mut Self) {
        if DEBUG_ENABLED && !self.vk_check_validation_layer_support() {
            panic!("Validation layers not available");
        }
        let mut app_info = VkApplicationInfo::default();
        app_info
            .set_p_application_name("Hello Triangle".as_ptr() as *const i8)
            .set_application_version(vk_make_version(1, 0, 0))
            .set_p_engine_name("No Engine".as_ptr() as *const i8)
            .set_engine_version(vk_make_version(1, 0, 0))
            .set_api_version(vk_make_api_version(0, 1, 0, 0));

        let (extension_count, extensions) = glfw_get_required_instance_extensions()
            .expect("Could not get glfw required extensions");
        let mut instance_create_info = VkInstanceCreateInfo::default();
        instance_create_info
            .set_p_application_info(&app_info)
            .set_enabled_extension_count(extension_count)
            .set_pp_enabled_extension_names(extensions)
            .set_enabled_layer_count(0);

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

        let device_properties = vk_get_physical_device_properties(self.vk_physical_device.unwrap());
        println!(
            "Device Name: {}",
            StringFfi::from_i8_array(&device_properties.deviceName).to_string()
        );
    }

    fn vk_is_device_suitable(self: &mut Self, device: VkPhysicalDevice) -> bool {
        let device_properties = vk_get_physical_device_properties(device);
        let device_features = vk_get_physical_device_features(device);

        let queue_family_indices = self.vk_find_queue_families(device);

        let extensions_supported = self.vk_check_device_extension_support(device);

        let swapchain_support_details = self.vk_query_swapchain_support(device);

        return device_features.geometryShader == 1
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
                .set_queue_family_index(family)
                .set_queue_count(1)
                .set_p_queue_priorities(&priority);
            queue_create_infos.push(queue_create_info);
        }

        let extension_names: Vec<*const u8> =
            REQUIRED_EXTENSIONS.iter().map(|s| s.as_ptr()).collect();

        let device_features = vk_get_physical_device_features(self.vk_physical_device.unwrap());
        let mut device_create_info = VkDeviceCreateInfo::default();
        device_create_info
            .set_p_queue_create_infos(queue_create_infos.as_ptr())
            .set_queue_create_info_count(queue_create_infos.len() as u32)
            .set_p_enabled_features(&device_features)
            .set_enabled_extension_count(extension_names.len() as u32)
            .set_pp_enabled_extension_names(extension_names.as_ptr() as *const *const i8);

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
            .set_surface(self.vk_surface_khr.unwrap())
            .set_min_image_count(image_count)
            .set_image_format(surface_format.format)
            .set_image_color_space(surface_format.colorSpace)
            .set_image_extent(extent)
            .set_image_array_layers(1)
            .set_image_usage(VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT);

        let queue_family_indices = self.vk_find_queue_families(self.vk_physical_device.unwrap());
        let queue_family_indices_vec = vec![
            queue_family_indices.graphics_family.unwrap(),
            queue_family_indices.present_family.unwrap(),
        ];

        if queue_family_indices.are_equal() {
            swapchain_create_info
                .set_image_sharing_mode(VkSharingMode_VK_SHARING_MODE_CONCURRENT)
                .set_queue_family_index_count(2)
                .set_p_queue_family_indices(queue_family_indices_vec.as_ptr());
        } else {
            swapchain_create_info
                .set_image_sharing_mode(VkSharingMode_VK_SHARING_MODE_EXCLUSIVE)
                .set_queue_family_index_count(0);
        }
        swapchain_create_info
            .set_pre_transform(swapchain_support_details.capabilities.currentTransform)
            .set_composite_alpha(VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR)
            .set_present_mode(present_mode)
            .set_clipped(true as u32)
            .set_old_swapchain(null_mut());

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

    fn vk_create_image_views(self: &mut Self) {
        for swapchain_image in &self.vk_swap_chain_images {
            let mut image_view_create_info = VkImageViewCreateInfo::default();
            image_view_create_info
                .set_image(*swapchain_image)
                .set_view_type(VkImageViewType_VK_IMAGE_VIEW_TYPE_2D)
                .set_format(self.vk_swap_chain_image_format.unwrap())
                .set_components(vulkan_bindings::VkComponentMapping {
                    r: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                    g: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                    b: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                    a: VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY,
                })
                .set_subresource_range(vulkan_bindings::VkImageSubresourceRange {
                    aspectMask: VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                });

            self.vk_swap_chain_image_views.push(
                match vk_create_image_view(self.vk_logical_device.unwrap(), image_view_create_info)
                {
                    Ok(image_view) => image_view,
                    Err(err) => panic!("Failed to create image view: {:?}", err),
                },
            );
        }
    }

    fn vk_create_shader_module(self: &mut Self, code: &[u8]) -> VkShaderModule {
        if code.len() % 4 != 0 {
            panic!("Shader code size is not a multiple of 4: {}", code.len());
        }
        if code.len() < 20 {
            panic!(
                "Shader code is too small to be valid SPIR-V: {}",
                code.len()
            );
        }
        let word_count = code.len() / 4;
        let mut code_u32 = Vec::with_capacity(word_count);
        for i in 0..word_count {
            let idx = i * 4;
            let word = u32::from_le_bytes([code[idx], code[idx + 1], code[idx + 2], code[idx + 3]]);
            code_u32.push(word);
        }
        if code_u32[0] != 0x07230203 {
            panic!("Invalid SPIR-V magic number: {:x}", code_u32[0]);
        }
        let mut shader_create_info = VkShaderModuleCreateInfo::default();
        shader_create_info
            .set_code_size(code.len())
            .set_p_code(code_u32.as_ptr());
        match vk_create_shader_module(self.vk_logical_device.unwrap(), shader_create_info) {
            Ok(shader_module) => shader_module,
            Err(err) => panic!("Failed to create shader module: {:?}", err),
        }
    }

    fn vk_create_graphics_pipeline(self: &mut Self) {
        println!("Vertex shader size: {}", VERT_SHADER.len());
        println!("Fragment shader size: {}", FRAG_SHADER.len());

        let vertex_shader_module = self.vk_create_shader_module(VERT_SHADER);
        let fragment_shader_module = self.vk_create_shader_module(FRAG_SHADER);

        let shaders_entry_point = StringFfi::from_string("main");

        let mut vert_shader_stage_create_info = VkPipelineShaderStageCreateInfo::default();
        vert_shader_stage_create_info
            .set_stage(VkShaderStageFlagBits_VK_SHADER_STAGE_VERTEX_BIT)
            .set_module(vertex_shader_module)
            .set_p_name(shaders_entry_point.as_ptr());

        let mut frag_shader_stage_create_info = VkPipelineShaderStageCreateInfo::default();
        frag_shader_stage_create_info
            .set_stage(VkShaderStageFlagBits_VK_SHADER_STAGE_FRAGMENT_BIT)
            .set_module(fragment_shader_module)
            .set_p_name(shaders_entry_point.as_ptr());

        let shader_stages = [vert_shader_stage_create_info, frag_shader_stage_create_info];

        let dynamic_states: Vec<VkDynamicState> = vec![
            VkDynamicState_VK_DYNAMIC_STATE_VIEWPORT,
            VkDynamicState_VK_DYNAMIC_STATE_SCISSOR,
        ];

        let mut dynamic_state_create_info = VkPipelineDynamicStateCreateInfo::default();
        dynamic_state_create_info
            .set_p_dynamic_states(dynamic_states.as_ptr())
            .set_dynamic_state_count(dynamic_states.len() as u32);

        let mut vertex_input_info = VkPipelineVertexInputStateCreateInfo::default();
        vertex_input_info
            .set_vertex_binding_description_count(0)
            .set_vertex_attribute_description_count(0);

        let mut input_assembly_info = VkPipelineInputAssemblyStateCreateInfo::default();
        input_assembly_info
            .set_topology(VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST)
            .set_primitive_restart_enable(false as u32);

        let mut viewport_state_info = VkPipelineViewportStateCreateInfo::default();
        viewport_state_info
            .set_viewport_count(0)
            .set_scissor_count(0);

        let mut rasterizer_create_info = VkPipelineRasterizationStateCreateInfo::default();
        rasterizer_create_info
            .set_depth_clamp_enable(false as u32)
            .set_rasterizer_discard_enable(false as u32)
            .set_polygon_mode(VkPolygonMode_VK_POLYGON_MODE_FILL)
            .set_line_width(1.0)
            .set_cull_mode(VkCullModeFlagBits_VK_CULL_MODE_BACK_BIT)
            .set_front_face(VkFrontFace_VK_FRONT_FACE_CLOCKWISE)
            .set_depth_bias_enable(false as u32)
            .set_depth_bias_constant_factor(0.0)
            .set_depth_bias_clamp(0.0)
            .set_depth_bias_slope_factor(0.0);

        let mut multisample_create_info = VkPipelineMultisampleStateCreateInfo::default();
        multisample_create_info
            .set_sample_shading_enable(false as u32)
            .set_rasterization_samples(VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT)
            .set_min_sample_shading(1.0)
            .set_alpha_to_coverage_enable(false as u32)
            .set_alpha_to_one_enable(false as u32);

        let mut color_blend_attachment = VkPipelineColorBlendAttachmentState::default();
        color_blend_attachment
            .set_color_write_mask(
                VkColorComponentFlagBits_VK_COLOR_COMPONENT_R_BIT
                    | VkColorComponentFlagBits_VK_COLOR_COMPONENT_G_BIT
                    | VkColorComponentFlagBits_VK_COLOR_COMPONENT_B_BIT
                    | VkColorComponentFlagBits_VK_COLOR_COMPONENT_A_BIT,
            )
            .set_blend_enable(false as u32)
            .set_src_blend_color_factor(VkBlendFactor_VK_BLEND_FACTOR_ONE)
            .set_dst_blend_color_factor(VkBlendFactor_VK_BLEND_FACTOR_ZERO)
            .set_color_blend_op(VkBlendOp_VK_BLEND_OP_ADD)
            .set_src_blend_alpha_factor(VkBlendFactor_VK_BLEND_FACTOR_ONE)
            .set_dst_blend_alpha_factor(VkBlendFactor_VK_BLEND_FACTOR_ZERO)
            .set_alpha_blend_op(VkBlendOp_VK_BLEND_OP_ADD);

        let mut color_blending = VkPipelineColorBlendStateCreateInfo::default();
        color_blending
            .set_logic_op_enable(false as u32)
            .set_logic_op(VkLogicOp_VK_LOGIC_OP_COPY)
            .set_attachment_count(1)
            .set_p_attachments(&color_blend_attachment);

        let mut pipeline_layout_info = VkPipelineLayoutCreateInfo::default();
        pipeline_layout_info
            .set_layout_count(0)
            .set_push_constant_range_count(0);

        self.vk_pipeline_layout = match vk_create_pipeline_layout(
            self.vk_logical_device.unwrap(),
            pipeline_layout_info,
        ) {
            Ok(layout) => Some(layout),
            Err(err) => panic!("Failed to create pipeline layout: {:?}", err),
        };

        let mut pipeline_create_info = VkGraphicsPipelineCreateInfo::default();
        pipeline_create_info
            .set_stage_count(2)
            .set_p_stages(shader_stages.as_ptr())
            .set_p_vertex_input_state(&vertex_input_info)
            .set_p_input_assembly_state(&input_assembly_info)
            .set_p_viewport_state(&viewport_state_info)
            .set_p_rasterization_state(&rasterizer_create_info)
            .set_p_multisample_state(&multisample_create_info)
            .set_p_color_blend_state(&color_blending)
            .set_p_dynamic_state(&dynamic_state_create_info)
            .set_layout(self.vk_pipeline_layout.unwrap())
            .set_render_pass(self.vk_render_pass.unwrap())
            .set_subpass(0);

        self.vk_graphics_pipeline = match vk_create_graphics_pipeline(
            self.vk_logical_device.unwrap(),
            pipeline_create_info,
        ) {
            Ok(pipeline) => Some(pipeline),
            Err(err) => panic!("Failed to create graphics pipeline: {:?}", err),
        };

        vk_destroy_shader_module(self.vk_logical_device.unwrap(), vertex_shader_module);
        vk_destroy_shader_module(self.vk_logical_device.unwrap(), fragment_shader_module);
    }

    fn vk_create_render_pass(self: &mut Self) {
        let mut color_attachment = VkAttachmentDescription::default();
        color_attachment
            .set_format(self.vk_swap_chain_image_format.unwrap())
            .set_samples(VkSampleCountFlagBits_VK_SAMPLE_COUNT_1_BIT)
            .set_load_op(VkAttachmentLoadOp_VK_ATTACHMENT_LOAD_OP_CLEAR)
            .set_store_op(VkAttachmentStoreOp_VK_ATTACHMENT_STORE_OP_STORE)
            .set_stencil_load_op(VkAttachmentLoadOp_VK_ATTACHMENT_LOAD_OP_DONT_CARE)
            .set_stencil_store_op(VkAttachmentStoreOp_VK_ATTACHMENT_STORE_OP_DONT_CARE)
            .set_initial_layout(VkImageLayout_VK_IMAGE_LAYOUT_UNDEFINED)
            .set_final_layout(VkImageLayout_VK_IMAGE_LAYOUT_PRESENT_SRC_KHR);

        let mut color_attachment_ref = VkAttachmentReference::default();
        color_attachment_ref
            .set_attachment(0)
            .set_layout(VkImageLayout_VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL);

        let mut subpass_description = VkSubpassDescription::default();
        subpass_description
            .set_pipeline_bind_point(VkPipelineBindPoint_VK_PIPELINE_BIND_POINT_GRAPHICS)
            .set_color_attachment_count(1)
            .set_p_color_attachments(&color_attachment_ref);

        let mut render_pass_info = VkRenderPassCreateInfo::default();
        render_pass_info
            .set_attachment_count(1)
            .set_p_attachments(&color_attachment)
            .set_subpass_count(1)
            .set_p_subpasses(&subpass_description);

        self.vk_render_pass =
            match vk_create_render_pass(self.vk_logical_device.unwrap(), render_pass_info) {
                Ok(render_pass) => Some(render_pass),
                Err(err) => panic!("Failed to create render pass: {:?}", err),
            };
    }

    fn vk_create_framebuffers(self: &mut Self) {
        self.vk_swap_chain_framebuffers = Vec::with_capacity(self.vk_swap_chain_image_views.len());
        for image_view in self.vk_swap_chain_image_views.iter() {
            let attachments = *image_view;

            let mut framebuffer_info = VkFramebufferCreateInfo::default();
            framebuffer_info
                .set_render_pass(self.vk_render_pass.unwrap())
                .set_attachment_count(1)
                .set_p_attachments(&attachments)
                .set_width(self.vk_swap_chain_image_extent.unwrap().width)
                .set_height(self.vk_swap_chain_image_extent.unwrap().height)
                .set_layers(1);

            match vk_create_framebuffer(self.vk_logical_device.unwrap(), framebuffer_info) {
                Ok(framebuffer) => self.vk_swap_chain_framebuffers.push(framebuffer),
                Err(err) => panic!("Failed to create framebuffer: {:?}", err),
            }
        }
    }

    fn vk_create_command_pool(self: &mut Self) {
        let queue_family_indices = self.vk_find_queue_families(self.vk_physical_device.unwrap());

        let mut command_pool_info = VkCommandPoolCreateInfo::default();
        command_pool_info
            .set_flags(VkCommandPoolCreateFlagBits_VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT)
            .set_queue_family_index(queue_family_indices.graphics_family.unwrap());

        self.vk_command_pool =
            match vk_create_command_pool(self.vk_logical_device.unwrap(), command_pool_info) {
                Ok(command_pool) => Some(command_pool),
                Err(err) => panic!("Failed to create command pool: {:?}", err),
            };
    }

    fn vk_create_command_buffers(self: &mut Self) {
        let mut alloc_info = VkCommandBufferAllocateInfo::default();
        alloc_info
            .set_command_pool(self.vk_command_pool.unwrap())
            .set_level(VkCommandBufferLevel_VK_COMMAND_BUFFER_LEVEL_PRIMARY)
            .set_command_buffer_count(1);

        self.vk_command_buffer =
            match vk_allocate_command_buffers(self.vk_logical_device.unwrap(), alloc_info) {
                Ok(command_buffer) => Some(command_buffer),
                Err(err) => panic!("Failed to allocate command buffers: {:?}", err),
            };
    }

    fn vk_record_command_buffer(
        self: &mut Self,
        command_buffer: VkCommandBuffer,
        image_index: u32,
    ) {
        let mut begin_info = VkCommandBufferBeginInfo::default();
        begin_info.set_flags(0);

        match vk_begin_command_buffer(command_buffer, begin_info) {
            Ok(_) => (),
            Err(err) => panic!("Failed to begin recording command buffer: {:?}", err),
        }

        let clear_color = VkClearValue {
            color: VkClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };
        let mut render_pass_begin_info = VkRenderPassBeginInfo::default();
        render_pass_begin_info
            .set_render_pass(self.vk_render_pass.unwrap())
            .set_framebuffer(self.vk_swap_chain_framebuffers[image_index as usize])
            .set_render_area(vulkan_bindings::VkRect2D {
                offset: vulkan_bindings::VkOffset2D { x: 0, y: 0 },
                extent: self.vk_swap_chain_image_extent.unwrap(),
            })
            .set_p_clear_values(&clear_color)
            .set_clear_value_count(1);

        vk_cmd_begin_render_pass(
            command_buffer,
            render_pass_begin_info,
            VkSubpassContents_VK_SUBPASS_CONTENTS_INLINE,
        );

        vk_cmd_bind_pipeline(
            command_buffer,
            self.vk_graphics_pipeline.unwrap(),
            VkPipelineBindPoint_VK_PIPELINE_BIND_POINT_GRAPHICS,
        );

        let mut viewport = VkViewport::default();
        viewport
            .set_x(0.0)
            .set_y(0.0)
            .set_width(self.vk_swap_chain_image_extent.unwrap().width as f32)
            .set_height(self.vk_swap_chain_image_extent.unwrap().height as f32)
            .set_min_depth(0.0)
            .set_max_depth(1.0);

        vk_cmd_set_viewport(command_buffer, viewport);

        let mut scissor = VkRect2D::default();
        scissor
            .set_offset(VkOffset2D { x: 0, y: 0 })
            .set_extent(self.vk_swap_chain_image_extent.unwrap());

        vk_cmd_set_scissor(command_buffer, scissor);

        vk_cmd_draw(command_buffer, 3, 1, 0, 0);
        vk_cmd_end_render_pass(command_buffer);

        match vk_end_command_buffer(command_buffer) {
            Ok(_) => (),
            Err(err) => panic!("Failed to end command buffer: {:?}", err),
        }
    }

    fn vk_create_sync_objects(self: &mut Self) {
        let semaphore_info = VkSemaphoreCreateInfo::default();

        self.vk_image_available_semaphore =
            match vk_create_semaphore(self.vk_logical_device.unwrap(), semaphore_info) {
                Ok(semaphore) => Some(semaphore),
                Err(err) => panic!("Failed to create image available semaphore: {:?}", err),
            };

        self.vk_render_finished_semaphore =
            match vk_create_semaphore(self.vk_logical_device.unwrap(), semaphore_info) {
                Ok(semaphore) => Some(semaphore),
                Err(err) => panic!("Failed to create render finished semaphore: {:?}", err),
            };

        let mut fence_info = VkFenceCreateInfo::default();
        fence_info.set_flags(VkFenceCreateFlagBits_VK_FENCE_CREATE_SIGNALED_BIT);
        self.vk_in_flight_fence = match vk_create_fence(self.vk_logical_device.unwrap(), fence_info)
        {
            Ok(fence) => Some(fence),
            Err(err) => panic!("Failed to create in-flight fence: {:?}", err),
        };
    }
    fn draw_frame(self: &mut Self) {
        match vk_wait_for_fences(
            self.vk_logical_device.unwrap(),
            1,
            &self.vk_in_flight_fence.unwrap(),
            UINT32_MAX as u64,
        ) {
            Ok(()) => (),
            Err(err) => panic!("Failed to wait for fences: {:?}", err),
        };

        match vk_reset_fences(
            self.vk_logical_device.unwrap(),
            1,
            &self.vk_in_flight_fence.unwrap(),
        ) {
            Ok(()) => (),
            Err(err) => panic!("Failed to reset fences: {:?}", err),
        };

        let image_index = match vk_acquire_next_image_khr(
            self.vk_logical_device.unwrap(),
            self.vk_swap_chain.unwrap(),
            UINT32_MAX as u64,
            self.vk_image_available_semaphore.unwrap(),
        ) {
            Ok(index) => index,
            Err(err) => panic!("Failed to acquire next image: {:?}", err),
        };
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
            self.draw_frame();
        }
    }

    fn cleanup(&mut self) {
        if let Some(device) = self.vk_logical_device.take() {
            if let Some(fence) = self.vk_in_flight_fence.take() {
                vk_destroy_fence(device, fence);
                println!("In-flight fence destroyed");
            }

            if let Some(semaphore) = self.vk_image_available_semaphore.take() {
                vk_destroy_semaphore(device, semaphore);
                println!("Image available semaphore destroyed");
            }

            if let Some(semaphore) = self.vk_render_finished_semaphore.take() {
                vk_destroy_semaphore(device, semaphore);
                println!("Render finished semaphore destroyed");
            }

            if let Some(command_pool) = self.vk_command_pool.take() {
                vk_destroy_command_pool(device, command_pool);
                println!("Command pool destroyed");
            }

            for (i, framebuffer) in self.vk_swap_chain_framebuffers.iter().enumerate() {
                if !framebuffer.is_null() {
                    vk_destroy_framebuffer(device, *framebuffer);
                    println!("Framebuffer {} destroyed", i);
                }
            }
            if let Some(pipeline) = self.vk_graphics_pipeline.take() {
                vk_destroy_graphics_pipeline(device, pipeline);
                println!("Graphics pipeline destroyed");
            }
            if let Some(pipeline_layout) = self.vk_pipeline_layout.take() {
                vk_destroy_pipeline_layout(device, pipeline_layout);
                println!("Pipeline layout destroyed");
            }
            if let Some(render_pass) = self.vk_render_pass.take() {
                vk_destroy_render_pass(device, render_pass);
                println!("Render pass destroyed");
            }
            for (i, &image_view) in self.vk_swap_chain_image_views.iter().enumerate() {
                if !image_view.is_null() {
                    vk_destroy_image_view(device, image_view);
                    println!("Image view destroyed: {}", i);
                }
            }
            if let Some(swapchain) = self.vk_swap_chain.take() {
                vk_destroy_swapchain_khr(device, swapchain);
                println!("Swapchain destroyed");
            }
            vk_destroy_device(device);
            println!("Device destroyed");
        }

        if let (Some(instance), Some(surface)) =
            (self.vk_instance.take(), self.vk_surface_khr.take())
        {
            vk_destroy_surface_khr(instance, surface);
            println!("Surface destroyed");
            vk_destroy_instance(instance);
            println!("Instance destroyed");
        }
        if let Some(window) = self.window.take() {
            glfw_destroy_window(window);
            println!("Window destroyed");
        }
        glfw_terminate();
        println!("Application terminated");
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
    pub fn are_equal(&self) -> bool {
        self.graphics_family == self.present_family
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
