// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::path::{Path, PathBuf};
use std::{env, fs};

pub struct Artefacts {
    pub ar: PathBuf,
    pub c_header: PathBuf,
}

const C_HEADER: &str = "simdutf_c.h";
const CPP_AMALGAM: &str = "simdutf.cpp";
const OUT_AR: &str = "libsimdutf.a";

pub fn build() -> Artefacts {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let out_dir = Path::new(&env::var_os("OUT_DIR").unwrap()).join("simdutf");
    cc::Build::new()
        .file(src_dir.join(CPP_AMALGAM))
        .cpp(true)
        .out_dir(&out_dir)
        .compile("simdutf");
    let c_header = out_dir.join(C_HEADER);
    fs::copy(src_dir.join(C_HEADER), &c_header).unwrap();
    Artefacts {
        ar: out_dir.join(OUT_AR),
        c_header,
    }
}
