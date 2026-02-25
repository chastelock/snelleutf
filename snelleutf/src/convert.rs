// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ffi::c_char;

use snelleutf_sys::*;

use crate::error::*;

pub fn utf8_to_latin1_len(input: &[u8]) -> usize {
    unsafe { simdutf_latin1_length_from_utf8(input.as_ptr() as *const c_char, input.len()) }
}
/// Convert UTF-8 to ISO-8859-1 and add the result to a [Vec<u8>]
///
/// ```
/// # use snelleutf::convert::utf8_to_latin1_append_to_vec;
/// let input = "Barmen skænker øl.";
/// // This is UTF-8, so "ø" is "\xc3\xb8".
/// assert_eq!(input.as_bytes(), b"Barmen sk\xc3\xa6nker \xc3\xb8l.");
///
/// // So far, this is all ASCII, so valid either as UTF-8 or Latin-1. Not for long.
/// let mut output = b"Vidste du, at: ".to_vec();
///
/// utf8_to_latin1_append_to_vec(input.as_bytes(), &mut output).unwrap();
/// // And this is Latin-1, so "ø" is "\xf8".
/// assert_eq!(output, b"Vidste du, at: Barmen sk\xe6nker \xf8l.");
/// ```
#[cfg(feature = "alloc")]
pub fn utf8_to_latin1_append_to_vec(input: &[u8], output: &mut Vec<u8>) -> Result<()> {
    let added_len = utf8_to_latin1_len(input);
    output.reserve_exact(added_len);
    unsafe {
        match conv_error(simdutf_convert_utf8_to_latin1_with_errors(
            input.as_ptr() as *const c_char,
            input.len(),
            (output.as_mut_ptr() as *mut c_char).wrapping_add(output.len()),
        )) {
            Ok(real_added_len) => {
                output.set_len(output.len() + real_added_len);
                debug_assert_eq!(real_added_len, added_len);
                Ok(())
            }
            Err(e) => {
                if e.count > 0 {
                    output.set_len(output.len() + e.count - 1);
                }
                Err(e)
            }
        }
    }
}
#[cfg(feature = "alloc")]
pub fn utf8_to_latin1(input: &[u8]) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    utf8_to_latin1_append_to_vec(input, &mut output)?;
    Ok(output)
}

pub fn utf16_to_latin1_len(input_len: usize) -> usize {
    unsafe { simdutf_latin1_length_from_utf16(input_len) }
}
/// Convert UTF-16 to ISO-8859-1 and add the result to a [Vec<u8>]
///
/// ```
/// # use snelleutf::convert::utf16_to_latin1_append_to_vec;
/// let input = [ 66, 97, 114, 109, 101, 110, 32, 115, 107, 230, 110, 107, 101, 114, 32, 248, 108, 46 ];
///
/// let mut output = b"Vidste du, at: ".to_vec();
///
/// utf16_to_latin1_append_to_vec(&input, &mut output).unwrap();
/// assert_eq!(output, b"Vidste du, at: Barmen sk\xe6nker \xf8l.");
/// ```
#[cfg(feature = "alloc")]
pub fn utf16_to_latin1_append_to_vec(input: &[u16], output: &mut Vec<u8>) -> Result<()> {
    let added_len = utf16_to_latin1_len(input.len());
    output.reserve_exact(added_len);
    unsafe {
        match conv_error(simdutf_convert_utf16_to_latin1_with_errors(
            input.as_ptr(),
            input.len(),
            (output.as_mut_ptr() as *mut c_char).wrapping_add(output.len()),
        )) {
            Ok(real_added_len) => {
                output.set_len(output.len() + real_added_len);
                debug_assert_eq!(real_added_len, added_len);
                Ok(())
            }
            Err(e) => {
                if e.count > 0 {
                    output.set_len(output.len() + e.count - 1);
                }
                Err(e)
            }
        }
    }
}
#[cfg(feature = "alloc")]
pub fn utf16_to_latin1(input: &[u16]) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    utf16_to_latin1_append_to_vec(input, &mut output)?;
    Ok(output)
}

pub fn utf32_to_latin1_len(input_len: usize) -> usize {
    unsafe { simdutf_latin1_length_from_utf32(input_len) }
}
/// Convert UTF-32 to ISO-8859-1 and add the result to a [Vec<u8>]
///
/// ```
/// # use snelleutf::convert::utf32_to_latin1_append_to_vec;
/// let input = [ 66, 97, 114, 109, 101, 110, 32, 115, 107, 230, 110, 107, 101, 114, 32, 248, 108, 46 ];
///
/// let mut output = b"Vidste du, at: ".to_vec();
///
/// utf32_to_latin1_append_to_vec(&input, &mut output).unwrap();
/// assert_eq!(output, b"Vidste du, at: Barmen sk\xe6nker \xf8l.");
/// ```
#[cfg(feature = "alloc")]
pub fn utf32_to_latin1_append_to_vec(input: &[u32], output: &mut Vec<u8>) -> Result<()> {
    let added_len = utf32_to_latin1_len(input.len());
    output.reserve_exact(added_len);
    unsafe {
        match conv_error(simdutf_convert_utf32_to_latin1_with_errors(
            input.as_ptr(),
            input.len(),
            (output.as_mut_ptr() as *mut c_char).wrapping_add(output.len()),
        )) {
            Ok(real_added_len) => {
                output.set_len(output.len() + real_added_len);
                debug_assert_eq!(real_added_len, added_len);
                Ok(())
            }
            Err(e) => {
                if e.count > 0 {
                    output.set_len(output.len() + e.count - 1);
                }
                Err(e)
            }
        }
    }
}
#[cfg(feature = "alloc")]
pub fn utf32_to_latin1(input: &[u32]) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    utf32_to_latin1_append_to_vec(input, &mut output)?;
    Ok(output)
}

pub fn latin1_to_utf8_len(input: &[u8]) -> usize {
    unsafe { simdutf_utf8_length_from_latin1(input.as_ptr() as *const c_char, input.len()) }
}

pub fn utf16le_to_utf8_len(input: &[u16]) -> usize {
    unsafe { simdutf_utf8_length_from_utf16le(input.as_ptr(), input.len()) }
}
/// Convert UTF-16 LE to UTF-8 and add the result to a [Vec<u8>]
///
/// ```
/// # use snelleutf::convert::utf16le_to_utf8_append_to_vec;
/// let input = [ 66, 97, 114, 109, 101, 110, 32, 115, 107, 230, 110, 107, 101, 114, 32, 248, 108, 46 ];
///
/// let mut output = b"Vidste du, at: ".to_vec();
///
/// utf16le_to_utf8_append_to_vec(&input, &mut output).unwrap();
/// assert_eq!(&output, "Vidste du, at: Barmen skænker øl.".as_bytes());
/// ```
#[cfg(feature = "alloc")]
pub fn utf16le_to_utf8_append_to_vec(input: &[u16], output: &mut Vec<u8>) -> Result<()> {
    let added_len = utf16le_to_utf8_len(input);
    output.reserve_exact(added_len);
    unsafe {
        match conv_error(simdutf_convert_utf16le_to_utf8_with_errors(
            input.as_ptr(),
            input.len(),
            (output.as_mut_ptr() as *mut c_char).wrapping_add(output.len()),
        )) {
            Ok(real_added_len) => {
                output.set_len(output.len() + real_added_len);
                debug_assert_eq!(real_added_len, added_len);
                Ok(())
            }
            Err(e) => {
                if e.count > 0 {
                    output.set_len(output.len() + e.count - 1);
                }
                Err(e)
            }
        }
    }
}
#[cfg(feature = "alloc")]
pub fn utf16le_to_utf8_append_to_string(input: &[u16], output: &mut String) -> Result<()> {
    utf16le_to_utf8_append_to_vec(input, unsafe { output.as_mut_vec() })
}
/// Convert UTF-16 LE to UTF-8, returning a [String].
///
/// ```
/// # use snelleutf::convert::utf16le_to_utf8;
/// let input = [ 66, 97, 114, 109, 101, 110, 32, 115, 107, 230, 110, 107, 101, 114, 32, 248, 108, 46 ];
/// assert_eq!(utf16le_to_utf8(&input).unwrap(), "Barmen skænker øl.");
/// ```
#[cfg(feature = "alloc")]
pub fn utf16le_to_utf8(input: &[u16]) -> Result<String> {
    let mut output = String::new();
    utf16le_to_utf8_append_to_string(input, &mut output)?;
    Ok(output)
}

pub fn utf16be_to_utf8_len(input: &[u16]) -> usize {
    unsafe { simdutf_utf8_length_from_utf16be(input.as_ptr(), input.len()) }
}
/// Convert UTF-16 BE to UTF-8 and add the result to a [Vec<u8>]
///
/// ```
/// # use snelleutf::convert::utf16be_to_utf8_append_to_vec;
/// let input = [ 16896, 24832, 29184, 27904, 25856, 28160, 8192, 29440, 27392, 58880, 28160, 27392, 25856, 29184, 8192, 63488, 27648, 11776 ];
///
/// let mut output = b"Vidste du, at: ".to_vec();
///
/// utf16be_to_utf8_append_to_vec(&input, &mut output).unwrap();
/// assert_eq!(&output, "Vidste du, at: Barmen skænker øl.".as_bytes());
/// ```
#[cfg(feature = "alloc")]
pub fn utf16be_to_utf8_append_to_vec(input: &[u16], output: &mut Vec<u8>) -> Result<()> {
    let added_len = utf16be_to_utf8_len(input);
    output.reserve_exact(added_len);
    unsafe {
        match conv_error(simdutf_convert_utf16be_to_utf8_with_errors(
            input.as_ptr(),
            input.len(),
            (output.as_mut_ptr() as *mut c_char).wrapping_add(output.len()),
        )) {
            Ok(real_added_len) => {
                output.set_len(output.len() + real_added_len);
                debug_assert_eq!(real_added_len, added_len);
                Ok(())
            }
            Err(e) => {
                if e.count > 0 {
                    output.set_len(output.len() + e.count - 1);
                }
                Err(e)
            }
        }
    }
}
#[cfg(feature = "alloc")]
pub fn utf16be_to_utf8_append_to_string(input: &[u16], output: &mut String) -> Result<()> {
    utf16be_to_utf8_append_to_vec(input, unsafe { output.as_mut_vec() })
}
/// Convert UTF-16 BE to UTF-8, returning a [String].
///
/// ```
/// # use snelleutf::convert::utf16be_to_utf8;
/// let input = [ 16896, 24832, 29184, 27904, 25856, 28160, 8192, 29440, 27392, 58880, 28160, 27392, 25856, 29184, 8192, 63488, 27648, 11776 ];
/// assert_eq!(utf16be_to_utf8(&input).unwrap(), "Barmen skænker øl.");
/// ```
#[cfg(feature = "alloc")]
pub fn utf16be_to_utf8(input: &[u16]) -> Result<String> {
    let mut output = String::new();
    utf16be_to_utf8_append_to_string(input, &mut output)?;
    Ok(output)
}

pub fn utf8_to_utf16_len(input: &[u8]) -> usize {
    unsafe { simdutf_utf16_length_from_utf8(input.as_ptr() as *const c_char, input.len()) }
}
/// Convert UTF-8 to UTF-16 and add the result to a [Vec<u16>]
///
/// ```
/// # use snelleutf::convert::utf8_to_utf16_append_to_vec;
/// let input = "Barmen skænker øl.";
/// // This is UTF-8, so "ø" is "\xc3\xb8".
/// assert_eq!(input.as_bytes(), b"Barmen sk\xc3\xa6nker \xc3\xb8l.");
///
/// let mut output = vec![ 86, 105, 100, 115, 116, 101, 32, 100, 117, 44, 32, 97, 116, 58, 32 ];
///
/// utf8_to_utf16_append_to_vec(input.as_bytes(), &mut output).unwrap();
/// assert_eq!(output, [86, 105, 100, 115, 116, 101, 32, 100, 117, 44, 32, 97, 116, 58, 32, 66, 97, 114, 109, 101, 110, 32, 115, 107, 230, 110, 107, 101, 114, 32, 248, 108, 46 ]);
/// ```
#[cfg(feature = "alloc")]
pub fn utf8_to_utf16_append_to_vec(input: &[u8], output: &mut Vec<u16>) -> Result<()> {
    let added_len = utf8_to_utf16_len(input);
    output.reserve_exact(added_len);
    unsafe {
        match conv_error(simdutf_convert_utf8_to_utf16_with_errors(
            input.as_ptr() as *const c_char,
            input.len(),
            output.as_mut_ptr().wrapping_add(output.len()),
        )) {
            Ok(real_added_len) => {
                output.set_len(output.len() + real_added_len);
                debug_assert_eq!(real_added_len, added_len);
                Ok(())
            }
            Err(e) => {
                if e.count > 0 {
                    output.set_len(output.len() + e.count - 1);
                }
                Err(e)
            }
        }
    }
}
#[cfg(feature = "alloc")]
pub fn utf8_to_utf16(input: &[u8]) -> Result<Vec<u16>> {
    let mut output = Vec::new();
    utf8_to_utf16_append_to_vec(input, &mut output)?;
    Ok(output)
}

pub fn utf8_to_utf32_len(input: &[u8]) -> usize {
    unsafe { simdutf_utf32_length_from_utf8(input.as_ptr() as *const c_char, input.len()) }
}
/// Convert UTF-8 to UTF-32 and add the result to a [Vec<u32>]
///
/// ```
/// # use snelleutf::convert::utf8_to_utf32_append_to_vec;
/// let input = "Barmen skænker øl.";
/// // This is UTF-8, so "ø" is "\xc3\xb8".
/// assert_eq!(input.as_bytes(), b"Barmen sk\xc3\xa6nker \xc3\xb8l.");
///
/// let mut output = vec![ 86, 105, 100, 115, 116, 101, 32, 100, 117, 44, 32, 97, 116, 58, 32 ];
///
/// utf8_to_utf32_append_to_vec(input.as_bytes(), &mut output).unwrap();
/// assert_eq!(output, [86, 105, 100, 115, 116, 101, 32, 100, 117, 44, 32, 97, 116, 58, 32, 66, 97, 114, 109, 101, 110, 32, 115, 107, 230, 110, 107, 101, 114, 32, 248, 108, 46 ]);
/// ```
#[cfg(feature = "alloc")]
pub fn utf8_to_utf32_append_to_vec(input: &[u8], output: &mut Vec<u32>) -> Result<()> {
    let added_len = utf8_to_utf32_len(input);
    output.reserve_exact(added_len);
    unsafe {
        match conv_error(simdutf_convert_utf8_to_utf32_with_errors(
            input.as_ptr() as *const c_char,
            input.len(),
            output.as_mut_ptr().wrapping_add(output.len()),
        )) {
            Ok(real_added_len) => {
                output.set_len(output.len() + real_added_len);
                debug_assert_eq!(real_added_len, added_len);
                Ok(())
            }
            Err(e) => {
                if e.count > 0 {
                    output.set_len(output.len() + e.count - 1);
                }
                Err(e)
            }
        }
    }
}
#[cfg(feature = "alloc")]
pub fn utf8_to_utf32(input: &[u8]) -> Result<Vec<u32>> {
    let mut output = Vec::new();
    utf8_to_utf32_append_to_vec(input, &mut output)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn unsuccessful_append() {
        let mut output = b"Computer says: ".to_vec();
        // These quotemarks inside don't exist in Latin-1.
        let err = utf8_to_latin1_append_to_vec(
            "Naïef met z’n ‘crème brûlée’-vibes".as_bytes(),
            &mut output,
        )
        .unwrap_err();
        assert_eq!(err.count, "Naïef met z".len());
        assert_eq!(err.code, SimdutfError::SIMDUTF_ERROR_TOO_LARGE);
        assert_eq!(output, b"Computer says: Na\xefef met z");
    }
}
