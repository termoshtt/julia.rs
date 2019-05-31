extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<std::error::Error>> {
    let bindings = bindgen::Builder::default()
        .header("/usr/include/julia/julia.h")
        .whitelist_function("jl_.*")
        .whitelist_type("jl_.*")
        .whitelist_var("jl_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=julia");
    Ok(())
}
