use std::fs;
use std::path::Path;

fn main() {
    let source = "..\\config"; // Folder to copy
    let target_dir = std::env::var("OUT_DIR").unwrap(); // Cargo build output dir
    let target = Path::new(&target_dir).join("..\\config");

    // Copy the entire folder
    fs_extra::dir::copy(
        source,
        &target,
        &fs_extra::dir::CopyOptions::new().overwrite(true).copy_inside(true),
    ).unwrap();

    // Re-run build script if the folder changes
    println!("cargo:rerun-if-changed={}", source);
}
