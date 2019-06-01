use failure::*;
use std::path::PathBuf;
use std::{env, process::Command};

// Seek Julia based on the document "Embedded Julia"
//
// https://docs.julialang.org/en/v1/manual/embedding/index.html
fn get_julia_config() -> Fallible<PathBuf> {
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
    Ok(jl_share_path.join("julia-config.jl"))
}

fn get_julia_cflags() -> Fallible<Vec<String>> {
    let config = get_julia_config()?;
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

fn get_julia_ldflags() -> Fallible<Vec<String>> {
    let config = get_julia_config()?;
    let out = Command::new(config)
        .arg("--ldflags")
        .output()
        .expect("julia-config.jl command is not found");
    if !out.status.success() {
        bail!("Failed to generate ldflags",);
    }
    let flags = std::str::from_utf8(&out.stdout)?
        .split(' ')
        .flat_map(|s| {
            let flag = s.replace("'", "").trim().to_string();
            if flag.starts_with("-L") {
                Some(flag[2..].into())
            } else {
                None
            }
        })
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

    for libdir in get_julia_ldflags()? {
        println!("cargo:rustc-link-search=native={}", libdir);
        println!("cargo:rustc-link-search=native={}/julia", libdir);
    }
    println!("cargo:rustc-link-lib=julia");
    println!("cargo:rustc-link-lib=LLVM");
    Ok(())
}
