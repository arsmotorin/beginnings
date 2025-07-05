use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = "target/c";

    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("Failed to create output directory");
    }

    let c_file = "src/main.c";
    let obj_file = format!("{}/main.o", out_dir);
    let lib_file = format!("{}/libmain.a", out_dir);

    Command::new("gcc")
        .args(&["-c", c_file, "-o", &obj_file])
        .status()
        .expect("Failed to compile main.c");

    Command::new("ar")
        .args(&["rcs", &lib_file, &obj_file])
        .status()
        .expect("Failed to create libmain.a");

    println!("cargo:rustc-link-lib=static=main");
    println!("cargo:rustc-link-search=native={}", out_dir);
}