// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::ffi::c_char;

use snelleutf_sys::*;

use crate::error::*;

/// Check if the input is valid UTF-8, return bool with the answer.
///
/// ```
/// # use snelleutf::validate::validate_utf8;
/// assert_eq!(validate_utf8(b"Ich muss lecker Kaffee trinken"), true);
///
/// // This is a CP1252 character, not UTF-8!
/// assert_eq!(validate_utf8(b"\x806,70, alsjeblieft"), false);
/// ```
pub fn validate_utf8(buf: &[u8]) -> bool {
    unsafe { simdutf_validate_utf8(buf.as_ptr() as *const c_char, buf.len()) }
}
/// Check if the input is valid UTF-8, return its length or a [SnelError].
///
/// ```
/// # use snelleutf::error::SimdutfError;
/// # use snelleutf::validate::validate_utf8_with_errors;
/// assert_eq!(validate_utf8_with_errors(b"Ich muss lecker Kaffee trinken"), Ok(30));
///
/// // This is a CP1252 character, not UTF-8!
/// let cp1252_error = validate_utf8_with_errors(b"Voor \x806,70").unwrap_err();
/// assert_eq!(cp1252_error.code, SimdutfError::SIMDUTF_ERROR_TOO_LONG);
/// // 5 bytes parsed successfully, error at byte 5.
/// assert_eq!(cp1252_error.count, 5);
/// ```
pub fn validate_utf8_with_errors(buf: &[u8]) -> Result<usize> {
    conv_error(unsafe {
        simdutf_validate_utf8_with_errors(buf.as_ptr() as *const c_char, buf.len())
    })
}
/// Check if the input bytes are valid UTF-8, return them back as [str], or an error.
///
/// ```
/// # use snelleutf::error::SimdutfError;
/// # use snelleutf::validate::validate_utf8_as_str;
/// assert_eq!(validate_utf8_as_str(
///     b"\xc3\x9cbercoole \xc3\xa4cc\xc3\xa8nts, na\xc3\xafve houding en clich\xc3\xa9matige fr\xc3\xb4lerie"),
///     Ok("Übercoole äccènts, naïve houding en clichématige frôlerie"),
/// );
///
/// // This is a CP1252 character, not UTF-8!
/// let cp1252_error = validate_utf8_as_str(b"Voor \x806,70").unwrap_err();
/// assert_eq!(cp1252_error.code, SimdutfError::SIMDUTF_ERROR_TOO_LONG);
/// // 5 bytes parsed successfully, error at byte 5.
/// assert_eq!(cp1252_error.count, 5);
/// ```
pub fn validate_utf8_as_str<'a>(buf: &'a [u8]) -> Result<&'a str> {
    match validate_utf8_with_errors(buf) {
        Ok(_) => Ok(unsafe { str::from_utf8_unchecked(buf) }),
        Err(e) => Err(e),
    }
}

/// Check if the input is valid UTF-32, return a bool with the answer.
///
/// ```
/// # use snelleutf::error::SimdutfError;
/// # use snelleutf::validate::validate_utf32;
/// assert!(validate_utf32(
///     // スポットレイト
///     &[255, 254, 0, 0, 185, 48, 0, 0, 221, 48, 0, 0, 195, 48, 0, 0, 200, 48, 0, 0, 236, 48, 0, 0, 164, 48, 0, 0, 200, 48, 0, 0]
/// ));
/// ```
pub fn validate_utf32(buf: &[u32]) -> bool {
    unsafe { simdutf_validate_utf32(buf.as_ptr(), buf.len()) }
}
/// Check if the input is valid UTF-32, return its length, or an error.
///
/// ```
/// # use snelleutf::error::SimdutfError;
/// # use snelleutf::validate::validate_utf32_with_errors;
/// assert_eq!(validate_utf32_with_errors(
///     // スポットレイト
///     &[65279, 12473, 12509, 12483, 12488, 12524, 12452, 12488]
/// ), Ok(8));
/// ```
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
