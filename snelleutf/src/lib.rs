// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod b64;
pub mod count;
pub mod detect;
pub mod error;
pub mod utils;
pub mod validate;

pub use snelleutf_sys::SIMDUTF_VERSION;
