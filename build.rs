extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-search=native=/opt/gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf/arm-linux-gnueabihf/libc/lib/");
    println!("cargo:rustc-link-lib=RTU_Module");
    println!("cargo:rustc-link-lib=IOs_Module");
    println!("cargo:rustc-link-lib=GPS2_Module");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    //let bindings = bindgen::Builder::default()
    let bindings = bindgen::builder()
        .derive_default(true)
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        //.no_unstable_rust()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-target")
        .clang_arg("arm-linux-gnueabihf")
        //.clang_arg("--sysroot=/opt/gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf")
        .clang_arg("-I/opt/gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf/arm-linux-gnueabihf/libc/usr/include")
        //.clang_arg("-I/opt/gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf/arm-linux-gnueabihf/libc/usr/include/gnu/")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
