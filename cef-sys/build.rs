extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

fn main() {
    builder().build();
}

struct Builder {
    cef_dir: PathBuf
}

fn builder() -> Builder {
    Builder { cef_dir: PathBuf::from(env_var("CEF_DIR")) }
}

impl Builder {
    fn build(&self) {
        self.write_wrapper_file();
        self.generate_bindings();
        self.cargo_config();
    }

    fn write_wrapper_file(&self) {
        // Let's create a wrapper.h file if it's not there
        let wrapper_file = PathBuf::from(env_var("CARGO_MANIFEST_DIR")).join("wrapper.h");
        if !wrapper_file.is_file() {
            // We want to include all capi headers
            let include_files = fs::read_dir(self.cef_dir.join("include").join("capi")).unwrap();
            let mut file = fs::File::create(wrapper_file).unwrap();
            for entry_res in include_files {
                let entry = entry_res.unwrap();
                // If it's a header, include it in the file as a string relative to cef_dir
                if entry.file_name().to_str().unwrap().ends_with(".h") {
                    let relative_name = entry.path().strip_prefix(&self.cef_dir).
                        unwrap().to_str().unwrap().replace("\\", "/");
                    writeln!(file, "#include \"{}\"", relative_name).unwrap();
                }
            }
        } else {
            println!("Not writing wrapper.h because it already exists");
        }
    }

    fn generate_bindings(&self) {
        let out_file = PathBuf::from(env_var("OUT_DIR")).join("bindings.rs");
        if !out_file.is_file() {
            let bindings = bindgen::builder()
                .header("wrapper.h")
                .clang_arg("--include-directory")
                .clang_arg(self.cef_dir.to_str().unwrap())
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

    fn cargo_config(&self) {
        // Tell the linker the lib name and the path
        // TODO: make this just "cef" on non-win
        println!("cargo:rustc-link-lib=libcef");
        println!("cargo:rustc-link-search={}", self.cef_dir.
            join(if env_var("PROFILE") == "release" { "Release" } else { "Debug" }).to_str().unwrap());
    }
}

fn env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> String {
    env::var(&key).expect(&format!("Unable to find env var {:?}", key.as_ref()))
}