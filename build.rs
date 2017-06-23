
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    builder().build();
}

struct Builder {
    target_dir: PathBuf
}

fn builder() -> Builder {
    Builder { target_dir: PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).
        join("target").join(env::var("PROFILE").unwrap()) }
}

impl Builder {
    fn build(&self) {
        if cfg!(not(windows)) { panic!("Only windows supported currently") }
        self.copy_qt_libs();
        self.copy_cef_resources();
    }

    fn copy_file_to_target<P: AsRef<Path>>(&self, src: P) {
        let target = self.target_dir.join(src.as_ref().file_name().unwrap());
        if target.is_file() {
            println!("Not copying {:?} because it already exists", target);
        } else {
            fs::copy(src.as_ref(), target.as_path()).
                expect(format!("Failed to copy from {:?} to {:?}", src.as_ref(), target.as_path()).as_ref());
        }
    }

    fn copy_qt_libs(&self) {
        let qt_dir = self.qt_bin_dir();
        self.copy_file_to_target(qt_dir.join("Qt5Core.dll"));
        self.copy_file_to_target(qt_dir.join("Qt5Gui.dll"));
        self.copy_file_to_target(qt_dir.join("Qt5Widgets.dll"));
    }

    fn qt_bin_dir(&self) -> PathBuf {
        env::split_paths(&env::var("PATH").unwrap()).filter(|dir| {
            let mut qmake_path = dir.join("qmake");
            if cfg!(target_os = "windows") { qmake_path.set_extension("exe"); }
            qmake_path.is_file()
        }).next().expect("Unable to find qmake executable on PATH")
    }

    fn copy_cef_resources(&self) {
        let cef_dir = PathBuf::from(env::var("CEF_DIR").expect("CEF_DIR not found"));
        let cef_dll_dir = cef_dir.join(if env::var("PROFILE").unwrap() == "release" { "Release" } else { "Debug" });
        let cef_res_dir = cef_dir.join("Resources");
        self.copy_file_to_target(cef_dll_dir.join("libcef.dll"));
        self.copy_file_to_target(cef_dll_dir.join("chrome_elf.dll"));
        self.copy_file_to_target(cef_res_dir.join("icudtl.dat"));
        self.copy_file_to_target(cef_dll_dir.join("natives_blob.bin"));
        self.copy_file_to_target(cef_dll_dir.join("snapshot_blob.bin"));
    }
}