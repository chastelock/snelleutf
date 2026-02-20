// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "alloc")]

use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

use snelleutf_sys::*;

pub use snelleutf_sys::{
    simdutf_base64_options as B64Options, simdutf_last_chunk_handling_options as LastChunkOptions,
};

use crate::error::*;

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
    output.reserve_exact(added_capacity);
    unsafe {
        let added_length = simdutf_binary_to_base64(
            input.as_ptr() as *const c_char,
            input.len(),
            (output.as_mut_ptr() as *mut c_char).wrapping_add(original_len),
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

/// Highest possible binary length when decoded from Base64
pub fn max_len_to_bytes<I>(input: I) -> usize
where
    I: AsRef<[u8]>,
{
    let input_ = input.as_ref();
    unsafe {
        simdutf_maximal_binary_length_from_base64(input_.as_ptr() as *const c_char, input_.len())
    }
}

pub fn to_bytes_add_into_vec(
    input: &[u8],
    options: B64Options,
    last_chunk_options: LastChunkOptions,
    output: &mut Vec<u8>,
) -> Result<()> {
    let capacity = max_len_to_bytes(input);
    let original_len = output.len();
    output.reserve_exact(capacity);
    unsafe {
        let added_length = conv_error(simdutf_base64_to_binary(
            input.as_ptr() as *const c_char,
            input.len(),
            (output.as_mut_ptr() as *mut c_char).wrapping_add(original_len),
            options,
            last_chunk_options,
        ))?;
        assert!(capacity >= added_length);
        output.set_len(original_len + added_length);
    }
    Ok(())
}

/// Decode a Base64 string into [Vec<u8>].
///
/// ```
/// # use snelleutf::b64::{to_bytes, B64Options, LastChunkOptions};
/// assert_eq!(
///     to_bytes(
///         "AAAAACAB".as_bytes(),
///         B64Options::SIMDUTF_BASE64_DEFAULT,
///         LastChunkOptions::SIMDUTF_LAST_CHUNK_LOOSE,
///     ).unwrap(),
///     [0, 0, 0, 0, 32, 1],
/// );
/// ```
pub fn to_bytes(
    input: &[u8],
    options: B64Options,
    last_chunk_options: LastChunkOptions,
) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    to_bytes_add_into_vec(input, options, last_chunk_options, &mut output)?;
    Ok(output)
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
        let mut prefixed = "prefix-preexisting-in-the-string-".to_string();
        from_bytes_add_to_string(&BYTES, B64Options::SIMDUTF_BASE64_DEFAULT, &mut prefixed);
        assert_eq!(prefixed, "prefix-preexisting-in-the-string-bWlhdXc=");
    }

    #[test]
    fn decode() {
        assert_eq!(
            to_bytes(
                b"bWlhdXc=",
                B64Options::SIMDUTF_BASE64_DEFAULT,
                LastChunkOptions::SIMDUTF_LAST_CHUNK_LOOSE
            )
            .unwrap(),
            b"miauw",
        );
        let mut prefixed = b"prefix-preexisting-in-the-vec-".to_vec();
        to_bytes_add_into_vec(
            b"bWlhdXc=",
            B64Options::SIMDUTF_BASE64_DEFAULT,
            LastChunkOptions::SIMDUTF_LAST_CHUNK_LOOSE,
            &mut prefixed,
        )
        .unwrap();
        assert_eq!(prefixed, b"prefix-preexisting-in-the-vec-miauw");
    }
}
