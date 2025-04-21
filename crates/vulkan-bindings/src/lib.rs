#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use std::ptr::{null, null_mut};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod c_structs;
mod macros;

pub fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

pub fn vk_make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

pub fn vk_create_instance(
    instance_create_info: &VkInstanceCreateInfo,
) -> Result<VkInstance, VulkanError> {
    unsafe {
        let mut vk_instance: VkInstance = std::mem::zeroed();
        let result = vkCreateInstance(instance_create_info, null(), &mut vk_instance);

        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreateInstance)
        } else {
            Ok(vk_instance)
        }
    }
}

pub fn vk_destroy_instance(instance: VkInstance) {
    unsafe {
        vkDestroyInstance(instance, null());
    };
}

pub fn vk_destroy_device(device: VkDevice) {
    unsafe {
        vkDestroyDevice(device, null());
    };
}

pub fn vk_destroy_surface_khr(instance: VkInstance, surface: VkSurfaceKHR) {
    unsafe {
        vkDestroySurfaceKHR(instance, surface, null());
    }
}

pub fn vk_destroy_swapchain_khr(device: VkDevice, swapchain: VkSwapchainKHR) {
    unsafe {
        vkDestroySwapchainKHR(device, swapchain, null());
    }
}

pub fn vk_destroy_image_view(device: VkDevice, image_view: VkImageView) {
    unsafe {
        vkDestroyImageView(device, image_view, null());
    }
}

pub fn vk_destroy_shader_module(device: VkDevice, shader_module: VkShaderModule) {
    unsafe {
        vkDestroyShaderModule(device, shader_module, null());
    }
}

pub fn vk_destroy_pipeline_layout(device: VkDevice, pipeline_layout: VkPipelineLayout) {
    unsafe {
        vkDestroyPipelineLayout(device, pipeline_layout, null());
    }
}

pub fn vk_destroy_render_pass(device: VkDevice, render_pass: VkRenderPass) {
    unsafe {
        vkDestroyRenderPass(device, render_pass, null());
    }
}

pub fn vk_destroy_graphics_pipeline(device: VkDevice, graphics_pipeline: VkPipeline) {
    unsafe {
        vkDestroyPipeline(device, graphics_pipeline, null());
    }
}

pub fn vk_destroy_framebuffer(device: VkDevice, framebuffer: VkFramebuffer) {
    unsafe {
        vkDestroyFramebuffer(device, framebuffer, null());
    }
}

pub fn vk_get_supported_extensions() -> Result<(u32, Vec<VkExtensionProperties>), VulkanError> {
    let mut extension_count: u32 = 0;
    unsafe {
        let mut result = vkEnumerateInstanceExtensionProperties(
            null(),
            &mut extension_count,
            core::ptr::null_mut(),
        );
        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateExtensionProperties);
        };

        let mut extensions: Vec<VkExtensionProperties> =
            vec![std::mem::zeroed(); extension_count as usize];

        result = vkEnumerateInstanceExtensionProperties(
            null(),
            &mut extension_count,
            extensions.as_mut_ptr(),
        );

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateExtensionProperties);
        } else {
            return Ok((extension_count, extensions));
        }
    };
}
pub fn vk_get_available_layer_properties() -> Result<(u32, Vec<VkLayerProperties>), VulkanError> {
    let mut layer_count: u32 = 0;
    unsafe {
        let mut result = vkEnumerateInstanceLayerProperties(&mut layer_count, null_mut());
        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateLayerProperties);
        };

        let mut layers: Vec<VkLayerProperties> = vec![std::mem::zeroed(); layer_count as usize];
        result = vkEnumerateInstanceLayerProperties(&mut layer_count, layers.as_mut_ptr());

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateLayerProperties);
        } else {
            return Ok((layer_count, layers));
        }
    };
}

pub fn vk_get_available_devices(
    instance: VkInstance,
) -> Result<(u32, Vec<VkPhysicalDevice>), VulkanError> {
    let mut device_count: u32 = 0;
    unsafe {
        let mut result = vkEnumeratePhysicalDevices(instance, &mut device_count, null_mut());
        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateDevices);
        };

        let mut physical_devices: Vec<VkPhysicalDevice> =
            vec![std::mem::zeroed(); device_count as usize];
        result =
            vkEnumeratePhysicalDevices(instance, &mut device_count, physical_devices.as_mut_ptr());

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateDevices);
        } else {
            return Ok((device_count, physical_devices));
        }
    };
}

pub fn vk_get_physical_device_properties(device: VkPhysicalDevice) -> VkPhysicalDeviceProperties {
    unsafe {
        let mut device_properties: VkPhysicalDeviceProperties = std::mem::zeroed();
        vkGetPhysicalDeviceProperties(device, &mut device_properties);
        device_properties
    }
}

pub fn vk_get_physical_device_features(device: VkPhysicalDevice) -> VkPhysicalDeviceFeatures {
    unsafe {
        let mut device_features: VkPhysicalDeviceFeatures = std::mem::zeroed();
        vkGetPhysicalDeviceFeatures(device, &mut device_features);
        device_features
    }
}

pub fn vk_get_physical_device_queue_family_properties(
    device: VkPhysicalDevice,
) -> Result<Vec<VkQueueFamilyProperties>, VulkanError> {
    unsafe {
        let mut queue_family_count: u32 = 0;
        vkGetPhysicalDeviceQueueFamilyProperties(device, &mut queue_family_count, null_mut());

        if queue_family_count == 0 {
            return Err(VulkanError::CouldNotGetDeviceQueueFamilyProperties);
        };

        let mut queue_families: Vec<VkQueueFamilyProperties> =
            vec![std::mem::zeroed(); queue_family_count as usize];

        vkGetPhysicalDeviceQueueFamilyProperties(
            device,
            &mut queue_family_count,
            queue_families.as_mut_ptr(),
        );
        Ok(queue_families)
    }
}

pub fn vk_get_device_extensions_properties(
    device: VkPhysicalDevice,
) -> Result<Vec<VkExtensionProperties>, VulkanError> {
    unsafe {
        let mut extension_count: u32 = 0;
        let mut result =
            vkEnumerateDeviceExtensionProperties(device, null(), &mut extension_count, null_mut());

        if extension_count == 0 || result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateDeviceExtensionProperties);
        };

        let mut available_extensions: Vec<VkExtensionProperties> =
            vec![std::mem::zeroed(); extension_count as usize];

        result = vkEnumerateDeviceExtensionProperties(
            device,
            null(),
            &mut extension_count,
            available_extensions.as_mut_ptr(),
        );

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotEnumerateDeviceExtensionProperties);
        };
        Ok(available_extensions)
    }
}

pub fn vk_create_logical_device(
    physical_device: VkPhysicalDevice,
    create_info: &VkDeviceCreateInfo,
) -> Result<VkDevice, VulkanError> {
    unsafe {
        let mut device: VkDevice = std::mem::zeroed();
        let result = vkCreateDevice(physical_device, create_info, null(), &mut device);

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotCreateLogicalDevice);
        };
        Ok(device)
    }
}

pub fn vk_get_device_queue(device: VkDevice, queue_family_index: u32, queue_index: u32) -> VkQueue {
    unsafe {
        let mut queue: VkQueue = std::mem::zeroed();
        vkGetDeviceQueue(device, queue_family_index, queue_index, &mut queue);
        queue
    }
}

pub fn vk_get_physical_device_surface_support_khr(
    physical_device: VkPhysicalDevice,
    index: u32,
    surface: VkSurfaceKHR,
) -> Result<bool, VulkanError> {
    unsafe {
        let mut supported: VkBool32 = std::mem::zeroed();
        let result =
            vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, index, surface, &mut supported);

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotDetermineSurfaceSupport);
        }

        Ok(supported == 1)
    }
}

pub fn vk_get_physical_device_surface_capabilities_khr(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> Result<VkSurfaceCapabilitiesKHR, VulkanError> {
    unsafe {
        let mut capabilities: VkSurfaceCapabilitiesKHR = std::mem::zeroed();
        let result =
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface, &mut capabilities);

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotDetermineSurfaceCapabilities);
        }

        Ok(capabilities)
    }
}

pub fn vk_get_physical_device_surface_formats_khr(
    device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> Result<Vec<VkSurfaceFormatKHR>, VulkanError> {
    unsafe {
        let mut format_count: u32 = 0;
        let mut result =
            vkGetPhysicalDeviceSurfaceFormatsKHR(device, surface, &mut format_count, null_mut());

        if format_count == 0 || result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotDeterminaSurfaceFormats);
        };

        let mut available_formats: Vec<VkSurfaceFormatKHR> =
            vec![std::mem::zeroed(); format_count as usize];

        result = vkGetPhysicalDeviceSurfaceFormatsKHR(
            device,
            surface,
            &mut format_count,
            available_formats.as_mut_ptr(),
        );

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotDeterminaSurfaceFormats);
        };
        Ok(available_formats)
    }
}

pub fn vk_get_physical_device_surface_present_modes_khr(
    device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> Result<Vec<VkPresentModeKHR>, VulkanError> {
    unsafe {
        let mut present_mode_count: u32 = 0;
        let mut result = vkGetPhysicalDeviceSurfacePresentModesKHR(
            device,
            surface,
            &mut present_mode_count,
            null_mut(),
        );

        if present_mode_count == 0 || result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotDetermineSurfacePresentModes);
        };

        let mut available_present_modes: Vec<VkPresentModeKHR> =
            vec![std::mem::zeroed(); present_mode_count as usize];

        result = vkGetPhysicalDeviceSurfacePresentModesKHR(
            device,
            surface,
            &mut present_mode_count,
            available_present_modes.as_mut_ptr(),
        );

        if result != VkResult_VK_SUCCESS {
            return Err(VulkanError::CouldNotDetermineSurfacePresentModes);
        };
        Ok(available_present_modes)
    }
}

pub fn vk_create_swapchain_khr(
    device: VkDevice,
    swapchain_create_info: VkSwapchainCreateInfoKHR,
) -> Result<VkSwapchainKHR, VulkanError> {
    unsafe {
        let mut swapchain: VkSwapchainKHR = std::mem::zeroed();
        let result = vkCreateSwapchainKHR(device, &swapchain_create_info, null(), &mut swapchain);

        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreateSwapchain)
        } else {
            Ok(swapchain)
        }
    }
}

pub fn vk_get_swapchain_images_khr(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> Result<Vec<VkImage>, VulkanError> {
    unsafe {
        let mut image_count: u32 = 0;
        let result = vkGetSwapchainImagesKHR(device, swapchain, &mut image_count, null_mut());

        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotGetSwapchainImages)
        } else {
            let mut images: Vec<VkImage> = vec![std::mem::zeroed(); image_count as usize];
            let result =
                vkGetSwapchainImagesKHR(device, swapchain, &mut image_count, images.as_mut_ptr());

            if result != VkResult_VK_SUCCESS {
                Err(VulkanError::CouldNotGetSwapchainImages)
            } else {
                Ok(images)
            }
        }
    }
}

pub fn vk_create_image_view(
    device: VkDevice,
    image_view_create_info: VkImageViewCreateInfo,
) -> Result<VkImageView, VulkanError> {
    unsafe {
        let mut image_view: VkImageView = std::mem::zeroed();
        let result = vkCreateImageView(device, &image_view_create_info, null(), &mut image_view);
        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreateImageView)
        } else {
            Ok(image_view)
        }
    }
}

pub fn vk_create_render_pass(
    device: VkDevice,
    render_pass_create_info: VkRenderPassCreateInfo,
) -> Result<VkRenderPass, VulkanError> {
    unsafe {
        let mut render_pass: VkRenderPass = std::mem::zeroed();
        let result = vkCreateRenderPass(device, &render_pass_create_info, null(), &mut render_pass);
        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreateRenderPass)
        } else {
            Ok(render_pass)
        }
    }
}

pub fn vk_create_pipeline_layout(
    device: VkDevice,
    pipeline_layout_create_info: VkPipelineLayoutCreateInfo,
) -> Result<VkPipelineLayout, VulkanError> {
    unsafe {
        let mut pipeline_layout: VkPipelineLayout = std::mem::zeroed();
        let result = vkCreatePipelineLayout(
            device,
            &pipeline_layout_create_info,
            null(),
            &mut pipeline_layout,
        );
        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreatePipelineLayout)
        } else {
            Ok(pipeline_layout)
        }
    }
}

pub fn vk_create_shader_module(
    device: VkDevice,
    shader_module_create_info: VkShaderModuleCreateInfo,
) -> Result<VkShaderModule, VulkanError> {
    unsafe {
        let mut shader_module: VkShaderModule = std::mem::zeroed();
        let result = vkCreateShaderModule(
            device,
            &shader_module_create_info,
            null(),
            &mut shader_module,
        );
        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreateShaderModule)
        } else {
            Ok(shader_module)
        }
    }
}

pub fn vk_create_framebuffer(
    device: VkDevice,
    framebuffer_create_info: VkFramebufferCreateInfo,
) -> Result<VkFramebuffer, VulkanError> {
    unsafe {
        let mut framebuffer: VkFramebuffer = std::mem::zeroed();
        let result =
            vkCreateFramebuffer(device, &framebuffer_create_info, null(), &mut framebuffer);
        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreateFramebuffer)
        } else {
            Ok(framebuffer)
        }
    }
}

pub fn vk_create_graphics_pipeline(
    device: VkDevice,
    graphics_pipeline_create_info: VkGraphicsPipelineCreateInfo,
) -> Result<VkPipeline, VulkanError> {
    unsafe {
        let mut pipeline: VkPipeline = std::mem::zeroed();
        let result = vkCreateGraphicsPipelines(
            device,
            null_mut(),
            1,
            &graphics_pipeline_create_info,
            null(),
            &mut pipeline,
        );
        if result != VkResult_VK_SUCCESS {
            Err(VulkanError::CouldNotCreateGraphicsPipeline)
        } else {
            Ok(pipeline)
        }
    }
}

#[derive(Debug)]
pub enum VulkanError {
    CouldNotCreateInstance,
    CouldNotEnumerateExtensionProperties,
    CouldNotEnumerateLayerProperties,
    CouldNotEnumerateDevices,
    CouldNotGetDeviceQueueFamilyProperties,
    CouldNotCreateLogicalDevice,
    CouldNotDetermineSurfaceSupport,
    CouldNotEnumerateDeviceExtensionProperties,
    CouldNotDetermineSurfaceCapabilities,
    CouldNotDeterminaSurfaceFormats,
    CouldNotDetermineSurfacePresentModes,
    CouldNotCreateSwapchain,
    CouldNotGetSwapchainImages,
    CouldNotCreateImageView,
    CouldNotCreateShaderModule,
    CouldNotCreatePipelineLayout,
    CouldNotCreateRenderPass,
    CouldNotCreateGraphicsPipeline,
    CouldNotCreateFramebuffer,
}
