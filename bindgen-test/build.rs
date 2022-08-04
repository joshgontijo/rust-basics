extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //FIXME, haven't got the hang yet on how the link is done
    // println!("cargo:rustc-link-search=native");
    println!("cargo:rerun-if-changed=native/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("native/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=gdi32");
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=shell32");

    println!("cargo:rustc-link-lib=static=raylib");

}