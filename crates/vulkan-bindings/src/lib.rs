#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use std::ptr::null;

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

#[derive(Debug)]
pub enum VulkanError {
    CouldNotCreateInstance,
    CouldNotEnumerateExtensionProperties,
}
