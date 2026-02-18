// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use snelleutf_sys::*;

pub fn count_utf8(input: &[u8]) -> usize {
    unsafe { simdutf_count_utf8(input.as_ptr() as *const i8, input.len()) }
}
pub fn count_str(input: &str) -> usize {
    count_utf8(input.as_bytes())
}

macro_rules! count_u16 {
    ($our_fn:ident, $their_fn:ident) => {
        pub fn $our_fn(buf: &[u16]) -> usize {
            unsafe { $their_fn(buf.as_ptr(), buf.len()) }
        }
    };
}
count_u16!(count_utf16, simdutf_count_utf16);
count_u16!(count_utf16be, simdutf_count_utf16be);
count_u16!(count_utf16le, simdutf_count_utf16le);
