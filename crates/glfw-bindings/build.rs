fn main() {
    println!("cargo:rustc-link-lib=dylib=glfw");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("glfw.*") // only generate GLFW functions
        .allowlist_type("GLFW.*") // only GLFW types
        .allowlist_var("GLFW_.*")
        .blocklist_type("Vk.*")
        .generate()
        .expect("Unable to create glfw bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings")
}
