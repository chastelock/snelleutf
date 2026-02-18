// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{env, path};

fn main() {
    let lib = pkg_config::Config::new()
        // This is when the C API was added
        .atleast_version("8.0.0")
        .probe("simdutf")
        .unwrap();

    let header_path = lib
        .include_paths
        .iter()
        .find_map(|d| Some(d.join("simdutf_c.h")).filter(|p| p.is_file()))
        .unwrap();

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
