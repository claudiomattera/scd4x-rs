// Copyright Claudio Mattera 2024.
//
// Distributed under the MIT License or the Apache 2.0 License at your option.
// See the accompanying files License-MIT.txt and License-Apache-2.0.txt, or
// online at
// https://opensource.org/licenses/MIT
// https://opensource.org/licenses/Apache-2.0

#![allow(clippy::semicolon_if_nothing_returned)]

//! Available commands
//!
//! Detailed description of each command can be found in the [SCD4x Datasheet].
//!
//! [SCD4x Datasheet]: https://sensirion.com/products/catalog/SCD40/

use core::time::Duration;

use crate::{
    conversion::{
        altitude_to_word, ambient_pressure_to_word, co2_to_word, signed_word_to_co2,
        temperature_offset_to_word, word_to_altitude, word_to_temperature_offset, words_to_sample,
        words_to_serial_number,
    },
    sample::Sample,
    Altitude, Co2, Pressure, Temperature,
};

use super::command::{
    Command, ReadThreeWordsSequence, ReadWordSequence, SendCommandAndFetchResultSequence,
    SendCommandSequence, WriteWordSequence,
};

/// Command for reinitializing the sensor
///
/// The command restores user settings from EEPROM
/// If reinitialization does not succeed, the sensor should be power-cycled.
pub(crate) struct Reinitialize;

impl Command for Reinitialize {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x3646
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(20)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output {
        output
    }
}

/// Command for getting the serial number
///
/// The serial number is a 48 bits unique number that can identify the chip and
/// verify the communication with the sensor.
pub(crate) struct GetSerialNumber;
impl Command for GetSerialNumber {
    type SequenceType = ReadThreeWordsSequence;

    type Input = ();

    type Output = u64;

    type SequenceInput = ();

    type SequenceOutput = (u16, u16, u16);

    fn register(&self) -> u16 {
        0x3682
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, (word0, word1, word2): Self::SequenceOutput) -> Self::Output {
        words_to_serial_number(word0, word1, word2)
    }
}

/// Command for reading a measurement from the sensor
///
/// The measurement data is only available once per signal-update interval,
/// otherwise the sensor will return an I²C NACK.
/// The command [`GetDataReadyStatus`] can be used to verify that data is
/// available before executing this command.
pub(crate) struct ReadMeasurement;
impl Command for ReadMeasurement {
    type SequenceType = ReadThreeWordsSequence;

    type Input = ();

    type Output = Sample;

    type SequenceInput = ();

    type SequenceOutput = (u16, u16, u16);

    fn register(&self) -> u16 {
        0xec05
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, (word0, word1, word2): Self::SequenceOutput) -> Self::Output {
        words_to_sample(word0, word1, word2)
    }
}

/// Command for starting periodic measurement
///
/// The signal-update interval is 5 seconds.
pub(crate) struct StartPeriodicMeasurement;
impl Command for StartPeriodicMeasurement {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x21b1
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(0)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output {
        output
    }
}

/// Command for stopping periodic measurement
pub(crate) struct StopPeriodicMeasurement;
impl Command for StopPeriodicMeasurement {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x3f86
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(500)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output {
        output
    }
}

/// Command for getting the temperature offset
pub(crate) struct GetTemperatureOffset;
impl Command for GetTemperatureOffset {
    type SequenceType = ReadWordSequence;

    type Input = ();

    type Output = Temperature;

    type SequenceInput = ();

    type SequenceOutput = u16;

    fn register(&self) -> u16 {
        0x2318
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, word: Self::SequenceOutput) -> Self::Output {
        word_to_temperature_offset(word)
    }
}

/// Command for setting the temperature offset
pub(crate) struct SetTemperatureOffset;
impl Command for SetTemperatureOffset {
    type SequenceType = WriteWordSequence;

    type Input = Temperature;

    type Output = ();

    type SequenceInput = u16;

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x241d
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, temperature_offset: Self::Input) -> Self::SequenceInput {
        temperature_offset_to_word(temperature_offset)
    }

    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output {
        output
    }
}

/// Command for getting the sensor altitude
pub(crate) struct GetSensorAltitude;
impl Command for GetSensorAltitude {
    type SequenceType = ReadWordSequence;

    type Input = ();

    type Output = Altitude;

    type SequenceInput = ();

    type SequenceOutput = u16;

    fn register(&self) -> u16 {
        0x2322
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, word: Self::SequenceOutput) -> Self::Output {
        word_to_altitude(word)
    }
}

/// Command for setting the sensor altitude
pub(crate) struct SetSensorAltitude;
impl Command for SetSensorAltitude {
    type SequenceType = WriteWordSequence;

    type Input = Altitude;

    type Output = ();

    type SequenceInput = u16;

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x2427
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, sensor_altitude: Self::Input) -> Self::SequenceInput {
        altitude_to_word(sensor_altitude)
    }

    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output {
        output
    }
}

/// Command for setting the ambient pressure
pub(crate) struct SetAmbientPressure;
impl Command for SetAmbientPressure {
    type SequenceType = WriteWordSequence;

    type Input = Pressure;

    type Output = ();

    type SequenceInput = u16;

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0xe000
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, ambient_pressure: Self::Input) -> Self::SequenceInput {
        ambient_pressure_to_word(ambient_pressure)
    }

    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output {
        output
    }
}

/// Command for starting low-power periodic measurement
pub(crate) struct StartLowPowerPeriodicMeasurement;
impl Command for StartLowPowerPeriodicMeasurement {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x21ac
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(0)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, output: Self::SequenceOutput) -> Self::Output {
        output
    }
}

/// Command for querying whether data is available to be read
pub(crate) struct GetDataReadyStatus;
impl Command for GetDataReadyStatus {
    type SequenceType = ReadWordSequence;

    type Input = ();

    type Output = bool;

    type SequenceInput = ();

    type SequenceOutput = u16;

    fn register(&self) -> u16 {
        0xe4b8
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, word: Self::SequenceOutput) -> Self::Output {
        let result = word & 0b0000_0111_1111_1111;
        result != 0
    }
}

/// Command for performing a self-test
pub(crate) struct PerformSelfTest;
impl Command for PerformSelfTest {
    type SequenceType = ReadWordSequence;

    type Input = ();

    type Output = bool;

    type SequenceInput = ();

    type SequenceOutput = u16;

    fn register(&self) -> u16 {
        0x3639
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(10000)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, word: Self::SequenceOutput) -> Self::Output {
        word == 0
    }
}

/// Command for reading a single-shot measurement
pub(crate) struct MeasureSingleShot;
impl Command for MeasureSingleShot {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x219d
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(5000)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, input: Self::SequenceOutput) -> Self::Output {
        input
    }
}

/// Command for reading a single-shot measurement of humidity and temperature
pub(crate) struct MeasureSingleShotRhtOnly;
impl Command for MeasureSingleShotRhtOnly {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x2196
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, input: Self::SequenceOutput) -> Self::Output {
        input
    }
}

/// Command for persisting settings
pub(crate) struct PersistSettings;
impl Command for PersistSettings {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x3615
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(800)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, input: Self::SequenceOutput) -> Self::Output {
        input
    }
}

/// Command for performing forced recalibration
pub(crate) struct PerformForcedRecalibration;
impl Command for PerformForcedRecalibration {
    type SequenceType = SendCommandAndFetchResultSequence;

    type Input = Co2;

    type Output = Option<Co2>;

    type SequenceInput = u16;

    type SequenceOutput = u16;

    fn register(&self) -> u16 {
        0x362f
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(800)
    }

    fn preprocess(&self, co2: Self::Input) -> Self::SequenceInput {
        co2_to_word(co2)
    }

    fn postprocess(&self, word: Self::SequenceOutput) -> Self::Output {
        if word == 0xffff {
            None
        } else {
            let wrapped_word: u16 = word.wrapping_sub(0x8000);

            #[allow(clippy::cast_possible_wrap)]
            let signed_word: i16 = wrapped_word as i16;

            Some(signed_word_to_co2(signed_word))
        }
    }
}

/// Command for querying whether automatic self-calibration is enabled
pub(crate) struct GetAutomaticSelfCalibrationEnabled;
impl Command for GetAutomaticSelfCalibrationEnabled {
    type SequenceType = ReadWordSequence;

    type Input = ();

    type Output = bool;

    type SequenceInput = ();

    type SequenceOutput = u16;

    fn register(&self) -> u16 {
        0x2313
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, word: Self::SequenceOutput) -> Self::Output {
        word & 0b0000_0001 != 0
    }
}

/// Command for setting whether automatic self-calibration is enabled
pub(crate) struct SetAutomaticSelfCalibrationEnabled;
impl Command for SetAutomaticSelfCalibrationEnabled {
    type SequenceType = WriteWordSequence;

    type Input = bool;

    type Output = ();

    type SequenceInput = u16;

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x2416
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1)
    }

    fn preprocess(&self, enabled: Self::Input) -> Self::SequenceInput {
        u16::from(enabled)
    }

    fn postprocess(&self, input: Self::SequenceOutput) -> Self::Output {
        input
    }
}

/// Command for performing factory reset
pub(crate) struct PerformFactoryReset;
impl Command for PerformFactoryReset {
    type SequenceType = SendCommandSequence;

    type Input = ();

    type Output = ();

    type SequenceInput = ();

    type SequenceOutput = ();

    fn register(&self) -> u16 {
        0x3632
    }

    fn max_duration(&self) -> Duration {
        Duration::from_millis(1200)
    }

    fn preprocess(&self, input: Self::Input) -> Self::SequenceInput {
        input
    }

    fn postprocess(&self, input: Self::SequenceOutput) -> Self::Output {
        input
    }
}
