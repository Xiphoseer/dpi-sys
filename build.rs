use std::path::Path;

use cmake::Config;

fn rerun_dir(path: &Path) {
    for dir in path.read_dir().unwrap() {
        let entry = dir.unwrap();
        let meta = entry.metadata().unwrap();
        if meta.is_dir() {
            rerun_dir(&entry.path());
        } else if meta.is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
}

// Example custom build script.
fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let dpi_lib_root = path.join("dpi_library");
    let dpi_src_root = dpi_lib_root.join("src");

    // Invoke CMake
    let dst = Config::new("../dpi_library").build_target("all").build();
    rerun_dir(&dpi_src_root);

    // Collect static libraries
    println!("cargo:rustc-link-search=native={}/build/src/dpi", dst.display());
    println!("cargo:rustc-link-search=native={}/build/src/utils", dst.display());
    println!("cargo:rustc-link-search=native={}/build/src/thread", dst.display());
    println!("cargo:rustc-link-search=native={}/build/src/net/proto", dst.display());
    println!("cargo:rustc-link-search=native={}/build/src/net/rdma", dst.display());
    println!("cargo:rustc-link-search=native={}/build/src/net/message", dst.display());

    // Select static libraries
    println!("cargo:rustc-link-lib=static=dpi");
    println!("cargo:rustc-link-lib=static=thread");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-lib=static=net_proto");
    println!("cargo:rustc-link-lib=static=net_rdma");
    println!("cargo:rustc-link-lib=static=net_message");

    // Link dynamic libraries
    println!("cargo:rustc-link-lib=dylib=rdmacm");

    // Process CXX FFI
    println!("cargo:rerun-if-changed=src/ffi.rs");

    cxx_build::bridge("src/ffi.rs")
        .file("src/context.cc")
        .file("src/init.cc")
        .file("src/create_buffer.cc")
        .file("src/config.cc")
        .file("src/server.cc")
        .include(format!("{}/dpi_library/src/memory-api", path.display()))
        .include(format!("{}/dpi_library/src", path.display()))
        .include(format!("{}/build/src/memory-api", dst.display()))
        .include(format!("{}/build/src/net/message", dst.display()))
        .compile("dpi-sys");
}
