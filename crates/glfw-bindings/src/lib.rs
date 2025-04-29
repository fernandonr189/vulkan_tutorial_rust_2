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

pub fn glfw_create_window(
    width: u32,
    height: u32,
    title: &str,
) -> Result<*mut GLFWwindow, GlfwError> {
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
            Ok(window_ptr)
        }
    }
}

pub fn glfw_destroy_window(window: *mut GLFWwindow) {
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

pub fn glfw_window_should_close(window: *mut GLFWwindow) -> bool {
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
    window: *mut GLFWwindow,
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

pub fn glfw_get_framebuffer_size(window: *mut GLFWwindow) -> (u32, u32) {
    unsafe {
        let mut width = 0;
        let mut height = 0;
        glfwGetFramebufferSize(window, &mut width, &mut height);
        (width as u32, height as u32)
    }
}

pub fn glfw_set_framebuffer_size_callback<F: FnMut(i32, i32)>(
    window: *mut GLFWwindow,
    user_callback: F,
) {
    unsafe {
        // Store the callback in user data
        let callback = Box::into_raw(Box::new(Box::new(user_callback) as Box<dyn FnMut(i32, i32)>));
        glfwSetWindowUserPointer(window, callback as *mut _);

        extern "C" fn framebuffer_size_callback(
            window: *mut GLFWwindow,
            width: ::std::os::raw::c_int,
            height: ::std::os::raw::c_int,
        ) {
            unsafe {
                let callback_ptr =
                    glfwGetWindowUserPointer(window) as *mut Box<dyn FnMut(i32, i32)>;
                if !callback_ptr.is_null() {
                    (*callback_ptr)(width, height);
                }
            }
        }

        glfwSetFramebufferSizeCallback(window, Some(framebuffer_size_callback));
    }
}

#[derive(Debug)]
pub enum GlfwError {
    CreateWindowError,
    InstanceExtensionsError,
    CreateWindowSurfaceError,
}
