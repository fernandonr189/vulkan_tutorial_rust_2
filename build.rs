use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let shader_dir = "glsl";
    let out_dir = std::env::var("OUT_DIR").unwrap();

    for entry in fs::read_dir(shader_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "vert" || ext == "frag" {
                let out_path = Path::new(&out_dir)
                    .join(path.file_name().unwrap().to_str().unwrap().to_owned() + ".spv");

                println!("cargo:rerun-if-changed={}", path.display());

                let status = Command::new("glslc")
                    .args([path.to_str().unwrap(), "-o", out_path.to_str().unwrap()])
                    .status()
                    .expect("Failed to run glslc");

                assert!(status.success(), "glslc failed on {:?}", path);
            }
        }
    }
}
