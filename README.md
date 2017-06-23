
## Building

### Windows

Prereqs:

* Qt 5.8+
* [MSVC 2015 Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools)
* Latest [Windows 64-bit standard dist of CEF](http://opensource.spotify.com/cefbuilds/index.html#windows64_builds)

Steps:

* Make sure `qmake.exe` is on the `PATH`
* Make sure env var `CEF_DIR` is set to the dir where CEF was extracted to
* Make sure 64-bit msvc tools are on path by running `vcvarsall.bat amd64` (e.g.
  `"C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" amd64`)
* In cloned dir, run `cargo build` (with `--release` for prod binary)

### Linux

(TODO)

### MacOS

(TODO)