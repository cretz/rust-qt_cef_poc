
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

extern crate embed_resource;

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
        self.embed_windows_manifest();
    }

    fn copy_file_to_target<P: AsRef<Path>>(&self, src: P) {
        self.copy_file_if_not_there(src.as_ref(), self.target_dir.join(src.as_ref().file_name().unwrap()))
    }

    fn copy_file_if_not_there<P1: AsRef<Path>, P2: AsRef<Path>>(&self, src: P1, dest: P2) {
        if dest.as_ref().is_file() {
            println!("Not copying {:?} because it already exists", dest.as_ref());
        } else {
            fs::copy(src.as_ref(), dest.as_ref()).
                expect(format!("Failed to copy from {:?} to {:?}", src.as_ref(), dest.as_ref()).as_ref());
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
        // Core lib
        self.copy_file_to_target(cef_dll_dir.join("libcef.dll"));
        // Crash reporting lib
        self.copy_file_to_target(cef_dll_dir.join("chrome_elf.dll"));
        // V8 snapshot data
        self.copy_file_to_target(cef_dll_dir.join("natives_blob.bin"));
        self.copy_file_to_target(cef_dll_dir.join("snapshot_blob.bin"));
        // Unicode support data
        self.copy_file_to_target(cef_res_dir.join("icudtl.dat"));
        // US English locale data
        let locale_dir = self.target_dir.join("locales");
        if !locale_dir.is_dir() { fs::create_dir(locale_dir.as_path()).unwrap(); }
        self.copy_file_if_not_there(cef_res_dir.join("locales").join("en-US.pak"), locale_dir.join("en-US.pak"));
        // Non-l10n resources
        self.copy_file_to_target(cef_res_dir.join("cef.pak"));
        self.copy_file_to_target(cef_res_dir.join("cef_100_percent.pak"));
        self.copy_file_to_target(cef_res_dir.join("cef_200_percent.pak"));
        // Extension resources
        self.copy_file_to_target(cef_res_dir.join("cef_extensions.pak"));
        // Dev tools resources
        self.copy_file_to_target(cef_res_dir.join("devtools_resources.pak"));
        // Angle and D3D libs
        self.copy_file_to_target(cef_dll_dir.join("d3dcompiler_43.dll"));
        self.copy_file_to_target(cef_dll_dir.join("d3dcompiler_47.dll"));
        self.copy_file_to_target(cef_dll_dir.join("libEGL.dll"));
        self.copy_file_to_target(cef_dll_dir.join("libGLESv2.dll"));
    }

    fn embed_windows_manifest(&self) {
        // See: http://magpcss.org/ceforum/viewtopic.php?f=6&t=14721
        embed_resource::compile("qt_cef_poc.rc");
    }
}