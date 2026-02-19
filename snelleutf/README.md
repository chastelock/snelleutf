<!--
SPDX-FileCopyrightText: 2026 The Snelleutf Authors
SPDX-License-Identifier: CC0-1.0
-->

# Snelleutf

Fast Unicode operations. Bindings of [simdutf] to Rust.

This is the package with "safe" thin abstractions over the C API.
Raw generated bindings in [snelleutf-sys].

## Safety

This library provides "safe" bindings to simdutf, a C++ library, over its C API,
which is unsafe. Put different, this library _should_ be safe, but doesn't have
the typical memory safety guarantees of Rust.

To report security issues: see [SECURITY.md]

[simdutf]: https://simdutf.github.io/simdutf/
[snelleutf-sys]: https://crates.io/crates/snelleutf-sys
[SECURITY.md]: SECURITY.md
