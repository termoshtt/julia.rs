use failure::*;
use std::path::PathBuf;
use std::{env, process::Command};

fn get_julia_cflags() -> Fallible<Vec<String>> {
    let out = Command::new("julia")
        .args(&[
            "-e",
            r#"print(joinpath(Sys.BINDIR,Base.DATAROOTDIR,"julia"))"#,
        ])
        .output()
        .expect("julia command is not found");
    if !out.status.success() {
        bail!("Cannot determine Julia share path ({})", out.status);
    }

    let jl_share_path = PathBuf::from(std::str::from_utf8(&out.stdout)?);
    let config = jl_share_path.join("julia-config.jl");
    let out = Command::new(config)
        .arg("--cflags")
        .output()
        .expect("julia-config.jl command is not found");
    if !out.status.success() {
        bail!("Failed to generate cflags",);
    }
    let flags = std::str::from_utf8(&out.stdout)?
        .split(' ')
        .map(|s| s.replace("'", "").trim().into())
        .collect();
    Ok(flags)
}

fn main() -> Fallible<()> {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("jl_.*")
        .whitelist_type("jl_.*")
        .whitelist_var("jl_.*")
        .clang_args(get_julia_cflags()?)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=julia");
    Ok(())
}
