// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for error handling

use embedded_hal::i2c::{Error as I2cError, ErrorKind as I2cErrorKind};

/// An error
#[derive(Debug, PartialEq)]
pub enum Error {
    /// A checksum was different than expected
    ChecksumMismatch {
        /// Actual checksum
        actual: u8,

        /// Expected checksum
        expected: u8,
    },

    /// An error in the  underlying I²C system
    I2c(I2cErrorKind),
}

impl<E> From<E> for Error
where
    E: I2cError,
{
    fn from(error: E) -> Self {
        Self::I2c(error.kind())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
