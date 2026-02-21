// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use snelleutf_sys::*;

use core::ffi::c_char;

pub use snelleutf_sys::simdutf_encoding_type as EncodingType;

pub fn autodetect_encoding(input: &[u8]) -> EncodingType {
    unsafe { simdutf_autodetect_encoding(input.as_ptr() as *const c_char, input.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto() {
        assert_eq!(
            autodetect_encoding(b"\x92"),
            EncodingType::SIMDUTF_ENCODING_UNSPECIFIED
        );
        assert_eq!(
            autodetect_encoding(b""),
            EncodingType::SIMDUTF_ENCODING_UTF8
        );
        assert_eq!(
            autodetect_encoding("Zażółć gęślą jaźń".as_bytes()),
            EncodingType::SIMDUTF_ENCODING_UTF8
        );
        assert_eq!(
            autodetect_encoding(b"\xff\xfe\x05\x01"),
            EncodingType::SIMDUTF_ENCODING_UTF16_LE
        );
    }
}
