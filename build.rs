extern crate gcc;
// Generates the Rust header for the C API.
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //gcc::compile_library("libBigWig.a", &["bwRead.c", "bwStats.c", "bwValues.c", "bwWrite.c", "io.c"]);
    gcc::Build::new().file("bwRead.c").file("bwStats.c").file("bwValues.c").file("bwWrite.c").file("io.c").flag("-DNOCURL").compile("libBigWig.a");
    // let here = env::
    // println!("cargo:rustc-link-search=native={}",format!("{}", here.display()));
    println!("cargo:rustc-link-lib=static=BigWig");
    println!("cargo:rustc-link-lib=z");
    //println!("cargo:rustc-link-lib=static=curl");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        //.clang_arg(format!("-L{}", here.display()))
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
