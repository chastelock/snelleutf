// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{env, path};

fn find_by_pkgconf() -> Result<path::PathBuf, pkg_config::Error> {
    let lib = pkg_config::Config::new()
        // This is when the C API was added
        .atleast_version("8.0.0")
        .probe("simdutf")?;

    Ok(lib
        .include_paths
        .iter()
        .find_map(|d| Some(d.join("simdutf_c.h")).filter(|p| p.is_file()))
        .expect("Found simdutf via pkg-config, didn't find its C header file"))
}

#[cfg(feature = "static-link")]
fn build_from_source() -> path::PathBuf {
    let artefacts = snelleutf_src::build();
    println!("cargo:rustc-link-search={}", artefacts.ar.display());
    artefacts.c_header
}

fn env_var(name: &'static str) -> Result<String, env::VarError> {
    println!("cargo:rerun-if-env-changed={name}");
    env::var(name)
}

fn find_library() -> path::PathBuf {
    let pkgconf_err = match env_var("SNELLEUTF_ALWAYS_STATIC").as_deref() {
        Ok("0") | Ok("") | Err(env::VarError::NotPresent) => match find_by_pkgconf() {
            Ok(p) => return p,
            Err(e) => Some(e),
        },
        Err(env::VarError::NotUnicode(_)) => panic!(),
        Ok(_) => None,
    };
    #[cfg(feature = "static-link")]
    {
        match env_var("SNELLEUTF_ALWAYS_DYNAMIC").as_deref() {
            Ok("0") | Ok("") | Err(env::VarError::NotPresent) => build_from_source(),
            Err(env::VarError::NotUnicode(_)) => panic!(),
            Ok(value) => {
                panic!(
                    "Failed to find simdutf. (Not built due to SNELLEUTF_ALWAYS_DYNAMIC={value:?}) {pkgconf_err:?}"
                );
            }
        }
    }
    #[cfg(not(feature = "static-link"))]
    {
        panic!(
            "Failed to find simdutf. (Can't be built without 'static-link' feature.) {pkgconf_err:?}"
        );
    }
}

fn main() {
    let header_path = find_library();

    let bindings = bindgen::Builder::default()
        .header(header_path.display().to_string())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate()
        .unwrap();

    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
