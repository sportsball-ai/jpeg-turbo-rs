extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default();

    println!("cargo:rustc-link-lib=turbojpeg");
    println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());

    #[cfg(target_os = "macos")]
    {
        std::fs::copy("vendor/macos/lib/libturbojpeg.a", out_dir.join("libturbojpeg.a")).unwrap();
    }

    let bindings = bindings.header("vendor/include/turbojpeg.h")
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("unable to write bindings");
}
