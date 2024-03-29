extern crate bindgen;

use bindgen::Builder;

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=dylib=phonon");
    println!("cargo:rerun-if-changed=build.rs");

    let builder = Builder::default()
        .header("stdint.h")
        .header("headers/phonon.h")
        .header("headers/phonon_version.h")
        .header("headers/steamaudio_fmod.h")
        .rustified_enum("IPL(.*)")
        .bitfield_enum("(.*)Flags");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bindgen.rs");

    let bindings = builder.generate().unwrap();
    bindings.write_to_file(&dest_path).unwrap();
}
