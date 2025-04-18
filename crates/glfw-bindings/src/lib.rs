#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use vulkan_bindings::{
    self, VkAllocationCallbacks, VkInstance, VkInternalAllocationType, VkPhysicalDevice, VkResult,
    VkResult_VK_SUCCESS, VkSurfaceKHR, VkSystemAllocationScope,
};

use std::{
    ffi::{CString, c_char},
    ptr::null_mut,
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn glfw_init() {
    unsafe {
        glfwInit();
    }
}

pub fn glfw_window_hint(hint: u32, value: u32) {
    unsafe {
        glfwWindowHint(hint as i32, value as i32);
    }
}

pub fn glfw_create_window(width: u32, height: u32, title: &str) -> Result<GLFWwindow, GlfwError> {
    let window_title = CString::new(title).unwrap();
    unsafe {
        let window_ptr = glfwCreateWindow(
            width as i32,
            height as i32,
            window_title.as_ptr(),
            null_mut(),
            null_mut(),
        );
        if window_ptr.is_null() {
            Err(GlfwError::CreateWindowError)
        } else {
            Ok(*window_ptr)
        }
    }
}

pub fn glfw_destroy_window(window: &mut GLFWwindow) {
    unsafe {
        glfwDestroyWindow(window);
    }
}

pub fn glfw_terminate() {
    unsafe {
        glfwTerminate();
    }
}

pub fn glfw_poll_events() {
    unsafe {
        glfwPollEvents();
    }
}

pub fn glfw_window_should_close(window: &mut GLFWwindow) -> bool {
    unsafe {
        let window_should_close = glfwWindowShouldClose(window);
        if window_should_close == 1 {
            true
        } else {
            false
        }
    }
}

pub fn glfw_get_required_instance_extensions() -> Result<(u32, *const *const i8), GlfwError> {
    let mut count: u32 = 0;
    let extensions: *const *const c_char;
    unsafe {
        extensions = glfwGetRequiredInstanceExtensions(&mut count);
    }
    if extensions.is_null() || count == 0 {
        return Err(GlfwError::InstanceExtensionsError);
    };
    Ok((count, extensions))
}

pub fn glfw_create_window_surface(
    instance: VkInstance,
    window: &mut GLFWwindow,
) -> Result<VkSurfaceKHR, GlfwError> {
    unsafe {
        let mut surface: VkSurfaceKHR = std::mem::zeroed();
        let result = glfwCreateWindowSurface(instance, window, null_mut(), &mut surface);

        if result != VkResult_VK_SUCCESS {
            Err(GlfwError::CreateWindowSurfaceError)
        } else {
            Ok(surface)
        }
    }
}

#[derive(Debug)]
pub enum GlfwError {
    CreateWindowError,
    InstanceExtensionsError,
    CreateWindowSurfaceError,
}
