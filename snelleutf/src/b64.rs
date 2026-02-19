// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "alloc")]

use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

use snelleutf_sys::*;

pub use snelleutf_sys::simdutf_base64_options as B64Options;

/// Expected length of base64 encoding from specified byte length
pub fn len_from_bytes(input_len: usize, options: B64Options) -> usize {
    unsafe { simdutf_base64_length_from_binary(input_len, options) }
}

/// Encode bytes as base64 and add the output to a [Vec<u8>].
///
/// To add the output to a [String], see [from_bytes_add_to_string].
/// To receive a [String] output, see [from_bytes].
pub fn from_bytes_add_to_vec(input: &[u8], options: B64Options, output: &mut Vec<u8>) {
    let added_capacity = len_from_bytes(input.len(), options);
    let original_len = output.len();
    output.reserve_exact(original_len + added_capacity);
    unsafe {
        let added_length = simdutf_binary_to_base64(
            input.as_ptr() as *const c_char,
            input.len(),
            output.as_mut_ptr() as *mut c_char,
            options,
        );
        debug_assert!(added_capacity >= added_length);
        output.set_len(original_len + added_length);
    }
}

/// Encode bytes as base64 and add the output to a [String].
///
/// To add the output to a [Vec<u8>], see [from_bytes_add_to_vec].
/// To receive a [String] output, see [from_bytes].
pub fn from_bytes_add_to_string(input: &[u8], options: B64Options, output: &mut String) {
    from_bytes_add_to_vec(input, options, unsafe { output.as_mut_vec() });
}

/// Encode bytes as base64 into a [String].
///
/// To add the output to a [Vec<u8>], see [from_bytes_add_to_vec].
/// To add the output to an existing [String], see [from_bytes_add_to_string].
///
/// ```
/// # use snelleutf::b64::{from_bytes, B64Options};
/// let bytes = "Zażółć gęślą jaźń".as_bytes();
/// assert_eq!(from_bytes(bytes, B64Options::SIMDUTF_BASE64_DEFAULT), "WmHFvMOzxYLEhyBnxJnFm2zEhSBqYcW6xYQ=");
/// ```
pub fn from_bytes(input: &[u8], options: B64Options) -> String {
    let mut output = String::new();
    from_bytes_add_to_string(input, options, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        const BYTES: [u8; 5] = [b'm', b'i', b'a', b'u', b'w'];
        assert_eq!(
            from_bytes(&BYTES, B64Options::SIMDUTF_BASE64_DEFAULT),
            "bWlhdXc="
        );
        assert_eq!(
            from_bytes(&BYTES, B64Options::SIMDUTF_BASE64_URL),
            "bWlhdXc"
        );
    }
}
