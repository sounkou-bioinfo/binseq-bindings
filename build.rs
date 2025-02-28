use std::env;
use std::path::PathBuf;

fn main() {
    // Only run cbindgen when building for release
    if env::var("PROFILE").unwrap() == "release" {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        // Configure cbindgen
        let config = cbindgen::Config::from_file("cbindgen.toml")
            .expect("Unable to find cbindgen.toml configuration file");

        // Write the bindings to the $OUT_DIR/binseq.h file
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let header_path = out_path.join("binseq.h");

        cbindgen::Builder::new()
            .with_crate(crate_dir)
            .with_config(config)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(header_path.clone());

        // Also create a copy in the root directory for easy access
        let root_header = PathBuf::from("binseq.h");
        std::fs::copy(header_path, root_header).expect("Failed to copy header file");

        println!("cargo:rerun-if-changed=src/lib.rs");
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=cbindgen.toml");
    }
}
