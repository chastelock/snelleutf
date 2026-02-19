// SPDX-FileCopyrightText: 2026 The Snelleutf Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "std")]
use std::io;

use crate::error::*;

#[cfg(feature = "std")]
pub fn read_file_to_string<P>(path: P) -> Result<String, io::Error>
where
    P: AsRef<std::path::Path>,
{
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    match crate::validate::validate_utf8_with_errors(&buf) {
        Ok(_) => Ok(unsafe { String::from_utf8_unchecked(buf) }),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}
