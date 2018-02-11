extern crate bindgen;

use bindgen::Builder;

use std::env;
use std::path::Path;

fn main() {
    for (var, value) in env::vars() {
        eprintln!("{var}={value}", var=var, value=value);
    }

    println!("cargo:rustc-link-lib=dylib=phonon");

    let bindings = Builder::default()
        .header("headers/phonon.h")
        .generate()
        .unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bindgen.rs");

    eprintln!("Bindings => {:?}", dest_path);
    bindings.write_to_file(dest_path).unwrap();
}
