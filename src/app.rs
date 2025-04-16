use glfw_bindings::{
    self, GLFW_CLIENT_API, GLFW_FALSE, GLFW_NO_API, GLFW_RESIZABLE, GLFWwindow, glfw_create_window,
    glfw_destroy_window, glfw_init, glfw_poll_events, glfw_terminate, glfw_window_hint,
    glfw_window_should_close,
};

#[derive(Default)]
pub struct App {
    window: Option<GLFWwindow>,
}

impl App {
    pub fn run(self: &mut Self) {
        self.init_window();
        self.main_loop();
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
        glfw_destroy_window(&mut self.window.unwrap());
        glfw_terminate();
    }
}

impl Drop for App {
    fn drop(self: &mut Self) {
        self.cleanup();
    }
}
