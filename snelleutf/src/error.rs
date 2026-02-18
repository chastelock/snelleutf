// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::{error, fmt};

use snelleutf_sys::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnelError {
    pub code: simdutf_error_code,
    pub count: usize,
}
pub(crate) type Result<T, E = SnelError> = core::result::Result<T, E>;

impl fmt::Display for SnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SnelError({:?} at char {})", self.code, self.count)
    }
}
impl error::Error for SnelError {}

pub(crate) fn conv_error(result: simdutf_result) -> Result<()> {
    match result.error {
        simdutf_error_code::SIMDUTF_ERROR_SUCCESS => Ok(()),
        code @ _ => Err(SnelError {
            code,
            count: result.count,
        }),
    }
}
