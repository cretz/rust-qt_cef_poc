extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

fn main() {
    write_wrapper_file();
    generate_bindings();
}

fn write_wrapper_file() {
    let cef_dir = PathBuf::from(env_var("CEF_DIR"));

    // Let's create a wrapper.h file if it's not there
    let wrapper_file = PathBuf::from(env_var("CARGO_MANIFEST_DIR")).join("wrapper.h");
    if !wrapper_file.is_file() {
        // We want to include all capi headers
        let include_files = fs::read_dir(cef_dir.join("include").join("capi")).unwrap();
        let mut file = fs::File::create(wrapper_file).unwrap();
        for entry_res in include_files {
            let entry = entry_res.unwrap();
            // If it's a header, include it in the file as a string relative to cef_dir
            if entry.file_name().to_str().unwrap().ends_with(".h") {
                let relative_name = entry.path().strip_prefix(&cef_dir).unwrap().to_str().unwrap().replace("\\", "/");
                writeln!(file, "#include \"{}\"", relative_name).unwrap();
            }
        }
    } else {
        println!("Not writing wrapper.h because it already exists");
    }
}

fn env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> String {
    env::var(&key).expect(&format!("Unable to find env var {:?}", key.as_ref()))
}

fn generate_bindings() {
    let out_file = PathBuf::from(env_var("OUT_DIR")).join("bindings.rs");
    if !out_file.is_file() {
        let cef_dir = PathBuf::from(env_var("CEF_DIR"));
        let bindings = bindgen::builder()
            .header("wrapper.h")
            .clang_arg("--include-directory")
            .clang_arg(cef_dir.to_str().unwrap())
            .layout_tests(false)
            // TODO: waiting for fix of https://github.com/servo/rust-bindgen/issues/648
            .opaque_type("tagMONITORINFOEXA")
            .generate()
            .expect("Unable to generate bindings");
        bindings.write_to_file(out_file).map_err(|e| format!("Unable to write bindings: {}", e)).unwrap();
    } else {
        println!("Not generating bindings.rs because it already exists");
    }
}
