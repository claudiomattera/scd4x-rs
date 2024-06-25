// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for miscellaneous utilities

use crate::verify_checksum;
use crate::Error;

/// Convert a 9-bytes buffer to three words
///
/// # Errors
///
/// Returns an error if the checksum does not match.
pub(crate) fn buffer_to_three_words(buffer: [u8; 9]) -> Result<(u16, u16, u16), Error> {
    let word0 = buffer_to_word(buffer[0], buffer[1], buffer[2])?;
    let word1 = buffer_to_word(buffer[3], buffer[4], buffer[5])?;
    let word2 = buffer_to_word(buffer[6], buffer[7], buffer[8])?;

    Ok((word0, word1, word2))
}

/// Convert three bytes buffer to a word
///
/// # Errors
///
/// Returns an error if the checksum does not match.
pub(crate) fn buffer_to_word(byte0: u8, byte1: u8, byte2: u8) -> Result<u16, Error> {
    let word = (u16::from(byte0) << 8) + u16::from(byte1);

    verify_checksum([byte0, byte1], byte2)?;

    Ok(word)
}
