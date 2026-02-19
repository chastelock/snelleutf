<!--
SPDX-FileCopyrightText: 2026 The Snelleutf Authors
SPDX-License-Identifier: CC0-1.0
-->

# Snelleutf

Fast Unicode operations. Bindings of [simdutf] to Rust.

This is the package with an amalgamation of [simdutf], and a build script,
intended to be used by [snelleutf-sys].
On top of that, there are "safe" thin abstractions in [snelleutf].

## Safety

This library provides raw, unsafe, generated bindings to simdutf, a C++ library,
over its C API. "Safe" abstractions on top of them are available in [snelleutf].

To report security issues: see [SECURITY.md]

[simdutf]: https://simdutf.github.io/simdutf/
[snelleutf]: https://crates.io/crates/snelleutf
[snelleutf-sys]: https://crates.io/crates/snelleutf-sys
[SECURITY.md]: SECURITY.md
