use cc::Build;
use std::{env, path::{Path, PathBuf}};
use std::fs::File;
use std::io::prelude::*;

mod tusb_config;

fn add_all_c_files_in_dir(build: &mut Build, path: impl AsRef<Path>) {
    for entry in glob::glob(path.as_ref().join("**/*.c").to_str().unwrap()).unwrap() {
        let path = entry.unwrap();
        if path.extension().and_then(|s| s.to_str()) == Some("c") {
           build.file(&path);
        }
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Missing OUT_DIR"));

    {
        let mut f = File::create(out_dir.join("tusb_config.h"))
            .expect("Failed to create tusb_config.h");
        f.write_all(tusb_config::generate_cfg().as_bytes())
            .expect("Failed to write to tusb_config.h");
    }

    let sysroot = String::from_utf8(
        Build::new()
            .get_compiler().to_command()
            .arg("-print-sysroot")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run the compiler")
            .wait_with_output()
            .expect("cc -print-sysroot failed")
            .stdout
        ).unwrap();
    let sysroot = sysroot.trim();

    let mut build = Build::new();
    add_all_c_files_in_dir(&mut build, "tinyusb/src");
    build
        .include("tinyusb/src")
        .include(&out_dir) // for the tusb_config.h file
        .compile("tinyusb");

    let target = env::var("TARGET").expect("Missing TARGET env var");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("tinyusb/src/tusb.h")
        .rustified_enum(".*")
        .clang_arg(&format!("-I{}", &out_dir.display()))
        .derive_default(true)
        .layout_tests(false)
        .use_core()
        .rustfmt_bindings(true)
        .ctypes_prefix("cty")
        .blocklist_file(&format!("{}/include/.*", &sysroot))
        .clang_args(&vec![
            "-target", &target,
            "-fvisibility=default",
            "-fshort-enums",
        ])
        .clang_arg("-Itinyusb/src")
        .clang_arg(&format!("-I{}/include", &sysroot))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Can't write bindings!");
}
