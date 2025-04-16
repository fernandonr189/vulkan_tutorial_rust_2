pub struct App {}

impl App {
    pub fn run(self: &mut Self) {}

    fn init_vulkan() {}

    fn main_loop() {}

    fn cleanup(self: &mut Self) {}
}

impl Default for App {
    fn default() -> Self {
        App {}
    }
}

impl Drop for App {
    fn drop(self: &mut Self) {
        self.cleanup();
    }
}
