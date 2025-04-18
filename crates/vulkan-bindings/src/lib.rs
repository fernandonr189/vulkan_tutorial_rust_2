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

#[derive(Debug)]
pub enum VulkanError {
    CouldNotCreateInstance,
    CouldNotEnumerateExtensionProperties,
    CouldNotEnumerateLayerProperties,
    CouldNotEnumerateDevices,
}
