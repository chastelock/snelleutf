// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use snelleutf_sys::*;

use crate::error::*;

pub fn validate_utf8(buf: &[u8]) -> bool {
    unsafe { simdutf_validate_utf8(buf.as_ptr() as *const i8, buf.len()) }
}
pub fn validate_utf8_with_errors(buf: &[u8]) -> Result<usize> {
    conv_error(unsafe { simdutf_validate_utf8_with_errors(buf.as_ptr() as *const i8, buf.len()) })
}
pub fn validate_utf8_as_str<'a>(buf: &'a [u8]) -> Result<&'a str> {
    match validate_utf8_with_errors(buf) {
        Ok(_) => Ok(unsafe { str::from_utf8_unchecked(buf) }),
        Err(e) => Err(e),
    }
}

pub fn validate_utf32(buf: &[u32]) -> bool {
    unsafe { simdutf_validate_utf32(buf.as_ptr(), buf.len()) }
}
pub fn validate_utf32_with_errors(buf: &[u32]) -> Result<usize> {
    conv_error(unsafe { simdutf_validate_utf32_with_errors(buf.as_ptr(), buf.len()) })
}

macro_rules! valid_utf16 {
    ($our_fn:ident, $their_fn:ident) => {
        pub fn $our_fn(buf: &[u16]) -> bool {
            unsafe { $their_fn(buf.as_ptr(), buf.len()) }
        }
    };
}
valid_utf16!(validate_utf16, simdutf_validate_utf16);
valid_utf16!(validate_utf16be, simdutf_validate_utf16be);
valid_utf16!(validate_utf16le, simdutf_validate_utf16le);
valid_utf16!(validate_utf16_as_ascii, simdutf_validate_utf16_as_ascii);
valid_utf16!(validate_utf16be_as_ascii, simdutf_validate_utf16be_as_ascii);
valid_utf16!(validate_utf16le_as_ascii, simdutf_validate_utf16le_as_ascii);

macro_rules! valid_utf16_err {
    ($our_fn:ident, $their_fn:ident) => {
        pub fn $our_fn(buf: &[u16]) -> Result<usize> {
            conv_error(unsafe { $their_fn(buf.as_ptr(), buf.len()) })
        }
    };
}
valid_utf16_err!(
    validate_utf16_with_errors,
    simdutf_validate_utf16_with_errors
);
valid_utf16_err!(
    validate_utf16be_with_errors,
    simdutf_validate_utf16be_with_errors
);
valid_utf16_err!(
    validate_utf16le_with_errors,
    simdutf_validate_utf16le_with_errors
);

#[cfg(test)]
mod tests {
    use super::*;

    const ZAZOLC_STR: &str = "zazółć gęślą jaźń";
    const ZAZOLC_U8: [u8; 25] = [
        122, 97, 122, 195, 179, 197, 130, 196, 135, 32, 103, 196, 153, 197, 155, 108, 196, 133, 32,
        106, 97, 197, 186, 197, 132,
    ];
    const ZAZOLC_U16: [u16; 36] = [
        255, 254, 122, 0, 97, 0, 122, 0, 243, 0, 66, 1, 7, 1, 32, 0, 103, 0, 25, 1, 91, 1, 108, 0,
        5, 1, 32, 0, 106, 0, 97, 0, 122, 1, 68, 1,
    ];
    const ZAZOLC_U16BE: [u16; 34] = [
        0, 122, 0, 97, 0, 122, 0, 243, 1, 66, 1, 7, 0, 32, 0, 103, 1, 25, 1, 91, 0, 108, 1, 5, 0,
        32, 0, 106, 0, 97, 1, 122, 1, 68,
    ];
    const ZAZOLC_U16LE: [u16; 34] = [
        122, 0, 97, 0, 122, 0, 243, 0, 66, 1, 7, 1, 32, 0, 103, 0, 25, 1, 91, 1, 108, 0, 5, 1, 32,
        0, 106, 0, 97, 0, 122, 1, 68, 1,
    ];

    #[test]
    fn u8() {
        assert!(validate_utf8(b""));
        assert!(validate_utf8(&ZAZOLC_U8));
        assert_eq!(validate_utf8_as_str(&ZAZOLC_U8).unwrap(), ZAZOLC_STR);
        assert!(validate_utf8_with_errors(&ZAZOLC_U8).is_ok());
    }

    #[test]
    fn u16() {
        assert!(validate_utf16(&[]));
        assert!(validate_utf16(&ZAZOLC_U16));
        assert!(validate_utf16_with_errors(&ZAZOLC_U16).is_ok());
        assert!(!validate_utf16_as_ascii(&ZAZOLC_U16));
    }

    #[test]
    fn u16be() {
        assert!(validate_utf16be(&[]));
        assert!(validate_utf16be(&ZAZOLC_U16BE));
        assert!(validate_utf16be_with_errors(&ZAZOLC_U16BE).is_ok());
        assert!(!validate_utf16be_as_ascii(&ZAZOLC_U16BE));
    }

    #[test]
    fn u16le() {
        assert!(validate_utf16le(&[]));
        assert!(validate_utf16le(&ZAZOLC_U16LE));
        assert!(validate_utf16le_with_errors(&ZAZOLC_U16LE).is_ok());
        assert!(!validate_utf16le_as_ascii(&ZAZOLC_U16LE));
    }
}
