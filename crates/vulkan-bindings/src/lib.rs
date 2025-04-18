#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use std::ptr::{null, null_mut};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod c_structs;

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

#[derive(Debug)]
pub enum VulkanError {
    CouldNotCreateInstance,
    CouldNotEnumerateExtensionProperties,
    CouldNotEnumerateLayerProperties,
    CouldNotEnumerateDevices,
    CouldNotGetDeviceQueueFamilyProperties,
    CouldNotCreateLogicalDevice,
}
