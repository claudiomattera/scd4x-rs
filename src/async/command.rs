// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

//! Data types and functions for SCD4x command types

use core::time::Duration;

use log::trace;

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

use crate::compute_checksum;
use crate::util::buffer_to_three_words;
use crate::util::buffer_to_word;
use crate::Error;

/// A command to the sensor
pub(crate) trait Command {
    /// The sequence type of the command
    type SequenceType: SequenceType<Input = Self::SequenceInput, Output = Self::SequenceOutput>;

    /// The input type for the sequence type
    type SequenceInput;

    /// The output type for the sequence type
    type SequenceOutput;

    /// The input type for the command
    type Input;

    /// The output type for the command
    type Output;

    /// Return the I²C register of the command
    fn register(&self) -> u16;

    /// Return the maximal duration of the command
    fn max_duration(&self) -> Duration;

    /// Map the input for the command to the input for the sequence
    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput;

    /// Map the output for the sequence to the output for the command
    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output;

    /// Execute the command over the I²C bus
    async fn execute(
        &self,
        address: u8,
        i2c: &mut impl I2c,
        delayer: &mut impl DelayNs,
        input: Self::Input,
    ) -> Result<Self::Output, Error> {
        let input = self.preprocess(input);
        let output = Self::SequenceType::execute(
            address,
            i2c,
            delayer,
            self.max_duration(),
            self.register(),
            input,
        )
        .await?;
        let output = self.postprocess(output);
        Ok(output)
    }
}

/// An I²C sequence
pub(crate) trait SequenceType {
    /// The input type for the sequence
    type Input;

    /// The output type for the sequence
    type Output;

    /// Execute the sequence over the I²C bus
    async fn execute(
        address: u8,
        i2c: &mut impl I2c,
        delayer: &mut impl DelayNs,
        delayer: Duration,
        register: u16,
        input: Self::Input,
    ) -> Result<Self::Output, Error>;
}

/// A sequence for sending standalone commands
pub(crate) struct SendCommandSequence;

impl SequenceType for SendCommandSequence {
    type Input = ();

    type Output = ();

    async fn execute(
        address: u8,
        i2c: &mut impl I2c,
        delayer: &mut impl DelayNs,
        delay: Duration,
        register: u16,
        (): Self::Input,
    ) -> Result<Self::Output, Error> {
        let buffer: [u8; 2] = register.to_be_bytes();

        trace!("Write data 0x{:02x}{:02x}", buffer[0], buffer[1]);
        i2c.write(address, &buffer).await?;

        trace!("Wait {delay:?}");
        #[allow(clippy::cast_possible_truncation)]
        delayer.delay_ms(delay.as_millis() as u32).await;

        Ok(())
    }
}

/// A sequence for sending commands and fetching result
pub(crate) struct SendCommandAndFetchResultSequence;

impl SequenceType for SendCommandAndFetchResultSequence {
    type Input = u16;

    type Output = u16;

    async fn execute(
        address: u8,
        i2c: &mut impl I2c,
        delayer: &mut impl DelayNs,
        delay: Duration,
        register: u16,
        input: Self::Input,
    ) -> Result<Self::Output, Error> {
        let register_buffer: [u8; 2] = register.to_be_bytes();

        let input_buffer = input.to_be_bytes();

        let checksum = compute_checksum(input_buffer);

        let buffer: [u8; 5] = [
            register_buffer[0],
            register_buffer[1],
            input_buffer[0],
            input_buffer[1],
            checksum,
        ];

        trace!(
            "Write data 0x{:02x}{:02x} 0x{:02x}{:02x} (CRC 0x{:02x})",
            buffer[0],
            buffer[1],
            buffer[2],
            buffer[3],
            buffer[4]
        );

        i2c.write(address, &buffer).await?;

        trace!("Wait {delay:?}");
        #[allow(clippy::cast_possible_truncation)]
        delayer.delay_ms(delay.as_millis() as u32).await;

        let mut output_buffer: [u8; 3] = [0; 3];

        trace!("Read data");
        i2c.read(address, &mut output_buffer).await?;

        trace!(
            "Read data 0x{:02x}{:02x} (CRC 0x{:02x})",
            output_buffer[0],
            output_buffer[1],
            output_buffer[2]
        );
        let word = buffer_to_word(output_buffer[0], output_buffer[1], output_buffer[2])?;
        Ok(word)
    }
}

/// A sequence for reading a single word
pub(crate) struct ReadWordSequence;

impl SequenceType for ReadWordSequence {
    type Input = ();

    type Output = u16;

    async fn execute(
        address: u8,
        i2c: &mut impl I2c,
        delayer: &mut impl DelayNs,
        delay: Duration,
        register: u16,
        (): Self::Input,
    ) -> Result<Self::Output, Error> {
        let buffer: [u8; 2] = register.to_be_bytes();
        let mut output_buffer: [u8; 3] = [0; 3];

        trace!("Write data 0x{:02x}{:02x}", buffer[0], buffer[1]);
        i2c.write(address, &buffer).await?;

        trace!("Wait {delay:?}");
        #[allow(clippy::cast_possible_truncation)]
        delayer.delay_ms(delay.as_millis() as u32).await;

        trace!("Read data");
        i2c.read(address, &mut output_buffer).await?;

        trace!(
            "Read data 0x{:02x}{:02x} (CRC 0x{:02x})",
            output_buffer[0],
            output_buffer[1],
            output_buffer[2]
        );
        let word = buffer_to_word(output_buffer[0], output_buffer[1], output_buffer[2])?;
        Ok(word)
    }
}

/// A sequence for reading three words
pub(crate) struct ReadThreeWordsSequence;

impl SequenceType for ReadThreeWordsSequence {
    type Input = ();

    type Output = (u16, u16, u16);

    async fn execute(
        address: u8,
        i2c: &mut impl I2c,
        delayer: &mut impl DelayNs,
        delay: Duration,
        register: u16,
        (): Self::Input,
    ) -> Result<Self::Output, Error> {
        let buffer: [u8; 2] = register.to_be_bytes();
        let mut output_buffer: [u8; 9] = [0; 9];

        trace!("Write data 0x{:02x}{:02x}", buffer[0], buffer[1]);
        i2c.write(address, &buffer).await?;

        trace!("Wait {delay:?}");
        #[allow(clippy::cast_possible_truncation)]
        delayer.delay_ms(delay.as_millis() as u32).await;

        trace!("Read data");
        i2c.read(address, &mut output_buffer).await?;

        trace!(
            "Read data 0x{:02x}{:02x} (CRC 0x{:02x}) 0x{:02x}{:02x} (CRC 0x{:02x}) 0x{:02x}{:02x} (CRC 0x{:02x})",
            output_buffer[0],
            output_buffer[1],
            output_buffer[2],
            output_buffer[3],
            output_buffer[4],
            output_buffer[5],
            output_buffer[6],
            output_buffer[7],
            output_buffer[8],
        );

        let word = buffer_to_three_words(output_buffer)?;
        Ok(word)
    }
}

/// A sequence for writing a word
pub(crate) struct WriteWordSequence;

impl SequenceType for WriteWordSequence {
    type Input = u16;

    type Output = ();

    async fn execute(
        address: u8,
        i2c: &mut impl I2c,
        delayer: &mut impl DelayNs,
        delay: Duration,
        register: u16,
        input: Self::Input,
    ) -> Result<Self::Output, Error> {
        let register_buffer: [u8; 2] = register.to_be_bytes();

        let input_buffer = input.to_be_bytes();

        let checksum = compute_checksum(input_buffer);

        let buffer: [u8; 5] = [
            register_buffer[0],
            register_buffer[1],
            input_buffer[0],
            input_buffer[1],
            checksum,
        ];

        trace!(
            "Write data 0x{:02x}{:02x} 0x{:02x}{:02x} (CRC 0x{:02x})",
            buffer[0],
            buffer[1],
            buffer[2],
            buffer[3],
            buffer[4]
        );

        i2c.write(address, &buffer).await?;

        trace!("Wait {delay:?}");
        #[allow(clippy::cast_possible_truncation)]
        delayer.delay_ms(delay.as_millis() as u32).await;

        Ok(())
    }
}
