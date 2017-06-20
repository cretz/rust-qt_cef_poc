
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if cfg!(not(windows)) { panic!("Only windows supported currently") }

    // Get the Qt dir
    let qt_dir = qt_bin_dir();

    // Copy the needed DLLs to the target dir
    let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).
        join("target").join(env::var("PROFILE").unwrap());
    let copy_qt_lib = |file: &str| {
        let target = target_dir.join(file);
        if target.is_file() { println!("Not copying {:?} because it already exists", target); }
        else { fs::copy(qt_dir.join(file), target).unwrap(); }
    };
    copy_qt_lib("Qt5Core.dll");
    copy_qt_lib("Qt5Gui.dll");
    copy_qt_lib("Qt5Widgets.dll");
}

fn qt_bin_dir() -> PathBuf {
    env::split_paths(&env::var("PATH").unwrap()).filter(|dir| {
        let mut qmake_path = dir.join("qmake");
        if cfg!(target_os = "windows") { qmake_path.set_extension("exe"); }
        qmake_path.is_file()
    }).next().expect("Unable to find qmake executable on PATH")
}
