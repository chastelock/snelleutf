// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

/// Version of simdutf at compile time.
///
/// If provided by pkg-config, the output of `pkg-config --modversion simdutf`.
/// If built from snelleutf-src, computed from `CARGO_PKG_VERSION`.
pub const SIMDUTF_VERSION: &str = env!("SIMDUTF_VERSION");

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
