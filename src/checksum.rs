// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for checksum computation

use crate::Error;

/// Verify that a buffer has a given checksum
///
/// # Errors
///
/// Returns an error if the checksum does not match.
pub fn verify(data: [u8; 2], expected: u8) -> Result<(), Error> {
    let actual = compute(data);
    if actual == expected {
        Ok(())
    } else {
        Err(Error::ChecksumMismatch { actual, expected })
    }
}

/// Compute the checksum of a buffer
pub fn compute(data: [u8; 2]) -> u8 {
    /// Polynomial for CRC computation
    const CRC8_POLYNOMIAL: u8 = 0x31;

    /// Initial value for CRC computation
    const CRC8_INIT: u8 = 0xff;

    let mut crc = CRC8_INIT;

    for datum in data {
        crc ^= datum;

        #[allow(clippy::assign_op_pattern)]
        for _ in 0..8 {
            if crc & 0x80 != 0x00 {
                crc = (crc << 1) ^ CRC8_POLYNOMIAL;
            } else {
                crc = crc << 1;
            }
        }
    }

    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_checksum() {
        let samples: &[(u16, u8)] = &[
            (0xbeef, 0x92),
            (0xf896, 0x31),
            (0x9f07, 0xc2),
            (0x3bbe, 0x89),
            (0x5eb9, 0x3c),
            (0x6667, 0xa2),
            (0x01f4, 0x33),
            (0xffff, 0xac),
        ];

        for &(input, expected) in samples {
            let buffer = input.to_be_bytes();
            let actual = compute(buffer);
            assert_eq!(
                actual, expected,
                "0x{input:04x} => 0x{actual:02x} == 0x{expected:02x}"
            );
        }
    }
}
